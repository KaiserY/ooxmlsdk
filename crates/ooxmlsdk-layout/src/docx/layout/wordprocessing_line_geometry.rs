//! Preserve a completed Word line's source geometry until fixed-output paint.
use super::*;

pub(super) struct Placement {
  pub frame_width_pt: f64,
  pub cell_baseline_reference_pt: Option<f32>,
  pub y: f32,
  pub height: f32,
  pub before: f32,
  pub after: f32,
  pub text_extents: WordLineTextExtents,
}

#[derive(Debug, PartialEq)]
struct LineBox {
  height: f32,
  below: f32,
  before: f32,
  after: f32,
}

impl LineBox {
  fn exact(height: f32, before: f32, after: f32) -> Option<Self> {
    // Native Exact-line inputs retain independently converted components.
    // For example, 11.15pt + .15pt is 45670 + 614 ideal units, not the
    // rounded conversion of their sum (46285). The fixed baseline leaves
    // one fifth of the authored line below it, independent of the font.
    let height = word_fixed_output_reference_units(height)?;
    let before = word_fixed_output_reference_units(before)?;
    let after = word_fixed_output_reference_units(after)?;
    // Word truncates this fixed-line fraction in ideal units, before the
    // separately converted lower paragraph space. Native plain/numbered
    // 18pt controls retain14745 below, rather than rounding it to14746.
    let below = (height / 5).checked_add(after)?;
    let complete = height.checked_add(before)?.checked_add(after)?;
    if height == 0 || complete > i64::from(i32::MAX) {
      return None;
    }
    let points = |units: i64| units as f32 / 4096.0;
    Some(Self {
      height: points(complete),
      below: points(below),
      before: points(before),
      after: points(after),
    })
  }
}

pub(super) struct CellLineFrame {
  pub top: f32,
  pub height: f32,
  pub upper: f32,
  pub lower: f32,
  pub lower_border: f32,
}

/// Preserve a whole single-line cell's independently converted source box.
/// Flow keeps its existing cursor; this endpoint belongs only to paint.
pub(super) fn cell_line_bottom(
  cell: &TableCell,
  measured_height: f32,
  frame: CellLineFrame,
) -> Option<f32> {
  let [Block::Paragraph(paragraph)] = cell.blocks.as_slice() else {
    return None;
  };
  let format = &paragraph.format;
  if !matches!(format.line_height_rule, LineHeightRule::Exact)
    || format.vertical_text_flow.is_some()
    || format
      .spacing_before_lines
      .is_some_and(|value| value != 0.0)
    || format.spacing_after_lines.is_some_and(|value| value != 0.0)
    || format.spacing_before_auto == Some(true)
    || format.spacing_after_auto == Some(true)
    || paragraph.list_label_image.is_some()
    || paragraph
      .inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::PageBreak))
    || !paragraph
      .inlines
      .iter()
      .all(InlineItem::leaves_host_line_metrics_text_owned)
  {
    return None;
  }
  let height = format.line_height_pt?;
  let expected_height = cell.margins.top_pt
    + format.spacing_before_pt
    + height
    + format.spacing_after_pt
    + cell.margins.bottom_pt;
  // The measurement must describe exactly one authored text box. Admit only
  // f32 accumulation error, not an extra line, border, object or grid reserve.
  let error = f32::EPSILON * measured_height.abs().max(expected_height.abs()).max(1.0) * 8.0;
  if !measured_height.is_finite() || (measured_height - expected_height).abs() > error {
    return None;
  }
  let line = LineBox::exact(height, format.spacing_before_pt, format.spacing_after_pt)?;
  let units = |points| {
    word_fixed_output_reference_units(points).filter(|value| *value <= i64::from(i32::MAX))
  };
  let line_height = units(line.height)?;
  let top = units(frame.top)?;
  let upper = units(frame.upper)?;
  let lower = units(frame.lower)?;
  let available = units(frame.height)? - upper - lower;
  // Native fixed-cell controls retain the integer half of the available
  // source space. Rounding a floating half can move the complete bottom by
  // one ideal unit, enough to cross a later 600-DPI half-pixel boundary.
  let offset = match cell.vertical_alignment {
    TableCellVerticalAlignment::Top => 0,
    TableCellVerticalAlignment::Center => {
      (available - units(frame.lower_border)? - line_height).max(0) / 2
    }
    TableCellVerticalAlignment::Bottom => (available - line_height).max(0),
  };
  let bottom = top
    .checked_add(upper)?
    .checked_add(offset)?
    .checked_add(line_height)?;
  (bottom <= i64::from(i32::MAX)).then_some(bottom as f32 / 4096.0)
}

fn bind_text_metrics(
  items: &mut [PageItem],
  flow: FlowContext,
  placement: &Placement,
  bottom: f32,
  line: LineBox,
) {
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
      height_pt: line.height,
      baseline_from_bottom_pt: line.below,
      spacing_before_pt: line.before,
      spacing_after_pt: line.after,
    });
  }
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
  wordprocessing_auto_gap::materialize(items, &paragraph.format, flow, text_metrics);
  // Native Line Services receives the complete line box for natural,
  // minimum-height and ordinary fixed-height text, including text numbers.
  // Story, grid and object lines keep their separate baseline contracts.
  let minimum = matches!(paragraph.format.line_height_rule, LineHeightRule::AtLeast);
  let resolved_cell = flow.text_segmentation == TextSegmentation::TableCell
    && flow.layout_cell_bounds.is_some()
    && flow.horizontal_table_cell
    && !flow.table_cell_cursor_is_line_top;
  let exact_line = matches!(paragraph.format.line_height_rule, LineHeightRule::Exact)
    && (flow.text_segmentation == TextSegmentation::Body || resolved_cell)
    && paragraph.list_label_image.is_none()
    && paragraph.format.vertical_text_flow.is_none()
    && !paragraph.format.bidi
    && placement.text_extents.has_text
    // Page-top suppression can replace the authored fixed box. Its resolved
    // first-line baseline is not the ordinary Exact-line input contract.
    && paragraph.format.line_height_pt.is_some_and(|height| {
      word_fixed_output_reference_units(height)
        == word_fixed_output_reference_units(placement.height)
    })
    && !items.iter().any(|item| {
      matches!(item, PageItem::Text(text)
        if text.origin_is_baseline
          || !matches!(text.style.line_vertical_alignment,
            common::LineVerticalAlignment::Auto | common::LineVerticalAlignment::Baseline))
    });
  // Ordinary body numbers share the completed text line's baseline. Their
  // ascent reserve is independent of proportional spacing; their descent
  // never replaces the body's. CJK gap, recovered and cell-cursor contracts
  // remain separate from these native Latin body controls.
  let numbered_body = matches!(paragraph.format.line_height_rule, LineHeightRule::Auto)
    && flow.text_segmentation == TextSegmentation::Body
    && paragraph.list_label.is_some()
    && !paragraph.format.office_recovered_line_height
    && paragraph.format.vertical_text_flow.is_none()
    && placement.text_extents.has_text
    && !paragraph.inlines.iter().any(|inline| {
      matches!(inline, InlineItem::Text(run) if run.text.chars().any(|character| {
        matches!(character.script(),
          Script::Han | Script::Hiragana | Script::Katakana | Script::Hangul
          | Script::Bopomofo | Script::Yi)
      }))
    });
  // Picture numbering uses the body font's baseline, including recovered
  // Auto spacing. A larger bitmap reserves ascent; proportional leading is
  // below that union. Native independent run/mark/font controls establish
  // this separately from textual numbering's legacy spacing contract.
  let picture_body = matches!(paragraph.format.line_height_rule, LineHeightRule::Auto)
    && flow.text_segmentation == TextSegmentation::Body
    && paragraph.list_label_image.is_some()
    && paragraph.format.vertical_text_flow.is_none()
    && placement.text_extents.has_text;
  // Flat character effects enlarge the completed line, not each foreground
  // independently. Native glow/shadow/reflection controls, with plain text
  // on either side and compressed/single/expanded auto spacing, all retain
  // one baseline. Reuse the reference-device outsets already accumulated
  // during flow; hosted/3-D/rotated effect stories retain their own owner.
  let flat_effect_line = matches!(paragraph.format.line_height_rule, LineHeightRule::Auto)
    && flow.text_segmentation == TextSegmentation::Body
    && paragraph.format.vertical_text_flow.is_none()
    && paragraph.list_label.is_none()
    && paragraph.list_label_image.is_none()
    && items.iter().any(|item| {
      matches!(item, PageItem::Text(text)
        if text.style.text_glow.is_some()
          || text.style.text_shadow.is_some()
          || text.style.text_reflection.is_some())
    })
    && items.iter().all(|item| match item {
      PageItem::Text(text) => {
        text.style.use_windows_font_metrics
          && text.style.rotation_deg == 0.0
          && !text.origin_is_baseline
          && text.wordprocessing_effect_host.is_none()
          && text.style.wordprocessing_text_3d_parts.is_none()
          && text.style.drawingml_text_static3d.is_none()
          && text.style.drawingml_text_effects.is_none()
          && text.style.pdf_glyph_outline_options.is_none()
          && matches!(
            text.style.line_vertical_alignment,
            common::LineVerticalAlignment::Auto | common::LineVerticalAlignment::Baseline
          )
      }
      _ => true,
    });
  let text_owned = if minimum || numbered_body || picture_body || exact_line {
    (paragraph.list_label_image.is_none() || picture_body)
      && paragraph
        .inlines
        .iter()
        .all(InlineItem::leaves_host_line_metrics_text_owned)
  } else {
    paragraph_line_spacing_base_covers_line_height(paragraph)
  };
  if (!matches!(paragraph.format.line_height_rule, LineHeightRule::Auto | LineHeightRule::AtLeast)
    && !exact_line)
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
    // Hosted and transformed effects still have a separate surface owner.
    || (!flat_effect_line && items.iter().any(|item| {
      matches!(item, PageItem::Text(text)
        if text.style.text_glow.is_some()
          || text.style.text_shadow.is_some()
          || text.style.text_reflection.is_some()
          || text.style.wordprocessing_text_3d_parts.is_some()
          || text.style.drawingml_text_effects.is_some()
          || text.style.pdf_glyph_outline_options.is_some())
    }))
  {
    return;
  }
  if exact_line {
    // A fixed line owns the number's baseline as well as the body font's.
    // Native 4/12pt bodies and4/24pt labels receive the same completed box.
    // Preserve each provisional origin and bind its distance to that box;
    // realigning the number first would alter PDF's inferred font union.
    if let Some(line) = LineBox::exact(placement.height, placement.before, placement.after) {
      // Real horizontal cells retain a resolved baseline cursor. Native
      // 34e78 controls use the same complete fixed box as body text, after
      // removing its provisional 80% ascent. Convert the complete endpoints
      // at paint instead of independently snapping that provisional baseline.
      let cursor_offset = if resolved_cell {
        placement.height * WORD_FIXED_LINE_ASCENT_RATIO
      } else {
        0.0
      };
      let bottom = if resolved_cell {
        flow.table_cell_exact_line_bottom_pt
      } else {
        None
      }
      .unwrap_or(placement.y - cursor_offset + placement.height + placement.after);
      bind_text_metrics(items, flow, &placement, bottom, line);
    }
    return;
  }
  let metrics = if flat_effect_line {
    Some(WordprocessingLineMetrics {
      ascent_pt: placement
        .text_extents
        .ascent_pt
        .max(placement.text_extents.effect_ascent_pt),
      descent_pt: placement
        .text_extents
        .descent_pt
        .max(placement.text_extents.effect_descent_pt),
      top_aligned: false,
    })
  } else if numbered_body || picture_body {
    Some(WordprocessingLineMetrics {
      ascent_pt: placement.text_extents.font_ascent_pt,
      descent_pt: placement.text_extents.font_descent_pt,
      top_aligned: false,
    })
  } else {
    items
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
  };
  let Some(metrics) = metrics else {
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
  let numbering_ascent = if picture_body {
    paragraph.list_label_image.as_ref().map_or(0.0, |label| {
      let has_marker = items
        .iter()
        .any(|item| picture_numbering_image(item, &label.image).is_some());
      if has_marker {
        let image = numbering_image_metrics(label, flow.content_width, &paragraph.list_label_style);
        (inline_image_ascent(image) - metrics.ascent_pt).max(0.0)
      } else {
        0.0
      }
    })
  } else if numbered_body {
    placement.text_extents.numbering.map_or(0.0, |numbering| {
      (numbering.ascent_pt - placement.text_extents.font_ascent_pt).max(0.0)
    })
  } else {
    0.0
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
    scaled(natural) + numbering_ascent
  };
  let extra = (placement.height - natural).max(0.0);
  // Word's minimum-line controls retain the natural descent and place all
  // excess above the font box. Auto multiples instead place it below.
  let leading_above = if minimum { extra } else { 0.0 };
  let leading_below = if minimum { 0.0 } else { extra };
  let ascent = if compressed_legacy_reference {
    natural - scaled(metrics.descent_pt)
  } else {
    scaled(metrics.ascent_pt) + numbering_ascent + leading_above
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
        // A mixed-size cell line can still carry the first run's provisional
        // baseline, plus its source-only hard-break coordinate conversion.
        // Its common paint ascent is resolved above from the complete line.
        // Keep this correction in paint geometry: native NEXT/AFTER controls
        // establish that line advance and row height were already correct.
        placement.cell_baseline_reference_pt.unwrap_or(ascent)
      }
    } else {
      0.0
    };
  let bottom = placement.y - cursor_offset + placement.height + placement.after;
  bind_text_metrics(
    items,
    flow,
    &placement,
    bottom,
    LineBox {
      height: placement.before + placement.height + placement.after,
      below: placement.height - ascent + placement.after,
      before: placement.before + leading_above,
      after: placement.after + leading_below,
    },
  );
  if picture_body
    && let Some(label) = paragraph.list_label_image.as_ref()
    && let Some(baseline) = items.iter().find_map(|item| {
      let PageItem::Text(text) = item else {
        return None;
      };
      common::wordprocessing_device::line_baseline(text.wordprocessing_line_metrics?, text.y_pt)
    })
  {
    for item in items {
      realize_picture_numbering(
        item,
        &label.image,
        f64::from(flow.content_left_pt),
        baseline,
      );
    }
  }
}

fn picture_numbering_image<'a>(
  item: &'a PageItem,
  source: &crate::docx::InlineImage,
) -> Option<&'a ImageItem> {
  let PageItem::InlineObjectGroup(parts) = item else {
    return None;
  };
  parts.iter().find_map(|part| {
    let PageItem::Image(image) = part else {
      return None;
    };
    (image.data == source.data).then_some(image)
  })
}

fn realize_picture_numbering(
  item: &mut PageItem,
  source: &crate::docx::InlineImage,
  frame_x: f64,
  baseline: f32,
) {
  // Paint-only resized surfaces retain their separate host contract.
  if source.picture_paint_size_pt.is_some() {
    return;
  }
  let Some(mut image) = picture_numbering_image(item, source).cloned() else {
    return;
  };
  let device = wordprocessing_table_device::source_coordinate_precise;
  let x = device(frame_x) + device(f64::from(image.x_pt) - frame_x);
  let width = device(f64::from(image.width_pt));
  let height = device(f64::from(image.height_pt));
  let y = device(f64::from(baseline)) - height - device(f64::from(image.inline_baseline_gap_pt));
  (image.x_pt, image.width_pt) = crate::docx::picture_bullet::paint_axis(x, width);
  (image.y_pt, image.height_pt) = crate::docx::picture_bullet::paint_axis(y, height);
  // Regenerate from the original host, preserving fill/stroke/clipping and
  // effects against the same realized bitmap box rather than moving only its
  // pixels. There are no other inline object stories in this text-owned lane.
  let mut parts = Vec::new();
  push_docx_picture_image(&mut parts, source, image);
  *item = PageItem::InlineObjectGroup(parts);
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
    | PageItem::CompositingGroup { items, .. } => {
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
  fn exact_line_box_preserves_native_independent_components() {
    // Current Word34104/34193 readbacks, two font families and eight
    // paragraph-before phases. Font metrics do not own these fixed boxes.
    for (height, before, expected) in [
      (11.15, 0.0, [45670, 9134, 0, 0]),
      (11.15, 0.05, [45875, 9134, 205, 0]),
      (11.15, 0.15, [46284, 9134, 614, 0]),
      (11.15, 0.35, [47104, 9134, 1434, 0]),
      (11.2, 0.0, [45875, 9175, 0, 0]),
      (11.2, 0.15, [46489, 9175, 614, 0]),
      (11.2, 0.35, [47309, 9175, 1434, 0]),
      (11.55, 0.0, [47309, 9461, 0, 0]),
      (18.0, 0.0, [73728, 14745, 0, 0]),
    ] {
      let line = LineBox::exact(height, before, 0.0).expect("native fixed line");
      assert_eq!(
        [line.height, line.below, line.before, line.after].map(|pt| (pt * 4096.0) as i32),
        expected,
      );
    }
    assert!(LineBox::exact(f32::NAN, 0.0, 0.0).is_none());
    assert!(LineBox::exact(0.0, 0.0, 0.0).is_none());
    assert!(LineBox::exact(11.2, -1.0, 0.0).is_none());
  }

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
