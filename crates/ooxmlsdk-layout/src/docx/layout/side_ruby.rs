//! A right-side phonetic guide is a separate, mixed-orientation text column.
use super::*;

#[derive(Clone, Copy, Debug)]
pub(super) struct GuidePlacement {
  base_item_offset: usize,
  base_center_pt: f32,
  inline_offset_pt: f32,
  cross_offset_pt: f32,
  rotated: bool,
}

#[derive(Clone, Copy, Default)]
struct ColumnMetrics {
  height: f32,
  below: f32,
}

fn column_metrics(ruby: &RubyInline, text_metrics: &mut TextMetrics) -> ColumnMetrics {
  let mut ascent = 0.0_f32;
  let mut below = 0.0_f32;
  for run in &ruby.guide {
    let metrics = text_metrics.rotated_cell_line_vertical_metrics(Some(&run.text), &run.style);
    ascent = ascent.max(metrics.directwrite_baseline_offset_pt);
    below = below.max(metrics.line_height_pt() - metrics.directwrite_baseline_offset_pt);
  }
  ColumnMetrics {
    height: ascent + below,
    below,
  }
}

pub(super) fn metrics(
  ruby: &RubyInline,
  base_width: f32,
  base_height: f32,
  text_metrics: &mut TextMetrics,
) -> CompoundTextMetrics {
  let column = column_metrics(ruby, text_metrics);
  // Native carrier-size/face opposites retain the trailing fifth of the
  // carrier's nominal cell. Its width is not the widest horizontally scaled
  // guide glyph. LO's SwRubyPortion likewise reserves a rotated guide line.
  CompoundTextMetrics {
    width_pt: base_width + (column.height - ruby.container_font_size_pt / 5.0).max(0.0),
    height_pt: base_height.max(text_runs_width(&ruby.guide, text_metrics)),
    overstrike_extents: None,
  }
}

pub(super) fn push_guides(
  page: &mut Page,
  paragraph: &crate::docx::Paragraph,
  ruby: &RubyInline,
  base_start: usize,
  placement: TextPlacement,
  segmentation: TextSegmentation,
  text_metrics: &mut TextMetrics,
) {
  if base_start == page.items.len() {
    return;
  }
  let column = column_metrics(ruby, text_metrics);
  let x = placement.x_pt + text_runs_width(&ruby.base, text_metrics)
    - ruby.container_font_size_pt / 5.0
    + column.below;
  let (mut ascent, mut descent) = (0.0_f32, 0.0_f32);
  for run in &ruby.base {
    let font = text_metrics.vertical_metrics_for_text(&run.text, &run.style);
    ascent = ascent.max(font.ascent_pt);
    descent = descent.max(font.descent_pt);
  }
  let base_center = (ascent - descent) / 2.0;
  let mut inline_offset = -text_runs_width(&ruby.guide, text_metrics) / 2.0;
  for run in &ruby.guide {
    let segments = shape_text_column_segments(&run.text, &run.style, true, text_metrics);
    for segment in segments {
      let text = &run.text[segment.byte_start..segment.byte_end];
      let advance = text_metrics.measure_text(text, &run.style);
      let mut style = run.style.clone();
      style.open_type_features.vertical_feature = match segment.orientation {
        VerticalGraphemeOrientation::Upright(feature) => feature,
        VerticalGraphemeOrientation::Rotated => None,
      };
      let (cross_offset, origin_offset, rotated) = match segment.orientation {
        VerticalGraphemeOrientation::Upright(_) => {
          let natural_origin = text_metrics.word_upright_glyph_offset(text, &style);
          if let Some(paint) = text_metrics.word_upright_paint_style(text, &style) {
            style = paint;
          }
          let paint_origin = text_metrics.word_upright_glyph_offset(text, &style);
          match (natural_origin, paint_origin) {
            (Some((cross, _)), Some((_, inline))) => (cross, inline, false),
            _ => (0.0, 0.0, true),
          }
        }
        VerticalGraphemeOrientation::Rotated => (0.0, 0.0, true),
      };
      if rotated {
        style.rotation_deg += 90.0;
      } else {
        // The logical guide advance is already on page Y. These horizontal
        // layout arrays must not be applied to the upright glyph again.
        style.semantic_character_advances_pt = None;
        style.wordprocessing_justification_expansion_pt = None;
        style.cjk_punctuation_compression_ratio = 0.0;
        style.wordprocessingml_punctuation_spacing = false;
      }
      let item_index = page.items.len();
      push_ruby_text(
        page,
        paragraph,
        run,
        text.to_string(),
        style,
        TextPlacement {
          x_pt: x,
          y_pt: placement.y_pt,
          line_height_pt: column.height,
        },
        segmentation,
      );
      if let Some(PageItem::Text(item)) = page.items.get_mut(item_index) {
        item.line_metrics_participant = false;
        item.side_ruby = Some(GuidePlacement {
          base_item_offset: item_index - base_start,
          base_center_pt: base_center,
          inline_offset_pt: inline_offset + origin_offset,
          cross_offset_pt: cross_offset,
          rotated,
        });
      }
      inline_offset += advance;
    }
  }
}

fn base_baseline(
  base: &TextItem,
  flow: FlowContext,
  metrics: Option<WordprocessingLineMetrics>,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if let Some(baseline) = base
    .wordprocessing_line_metrics
    .and_then(|line| common::wordprocessing_device::line_baseline(line, base.y_pt))
  {
    return baseline - base.style.baseline_shift_pt;
  }
  if base.origin_is_baseline
    || (flow.text_segmentation == TextSegmentation::TableCell
      && flow.layout_cell_bounds.is_some()
      && flow.horizontal_table_cell
      && !flow.table_cell_cursor_is_line_top)
  {
    return base.y_pt - base.style.baseline_shift_pt;
  }
  metrics.map_or_else(
    || section_body_paint_baseline(base, text_metrics),
    |line| {
      base.y_pt + line.paint_baseline_offset_pt(base.line_height_pt) - base.style.baseline_shift_pt
    },
  )
}

fn realized_line_metrics(
  items: &[PageItem],
  text_metrics: &mut TextMetrics,
) -> Option<WordprocessingLineMetrics> {
  // Share one completed line calculation across all guides on that line.
  // Ordinary body lines never enter this font-metric path.
  items
    .iter()
    .filter_map(|item| {
      let PageItem::Text(text) = item else {
        return None;
      };
      if !text.line_metrics_participant || text.wordprocessing_effect_host.is_some() {
        return None;
      }
      let mut text = text.clone();
      crate::docx::quantize_word_fixed_output_text_style(&mut text.style);
      wordprocessing_line_metrics(&PageItem::Text(text), text_metrics)
    })
    .reduce(WordprocessingLineMetrics::include)
}

pub(super) fn materialize(
  items: &mut [PageItem],
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) {
  if !items
    .iter()
    .any(|item| matches!(item, PageItem::Text(text) if text.side_ruby.is_some()))
  {
    return;
  }
  let metrics = realized_line_metrics(items, text_metrics);
  for index in 0..items.len() {
    let PageItem::Text(text) = &items[index] else {
      continue;
    };
    let Some(guide) = text.side_ruby else {
      continue;
    };
    let Some(base_index) = index.checked_sub(guide.base_item_offset) else {
      continue;
    };
    let Some(PageItem::Text(base)) = items.get(base_index) else {
      continue;
    };
    let baseline = base_baseline(base, flow, metrics, text_metrics);
    let PageItem::Text(text) = &mut items[index] else {
      continue;
    };
    text.x_pt += guide.cross_offset_pt;
    text.y_pt = baseline - guide.base_center_pt + guide.inline_offset_pt;
    text.origin_is_baseline = true;
    text.wordprocessing_line_metrics = None;
    text.side_ruby = None;
    if guide.rotated {
      text.rotation_center_pt = Some((text.x_pt, text.y_pt));
    }
  }
}
