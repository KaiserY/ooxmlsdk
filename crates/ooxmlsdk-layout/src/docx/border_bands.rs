//! Logical Word border bands, from the negative side of a centred line.
//!
//! Even entries paint rails; odd entries reserve gaps. Five fixed-capacity
//! entries preserve three-rail styles without allocating for ordinary borders.
use super::{BorderCompoundPattern, BorderStyle};

pub(super) fn widths(border: BorderStyle) -> Option<[f32; 5]> {
  if !border.compound {
    return Some([border.width_pt, 0.0, 0.0, 0.0, 0.0]);
  }
  use BorderCompoundPattern as P;
  let width = border.width_pt;
  Some(match border.compound_pattern {
    P::Equal | P::Double => [width / 3.0, width / 3.0, width / 3.0, 0.0, 0.0],
    P::ThinThickSmallGap => [(width - 1.5).max(0.0), 0.75, 0.75, 0.0, 0.0],
    P::ThickThinSmallGap => [0.75, 0.75, (width - 1.5).max(0.0), 0.0, 0.0],
    // Word's three-rail controls retain both fixed 15-twip thin rails and
    // both fixed 15-twip gaps; sz controls the central rail independently.
    P::ThinThickThinSmallGap => [0.75, 0.75, (width - 3.0).max(0.0), 0.75, 0.75],
    P::ThinThickLargeGap => [1.5, (width - 2.25).max(0.0), 0.75, 0.0, 0.0],
    P::ThickThinLargeGap => [0.75, (width - 2.25).max(0.0), 1.5, 0.0, 0.0],
    P::ThinThickMediumGap => [width / 2.0, width / 4.0, width / 4.0, 0.0, 0.0],
    P::ThickThinMediumGap => [width / 4.0, width / 4.0, width / 2.0, 0.0, 0.0],
    P::ThreeDEmboss | P::ThreeDEngrave => return None,
  })
}

/// Printer bands keep Word's whole-twip then whole-dot conversions separate
/// from the logical footprint used by line/cell layout.
pub(super) fn device_widths(border: BorderStyle) -> Option<[i64; 5]> {
  if !border.compound || border.dash_pattern != super::BorderDashPattern::Solid {
    return None;
  }
  if let Some([a, b, c]) = super::border_3d::device_band_widths(border) {
    return Some([a, b, c, 0, 0]);
  }
  use BorderCompoundPattern as P;
  let dots = |points: f64| ((points.max(0.0) * 20.0).floor() as i64 * 5 / 12).max(1);
  Some(match border.compound_pattern {
    P::Equal | P::Double => {
      let rail = dots(f64::from(border.width_pt / 3.0));
      [rail, rail, rail, 0, 0]
    }
    P::ThinThickLargeGap | P::ThickThinLargeGap => {
      let gap = ((f64::from(border.width_pt) - 2.25).max(0.0) * 20.0).floor() as i64 * 5 / 12;
      if border.compound_pattern == P::ThinThickLargeGap {
        [12, gap, 6, 0, 0]
      } else {
        [6, gap, 12, 0, 0]
      }
    }
    P::ThinThickSmallGap => [dots(f64::from(border.width_pt) - 1.5), 6, 6, 0, 0],
    P::ThickThinSmallGap => [6, 6, dots(f64::from(border.width_pt) - 1.5), 0, 0],
    P::ThinThickThinSmallGap => [6, 6, dots(f64::from(border.width_pt) - 3.0), 6, 6],
    _ => return None,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

  #[test]
  fn small_gap_triple_keeps_native_layout_footprint_and_three_printer_rails() {
    // Actual Word table controls sz8/24/48 retain six-dot thin rails/gaps
    // around the variable8/25/50-dot central rail. The logical footprint
    // remains4/6/9pt, independently of the truncated painted extent.
    for (size, footprint, central) in [(8, 4.0, 8), (24, 6.0, 25), (48, 9.0, 50)] {
      for border in [
        crate::docx::table_border_style(
          w::BorderValues::ThinThickThinSmallGap,
          Some(size),
          None,
          None,
          None,
        )
        .unwrap(),
        crate::docx::page_border_style(
          w::BorderValues::ThinThickThinSmallGap,
          Some(size),
          None,
          None,
          None,
        )
        .unwrap(),
      ] {
        assert_eq!(
          border.compound_pattern,
          BorderCompoundPattern::ThinThickThinSmallGap
        );
        assert_eq!(border.width_pt, footprint);
        assert_eq!(device_widths(border), Some([6, 6, central, 6, 6]));
        assert_eq!(widths(border).unwrap().into_iter().sum::<f32>(), footprint);
      }
    }
  }

  #[test]
  fn small_gap_five_band_storage_retains_two_rail_order_and_three_d_shading() {
    for (value, native) in [
      (w::BorderValues::ThinThickSmallGap, [25, 6, 6, 0, 0]),
      (w::BorderValues::ThickThinSmallGap, [6, 6, 25, 0, 0]),
      (w::BorderValues::Double, [25, 25, 25, 0, 0]),
      (w::BorderValues::ThreeDEmboss, [12, 25, 12, 0, 0]),
    ] {
      let border = crate::docx::table_border_style(value, Some(24), None, None, None).unwrap();
      assert_eq!(device_widths(border), Some(native));
    }
  }
}
