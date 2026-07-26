use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::model::ImageCrop;
use crate::units;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ImageTilePlacement {
  pub(crate) x_pt: f32,
  pub(crate) y_pt: f32,
  pub(crate) width_pt: f32,
  pub(crate) height_pt: f32,
  pub(crate) crop: ImageCrop,
  pub(crate) flip_horizontal: bool,
  pub(crate) flip_vertical: bool,
}

/// Expands a DrawingML bitmap tile into clipped page-space placements.
///
/// The tile origin is anchored by `algn`, then shifted by `tx`/`ty`. Alternating
/// flips use the logical row and column relative to that authored origin, so
/// clipping the first visible tile does not restart the flip sequence.
pub(crate) fn placements(
  frame: (f32, f32, f32, f32),
  natural_size_pt: (f32, f32),
  tile: &a::Tile,
  source_crop: ImageCrop,
  limit: usize,
) -> Vec<ImageTilePlacement> {
  let (frame_x, frame_y, frame_width, frame_height) = frame;
  if frame_width <= f32::EPSILON || frame_height <= f32::EPSILON || limit == 0 {
    return Vec::new();
  }
  // MS-OI29500 §20.1.8.55: negative srcRect values are outsets only for
  // non-tiled BLIPs. A tiled fill clamps those edges to the source boundary.
  let source_crop = ImageCrop {
    left: source_crop.left.max(0.0),
    top: source_crop.top.max(0.0),
    right: source_crop.right.max(0.0),
    bottom: source_crop.bottom.max(0.0),
  };
  let scale_x = tile
    .horizontal_ratio
    .as_ref()
    .map(|value| value.as_ratio() as f32)
    .unwrap_or(1.0);
  let scale_y = tile
    .vertical_ratio
    .as_ref()
    .map(|value| value.as_ratio() as f32)
    .unwrap_or(1.0);
  // ECMA-376 §20.1.8.58 defines sx/sy over srcRect, not the uncropped BLIP.
  let source_width_ratio = (1.0 - source_crop.left - source_crop.right).max(0.0);
  let source_height_ratio = (1.0 - source_crop.top - source_crop.bottom).max(0.0);
  let tile_width = (natural_size_pt.0 * source_width_ratio * scale_x.abs()).max(0.01);
  let tile_height = (natural_size_pt.1 * source_height_ratio * scale_y.abs()).max(0.01);
  let alignment = tile.alignment.unwrap_or_default();
  let anchor_x = match alignment {
    a::RectangleAlignmentValues::TopLeft
    | a::RectangleAlignmentValues::Left
    | a::RectangleAlignmentValues::BottomLeft => frame_x,
    a::RectangleAlignmentValues::Top
    | a::RectangleAlignmentValues::Center
    | a::RectangleAlignmentValues::Bottom => frame_x + (frame_width - tile_width) * 0.5,
    a::RectangleAlignmentValues::TopRight
    | a::RectangleAlignmentValues::Right
    | a::RectangleAlignmentValues::BottomRight => frame_x + frame_width - tile_width,
  };
  let anchor_y = match alignment {
    a::RectangleAlignmentValues::TopLeft
    | a::RectangleAlignmentValues::Top
    | a::RectangleAlignmentValues::TopRight => frame_y,
    a::RectangleAlignmentValues::Left
    | a::RectangleAlignmentValues::Center
    | a::RectangleAlignmentValues::Right => frame_y + (frame_height - tile_height) * 0.5,
    a::RectangleAlignmentValues::BottomLeft
    | a::RectangleAlignmentValues::Bottom
    | a::RectangleAlignmentValues::BottomRight => frame_y + frame_height - tile_height,
  };
  let origin_x = anchor_x
    + tile
      .horizontal_offset
      .map(|value| units::emu_to_points(value.to_emu()))
      .unwrap_or_default();
  let origin_y = anchor_y
    + tile
      .vertical_offset
      .map(|value| units::emu_to_points(value.to_emu()))
      .unwrap_or_default();
  let first_column = ((frame_x - origin_x) / tile_width).floor() as i64;
  let first_row = ((frame_y - origin_y) / tile_height).floor() as i64;
  let columns = ((frame_x + frame_width - (origin_x + first_column as f32 * tile_width))
    / tile_width)
    .ceil()
    .max(1.0) as usize;
  let rows = ((frame_y + frame_height - (origin_y + first_row as f32 * tile_height)) / tile_height)
    .ceil()
    .max(1.0) as usize;
  let mut result = Vec::with_capacity(columns.saturating_mul(rows).min(limit));
  for row_offset in 0..rows {
    let row = first_row + row_offset as i64;
    let tile_y = origin_y + row as f32 * tile_height;
    for column_offset in 0..columns {
      if result.len() == limit {
        return result;
      }
      let column = first_column + column_offset as i64;
      let tile_x = origin_x + column as f32 * tile_width;
      let visible_left = tile_x.max(frame_x);
      let visible_top = tile_y.max(frame_y);
      let visible_right = (tile_x + tile_width).min(frame_x + frame_width);
      let visible_bottom = (tile_y + tile_height).min(frame_y + frame_height);
      if visible_right <= visible_left || visible_bottom <= visible_top {
        continue;
      }
      let left_ratio = (visible_left - tile_x) / tile_width;
      let top_ratio = (visible_top - tile_y) / tile_height;
      let right_ratio = (tile_x + tile_width - visible_right) / tile_width;
      let bottom_ratio = (tile_y + tile_height - visible_bottom) / tile_height;
      let crop = ImageCrop {
        left: source_crop.left + left_ratio * source_width_ratio,
        top: source_crop.top + top_ratio * source_height_ratio,
        right: source_crop.right + right_ratio * source_width_ratio,
        bottom: source_crop.bottom + bottom_ratio * source_height_ratio,
      };
      let horizontal_alternate = matches!(
        tile.flip.unwrap_or_default(),
        a::TileFlipValues::Horizontal | a::TileFlipValues::HorizontalAndVertical
      ) && column.rem_euclid(2) != 0;
      let vertical_alternate = matches!(
        tile.flip.unwrap_or_default(),
        a::TileFlipValues::Vertical | a::TileFlipValues::HorizontalAndVertical
      ) && row.rem_euclid(2) != 0;
      result.push(ImageTilePlacement {
        x_pt: visible_left,
        y_pt: visible_top,
        width_pt: visible_right - visible_left,
        height_pt: visible_bottom - visible_top,
        crop,
        flip_horizontal: horizontal_alternate ^ (scale_x < 0.0),
        flip_vertical: vertical_alternate ^ (scale_y < 0.0),
      });
    }
  }
  result
}

pub(crate) fn rotate_placement_about_frame(
  mut placement: ImageTilePlacement,
  frame: (f32, f32, f32, f32),
  rotation_degrees: f32,
) -> ImageTilePlacement {
  if rotation_degrees.abs() <= f32::EPSILON {
    return placement;
  }
  let radians = rotation_degrees.to_radians();
  let (sin, cos) = radians.sin_cos();
  let frame_center_x = frame.0 + frame.2 * 0.5;
  let frame_center_y = frame.1 + frame.3 * 0.5;
  let tile_center_x = placement.x_pt + placement.width_pt * 0.5;
  let tile_center_y = placement.y_pt + placement.height_pt * 0.5;
  let relative_x = tile_center_x - frame_center_x;
  let relative_y = tile_center_y - frame_center_y;
  let rotated_center_x = frame_center_x + relative_x * cos - relative_y * sin;
  let rotated_center_y = frame_center_y + relative_x * sin + relative_y * cos;
  placement.x_pt = rotated_center_x - placement.width_pt * 0.5;
  placement.y_pt = rotated_center_y - placement.height_pt * 0.5;
  placement
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn alignment_offsets_and_flip_phase_survive_edge_clipping() {
    let tile = a::Tile {
      alignment: Some(a::RectangleAlignmentValues::Center),
      horizontal_offset: Some(ooxmlsdk::units::CoordinateValue::Emu(9_525)),
      flip: Some(a::TileFlipValues::Horizontal),
      ..Default::default()
    };
    let tiles = placements(
      (0.0, 0.0, 25.0, 10.0),
      (10.0, 10.0),
      &tile,
      ImageCrop::default(),
      16,
    );
    assert_eq!(tiles.len(), 3);
    assert_eq!(tiles[0].x_pt, 0.0);
    assert!(tiles[0].crop.left > 0.0);
    assert_ne!(tiles[0].flip_horizontal, tiles[1].flip_horizontal);
  }
}
