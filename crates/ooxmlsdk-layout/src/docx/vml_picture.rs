//! Word's legacy inline bitmap size, before fixed-output device mapping.
//!
//! MS-DOC PICF stores natural dimensions in twips and mx/my in tenths of a
//! percent. Native Word DOC readback agrees for independent 96/150/300/600-DPI
//! and anisotropic PNG controls. Floating VML pictures bypass this conversion.

use std::io::Cursor;

use crate::common;

pub(super) fn png_goal_twips(data: &[u8]) -> Option<(f64, f64)> {
  let reader = png::Decoder::new(Cursor::new(data)).read_info().ok()?;
  let info = reader.info();
  let density = info.pixel_dims?;
  if density.unit != png::Unit::Meter || density.xppu == 0 || density.yppu == 0 {
    return None;
  }
  let axis = |pixels, ppu| {
    let goal = (f64::from(pixels) * 1440.0 / (f64::from(ppu) * 0.0254)).round();
    (goal >= 1.0 && goal <= f64::from(i16::MAX)).then_some(goal)
  };
  Some((
    axis(info.width, density.xppu)?,
    axis(info.height, density.yppu)?,
  ))
}

fn scaled_dimension_pt(authored_pt: f32, goal_twips: f64) -> Option<f32> {
  let authored_twips = (f64::from(authored_pt) * 20.0).round();
  let scale = (authored_twips * 1000.0 / goal_twips).round();
  if !(1.0..=f64::from(u16::MAX)).contains(&scale) {
    return None;
  }
  Some(((goal_twips * scale / 1000.0).round() / 20.0) as f32)
}

pub(super) fn inline_png_size(data: &[u8], width: f32, height: f32) -> Option<(f32, f32)> {
  let (goal_x, goal_y) = png_goal_twips(data)?;
  Some((
    scaled_dimension_pt(width, goal_x)?,
    scaled_dimension_pt(height, goal_y)?,
  ))
}

/// The legacy inline producer realizes origins and extents independently on
/// the 600-DPI printer, then exports the two endpoints in integral twips.
/// In particular, rounding a width alone loses the origin's fractional twip.
pub(super) fn inline_paint_bounds(mut bounds: common::Rect) -> common::Rect {
  let axis = |origin: f32, extent: f32| {
    let first = (f64::from(origin) / 0.12).round();
    let last = first + (f64::from(extent) / 0.12).round();
    let first_twips = (first * 2.4).round();
    let last_twips = (last * 2.4).round();
    (
      (first_twips / 20.0) as f32,
      ((last_twips - first_twips) / 20.0) as f32,
    )
  };
  (bounds.origin.x.0, bounds.size.width.0) = axis(bounds.origin.x.0, bounds.size.width.0);
  (bounds.origin.y.0, bounds.size.height.0) = axis(bounds.origin.y.0, bounds.size.height.0);
  bounds
}

#[cfg(test)]
mod tests {
  use super::*;

  fn png(width: u32, height: u32, density: Option<png::PixelDimensions>) -> Vec<u8> {
    let mut bytes = Vec::new();
    {
      let mut encoder = png::Encoder::new(&mut bytes, width, height);
      encoder.set_color(png::ColorType::Grayscale);
      encoder.set_depth(png::BitDepth::Eight);
      encoder.set_pixel_dims(density);
      encoder
        .write_header()
        .unwrap()
        .write_image_data(&vec![0; (width * height) as usize])
        .unwrap();
    }
    bytes
  }

  #[test]
  fn vml_picture_physical_png_goals_match_native_density_controls() {
    for (ppu, right, left) in [
      ([5905, 5905], [3322.0, 2813.0], [2861.0, 2554.0]),
      ([3780, 3780], [5189.0, 4394.0], [4469.0, 3990.0]),
      ([5906, 5906], [3321.0, 2813.0], [2861.0, 2553.0]),
      ([11811, 11811], [1661.0, 1406.0], [1430.0, 1277.0]),
      ([23622, 23622], [830.0, 703.0], [715.0, 638.0]),
      ([5905, 11811], [3322.0, 1406.0], [2861.0, 1277.0]),
    ] {
      let density = Some(png::PixelDimensions {
        xppu: ppu[0],
        yppu: ppu[1],
        unit: png::Unit::Meter,
      });
      assert_eq!(
        png_goal_twips(&png(346, 293, density)),
        Some((right[0], right[1]))
      );
      assert_eq!(
        png_goal_twips(&png(298, 266, density)),
        Some((left[0], left[1]))
      );
    }
  }

  #[test]
  fn vml_picture_inline_import_preserves_floating_and_unknown_density() {
    use crate::docx::{ImageCatalog, package::ImageResource, v, vml_image_data};
    let mut catalog = ImageCatalog::default();
    for density in [
      None,
      Some(png::PixelDimensions {
        xppu: 5905,
        yppu: 5905,
        unit: png::Unit::Meter,
      }),
    ] {
      catalog.by_relationship_id.insert(
        "rId1".into(),
        ImageResource {
          data: png(298, 266, density).into(),
          content_type: Some("image/png".into()),
        },
      );
      let data = v::ImageData {
        relationship_id: Some("rId1".into()),
        ..Default::default()
      };
      for (prefix, expected) in [
        (
          "",
          if density.is_some() {
            (143.2, 127.2)
          } else {
            (143.15, 127.25)
          },
        ),
        ("position:absolute;", (143.15, 127.25)),
        ("rotation:90;", (143.15, 127.25)),
      ] {
        let style = format!("{prefix}width:143.15pt;height:127.25pt");
        let image = vml_image_data(&data, Some(&style), true, None, &catalog).unwrap();
        assert_eq!((image.width_pt, image.height_pt), expected);
      }
    }
    assert!(
      png_goal_twips(&png(
        1,
        1,
        Some(png::PixelDimensions {
          xppu: 5905,
          yppu: 5905,
          unit: png::Unit::Unspecified,
        })
      ))
      .is_none()
    );
  }

  #[test]
  fn vml_picture_native_picf_density_scales() {
    // Actual Word PICF records: goals and independently observed mx/my.
    for (goal, authored, scale, realized) in [
      (3322.0, 127.25, 766.0, 127.25),
      (2813.0, 127.25, 905.0, 127.3),
      (2861.0, 143.15, 1001.0, 143.2),
      (2554.0, 127.25, 996.0, 127.2),
      (3322.0, 127.35, 767.0, 127.4),
      (1406.0, 127.25, 1810.0, 127.25),
      (1277.0, 127.25, 1993.0, 127.25),
      (830.0, 127.25, 3066.0, 127.25),
      (638.0, 127.25, 3989.0, 127.25),
      (5189.0, 127.25, 490.0, 127.15),
    ] {
      assert_eq!(
        ((authored as f64 * 20.0).round() * 1000.0 / goal).round(),
        scale
      );
      assert_eq!(scaled_dimension_pt(authored, goal), Some(realized));
    }
  }

  #[test]
  fn vml_picture_native_printer_endpoints() {
    for (x, y, width, height, expected) in [
      (
        423.72998,
        118.39331,
        127.25,
        127.3,
        [423.7, 118.45, 127.2, 127.3],
      ),
      (
        36.033672,
        120.64331,
        143.2,
        127.2,
        [36.0, 120.6, 143.15, 127.2],
      ),
      (
        423.72998,
        118.32,
        127.4,
        127.3,
        [423.7, 118.3, 127.45, 127.35],
      ),
    ] {
      let actual = inline_paint_bounds(common::Rect {
        origin: common::Point {
          x: common::Pt(x),
          y: common::Pt(y),
        },
        size: common::Size {
          width: common::Pt(width),
          height: common::Pt(height),
        },
      });
      assert_eq!(
        [
          actual.origin.x.0,
          actual.origin.y.0,
          actual.size.width.0,
          actual.size.height.0
        ],
        expected
      );
    }
  }
}
