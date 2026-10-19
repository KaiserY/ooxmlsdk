//! Legacy Word lines crossing separate VML Square wrapping bands.
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

#[cfg(test)]
mod tests {
  use super::*;

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
