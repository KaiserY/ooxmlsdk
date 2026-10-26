//! Word's legacy inline bitmap size, before fixed-output device mapping.
//!
//! MS-DOC PICF stores natural dimensions in twips and mx/my in tenths of a
//! percent. Native Word DOC readback agrees for independent 96/150/300/600-DPI
//! and anisotropic PNG/JPEG controls. Floating VML pictures bypass this conversion.

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

fn jpeg_goal_twips(data: &[u8]) -> Option<(f64, f64)> {
  let (unit, horizontal, vertical) = common::drawingml_image_tile::jpeg_jfif_density(data)?;
  // Native Word PICF readback uses 96 DPI for unitless or zero-density JFIF,
  // and the authored physical density otherwise. Read only the JPEG headers;
  // recovering the layout size must not decode the photographic sample plane.
  let density = match unit {
    0 => (96.0, 96.0),
    1 | 2 if horizontal == 0 || vertical == 0 => (96.0, 96.0),
    1 => (f64::from(horizontal), f64::from(vertical)),
    2 => (f64::from(horizontal) * 2.54, f64::from(vertical) * 2.54),
    _ => return None,
  };
  let (width, height) = common::drawingml_image_tile::jpeg_dimensions(data)?;
  let axis = |pixels: u32, dpi: f64| {
    let goal = (f64::from(pixels) * 1440.0 / dpi).round();
    (goal >= 1.0 && goal <= f64::from(i16::MAX)).then_some(goal)
  };
  Some((axis(width, density.0)?, axis(height, density.1)?))
}

pub(super) fn bitmap_goal_twips(data: &[u8]) -> Option<(f64, f64)> {
  if data.starts_with(&[0xff, 0xd8]) {
    jpeg_goal_twips(data)
  } else {
    png_goal_twips(data)
  }
}

fn scaled_dimension_twips(authored_pt: f32, goal_twips: f64) -> Option<f64> {
  let authored_twips = (f64::from(authored_pt) * 20.0).round();
  let scale = (authored_twips * 1000.0 / goal_twips).round();
  if !(1.0..=f64::from(u16::MAX)).contains(&scale) {
    return None;
  }
  Some(goal_twips * scale / 1000.0)
}

#[cfg(test)]
fn scaled_dimension_pt(authored_pt: f32, goal_twips: f64) -> Option<f32> {
  scaled_dimension_twips(authored_pt, goal_twips).map(|twips| (twips.round() / 20.0) as f32)
}

pub(super) struct InlineBitmapSize {
  pub(super) layout_pt: (f32, f32),
  pub(super) paint_pt: Option<(f32, f32)>,
}

pub(super) fn inline_bitmap_size(data: &[u8], width: f32, height: f32) -> Option<InlineBitmapSize> {
  let (goal_x, goal_y) = bitmap_goal_twips(data)?;
  let width_twips = scaled_dimension_twips(width, goal_x)?;
  let height_twips = scaled_dimension_twips(height, goal_y)?;
  Some(InlineBitmapSize {
    layout_pt: (
      (width_twips.round() / 20.0) as f32,
      (height_twips.round() / 20.0) as f32,
    ),
    // JPEG density controls distinguish the PICF product from the rounded
    // character extent: 576 * 13177 / 1000 twips paints 3162 printer dots,
    // whereas first rounding that product to twips would paint 3163 dots.
    // Preserve it until the final printer mapping; paragraph advance still
    // uses integral twips. PNG retains its separately established path.
    paint_pt: data
      .starts_with(&[0xff, 0xd8])
      .then_some(((width_twips / 20.0) as f32, (height_twips / 20.0) as f32)),
  })
}

/// The legacy inline producer consumes a completed origin in integral Word
/// twips, realizes origin and extent independently on the 600-DPI printer,
/// then exports the two endpoints in integral twips.
/// In particular, rounding a width alone loses the origin's fractional twip.
pub(super) fn inline_paint_bounds(mut bounds: common::Rect) -> common::Rect {
  let axis = |origin: f32, extent: f32| {
    let origin_twips = f64::from((origin * 20.0).round());
    let first = (origin_twips / 2.4).round();
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

  fn jpeg(unit: u8, horizontal: u16, vertical: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    image::codecs::jpeg::JpegEncoder::new(&mut bytes)
      .encode_image(&image::RgbImage::new(120, 106))
      .unwrap();
    assert_eq!(&bytes[6..11], b"JFIF\0");
    bytes[13] = unit;
    bytes[14..16].copy_from_slice(&horizontal.to_be_bytes());
    bytes[16..18].copy_from_slice(&vertical.to_be_bytes());
    bytes
  }

  #[test]
  fn vml_picture_jpeg_goals_and_scales_match_native_picf() {
    // Word's materialized DOC records, with the same authored 379.5x244.5pt
    // extent. Density changes the initial goals before mx/my quantization.
    for (unit, density, goals, expected) in [
      (1, [96, 96], [1800.0, 1590.0], [379.55, 244.45]),
      (1, [150, 150], [1152.0, 1018.0], [379.55, 244.5]),
      (1, [300, 300], [576.0, 509.0], [379.5, 244.5]),
      (1, [600, 600], [288.0, 254.0], [379.5, 244.5]),
      (1, [150, 300], [1152.0, 509.0], [379.55, 244.5]),
      (0, [1, 1], [1800.0, 1590.0], [379.55, 244.45]),
      (0, [96, 96], [1800.0, 1590.0], [379.55, 244.45]),
      (2, [96, 96], [709.0, 626.0], [379.5, 244.5]),
      (1, [0, 0], [1800.0, 1590.0], [379.55, 244.45]),
    ] {
      let data = jpeg(unit, density[0], density[1]);
      assert_eq!(bitmap_goal_twips(&data), Some((goals[0], goals[1])));
      assert_eq!(
        inline_bitmap_size(&data, 379.5, 244.5).map(|size| size.layout_pt),
        Some((expected[0], expected[1]))
      );
    }
    let data = jpeg(1, 96, 96);
    for (width, expected) in [
      (379.4, 379.45),
      (379.45, 379.45),
      (379.55, 379.55),
      (379.6, 379.6),
    ] {
      assert_eq!(
        inline_bitmap_size(&data, width, 244.5).map(|size| size.layout_pt),
        Some((expected, 244.45))
      );
    }
  }

  #[test]
  fn vml_picture_jpeg_paint_keeps_the_picf_product_until_printer_mapping() {
    let size = inline_bitmap_size(&jpeg(1, 300, 300), 379.5, 244.5).unwrap();
    assert_eq!(size.layout_pt, (379.5, 244.5));
    let (width, height) = size.paint_pt.unwrap();
    let bounds = inline_paint_bounds(common::Rect {
      origin: common::Point {
        x: common::Pt(85.1),
        y: common::Pt(76.8),
      },
      size: common::Size {
        width: common::Pt(width),
        height: common::Pt(height),
      },
    });
    assert_eq!(bounds.size.width.0, 379.4);
    assert_eq!(bounds.size.height.0, 244.45);
  }

  #[test]
  fn vml_picture_jpeg_import_keeps_floating_and_transformed_sizes() {
    use crate::docx::{ImageCatalog, package::ImageResource, v, vml_image_data};
    let mut catalog = ImageCatalog::default();
    catalog.by_relationship_id.insert(
      "rId1".into(),
      ImageResource {
        data: jpeg(1, 96, 96).into(),
        content_type: Some("image/jpeg".into()),
      },
    );
    let data = v::ImageData {
      relationship_id: Some("rId1".into()),
      ..Default::default()
    };
    for (prefix, expected) in [
      ("", (379.55, 244.45)),
      ("position:absolute;", (379.5, 244.5)),
      ("rotation:90;", (379.5, 244.5)),
      ("flip:x;", (379.5, 244.5)),
    ] {
      let style = format!("{prefix}width:379.5pt;height:244.5pt");
      let image = vml_image_data(&data, Some(&style), true, None, &catalog).unwrap();
      assert_eq!((image.width_pt, image.height_pt), expected);
    }
  }

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
        85.06773,
        76.725,
        379.53,
        244.4625,
        [85.1, 76.8, 379.55, 244.45],
      ),
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
