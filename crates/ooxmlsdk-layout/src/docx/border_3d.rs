//! Word's colored three-band table and page rules (MS-DOC BrcType 0x18/0x19).
use crate::common::color_math::HslColor;
use crate::model::{BorderCompoundPattern, BorderStyle, RgbColor};
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

pub(super) fn extent(
  value: w::BorderValues,
  line_width: f32,
) -> Option<(f32, BorderCompoundPattern)> {
  let pattern = match value {
    w::BorderValues::ThreeDEmboss => BorderCompoundPattern::ThreeDEmboss,
    w::BorderValues::ThreeDEngrave => BorderCompoundPattern::ThreeDEngrave,
    _ => return None,
  };
  // Native widths 2..96, including sz23/24/25: sz is the middle band.
  // Both outside bands are .75pt below 3pt, and 1.5pt from 3pt onwards.
  let rail = if line_width < 3.0 { 0.75 } else { 1.5 };
  Some((line_width + 2.0 * rail, pattern))
}

pub(super) fn band_widths(border: BorderStyle) -> Option<[f32; 3]> {
  if !matches!(
    border.compound_pattern,
    BorderCompoundPattern::ThreeDEmboss | BorderCompoundPattern::ThreeDEngrave
  ) {
    return None;
  }
  // Imported extents jump from <4.5pt to >=6pt at the authored 3pt boundary.
  let rail = if border.width_pt < 6.0 { 0.75 } else { 1.5 };
  Some([rail, (border.width_pt - 2.0 * rail).max(0.0), rail])
}

/// Native table/page exporters truncate to whole twips before the 600-DPI
/// grid. In particular, authored sz25 (3.125pt) has a 25-dot middle band.
pub(super) fn device_band_widths(border: BorderStyle) -> Option<[i64; 3]> {
  Some(band_widths(border)?.map(|width| {
    let twips = (f64::from(width) * 20.0).floor() as i64;
    (twips * 5 / 12).max(1)
  }))
}

pub(super) fn band_colors(border: BorderStyle) -> [RgbColor; 3] {
  let rgb = [border.color.r, border.color.g, border.color.b];
  let colors = match rgb {
    [0, 0, 0] => [192, 96, 0].map(|v| RgbColor { r: v, g: v, b: v }),
    [255, 255, 255] => [255, 160, 128].map(|v| RgbColor { r: v, g: v, b: v }),
    _ => {
      // Preserve hue/saturation in the Win32 HLS domain (0..240). Native
      // RGB, neutral and luminance-boundary controls distinguish these
      // contrast bands from multiplying each RGB channel by a fixed shade.
      let mut hsl = HslColor::from_srgb8(rgb);
      hsl.hue_degrees = (hsl.hue_degrees * (240.0 / 360.0)).round() * (360.0 / 240.0);
      hsl.saturation = (hsl.saturation * 240.0).round() / 240.0;
      let luminance = (hsl.lightness * 240.0).round();
      let (light, middle, dark) = if luminance >= 120.0 {
        (luminance, luminance - 70.0, luminance - 80.0)
      } else if luminance >= 60.0 {
        (luminance + 40.0, luminance - 30.0, luminance - 40.0)
      } else {
        (
          luminance + 80.0,
          (luminance + 10.0).max(luminance / 2.0 + 20.0),
          luminance,
        )
      };
      [light, middle, dark].map(|luminance| {
        hsl.lightness = luminance / 240.0;
        let [r, g, b] = hsl.to_srgb8();
        RgbColor { r, g, b }
      })
    }
  };
  if border.compound_pattern == BorderCompoundPattern::ThreeDEngrave {
    [colors[2], colors[1], colors[0]]
  } else {
    colors
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn three_d_border_native_width_boundaries_keep_the_middle_and_both_faces() {
    for (size, expected) in [
      (2, [0.75, 0.25, 0.75]),
      (6, [0.75, 0.75, 0.75]),
      (12, [0.75, 1.5, 0.75]),
      (18, [0.75, 2.25, 0.75]),
      (23, [0.75, 2.875, 0.75]),
      (24, [1.5, 3.0, 1.5]),
      (25, [1.5, 3.125, 1.5]),
      (48, [1.5, 6.0, 1.5]),
      (96, [1.5, 12.0, 1.5]),
    ] {
      for value in [
        w::BorderValues::ThreeDEmboss,
        w::BorderValues::ThreeDEngrave,
      ] {
        let border = super::super::table_border_style(value, Some(size), None, None, None)
          .expect("authored table border");
        assert!(border.compound);
        assert_eq!(band_widths(border), Some(expected));
        assert_eq!(border.width_pt, expected.into_iter().sum());
      }
    }
    assert!(extent(w::BorderValues::Single, 0.75).is_none());
    assert!(band_widths(BorderStyle::default()).is_none());
  }

  #[test]
  fn three_d_border_direct_cell_overrides_retain_the_table_codec() {
    for value in [
      w::BorderValues::ThreeDEmboss,
      w::BorderValues::ThreeDEngrave,
    ] {
      let direct = w::TableCellBorders {
        top_border: Some(w::TopBorder {
          val: value,
          size: Some(48),
          ..Default::default()
        }),
        bottom_border: Some(w::BottomBorder {
          val: value,
          size: Some(48),
          ..Default::default()
        }),
        left_border: Some(w::LeftBorder {
          val: value,
          size: Some(48),
          ..Default::default()
        }),
        right_border: Some(w::RightBorder {
          val: value,
          size: Some(48),
          ..Default::default()
        }),
        ..Default::default()
      };
      let inherited = super::super::cell_borders_model(&direct);
      let actual = super::super::direct_cell_borders_model(Default::default(), &direct);
      assert_eq!(actual, inherited);
      for border in [actual.top, actual.bottom, actual.left, actual.right] {
        let border = border.expect("direct cell edge");
        assert_eq!(border.width_pt, 9.0);
        assert_eq!(band_widths(border), Some([1.5, 6.0, 1.5]));
      }
    }
  }

  #[test]
  fn three_d_border_native_contrast_keeps_luminance_hue_and_engraving_order() {
    for (color, expected) in [
      ("auto", [[192, 192, 192], [96, 96, 96], [0, 0, 0]]),
      ("000000", [[192, 192, 192], [96, 96, 96], [0, 0, 0]]),
      (
        "FFFFFF",
        [[255, 255, 255], [160, 160, 160], [128, 128, 128]],
      ),
      ("010101", [[86, 86, 86], [22, 22, 22], [1, 1, 1]]),
      ("101010", [[100, 100, 100], [29, 29, 29], [15, 15, 15]]),
      ("3F3F3F", [[147, 147, 147], [73, 73, 73], [62, 62, 62]]),
      ("404040", [[106, 106, 106], [32, 32, 32], [21, 21, 21]]),
      ("808080", [[127, 127, 127], [53, 53, 53], [42, 42, 42]]),
      ("FF0000", [[255, 0, 0], [107, 0, 0], [85, 0, 0]]),
      ("19334D", [[75, 136, 197], [31, 62, 93], [26, 51, 77]]),
      ("99B3CC", [[153, 179, 204], [73, 104, 136], [63, 94, 124]]),
      ("336699", [[89, 145, 200], [35, 70, 105], [30, 60, 89]]),
    ] {
      for value in [
        w::BorderValues::ThreeDEmboss,
        w::BorderValues::ThreeDEngrave,
      ] {
        let border = super::super::table_border_style(value, Some(24), None, Some(color), None)
          .expect("colored table border");
        let actual = band_colors(border);
        let expected = if value == w::BorderValues::ThreeDEngrave {
          [expected[2], expected[1], expected[0]]
        } else {
          expected
        };
        for (actual, expected) in actual.into_iter().zip(expected) {
          let actual_rgb = [actual.r, actual.g, actual.b];
          if matches!(color, "auto" | "000000" | "FFFFFF") {
            assert_eq!(actual_rgb, expected);
          }
          // The native luminance roles, including the dark-color boundary,
          // are independent of RGB reconstruction. The shared floating HSL
          // converter does not reproduce every native saturation quantizer.
          let actual_hsl = HslColor::from_srgb8(actual_rgb);
          let expected_hsl = HslColor::from_srgb8(expected);
          assert!(
            (actual_hsl.lightness - expected_hsl.lightness).abs() <= 1.0 / 240.0,
            "{color}: {actual_rgb:?} != {expected:?}"
          );
          if expected_hsl.saturation > 0.1 {
            let hue_delta = (actual_hsl.hue_degrees - expected_hsl.hue_degrees).rem_euclid(360.0);
            assert!(hue_delta.min(360.0 - hue_delta) < 3.0);
          }
        }
      }
    }
  }
}
