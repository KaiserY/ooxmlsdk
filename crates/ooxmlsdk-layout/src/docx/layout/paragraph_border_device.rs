//! Word's single horizontal paragraph rules are printer-grid brush rectangles.
use super::*;

const FRAME_OUTSET_DOTS: i64 = 12;

pub(super) fn solid(border: BorderStyle) -> bool {
  !border.compound
    && !border.shadow
    && border.relief.is_none()
    && border.dash_pattern == BorderDashPattern::Solid
    && border.width_pt.is_finite()
    && border.width_pt > 0.0
    && border.spacing_pt.is_finite()
    && border.spacing_pt >= 0.0
}

fn truncated_dots(points: f32) -> Option<i64> {
  if !points.is_finite() || points < 0.0 {
    return None;
  }
  let twips = (f64::from(points) * f64::from(units::TWIPS_PER_POINT)).floor() as i64;
  Some(twips.checked_mul(5)? / 12)
}

pub(super) fn thickness(border: BorderStyle) -> Option<i64> {
  if !solid(border) {
    return None;
  }
  Some(truncated_dots(border.width_pt)?.max(1))
}

pub(super) fn distance(border: BorderStyle) -> Option<i64> {
  if !solid(border) {
    return None;
  }
  truncated_dots(border.spacing_pt)
}

fn source_rectangle(
  (frame_x, frame_width, edge): (f32, f32, f32),
  border: BorderStyle,
  top: bool,
) -> Option<(i64, i64, i64, i64)> {
  if !solid(border)
    || ![frame_x, frame_width, edge, border.spacing_pt]
      .into_iter()
      .all(f32::is_finite)
  {
    return None;
  }
  let coordinate = wordprocessing_table_device::source_coordinate;
  // Native EMR_BITBLT/PATCOPY controls for all 1..32 eighth-point widths
  // first truncate to whole twips, then to printer dots. Directly flooring
  // the point-to-dot ratio is wrong for sizes 3, 25 and 27, among others.
  let thickness = thickness(border)?;
  // The text frame and its twelve-dot outsets have separate ownership.
  // Rounding the old approximate 1.5pt enclosure as one sum loses endpoints.
  let left = coordinate(frame_x) - FRAME_OUTSET_DOTS;
  let right = coordinate(frame_x + frame_width) + FRAME_OUTSET_DOTS;
  let space = distance(border)?;
  let y = if top {
    coordinate(edge) - space - thickness
  } else {
    coordinate(edge) + space
  };
  (right > left).then_some((left, y, right - left, thickness))
}

pub(super) fn paint(
  page: &mut Page,
  frame: (f32, f32, f32),
  border: BorderStyle,
  top: bool,
) -> bool {
  let Some(rectangle) = source_rectangle(frame, border, top) else {
    return false;
  };
  let Some(device) =
    wordprocessing_table_device::page_coordinates(page.setup.width_pt, page.setup.height_pt)
  else {
    return false;
  };
  let (x, y, width, height) = device.map_device_rectangle(rectangle);
  page.items.push(PageItem::Fill(FillItem {
    x_pt: x,
    y_pt: y,
    width_pt: width,
    height_pt: height,
    paint: ShadingPaint::Solid(border.color),
  }));
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_solid_distances_truncate_to_printer_dots_independently_of_the_frame() {
    // All16edge masks, uniform/unequal widths and shading opposites:
    // 2pt is16printer dots, rather than the rounded17-dot frame coordinate.
    for (points, expected) in [
      (0.0, 0),
      (1.0, 8),
      (2.0, 16),
      (3.0, 25),
      (4.0, 33),
      (10.0, 83),
    ] {
      assert_eq!(
        distance(BorderStyle {
          spacing_pt: points,
          ..Default::default()
        }),
        Some(expected)
      );
    }
  }

  #[test]
  fn native_horizontal_paragraph_pen_widths_retain_both_integer_conversions() {
    // Actual Word EMF thickness for every source width. Each was observed
    // independently with three distances and both top/bottom edges.
    let expected = [
      1, 2, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25, 25,
      27, 27, 29, 30, 31, 32, 33,
    ];
    for (index, thickness) in expected.into_iter().enumerate() {
      let border = BorderStyle {
        width_pt: (index + 1) as f32 / 8.0,
        ..Default::default()
      };
      for top in [false, true] {
        assert_eq!(
          source_rectangle((137.4, 379.2, 72.5), border, top)
            .unwrap()
            .3,
          thickness,
          "size={}, top={top}",
          index + 1
        );
      }
    }
  }

  #[test]
  fn native_paragraph_frame_outsets_keep_twip_phase_and_brush_endpoints() {
    let border = BorderStyle {
      width_pt: 0.5,
      ..Default::default()
    };
    assert_eq!(
      source_rectangle((137.4, 379.2, 72.5), border, true),
      Some((1133, 600, 3184, 4))
    );
    assert_eq!(
      source_rectangle(
        (137.5, 379.1, 101.76),
        BorderStyle {
          spacing_pt: 1.0,
          ..border
        },
        true
      ),
      Some((1134, 836, 3183, 4))
    );
    assert!(
      source_rectangle(
        (137.4, 379.2, 72.5),
        BorderStyle {
          dash_pattern: BorderDashPattern::Dashed,
          ..border
        },
        true
      )
      .is_none()
    );
  }
}
