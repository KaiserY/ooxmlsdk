use super::{
  LAYOUT_EPSILON_PT, LineJustificationContext, PageItem, TextItem, TextMetrics,
  ltr_justified_leading_space_count, rtl_justified_leading_space_count,
};
use crate::docx::JustificationWordSpacing;

struct Portion {
  index: usize,
  spaces: usize,
  fixed_visual_leading_spaces: usize,
  left_space: bool,
  right_space: bool,
  space_capacity: f32,
  note: bool,
  before: Option<f32>,
  after: Option<f32>,
}

/// Word treats automatic note marks as inline objects for expansion.
/// Both Line Services paths expand bounded word spaces before sharing residual
/// expansion with note edges. Modern Word also has a bounded edge level and
/// joins adjacent marks; legacy Word retains their interior opportunities.
pub(super) fn adjust(
  items: &mut [PageItem],
  line: LineJustificationContext,
  first_text_index: Option<usize>,
  last_text_index: Option<usize>,
  natural_right: f32,
  word_spacing: JustificationWordSpacing,
  text_metrics: &mut TextMetrics,
) -> bool {
  let extra = line.right_pt - natural_right;
  if extra <= LAYOUT_EPSILON_PT
    || !items.iter().skip(line.item_start).any(|item| {
      matches!(item, PageItem::Text(text)
        if (text.y_pt - line.y_pt).abs() < 0.01 && is_note_mark(text.hyperlink_url.as_deref()))
    })
  {
    // Contraction and ordinary superscripts retain their existing contracts.
    return false;
  }

  let legacy = word_spacing.maximum_pct <= word_spacing.desired_pct;
  let maximum_growth = f32::from(
    word_spacing
      .maximum_pct
      .saturating_sub(word_spacing.desired_pct),
  ) / 100.0;
  let mut portions = Vec::new();
  for (index, item) in items.iter().enumerate().skip(line.item_start) {
    let PageItem::Text(text) = item else {
      continue;
    };
    if (text.y_pt - line.y_pt).abs() >= 0.01 {
      continue;
    }
    // The caller may identify the last visible portion before a suffix of
    // separate LTR blank portions. None of those tails owns expansion glue.
    let visible = if last_text_index.is_some_and(|last| index >= last) {
      text.text.trim_end_matches(' ')
    } else {
      &text.text
    };
    if visible.is_empty() {
      continue;
    }
    let leading_spaces = if Some(index) == first_text_index {
      ltr_justified_leading_space_count(text) + rtl_justified_leading_space_count(text)
    } else {
      0
    };
    let spaces = visible.matches(' ').count().saturating_sub(leading_spaces);
    let rtl = text
      .style
      .resolved_bidi_level
      .map_or(text.style.right_to_left.unwrap_or(false), |level| {
        level % 2 != 0
      });
    let space_capacity = if spaces == 0 {
      0.0
    } else if legacy {
      text_metrics
        .legacy_word_space_expansion_capacity(&text.style)
        .unwrap_or_default()
    } else {
      text_metrics.measure_text(" ", &text.style) * maximum_growth
    };
    portions.push(Portion {
      index,
      spaces,
      // RTL logical line-ending blanks precede the ink in visual order.
      // They remain semantic text but must not move that ink when stretched.
      fixed_visual_leading_spaces: if rtl && Some(index) == last_text_index {
        text.text.len() - visible.len()
      } else if Some(index) == first_text_index {
        ltr_justified_leading_space_count(text)
      } else {
        0
      },
      left_space: if rtl {
        visible.ends_with(' ')
      } else {
        leading_spaces == 0 && visible.starts_with(' ')
      },
      right_space: if rtl {
        leading_spaces == 0 && visible.starts_with(' ')
      } else {
        visible.ends_with(' ')
      },
      space_capacity,
      note: is_note_mark(text.hyperlink_url.as_deref()),
      before: None,
      after: None,
    });
  }
  portions.sort_by(|a, b| {
    let PageItem::Text(left) = &items[a.index] else {
      unreachable!();
    };
    let PageItem::Text(right) = &items[b.index] else {
      unreachable!();
    };
    left.x_pt.total_cmp(&right.x_pt)
  });

  for position in 0..portions.len() {
    if !portions[position].note {
      continue;
    }
    if position > 0 && !portions[position - 1].note {
      let PageItem::Text(neighbor) = &items[portions[position - 1].index] else {
        unreachable!();
      };
      portions[position].before = Some(if legacy {
        0.0
      } else {
        note_edge_capacity(neighbor, text_metrics)
      });
      // The immediately adjacent blank owns the object-edge opportunity,
      // not a second ordinary-space opportunity. Native controls with blanks
      // before, after, and on both sides of a mark retain only one slot at
      // each boundary; an additional adjacent blank remains ordinary glue.
      // The portions are in visual order, while their strings remain in
      // logical order. An RTL neighbor's logical start touches this edge.
      if portions[position - 1].right_space {
        portions[position - 1].spaces = portions[position - 1].spaces.saturating_sub(1);
      }
    } else if legacy && position > 0 {
      // Native compatibility 12/14 retains one opportunity between distinct
      // automatic marks. The modern path treats them as a single object.
      portions[position].before = Some(0.0);
    }
    if position + 1 < portions.len() && !portions[position + 1].note {
      let PageItem::Text(neighbor) = &items[portions[position + 1].index] else {
        unreachable!();
      };
      portions[position].after = Some(if legacy {
        0.0
      } else {
        note_edge_capacity(neighbor, text_metrics)
      });
      if portions[position + 1].left_space {
        portions[position + 1].spaces = portions[position + 1].spaces.saturating_sub(1);
        portions[position + 1].fixed_visual_leading_spaces += 1;
      }
    }
  }
  let edge_count = portions
    .iter()
    .map(|portion| usize::from(portion.before.is_some()) + usize::from(portion.after.is_some()))
    .sum::<usize>();
  if edge_count == 0 {
    return false;
  }

  let spaces = portions.iter().map(|portion| portion.spaces).sum::<usize>();
  let space_capacity = portions
    .iter()
    .map(|portion| portion.space_capacity * portion.spaces as f32)
    .sum::<f32>();
  let edge_capacity = portions
    .iter()
    .map(|portion| portion.before.unwrap_or_default() + portion.after.unwrap_or_default())
    .sum::<f32>();
  let space_used = extra.min(space_capacity);
  let edge_used = (extra - space_used).min(edge_capacity);
  // Line Services apportions a bounded level in proportion to its capacities.
  // Its last level has the same limit (the column width) at every opportunity,
  // hence divides the remaining width equally between spaces and object edges.
  let space_ratio = if space_capacity > 0.0 {
    space_used / space_capacity
  } else {
    0.0
  };
  let edge_ratio = if edge_capacity > 0.0 {
    edge_used / edge_capacity
  } else {
    0.0
  };
  let residual = (extra - space_used - edge_used).max(0.0) / (spaces + edge_count) as f32;

  let mut preceding_advance = 0.0;
  for portion in portions {
    let PageItem::Text(text) = &mut items[portion.index] else {
      unreachable!();
    };
    if let Some(capacity) = portion.before {
      preceding_advance += capacity * edge_ratio + residual;
    }
    let space_advance = if portion.spaces == 0 {
      0.0
    } else {
      portion.space_capacity * space_ratio + residual
    };
    text.x_pt += preceding_advance;
    text.word_spacing_pt = space_advance;
    // The PDF portion has one word-spacing value. Cancel it before fixed
    // visual leading blanks, preserving expansion of all interior blanks.
    text.x_pt -= portion.fixed_visual_leading_spaces as f32 * space_advance;
    preceding_advance += portion.spaces as f32 * space_advance;
    if let Some(capacity) = portion.after {
      preceding_advance += capacity * edge_ratio + residual;
    }
  }
  true
}

fn is_note_mark(url: Option<&str>) -> bool {
  url.is_some_and(|url| {
    [
      "ooxmlsdk-pdf:footnote-reference:",
      "ooxmlsdk-pdf:endnote-reference:",
      "ooxmlsdk-pdf:footnote-backlink:",
      "ooxmlsdk-pdf:endnote-backlink:",
    ]
    .iter()
    .any(|prefix| url.starts_with(prefix))
  })
}

fn note_edge_capacity(text: &TextItem, text_metrics: &mut TextMetrics) -> f32 {
  // The neighboring text face owns this hint, not the reduced marker face.
  // Native Times 10/12/16pt and Arial 12pt controls give 5% of its space advance
  // (within 0.002pt of the producer's quantized hint). This bounded hint is
  // separate from the much larger residual expansion; it is not a gap added
  // to natural layout and never changes wrapping or the PDF text payload.
  text_metrics.measure_text(" ", &text.style) * 0.05
}

#[cfg(test)]
mod tests {
  use super::super::PdfTextSegmentation;
  use super::*;
  use crate::common::OpenTypeLigatures;
  use crate::model::TextStyle;
  use std::sync::Arc;

  fn style(size: f32) -> TextStyle {
    TextStyle {
      font_family: Some(Arc::from("Times New Roman")),
      high_ansi_font_family: Some(Arc::from("Times New Roman")),
      font_size_pt: size,
      kerning_minimum_size_pt: Some(100.0),
      ligatures: Some(OpenTypeLigatures::default()),
      use_windows_font_metrics: true,
      ..Default::default()
    }
  }

  fn portions(texts: &[(&str, Option<&str>)], metrics: &mut TextMetrics) -> Vec<PageItem> {
    let mut x = 85.05;
    texts
      .iter()
      .map(|&(text, url)| {
        let style = style(if url.is_some() { 8.0 } else { 12.0 });
        let width = metrics.measure_text(text, &style);
        let item = PageItem::Text(Box::new(TextItem {
          x_pt: x,
          y_pt: 20.0,
          line_height_pt: 14.0,
          wordprocessing_auto_line_spacing_units: None,
          wordprocessing_line_metrics: None,
          origin_is_baseline: false,
          line_metrics_participant: true,
          paint_clip: None,
          wordprocessing_effect_host: None,
          wordprocessing_terminal_effect_style: None,
          text: text.into(),
          style,
          rotation_center_pt: None,
          hyperlink_url: url.map(str::to_string),
          dynamic_field: None,
          dynamic_field_line_anchor: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          form_widget_id: None,
          paragraph_bidi: false,
          word_spacing_pt: 0.0,
          preserve_text_portion: true,
          decoration_span_start_x_pt: None,
          pdf_text_segmentation: PdfTextSegmentation::Portion,
        }));
        x += width;
        item
      })
      .collect()
  }

  fn text(item: &PageItem) -> &TextItem {
    let PageItem::Text(text) = item else {
      unreachable!();
    };
    text
  }

  fn apply(items: &mut [PageItem], extra: f32, metrics: &mut TextMetrics) -> bool {
    apply_with_spacing(
      items,
      extra,
      JustificationWordSpacing {
        desired_pct: 100,
        minimum_pct: 75,
        maximum_pct: 150,
      },
      metrics,
    )
  }

  fn apply_with_spacing(
    items: &mut [PageItem],
    extra: f32,
    word_spacing: JustificationWordSpacing,
    metrics: &mut TextMetrics,
  ) -> bool {
    let last = items.len() - 1;
    let tail = text(&items[last]);
    let natural_right =
      tail.x_pt + metrics.measure_text(tail.text.trim_end_matches(' '), &tail.style);
    adjust(
      items,
      LineJustificationContext {
        item_start: 0,
        y_pt: 20.0,
        left_pt: 85.05,
        right_pt: natural_right + extra,
        terminal_overhang_pt: 0.0,
        source_frame_right_pt: None,
        preserve_foreign_tail: false,
      },
      Some(0),
      Some(last),
      natural_right,
      word_spacing,
      metrics,
    )
  }

  fn apply_legacy(items: &mut [PageItem], extra: f32, metrics: &mut TextMetrics) -> bool {
    apply_with_spacing(items, extra, JustificationWordSpacing::default(), metrics)
  }

  fn rtl_portions(texts: &[(&str, Option<&str>)], metrics: &mut TextMetrics) -> Vec<PageItem> {
    let mut items = portions(texts, metrics);
    let mut x = 85.05;
    for item in items.iter_mut().rev() {
      let text = text_mut(item);
      text.paragraph_bidi = true;
      let rtl = text.hyperlink_url.is_none();
      text.style.right_to_left = Some(rtl);
      text.style.resolved_bidi_level = Some(if rtl { 1 } else { 2 });
      text.x_pt = x;
      x += metrics.measure_text(&text.text, &text.style);
    }
    items
  }

  fn apply_rtl(
    items: &mut [PageItem],
    extra: f32,
    spacing: JustificationWordSpacing,
    metrics: &mut TextMetrics,
  ) -> f32 {
    let last = items.len() - 1;
    let tail = text(&items[last]);
    let tail_width = metrics.measure_text(&tail.text, &tail.style)
      - metrics.measure_text(tail.text.trim_end_matches(' '), &tail.style);
    let natural_right = items
      .iter()
      .map(|item| {
        let text = text(item);
        text.x_pt + metrics.measure_text(&text.text, &text.style)
      })
      .fold(85.05, f32::max)
      - tail_width;
    assert!(adjust(
      items,
      LineJustificationContext {
        item_start: 0,
        y_pt: 20.0,
        left_pt: 85.05,
        right_pt: natural_right + extra,
        terminal_overhang_pt: 0.0,
        source_frame_right_pt: None,
        preserve_foreign_tail: false,
      },
      Some(0),
      Some(last),
      natural_right,
      spacing,
      metrics,
    ));
    tail_width
  }

  #[test]
  fn rtl_terminal_blanks_preserve_visible_origins_and_note_expansion() {
    let mut metrics = TextMetrics::new();
    // Native legacy/modern controls retain the same painted line when zero,
    // one or two logical line-ending blanks are preserved in the source.
    for spacing in [
      JustificationWordSpacing::default(),
      JustificationWordSpacing {
        desired_pct: 100,
        minimum_pct: 75,
        maximum_pct: 150,
      },
    ] {
      let mut reference: Option<[f32; 4]> = None;
      for trailing in 0..=2 {
        let suffix = format!("D E F{}", " ".repeat(trailing));
        let mut items = rtl_portions(
          &[
            ("A B C", None),
            ("3", Some("ooxmlsdk-pdf:footnote-reference:42")),
            (&suffix, None),
          ],
          &mut metrics,
        );
        let tail_width = apply_rtl(&mut items, 40.0, spacing, &mut metrics);
        let tail = text(&items[2]);
        // Physical RTL alignment removes the natural trailing-blank width.
        // Its additional justification must cancel before the first ink too.
        let origins = [
          tail.x_pt + trailing as f32 * tail.word_spacing_pt,
          text(&items[1]).x_pt - tail_width,
          text(&items[0]).x_pt - tail_width,
          tail.word_spacing_pt,
        ];
        if let Some(reference) = reference {
          for (actual, expected) in origins.into_iter().zip(reference) {
            assert!((actual - expected).abs() < 0.001);
          }
        } else {
          reference = Some(origins);
        }
        assert_eq!(tail.text, suffix);
      }
    }
  }

  #[test]
  fn rtl_note_boundary_blanks_use_visual_edges() {
    let mut metrics = TextMetrics::new();
    let capacity = 7416.0 / 4096.0;
    for before in 0usize..=2 {
      for after in 0usize..=2 {
        let prefix = format!("AB CD{}", " ".repeat(before));
        let suffix = format!("{}EF GH IJ", " ".repeat(after));
        let mut items = rtl_portions(
          &[
            (&prefix, None),
            ("3", Some("ooxmlsdk-pdf:footnote-reference:42")),
            (&suffix, None),
          ],
          &mut metrics,
        );
        let marker_x = text(&items[1]).x_pt;
        let prefix_x = text(&items[0]).x_pt;
        let left_spaces = 2 + after.saturating_sub(1);
        let spaces = 3 + before.saturating_sub(1) + after.saturating_sub(1);
        let edge = (40.0 - spaces as f32 * capacity) / (spaces + 2) as f32;
        let space = capacity + edge;
        apply_rtl(
          &mut items,
          40.0,
          JustificationWordSpacing::default(),
          &mut metrics,
        );
        assert!((text(&items[2]).word_spacing_pt - space).abs() < 0.001);
        assert!(
          (text(&items[1]).x_pt - marker_x - left_spaces as f32 * space - edge).abs() < 0.001
        );
        let fixed_boundary = if before > 0 { space } else { 0.0 };
        assert!(
          (text(&items[0]).x_pt - prefix_x - left_spaces as f32 * space - 2.0 * edge
            + fixed_boundary)
            .abs()
            < 0.001
        );
        assert_eq!(text(&items[0]).text, prefix);
        assert_eq!(text(&items[2]).text, suffix);
      }
    }
  }

  #[test]
  fn legacy_bounded_spaces_use_the_selected_font_average_width() {
    let mut metrics = TextMetrics::new();
    // Actual native compatibility 12/14 first-level hints, in 1/4096pt units.
    // Font averages/space advances are Times 821/512 and Arial 904/569 FU.
    for (family, size, native_capacity) in [
      ("Times New Roman", 10.0, 6180.0),
      ("Times New Roman", 12.0, 7416.0),
      ("Times New Roman", 16.0, 9888.0),
      ("Arial", 12.0, 8040.0),
    ] {
      let mut ordinary = style(size);
      ordinary.font_family = Some(Arc::from(family));
      ordinary.high_ansi_font_family = Some(Arc::from(family));
      let capacity = metrics
        .legacy_word_space_expansion_capacity(&ordinary)
        .unwrap();
      assert!((capacity - native_capacity / 4096.0).abs() < 0.0001);
    }
  }

  #[test]
  fn legacy_native_bounded_and_residual_levels_keep_note_edges_separate() {
    let mut metrics = TextMetrics::new();
    let capacity = 7416.0 / 4096.0;
    // Independent 252/260/268/270/275/300pt and +1pt native width controls.
    for extra in [
      2.9921875, 10.9921875, 18.992188, 20.992188, 25.992188, 50.992188, 204.59229, 205.59229,
    ] {
      let mut items = portions(
        &[
          ("AB CD", None),
          ("12", Some("ooxmlsdk-pdf:footnote-reference:12")),
          ("EF GH IJ KL MN OP QR ST UV WX YZ", None),
        ],
        &mut metrics,
      );
      let marker_x = text(&items[1]).x_pt;
      let following_x = text(&items[2]).x_pt;
      let (space, edge) = if extra <= 11.0 * capacity {
        (extra / 11.0, 0.0)
      } else {
        let edge = (extra - 11.0 * capacity) / 13.0;
        (capacity + edge, edge)
      };
      assert!(apply_legacy(&mut items, extra, &mut metrics));
      assert!((text(&items[0]).word_spacing_pt - space).abs() < 0.001);
      assert!((text(&items[1]).x_pt - marker_x - space - edge).abs() < 0.001);
      assert!((text(&items[2]).x_pt - following_x - space - 2.0 * edge).abs() < 0.001);
    }
  }

  #[test]
  fn legacy_consecutive_marks_retain_the_native_interior_opportunity() {
    let mut metrics = TextMetrics::new();
    let mut items = portions(
      &[
        ("AB CD", None),
        ("21", Some("ooxmlsdk-pdf:footnote-reference:21")),
        ("22", Some("ooxmlsdk-pdf:footnote-reference:22")),
        ("EF GH IJ KL MN OP QR ST UV WX YZ", None),
      ],
      &mut metrics,
    );
    let old_distance = text(&items[2]).x_pt - text(&items[1]).x_pt;
    let extra = 805242.0 / 4096.0;
    let native_edge = 51690.0 / 4096.0;
    assert!(apply_legacy(&mut items, extra, &mut metrics));
    assert!(
      (text(&items[2]).x_pt - text(&items[1]).x_pt - old_distance - native_edge).abs() < 0.001
    );
  }

  #[test]
  fn legacy_adjacent_blank_owns_one_edge_with_unchanged_text() {
    let mut metrics = TextMetrics::new();
    for (prefix, suffix) in [
      ("AB CD", "EF GH IJ KL MN OP QR ST UV WX YZ"),
      ("AB CD ", "EF GH IJ KL MN OP QR ST UV WX YZ"),
      ("AB CD", " EF GH IJ KL MN OP QR ST UV WX YZ"),
      ("AB CD ", " EF GH IJ KL MN OP QR ST UV WX YZ"),
    ] {
      let mut items = portions(
        &[
          (prefix, None),
          ("12", Some("ooxmlsdk-pdf:footnote-reference:12")),
          (suffix, None),
        ],
        &mut metrics,
      );
      let marker_x = text(&items[1]).x_pt;
      let following_x = text(&items[2]).x_pt;
      let edge = (40.0 - 11.0 * 7416.0 / 4096.0) / 13.0;
      let space = edge + 7416.0 / 4096.0;
      assert!(apply_legacy(&mut items, 40.0, &mut metrics));
      assert!((text(&items[1]).x_pt - marker_x - space - edge).abs() < 0.001);
      let fixed_blank = if suffix.starts_with(' ') { space } else { 0.0 };
      assert!(
        (text(&items[2]).x_pt - following_x - space - 2.0 * edge + fixed_blank).abs() < 0.001
      );
      assert_eq!(text(&items[0]).text, prefix);
      assert_eq!(text(&items[2]).text, suffix);
    }
  }

  #[test]
  fn native_note_reference_uses_two_residual_opportunities() {
    let mut metrics = TextMetrics::new();
    let mut items = portions(
      &[
        ("1\u{a0}996\u{a0}085,00", None),
        (" рублей", None),
        ("4", Some("ooxmlsdk-pdf:footnote-reference:4")),
        (", при этом ", None),
        ("исправление некачественно ", None),
        ("выполненных работ", None),
      ],
      &mut metrics,
    );
    let marker_x = text(&items[2]).x_pt;
    let following_x = text(&items[3]).x_pt;
    // Actual MSLS70 level-0/2/4 allocations: seven spaces and two note edges.
    // These widths also bracket the independent +1pt native capture.
    for (extra, native_space, native_edge) in [
      (59.239014, 6.881836, 5.532471),
      (60.239014, 6.99292, 5.643555),
    ] {
      let mut actual = items.clone();
      assert!(apply(&mut actual, extra, &mut metrics));
      assert!((text(&actual[1]).word_spacing_pt - native_space).abs() < 0.002);
      assert!((text(&actual[2]).x_pt - marker_x - native_space - native_edge).abs() < 0.002);
      assert!(
        (text(&actual[3]).x_pt - following_x - native_space - 2.0 * native_edge).abs() < 0.002
      );
      assert_eq!(text(&actual[2]).text, "4");
    }
    text_mut(&mut items[2]).hyperlink_url = None;
    assert!(!apply(&mut items, 59.239014, &mut metrics));
  }

  fn text_mut(item: &mut PageItem) -> &mut TextItem {
    let PageItem::Text(text) = item else {
      unreachable!();
    };
    text
  }

  #[test]
  fn limited_levels_are_consumed_before_residual_expansion() {
    let mut metrics = TextMetrics::new();
    for (extra, space, edge) in [(3.0, 1.0, 0.0), (4.65, 1.5, 0.075), (5.0, 1.54, 0.19)] {
      let mut items = portions(
        &[
          ("A B C", None),
          ("1", Some("ooxmlsdk-pdf:footnote-reference:4")),
          ("D E ", None),
        ],
        &mut metrics,
      );
      let marker_x = text(&items[1]).x_pt;
      assert!(apply(&mut items, extra, &mut metrics));
      assert!((text(&items[0]).word_spacing_pt - space).abs() < 0.001);
      assert!((text(&items[1]).x_pt - marker_x - 2.0 * space - edge).abs() < 0.001);
    }
  }

  #[test]
  fn adjacent_blank_reuses_the_note_edge_opportunity() {
    let mut metrics = TextMetrics::new();
    // Independent native controls keep eleven word-space opportunities for
    // each of these four boundaries, plus the same two object-edge slots.
    for (prefix, suffix) in [
      ("AB CD", "EF GH IJ KL MN OP QR ST UV WX YZ"),
      ("AB CD ", "EF GH IJ KL MN OP QR ST UV WX YZ"),
      ("AB CD", " EF GH IJ KL MN OP QR ST UV WX YZ"),
      ("AB CD ", " EF GH IJ KL MN OP QR ST UV WX YZ"),
    ] {
      let mut items = portions(
        &[
          (prefix, None),
          ("1", Some("ooxmlsdk-pdf:footnote-reference:4")),
          (suffix, None),
        ],
        &mut metrics,
      );
      let marker_x = text(&items[1]).x_pt;
      let following_x = text(&items[2]).x_pt;
      assert!(apply(&mut items, 40.0, &mut metrics));
      let residual = (40.0 - 11.0 * 1.5 - 2.0 * 0.15) / 13.0;
      let space = 1.5 + residual;
      let edge = 0.15 + residual;
      assert!((text(&items[0]).word_spacing_pt - space).abs() < 0.001);
      assert!((text(&items[1]).x_pt - marker_x - space - edge).abs() < 0.001);
      let cancelled_blank = usize::from(suffix.starts_with(' ')) as f32 * space;
      assert!(
        (text(&items[2]).x_pt - following_x - space - 2.0 * edge + cancelled_blank).abs() < 0.001
      );
    }
  }

  #[test]
  fn leading_note_blank_is_fixed_across_text_portions() {
    let mut metrics = TextMetrics::new();
    for blanks in [1usize, 2] {
      let spaces = " ".repeat(blanks);
      let joined = format!("{spaces}EF GH IJ KL MN OP QR ST UV WX YZ");
      for split in [false, true] {
        let marker = ("1", Some("ooxmlsdk-pdf:footnote-backlink:4"));
        let mut items = if split {
          portions(
            &[
              marker,
              (&spaces, None),
              ("EF GH IJ KL MN OP QR ST UV WX YZ", None),
            ],
            &mut metrics,
          )
        } else {
          portions(&[marker, (&joined, None)], &mut metrics)
        };
        let last = items.len() - 1;
        let old_first_ink = text(&items[last]).x_pt + if split { 0.0 } else { blanks as f32 * 3.0 };
        let spaces_count = 10 + blanks - 1;
        let residual = (40.0 - spaces_count as f32 * 1.5 - 0.15) / (spaces_count + 1) as f32;
        assert!(apply(&mut items, 40.0, &mut metrics));
        let portion = text(&items[last]);
        let first_ink = portion.x_pt
          + if split {
            0.0
          } else {
            blanks as f32 * (3.0 + portion.word_spacing_pt)
          };
        let expected = 0.15 + residual + (blanks - 1) as f32 * (1.5 + residual);
        assert!((first_ink - old_first_ink - expected).abs() < 0.001);
      }
    }
  }

  #[test]
  fn consecutive_notes_keep_their_interior_and_line_edges_fixed() {
    let mut metrics = TextMetrics::new();
    for (prefix, suffix) in [("AB CD", " EF GH"), ("", " EF GH"), ("AB CD", " ")] {
      let mut items = portions(
        &[
          (prefix, None),
          ("1", Some("ooxmlsdk-pdf:footnote-reference:4")),
          ("2", Some("ooxmlsdk-pdf:footnote-reference:5")),
          (suffix, None),
        ],
        &mut metrics,
      );
      let distance = text(&items[2]).x_pt - text(&items[1]).x_pt;
      let before = text(&items[1]).x_pt;
      assert!(apply(&mut items, 40.0, &mut metrics));
      assert!((text(&items[2]).x_pt - text(&items[1]).x_pt - distance).abs() < 0.001);
      if prefix.is_empty() {
        assert_eq!(text(&items[1]).x_pt, before);
      }
      if suffix.trim_end_matches(' ').is_empty() {
        let marker = text(&items[2]);
        let right = marker.x_pt + metrics.measure_text(&marker.text, &marker.style);
        let old = before + distance + metrics.measure_text("2", &style(8.0));
        assert!((right - old - 40.0).abs() < 0.001);
      }
    }
  }

  #[test]
  fn contraction_does_not_insert_note_edge_glue() {
    let mut metrics = TextMetrics::new();
    let mut items = portions(
      &[
        ("A B C", None),
        ("1", Some("ooxmlsdk-pdf:endnote-reference:4")),
        ("D E", None),
      ],
      &mut metrics,
    );
    let before = items.clone();
    assert!(!apply(&mut items, -1.0, &mut metrics));
    for (actual, original) in items.iter().zip(&before) {
      let actual = text(actual);
      let original = text(original);
      assert_eq!(actual.x_pt, original.x_pt);
      assert_eq!(actual.word_spacing_pt, original.word_spacing_pt);
    }
  }
}
