//! Word's Cartesian 3-D scene uses independently projected raster tiles.
//!
//! Native GFX Print captures and its tile iterator establish a 512-pixel
//! limit, balanced tile counts and independent integer device/pixel extents.
//! An odd 513-row image has 257 rows in its first tile and 256 in its second;
//! stretching the entire viewport uniformly moves edges by half a pixel.

use crate::common::drawingml_shape_raster::{
  PageToRasterMapping, rasterize_vector_scene_at_mapping,
};
use crate::common::{DisplayItem, Rect};
use image::RgbaImage;

const MAXIMUM_TILE_EXTENT: u32 = 512;

#[derive(Clone, Copy, Debug)]
struct Tile {
  x: u32,
  y: u32,
  mapping: PageToRasterMapping,
}

fn tiles(
  viewport: Rect,
  printer_size: (u32, u32),
  mapping: PageToRasterMapping,
) -> Option<Vec<Tile>> {
  let (width, height) = (mapping.width_px, mapping.height_px);
  if width == 0
    || height == 0
    || printer_size.0 == 0
    || printer_size.1 == 0
    || !viewport.size.width.0.is_finite()
    || viewport.size.width.0 <= 0.0
    || !viewport.size.height.0.is_finite()
    || viewport.size.height.0 <= 0.0
  {
    return None;
  }
  let columns = width.div_ceil(MAXIMUM_TILE_EXTENT);
  let rows = height.div_ceil(MAXIMUM_TILE_EXTENT);
  let pixel_step = (width.div_ceil(columns), height.div_ceil(rows));
  let printer_step = (
    printer_size.0.div_ceil(columns),
    printer_size.1.div_ceil(rows),
  );
  let mut tiles = Vec::new();
  for row in 0..rows {
    for column in 0..columns {
      let x = column * pixel_step.0;
      let y = row * pixel_step.1;
      let printer_x = column * printer_step.0;
      let printer_y = row * printer_step.1;
      let width_px = pixel_step.0.min(width - x);
      let height_px = pixel_step.1.min(height - y);
      let printer_width = printer_step.0.min(printer_size.0.checked_sub(printer_x)?);
      let printer_height = printer_step.1.min(printer_size.1.checked_sub(printer_y)?);
      if printer_width == 0 || printer_height == 0 {
        return None;
      }
      let logical_x =
        viewport.origin.x.0 + viewport.size.width.0 * (printer_x as f32 / printer_size.0 as f32);
      let logical_y =
        viewport.origin.y.0 + viewport.size.height.0 * (printer_y as f32 / printer_size.1 as f32);
      let scale_x =
        width_px as f32 / (viewport.size.width.0 * (printer_width as f32 / printer_size.0 as f32));
      let scale_y = height_px as f32
        / (viewport.size.height.0 * (printer_height as f32 / printer_size.1 as f32));
      tiles.push(Tile {
        x,
        y,
        mapping: PageToRasterMapping {
          width_px,
          height_px,
          scale_x,
          scale_y,
          translate_x: -logical_x * scale_x,
          translate_y: -logical_y * scale_y,
          text_hinting: mapping.text_hinting,
        },
      });
    }
  }
  Some(tiles)
}

pub(super) fn rasterize(
  items: &[DisplayItem<'static>],
  viewport: Rect,
  printer_size: (u32, u32),
  mapping: PageToRasterMapping,
) -> Option<RgbaImage> {
  if mapping.width_px <= MAXIMUM_TILE_EXTENT && mapping.height_px <= MAXIMUM_TILE_EXTENT {
    return rasterize_vector_scene_at_mapping(items, mapping);
  }
  let mut output = RgbaImage::new(mapping.width_px, mapping.height_px);
  for tile in tiles(viewport, printer_size, mapping)? {
    let image = rasterize_vector_scene_at_mapping(items, tile.mapping)?;
    image::imageops::replace(&mut output, &image, i64::from(tile.x), i64::from(tile.y));
  }
  Some(output)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn mapping(viewport: Rect, width: u32, height: u32) -> PageToRasterMapping {
    let scale_x = width as f32 / viewport.size.width.0;
    let scale_y = height as f32 / viewport.size.height.0;
    PageToRasterMapping {
      width_px: width,
      height_px: height,
      scale_x,
      scale_y,
      translate_x: -viewport.origin.x.0 * scale_x,
      translate_y: -viewport.origin.y.0 * scale_y,
      text_hinting: None,
    }
  }

  #[test]
  fn word_chart_tiles_match_native_device_and_target_rectangles() {
    // Independent GFX Rasterizer draws: device rects 846,692..3624,2492
    // and 1299,3061..3632,4599. SetTargetRect contains these pixel bounds.
    for (printer_size, image_size, expected) in [
      (
        (2778, 1800),
        (926, 601),
        [
          (0, 0, 463, 301),
          (463, 0, 463, 301),
          (0, 301, 463, 300),
          (463, 301, 463, 300),
        ],
      ),
      (
        (2333, 1538),
        (778, 513),
        [
          (0, 0, 389, 257),
          (389, 0, 389, 257),
          (0, 257, 389, 256),
          (389, 257, 389, 256),
        ],
      ),
    ] {
      let viewport = crate::model::common_rect(101.0, 83.0, 333.0, 216.0);
      let tiles = tiles(
        viewport,
        printer_size,
        mapping(viewport, image_size.0, image_size.1),
      )
      .unwrap();
      let actual: Vec<_> = tiles
        .iter()
        .map(|t| (t.x, t.y, t.mapping.width_px, t.mapping.height_px))
        .collect();
      assert_eq!(actual, expected);
      // The native lower-left tile maps the viewport midpoint to its own
      // origin, then its quarter point to exactly half of the final tile.
      let lower = tiles[2];
      let at = |fraction: f32| {
        (viewport.origin.y.0 + viewport.size.height.0 * fraction) * lower.mapping.scale_y
          + lower.mapping.translate_y
          + lower.y as f32
      };
      assert!((at(0.5) - expected[2].1 as f32).abs() < 0.001);
      assert!((at(0.75) - (expected[2].1 as f32 + expected[2].3 as f32 * 0.5)).abs() < 0.001);
    }
  }

  #[test]
  fn word_chart_tile_resolve_has_no_seam_on_a_translated_opaque_wall() {
    let viewport = crate::model::common_rect(-81.3, 17.9, 400.0, 230.0);
    let item = DisplayItem::Rect(crate::common::RectItem {
      bounds: viewport,
      fill: crate::common::Fill::Solid(crate::common::Color {
        r: 33,
        g: 66,
        b: 99,
        a: 255,
      }),
      ..Default::default()
    });
    let image = rasterize(
      &[item],
      viewport,
      (3334, 1917),
      mapping(viewport, 1112, 640),
    )
    .unwrap();
    assert!(image.pixels().all(|pixel| pixel.0 == [33, 66, 99, 255]));
  }
}
