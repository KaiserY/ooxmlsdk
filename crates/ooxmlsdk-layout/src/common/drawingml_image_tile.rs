use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::model::ImageCrop;
use crate::units;

/// Physical size of an image tile. JPEG/JFIF density overrides the CSS-pixel
/// fallback; Word's VML tiles and PowerPoint's DrawingML tiles both use it.
pub(crate) fn natural_size_pt(data: &[u8]) -> Option<(f32, f32)> {
  let image = image::load_from_memory(data).ok()?;
  if let Some((horizontal_dpi, vertical_dpi)) = jpeg_density_dpi(data) {
    return Some((
      image.width() as f32 * units::POINTS_PER_INCH / horizontal_dpi,
      image.height() as f32 * units::POINTS_PER_INCH / vertical_dpi,
    ));
  }
  Some((
    image.width() as f32 * units::POINTS_PER_CSS_PIXEL,
    image.height() as f32 * units::POINTS_PER_CSS_PIXEL,
  ))
}

pub(crate) fn jpeg_density_dpi(data: &[u8]) -> Option<(f32, f32)> {
  let (unit, horizontal, vertical) = jpeg_jfif_density(data)?;
  if horizontal == 0 || vertical == 0 {
    return None;
  }
  let (horizontal, vertical) = (f32::from(horizontal), f32::from(vertical));
  match unit {
    1 => Some((horizontal, vertical)),
    2 => Some((horizontal * 2.54, vertical * 2.54)),
    _ => None,
  }
}

// Borrow header segments only. In contrast to image::JpegDecoder::new, this
// neither copies the complete compressed image nor reads its entropy payload.
fn jpeg_header_segments(data: &[u8]) -> impl Iterator<Item = (u8, &[u8])> {
  let mut rest = data.strip_prefix(&[0xff, 0xd8]).unwrap_or_default();
  std::iter::from_fn(move || {
    loop {
      if rest.first() != Some(&0xff) {
        return None;
      }
      let marker_index = rest.iter().position(|&byte| byte != 0xff)?;
      let marker = rest[marker_index];
      rest = &rest[marker_index + 1..];
      if matches!(marker, 0xda | 0xd9) {
        rest = &[];
        return None;
      }
      if matches!(marker, 0x01 | 0xd0..=0xd8) {
        continue;
      }
      let length = usize::from(u16::from_be_bytes([*rest.first()?, *rest.get(1)?]));
      let payload = rest.get(2..length)?;
      rest = &rest[length..];
      return Some((marker, payload));
    }
  })
}

pub(crate) fn jpeg_jfif_density(data: &[u8]) -> Option<(u8, u16, u16)> {
  jpeg_header_segments(data).find_map(|(marker, payload)| {
    if marker != 0xe0 || payload.len() < 12 || !payload.starts_with(b"JFIF\0") {
      return None;
    }
    Some((
      payload[7],
      u16::from_be_bytes([payload[8], payload[9]]),
      u16::from_be_bytes([payload[10], payload[11]]),
    ))
  })
}

pub(crate) fn jpeg_dimensions(data: &[u8]) -> Option<(u32, u32)> {
  jpeg_header_segments(data).find_map(|(marker, payload)| {
    // Baseline, extended sequential and progressive DCT share this frame
    // header. Lossless/hierarchical producers keep their existing fallback.
    if !matches!(marker, 0xc0..=0xc2) || payload.len() < 6 {
      return None;
    }
    let components = usize::from(payload[5]);
    if components == 0 || payload.len() < 6 + 3 * components {
      return None;
    }
    let height = u16::from_be_bytes([payload[1], payload[2]]);
    let width = u16::from_be_bytes([payload[3], payload[4]]);
    (width != 0 && height != 0).then_some((u32::from(width), u32::from(height)))
  })
}

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
  #[test]
  fn jpeg_metadata_is_available_without_reading_the_entropy_stream() {
    let mut bytes = Vec::new();
    image::codecs::jpeg::JpegEncoder::new(&mut bytes)
      .encode_image(&image::RgbImage::new(120, 106))
      .unwrap();
    let scan = bytes
      .windows(2)
      .position(|pair| pair == [0xff, 0xda])
      .unwrap();
    let headers = &bytes[..scan];
    assert_eq!(super::jpeg_dimensions(headers), Some((120, 106)));
    assert_eq!(super::jpeg_jfif_density(headers), Some((0, 1, 1)));
    assert_eq!(super::jpeg_density_dpi(headers), None);
    bytes[13] = 1;
    bytes[14..16].copy_from_slice(&150_u16.to_be_bytes());
    bytes[16..18].copy_from_slice(&300_u16.to_be_bytes());
    assert_eq!(
      super::jpeg_density_dpi(&bytes[..scan]),
      Some((150.0, 300.0))
    );
  }

  #[test]
  fn jpeg_metadata_rejects_truncated_segments_and_stops_at_scan_data() {
    for data in [
      &[0xff, 0xd8, 0xff][..],
      &[0xff, 0xd8, 0xff, 0xc0, 0, 1][..],
      &[0xff, 0xd8, 0xff, 0xc0, 0, 17, 8, 0, 106][..],
      &[
        0xff, 0xd8, 0xff, 0xda, 0xff, 0xc0, 0, 11, 8, 0, 106, 0, 120, 1, 1, 0x11, 0,
      ][..],
    ] {
      assert_eq!(super::jpeg_dimensions(data), None);
      assert_eq!(super::jpeg_jfif_density(data), None);
    }
  }

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
