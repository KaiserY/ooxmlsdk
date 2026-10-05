use std::collections::BTreeMap;

use crate::common::fixed_output_device::PageCoordinates;
use crate::model::{BorderDashPattern, PageItem, common_rect};
use crate::text_metrics::TextMetrics;
use crate::{common, units};

use super::{CellBorderPaint, CellRect};

type BorderAxis = (bool, i64, [u8; 3]);
type BorderSegments = BTreeMap<BorderAxis, Vec<(i64, i64)>>;

/// Pictures truncate the page offset independently of their fractional
/// worksheet anchor. Native PDF margin controls distinguish this from
/// rounding the combined origin. The EMR_BITBLT's integer snapshot bounds
/// do not replace the fractional picture bounds in the PDF stream.
pub(super) fn printer_picture_page_origin(x_pt: f32, y_pt: f32) -> (f32, f32) {
  let truncate = |point| {
    let dots = (f64::from(point) * f64::from(units::OFFICE_FIXED_OUTPUT_DPI)
      / f64::from(units::POINTS_PER_INCH))
    .floor();
    (dots * f64::from(units::POINTS_PER_INCH) / f64::from(units::OFFICE_FIXED_OUTPUT_DPI)) as f32
  };
  (truncate(x_pt), truncate(y_pt))
}

/// Excel emits an extra cosmetic GDI line for unscaled, solid thin borders.
/// Native 13-style controls distinguish this from the same eight-dot filled
/// band at 95%, and from medium/thick borders. The PDF exporter realizes the
/// zero-width pen as 0.14pt, with square caps and a half-source-pixel inset.
pub(super) fn render_cosmetic_cell_borders(
  items: &mut Vec<PageItem>,
  paints: &[CellBorderPaint],
  print_scale: f32,
  page: CellRect,
  physical_page: CellRect,
) {
  if print_scale != 1.0 {
    return;
  }
  let Some(device) =
    PageCoordinates::for_standard_size(physical_page.width_pt, physical_page.height_pt)
  else {
    return;
  };
  // Keep producer coordinates integral while joining adjacent/duplicated
  // cell segments. A separate capped path per cell would darken each joint.
  let mut runs = BorderSegments::new();
  for paint in paints.iter().filter(|paint| paint.shear.is_none()) {
    let rect = super::cell_border_device_rect(paint.rect, page);
    for (border, vertical, band) in
      super::single_cell_border_bands(rect, paint.borders, print_scale)
    {
      if border.compound
        || border.width_pt != 1.0
        || border.dash_pattern != BorderDashPattern::Solid
      {
        continue;
      }
      let (coordinate, start, end) = if vertical {
        (band.x_pt, band.y_pt, band.y_pt + band.height_pt)
      } else {
        (band.y_pt, band.x_pt, band.x_pt + band.width_pt)
      };
      runs
        .entry((
          vertical,
          source_text_coordinate(coordinate),
          [border.color.r, border.color.g, border.color.b],
        ))
        .or_default()
        .push((source_text_coordinate(start), source_text_coordinate(end)));
    }
  }
  for ((vertical, coordinate, color), mut segments) in runs {
    segments.sort_unstable();
    let mut merged: Option<(i64, i64)> = None;
    for (start, end) in segments {
      if let Some((old_start, old_end)) = merged {
        if start <= old_end {
          merged = Some((old_start, old_end.max(end)));
          continue;
        }
        push_cosmetic_line(
          items, &device, vertical, coordinate, old_start, old_end, color,
        );
      }
      merged = Some((start, end));
    }
    if let Some((start, end)) = merged {
      push_cosmetic_line(items, &device, vertical, coordinate, start, end, color);
    }
  }
}

fn push_cosmetic_line(
  items: &mut Vec<PageItem>,
  device: &PageCoordinates,
  vertical: bool,
  coordinate: i64,
  start: i64,
  end: i64,
  color: [u8; 3],
) {
  if end <= start {
    return;
  }
  let (x0, y0, x1, y1) = if vertical {
    let (x, top) = device.map_device_point(coordinate, start);
    let (_, bottom) = device.map_device_point(coordinate, end);
    (x, top, x, bottom)
  } else {
    let (left, y) = device.map_device_point(start, coordinate);
    let (right, _) = device.map_device_point(end, coordinate);
    (left, y, right, y)
  };
  // The GDI line follows the leading edge of the brush band. Its reference
  // covers pixel centres and excludes LineTo's final pixel, independently of
  // the frame transform used for the filled rectangle's exclusive endpoints.
  let half_dot = units::POINTS_PER_INCH / units::OFFICE_FIXED_OUTPUT_DPI * 0.5;
  let first = common::Point {
    x: common::Pt(x0 + half_dot),
    y: common::Pt(y0 + half_dot),
  };
  let last = common::Point {
    x: common::Pt(x1 + if vertical { half_dot } else { -half_dot }),
    y: common::Pt(y1 + if vertical { -half_dot } else { half_dot }),
  };
  items.push(PageItem::Path(common::PathItem {
    bounds: common_rect(
      first.x.0,
      first.y.0,
      last.x.0 - first.x.0,
      last.y.0 - first.y.0,
    ),
    points: vec![first, last],
    commands: Vec::new(),
    closed: false,
    fill: common::Fill::None,
    stroke: Some(common::Stroke {
      width: common::Pt(0.14),
      color: common::Color {
        r: color[0],
        g: color[1],
        b: color[2],
        a: 255,
      },
      cap: Some(common::StrokeCap::Square),
      join: Some(common::StrokeJoin::Round),
      ..Default::default()
    }),
  }));
}

/// Replay ordinary cell paint after layout, border arbitration and clipping.
/// Excel's native PDF EMF uses the same exclusive frame as Word. Applying its
/// transform to row heights or widths instead would also change pagination.
pub(super) fn replay_cell_paint(
  items: &mut [PageItem],
  physical_page: CellRect,
  text_metrics: &mut TextMetrics,
) {
  let Some(device) =
    PageCoordinates::for_standard_size(physical_page.width_pt, physical_page.height_pt)
  else {
    return;
  };
  for item in items {
    match item {
      PageItem::Rect(rect) => {
        (rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt) =
          device.map_rectangle((rect.x_pt, rect.y_pt, rect.width_pt, rect.height_pt));
      }
      PageItem::Text(text)
        if text.style.rotation_deg == 0.0 && text.style.baseline_shift_pt == 0.0 =>
      {
        let baseline_offset = if text.style.use_windows_font_metrics {
          text_metrics.baseline_offset_in_line_with_windows_metrics_for_text(
            &text.text,
            &text.style,
            text.line_height_pt,
          )
        } else {
          text_metrics.baseline_offset_in_line_for_text(
            &text.text,
            &text.style,
            text.line_height_pt,
          )
        };
        // GDI string references are integral printer coordinates. Quantize
        // the actual baseline, rather than the font-dependent line-box top;
        // retain the realized font size and character advances unchanged.
        let baseline = text.y_pt + baseline_offset;
        let (x, y) = device.map_device_point(
          source_text_coordinate(text.x_pt),
          source_text_coordinate(baseline),
        );
        text.x_pt = x;
        text.y_pt = y - baseline_offset;
        text.paint_clip = text.paint_clip.map(|clip| replay_clip(&device, clip));
        text.page_culling_bounds = text
          .page_culling_bounds
          .map(|bounds| replay_clip(&device, bounds));
      }
      // Rotated/edit-layout paths and DrawingML have separate producer
      // coordinates. This pass owns ordinary cell rectangles and strings.
      _ => {}
    }
  }
}

fn source_text_coordinate(point: f32) -> i64 {
  (f64::from(point) * units::OFFICE_FIXED_OUTPUT_DPI as f64 / units::POINTS_PER_INCH as f64).round()
    as i64
}

fn replay_clip(device: &PageCoordinates, clip: common::Rect) -> common::Rect {
  let (x, y, width, height) = device.map_rectangle((
    clip.origin.x.0,
    clip.origin.y.0,
    clip.size.width.0,
    clip.size.height.0,
  ));
  common_rect(x, y, width, height)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::{PdfTextSegmentation, RectItem, TextItem, TextStyle};

  fn landscape_a4() -> CellRect {
    CellRect {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: 841.92,
      height_pt: 595.32,
    }
  }

  #[test]
  fn picture_page_margins_truncate_their_native_device_origin() {
    // Native EMR_BITBLT references from four page-margin controls. The
    // unchanged two-EMU worksheet origin cannot round the page margin up.
    for (phase, expected_x, expected_y) in [
      (0.03, 420.0, 600.0),
      (0.06, 420.0, 600.0),
      (0.09, 420.0, 600.0),
      (0.15, 421.0, 601.0),
    ] {
      let (x, y) = printer_picture_page_origin(50.4 + phase, 72.0 + phase);
      assert!((x - expected_x * 0.12).abs() < 0.0001);
      assert!((y - expected_y * 0.12).abs() < 0.0001);
    }
  }

  #[test]
  fn cosmetic_cell_border_matches_native_emf_lines_and_pdf_pen() {
    let page = landscape_a4();
    let device = PageCoordinates::for_standard_size(page.width_pt, page.height_pt).unwrap();
    for (vertical, coordinate, start, end, expected) in [
      (false, 1959, 271, 6756, [32.58, 235.19, 810.78, 235.19]),
      (true, 1642, 1845, 4457, [197.12, 221.51, 197.12, 534.876]),
    ] {
      let mut items = Vec::new();
      push_cosmetic_line(
        &mut items,
        &device,
        vertical,
        coordinate,
        start,
        end,
        [0, 0, 0],
      );
      let PageItem::Path(path) = &items[0] else {
        panic!("native cosmetic path");
      };
      let actual = [
        path.points[0].x.0,
        path.points[0].y.0,
        path.points[1].x.0,
        path.points[1].y.0,
      ];
      for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.005);
      }
      let stroke = path.stroke.as_ref().unwrap();
      assert_eq!(stroke.width, common::Pt(0.14));
      assert_eq!(stroke.cap, Some(common::StrokeCap::Square));
      assert_eq!(stroke.join, Some(common::StrokeJoin::Round));
    }
  }

  #[test]
  fn cosmetic_border_preserves_native_style_and_scale_counterexamples() {
    let page = landscape_a4();
    for (weight, pattern, compound, scale, expected) in [
      (1.0, BorderDashPattern::Solid, false, 1.0, 1),
      (1.0, BorderDashPattern::Solid, false, 0.95, 0),
      (1.0, BorderDashPattern::Solid, false, 0.45, 0),
      (1.0, BorderDashPattern::Solid, false, 1.5, 0),
      (2.0, BorderDashPattern::Solid, false, 1.0, 0),
      (3.0, BorderDashPattern::Solid, false, 1.0, 0),
      (1.0, BorderDashPattern::Dotted, false, 1.0, 0),
      (1.0, BorderDashPattern::Solid, true, 1.0, 0),
    ] {
      let border = crate::model::BorderStyle {
        width_pt: weight,
        dash_pattern: pattern,
        compound,
        ..Default::default()
      };
      // Native adjacent D:F cells have one continuous capped top line, even
      // when the owning border appears on multiple neighboring cells.
      let mut paints = (0..3)
        .map(|col| CellBorderPaint {
          rect: CellRect {
            x_pt: 24.0 + col as f32 * 24.0,
            y_pt: 60.0,
            width_pt: 24.0,
            height_pt: 12.0,
          },
          borders: super::super::super::styles::BorderRecord {
            top: Some(border),
            ..Default::default()
          },
          shear: None,
        })
        .collect::<Vec<_>>();
      paints.push(paints[0]);
      let mut items = Vec::new();
      render_cosmetic_cell_borders(&mut items, &paints, scale, page, page);
      assert_eq!(
        items.len(),
        expected,
        "weight={weight} pattern={pattern:?} scale={scale}"
      );
    }
  }

  #[test]
  fn cell_border_replays_both_native_emf_endpoints() {
    // Native EMR_BITBLT: y=2589, height=8 printer dots. PDF playback gives
    // y=310.728 and height=0.984, independently of the authored border weight.
    let mut items = [PageItem::Rect(RectItem {
      x_pt: 32.52,
      y_pt: 310.68,
      width_pt: 778.32,
      height_pt: 0.96,
      fill_color: None,
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    })];
    replay_cell_paint(&mut items, landscape_a4(), &mut TextMetrics::new());
    let PageItem::Rect(rect) = &items[0] else {
      panic!("rectangle retained");
    };
    assert!((rect.y_pt - 310.728).abs() < 0.0001);
    assert!((rect.height_pt - 0.984).abs() < 0.0001);
  }

  #[test]
  fn cell_text_replays_printer_reference_without_scaling_the_font() {
    let style = TextStyle {
      font_family: Some("Arial".into()),
      font_size_pt: 9.0,
      bold: true,
      ..Default::default()
    };
    let mut metrics = TextMetrics::new();
    let offset = metrics.baseline_offset_in_line_for_text("800", &style, 10.8);
    let mut items = [PageItem::Text(TextItem {
      x_pt: 353.212,
      y_pt: 232.2 - offset,
      line_height_pt: 10.8,
      drawingml_text_effect_anchor: None,
      paint_clip: None,
      page_culling_bounds: None,
      discard_if_horizontally_clipped: false,
      text: "800".into(),
      style: Box::new(style),
      rotation_center_pt: None,
      hyperlink_url: None,
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: false,
      pdf_text_segmentation: PdfTextSegmentation::Line,
      source_path: vec![7, 2],
    })];
    replay_cell_paint(&mut items, landscape_a4(), &mut metrics);
    let PageItem::Text(text) = &items[0] else {
      panic!("text retained");
    };
    assert!((text.x_pt - 353.208).abs() < 0.0001);
    assert!((text.y_pt + offset - 232.248).abs() < 0.0001);
    assert_eq!(text.style.font_size_pt, 9.0);
    assert_eq!(text.source_path, vec![7, 2]);
  }
}
