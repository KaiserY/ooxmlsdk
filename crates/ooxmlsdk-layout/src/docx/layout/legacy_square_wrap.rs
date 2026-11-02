//! Legacy Word lines crossing Square drawing wrapping bands.
use super::*;

struct Interval {
  top: f32,
  bottom: f32,
  left: f32,
}

struct Band {
  entries: std::ops::Range<usize>,
  top: f32,
  bottom: f32,
  left: f32,
}

pub(super) fn line_start(
  flow: &FlowContext,
  mut y: f32,
  height: f32,
  left: f32,
  right: f32,
  exclusions: &[WrapExclusion],
) -> f32 {
  if flow.compatibility_mode >= 15
    || flow.text_segmentation != TextSegmentation::Body
    || flow.inside_paragraph_frame
    || !y.is_finite()
    || !height.is_finite()
    || height <= 0.0
    || !(y + height).is_finite()
  {
    return y;
  }
  y = drawing_transition_start(y, height, left, right, exclusions);
  y = frame_table_transition_start(y, height, left, right, exclusions);
  let initial_y = y;
  let eligible = |exclusion: &&WrapExclusion| {
    exclusion.owner == WrapExclusionOwner::VmlBodyDrawing
      && !exclusion.blocks_flow
      && !exclusion.uses_contour
      && exclusion.contour_band.is_none()
      && exclusion.side == ImageWrapSide::BothSides
      && exclusion.left_pt.is_finite()
      && exclusion.top_pt.is_finite()
      && exclusion.bottom_pt.is_finite()
      && exclusion.left_pt > left
      && exclusion.left_pt < right
      && exclusion.right_pt > exclusion.left_pt
      && exclusion.top_pt < exclusion.bottom_pt
      && exclusion.bottom_pt > initial_y
  };
  // Ordinary lines and overlapping objects need no allocation or sorting.
  let mut count = 0;
  let mut common_top = f32::NEG_INFINITY;
  let mut common_bottom = f32::INFINITY;
  for exclusion in exclusions.iter().filter(eligible) {
    if exclusion.top_pt < y + height {
      count += 1;
      common_top = common_top.max(exclusion.top_pt);
      common_bottom = common_bottom.min(exclusion.bottom_pt);
    }
  }
  if count < 2 || common_top < common_bottom {
    return y;
  }
  let mut intervals = exclusions
    .iter()
    .filter(eligible)
    .map(|exclusion| Interval {
      top: exclusion.top_pt,
      bottom: exclusion.bottom_pt,
      left: exclusion.left_pt,
    })
    .collect::<Vec<_>>();
  intervals.sort_unstable_by(|a, b| a.top.total_cmp(&b.top));
  let mut bands: Vec<Band> = Vec::new();
  for (index, interval) in intervals.iter().enumerate() {
    if let Some(band) = bands.last_mut()
      && interval.top < band.bottom
    {
      band.entries.end = index + 1;
      band.bottom = band.bottom.max(interval.bottom);
      band.left = band.left.min(interval.left);
    } else {
      bands.push(Band {
        entries: index..index + 1,
        top: interval.top,
        bottom: interval.bottom,
        left: interval.left,
      });
    }
  }
  let mut index = bands.partition_point(|band| band.bottom <= y);
  while index + 1 < bands.len() {
    let current = &bands[index];
    let next = &bands[index + 1];
    if current.top > y || next.top >= y + height {
      break;
    }
    let next_left = intervals[next.entries.clone()]
      .iter()
      .take_while(|interval| interval.top < y + height)
      .map(|interval| interval.left)
      .fold(f32::INFINITY, f32::min);
    if next_left >= current.left {
      break;
    }
    // Native plain/numbered, first/following-line controls distinguish a
    // narrowing separate band from overlapping bands or an opening lane.
    // Clear the current band without moving either floating anchor. Touching
    // bands remain separate; a later top outside this line has no effect.
    y = current.bottom;
    index += 1;
  }
  y
}

fn drawing_transition_start(
  mut y: f32,
  height: f32,
  left: f32,
  right: f32,
  exclusions: &[WrapExclusion],
) -> f32 {
  let eligible = |exclusion: &&WrapExclusion| {
    exclusion.owner == WrapExclusionOwner::Drawing
      && !exclusion.blocks_flow
      && !exclusion.uses_contour
      && exclusion.contour_band.is_none()
      && exclusion.left_pt.is_finite()
      && exclusion.right_pt.is_finite()
      && exclusion.top_pt.is_finite()
      && exclusion.bottom_pt.is_finite()
      && exclusion.left_pt < exclusion.right_pt
      && exclusion.top_pt < exclusion.bottom_pt
      && exclusion.overlaps_horizontal_span(left, right)
  };
  let twips = |value: f32| (value * units::TWIPS_PER_POINT).round();
  loop {
    // Native line/rectangle edges are integral twips. Keep adjacent controls
    // distinct without inheriting binary accumulation of the line height.
    let top = twips(y);
    let bottom = twips(y + height);
    let next = exclusions
      .iter()
      .filter(eligible)
      .filter(|closing| {
        twips(closing.top_pt) <= top
          && twips(closing.bottom_pt) > top
          && twips(closing.bottom_pt) < bottom
      })
      .filter(|closing| {
        let lanes = square_drawing_lanes(closing, left, right);
        exclusions.iter().filter(eligible).any(|opening| {
          if twips(opening.top_pt) < top
            || twips(opening.top_pt) >= bottom
            || opening.bottom_pt <= closing.bottom_pt
          {
            return false;
          }
          let following = square_drawing_lanes(opening, left, right);
          lanes.iter().flatten().any(|&(start, end)| {
            !following
              .iter()
              .flatten()
              .any(|&(next_start, next_end)| next_start <= start && next_end >= end)
          })
        })
      })
      .map(|closing| closing.bottom_pt)
      .min_by(f32::total_cmp);
    let Some(next) = next else { return y };
    // Word 12/14 joins the closing picture's side lanes below its bottom
    // when another picture starts inside the same line and cuts one of those
    // lanes. Merely intersecting the two rectangles retains a left fragment
    // that Word does not format. Native one-twip, side, height, source-order
    // and mode15 controls distinguish this transition from overlapping
    // bands which remain active throughout the line. Move only the text.
    y = next;
  }
}

fn square_drawing_lanes(
  exclusion: &WrapExclusion,
  left: f32,
  right: f32,
) -> [Option<(f32, f32)>; 2] {
  let before = (left, exclusion.left_pt.clamp(left, right));
  let after = (exclusion.right_pt.clamp(left, right), right);
  let before_width = before.1 - before.0;
  let after_width = after.1 - after.0;
  let side = match exclusion.side {
    // Word measures its preferred side against the column: give up a side
    // narrower than a quarter inch only when the other side has that room.
    // Native18/17.95pt controls and Writer's WORD_TEXT_MIN agree.
    ImageWrapSide::BothSides if before_width < 18.0 && after_width >= 18.0 => ImageWrapSide::Right,
    ImageWrapSide::BothSides if after_width < 18.0 && before_width >= 18.0 => ImageWrapSide::Left,
    ImageWrapSide::Left if before_width == 0.0 => ImageWrapSide::BothSides,
    ImageWrapSide::Largest if before_width > after_width => ImageWrapSide::Left,
    ImageWrapSide::Largest => ImageWrapSide::Right,
    side => side,
  };
  [
    (side != ImageWrapSide::Right && before_width >= DEFAULT_FONT_SIZE_PT).then_some(before),
    (side != ImageWrapSide::Left && after_width >= DEFAULT_FONT_SIZE_PT).then_some(after),
  ]
}

fn frame_table_transition_start(
  mut y: f32,
  height: f32,
  left: f32,
  right: f32,
  exclusions: &[WrapExclusion],
) -> f32 {
  let rectangular = |exclusion: &&WrapExclusion| {
    !exclusion.blocks_flow
      && !exclusion.uses_contour
      && exclusion.contour_band.is_none()
      && exclusion.left_pt.is_finite()
      && exclusion.right_pt.is_finite()
      && exclusion.top_pt.is_finite()
      && exclusion.bottom_pt.is_finite()
      && exclusion.left_pt < exclusion.right_pt
      && exclusion.top_pt < exclusion.bottom_pt
      && exclusion.overlaps_horizontal_span(left, right)
  };
  let twips = |value: f32| (value * units::TWIPS_PER_POINT).round();
  loop {
    let top = twips(y);
    let bottom = twips(y + height);
    let next = exclusions
      .iter()
      .filter(rectangular)
      .filter(|closing| {
        closing.owner == WrapExclusionOwner::ParagraphFrame
          && twips(closing.top_pt) < bottom
          && twips(closing.bottom_pt) > top
          && twips(closing.bottom_pt) < bottom
      })
      .filter(|closing| {
        let lanes = square_drawing_lanes(closing, left, right);
        exclusions.iter().filter(rectangular).any(|opening| {
          if opening.owner != WrapExclusionOwner::FloatingTable
            || twips(opening.top_pt) < twips(closing.bottom_pt)
            || twips(opening.top_pt) >= bottom
          {
            return false;
          }
          let following = square_drawing_lanes(opening, left, right);
          lanes.iter().flatten().any(|&(start, end)| {
            !following
              .iter()
              .flatten()
              .any(|&(next_start, next_end)| next_start <= start && next_end >= end)
          })
        })
      })
      .map(|closing| closing.bottom_pt)
      .min_by(f32::total_cmp);
    let Some(next) = next else { return y };
    // Word12/14 clears a paragraph frame when a later, separate table band
    // cuts its side lane inside the same physical line. The frame may start
    // within that line. Native font/y/table-y controls distinguish this from
    // ordinary side wrap and overlapping bands; only the line origin moves.
    y = next;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn paragraph_frame_and_table_transition_uses_the_complete_legacy_line() {
    let mut flow = flow_context(
      PageSetup::default(),
      0,
      SectionColumns::default(),
      0,
      0,
      DEFAULT_TAB_STOP_PT,
    );
    let frame = WrapExclusion {
      left_pt: 465.45,
      right_pt: 575.2,
      top_pt: 85.25,
      bottom_pt: 95.65,
      side: ImageWrapSide::BothSides,
      blocks_flow: false,
      uses_contour: false,
      contour_band: None,
      owner: WrapExclusionOwner::ParagraphFrame,
    };
    let table = WrapExclusion {
      left_pt: 367.15,
      right_pt: 615.45,
      top_pt: 100.8,
      bottom_pt: 185.4,
      owner: WrapExclusionOwner::FloatingTable,
      ..frame
    };
    for mode in [12, 14, 15] {
      flow.compatibility_mode = mode;
      for (top, bottom, table_top, line_height, expected_legacy) in [
        (85.25, 95.65, 100.8, 47.6, 95.65),
        (65.0, 75.4, 100.8, 47.6, 75.4),
        (85.25, 95.65, 100.8, 15.9, 72.0),
        (65.0, 75.4, 100.8, 15.9, 72.0),
        (120.0, 130.4, 100.8, 47.6, 72.0),
        (85.25, 95.65, 42.0, 47.6, 72.0),
        (85.25, 95.65, 72.0, 47.6, 72.0),
        (85.25, 95.65, 132.0, 47.6, 72.0),
      ] {
        let exclusions = [
          WrapExclusion {
            top_pt: top,
            bottom_pt: bottom,
            ..frame
          },
          WrapExclusion {
            top_pt: table_top,
            bottom_pt: table_top + 84.6,
            ..table
          },
        ];
        let expected = if mode < 15 { expected_legacy } else { 72.0 };
        for reverse in [false, true] {
          let mut ordered = exclusions;
          if reverse {
            ordered.reverse();
          }
          assert_eq!(
            line_start(&flow, 72.0, line_height, 72.0, 523.35, &ordered),
            expected,
            "mode={mode}, top={top}, table_top={table_top}, height={line_height}"
          );
        }
      }
    }
    flow.compatibility_mode = 14;
    assert_eq!(line_start(&flow, 72.0, 47.6, 72.0, 523.35, &[frame]), 72.0);
    assert_eq!(line_start(&flow, 72.0, 47.6, 72.0, 523.35, &[table]), 72.0);
    let opening_lane = WrapExclusion {
      left_pt: 490.0,
      ..table
    };
    assert_eq!(
      line_start(&flow, 72.0, 47.6, 72.0, 523.35, &[frame, opening_lane]),
      72.0
    );
    flow.text_segmentation = TextSegmentation::TableCell;
    assert_eq!(
      line_start(&flow, 72.0, 47.6, 72.0, 523.35, &[frame, table]),
      72.0
    );
  }

  #[test]
  fn picture_band_transition_preserves_native_mode_side_and_edge_opposites() {
    let mut flow = flow_context(
      PageSetup::default(),
      0,
      SectionColumns::default(),
      0,
      0,
      36.0,
    );
    let first = WrapExclusion {
      left_pt: 178.2,
      right_pt: 241.4,
      top_pt: 86.2,
      bottom_pt: 180.7,
      side: ImageWrapSide::BothSides,
      blocks_flow: false,
      uses_contour: false,
      contour_band: None,
      owner: WrapExclusionOwner::Drawing,
    };
    let second = WrapExclusion {
      left_pt: 232.35,
      right_pt: 363.15,
      top_pt: 170.9,
      bottom_pt: 255.65,
      ..first
    };
    for mode in [12, 14, 15] {
      flow.compatibility_mode = mode;
      for (top, bottom, side, expected_legacy) in [
        (170.9, 180.7, ImageWrapSide::BothSides, 180.7),
        (168.55, 180.7, ImageWrapSide::BothSides, 168.6),
        (168.6, 180.7, ImageWrapSide::BothSides, 180.7),
        (182.4, 180.7, ImageWrapSide::BothSides, 168.6),
        (170.9, 182.35, ImageWrapSide::BothSides, 182.35),
        (170.9, 182.4, ImageWrapSide::BothSides, 168.6),
        (170.9, 196.35, ImageWrapSide::BothSides, 168.6),
        (170.9, 180.7, ImageWrapSide::Left, 168.6),
      ] {
        let exclusions = [
          WrapExclusion {
            bottom_pt: bottom,
            side,
            ..first
          },
          WrapExclusion {
            top_pt: top,
            ..second
          },
        ];
        let expected = if mode < 15 { expected_legacy } else { 168.6 };
        for reverse in [false, true] {
          let mut ordered = exclusions;
          if reverse {
            ordered.reverse();
          }
          assert_eq!(
            line_start(&flow, 168.6, 13.8, 144.0, 275.6, &ordered),
            expected
          );
        }
      }
    }
    flow.compatibility_mode = 14;
    for (right_width, expected) in [(18.0, 180.7), (17.95, 168.6)] {
      let closing = WrapExclusion {
        left_pt: 275.6 - right_width - 63.2,
        right_pt: 275.6 - right_width,
        ..first
      };
      assert_eq!(
        line_start(&flow, 168.6, 13.8, 144.0, 275.6, &[closing, second]),
        expected
      );
    }
  }

  #[test]
  fn separate_square_bands_clear_only_a_narrowing_legacy_body_lane() {
    let mut flow = flow_context(
      PageSetup::default(),
      0,
      SectionColumns::default(),
      0,
      0,
      36.0,
    );
    flow.compatibility_mode = 14;
    let first = WrapExclusion {
      left_pt: 124.8,
      right_pt: 385.9,
      top_pt: 60.65,
      bottom_pt: 93.6,
      side: ImageWrapSide::BothSides,
      blocks_flow: false,
      uses_contour: false,
      contour_band: None,
      owner: WrapExclusionOwner::VmlBodyDrawing,
    };
    for (top, left, expected) in [
      (88.0, 96.25, 84.7),
      (93.55, 96.25, 84.7),
      (93.6, 96.25, 93.6),
      (94.0, 96.25, 93.6),
      (96.25, 96.25, 84.7),
      (93.6, 124.8, 84.7),
      (93.6, 140.0, 84.7),
    ] {
      let second = WrapExclusion {
        left_pt: left,
        right_pt: left + 20.0,
        top_pt: top,
        bottom_pt: top + 21.35,
        ..first
      };
      assert!(
        (line_start(&flow, 84.7, 11.55, 46.1, 197.05, &[first, second]) - expected).abs() < 0.0001
      );
      flow.compatibility_mode = 15;
      assert_eq!(
        line_start(&flow, 84.7, 11.55, 46.1, 197.05, &[first, second]),
        84.7
      );
      flow.compatibility_mode = 14;
    }
  }

  #[test]
  fn square_bands_move_the_actual_first_or_following_text_line() {
    let mut text_metrics = TextMetrics::new();
    for mode in [14, 15] {
      for height in [6.0, 11.55, 18.0] {
        for leading in 0..3 {
          for gap in [-0.05, 0.05] {
            let target_top = 84.7 + leading as f32 * height;
            let first_bottom = target_top + 0.75 * height;
            let style = TextStyle {
              font_family: Some("DejaVu Sans".into()),
              font_size_pt: 6.5,
              bold: true,
              character_spacing_pt: -1.05,
              use_windows_font_metrics: true,
              ..TextStyle::default()
            };
            let run = TextRun {
              text: format!("{}P Q", "\n".repeat(leading)),
              style: style.clone(),
              hyperlink_url: None,
              dynamic_field: None,
              style_ref_keys: Vec::new(),
              style_ref_text: None,
              style_ref_numbering_text: None,
              preserve_text_portion: false,
            };
            let paragraph = crate::docx::Paragraph {
              inlines: vec![InlineItem::Text(run.clone())],
              runs: vec![run],
              field_events: Vec::new(),
              footnote_reference_ids: Vec::new(),
              endnote_reference_ids: Vec::new(),
              starts_after_last_rendered_page_break: false,
              base_style: style.clone(),
              format: Box::new(crate::docx::ParagraphFormat {
                line_height_rule: LineHeightRule::Exact,
                line_height_pt: Some(height),
                indent_left_pt: 14.4,
                first_line_indent_pt: -10.8,
                widow_control: Some(false),
                ..crate::docx::ParagraphFormat::default()
              }),
              style_ref_keys: Vec::new(),
              style_ref_text: None,
              style_ref_numbering_text: None,
              list_label: None,
              list_label_image: None,
              list_label_style: style,
              list_label_hyperlink_url: None,
              list_label_tab_stop_pt: None,
            };
            let setup = PageSetup {
              width_pt: 573.1,
              height_pt: 770.15,
              margin_top_pt: 84.7,
              margin_bottom_pt: 8.55,
              margin_left_pt: 31.7,
              margin_right_pt: 376.05,
              ..PageSetup::default()
            };
            let mut flow = flow_context(setup, 0, SectionColumns::default(), 0, 0, 36.0);
            flow.compatibility_mode = mode;
            let mut page = empty_page(setup, 0);
            let first = WrapExclusion {
              left_pt: 124.8,
              right_pt: 385.9,
              top_pt: first_bottom - 32.95,
              bottom_pt: first_bottom,
              side: ImageWrapSide::BothSides,
              blocks_flow: false,
              uses_contour: false,
              contour_band: None,
              owner: WrapExclusionOwner::VmlBodyDrawing,
            };
            page.wrap_exclusions = vec![
              first,
              WrapExclusion {
                left_pt: 96.25,
                right_pt: 376.55,
                top_pt: first_bottom + gap,
                bottom_pt: first_bottom + gap + 21.35,
                ..first
              },
            ];
            let mut pages = Vec::new();
            let (_, next_top) = layout_paragraph(
              &paragraph,
              flow,
              ParagraphLayoutTarget {
                current: &mut page,
                pages: &mut pages,
                anchor_pages: None,
                text_metrics: &mut text_metrics,
              },
              84.7,
              84.7,
              0.0,
            );
            assert!(pages.is_empty());
            let text = page
              .items
              .iter()
              .find_map(|item| match item {
                PageItem::Text(text) if text.text.contains('P') => Some(text),
                _ => None,
              })
              .expect("the marker stays on the source page");
            let metrics = text
              .wordprocessing_line_metrics
              .expect("completed fixed text line");
            let top = text.y_pt + metrics.bottom_offset_pt - metrics.height_pt;
            let expected = if mode < 15 && gap > 0.0 {
              first_bottom
            } else {
              target_top
            };
            assert!(
              (top - expected).abs() < 0.001,
              "mode={mode} height={height} leading={leading} gap={gap}: {top} vs {expected}"
            );
            assert!((next_top - expected - height).abs() < 0.001);
          }
        }
      }
    }
  }
}
