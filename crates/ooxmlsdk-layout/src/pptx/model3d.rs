//! Realization of the stored model preview, without loading the GLB viewer.
//!
//! MS-ODRAWXML 2.31.3.5 permits this cached representation. Section 2.31.3.7
//! distinguishes the visible graphic frame from the centered square camera
//! viewport. Neither is an ordinary picture stretch rectangle: the PNG has
//! its own physical pixel density. Office controls spanning model poses and
//! sizes expose the extra enlargement caused by fitting those pixels to the
//! selection frame. Preserve their physical size and sample the viewport once.
//! This does not regenerate a stale preview after model/camera/light changes.

use std::io::Cursor;

use bytes::Bytes;
use image::{ColorType, ImageEncoder, Rgba, RgbaImage, codecs::png::PngEncoder};

pub(crate) const BITMAP_CONTENT_TYPE: &str = "application/vnd.ooxmlsdk.powerpoint-model3d+png";
const MAX_RASTER_PIXELS: u64 = 16_777_216;
const POINTS_PER_METER: f64 = 72.0 / 0.0254;

/// Returns `None` for a missing physical density or an unrepresentable target;
/// callers retain the original preview rather than inventing a source DPI or
/// silently reducing the requested density to fit a pixel budget.
pub(crate) fn realize_object_preview(
  data: &[u8],
  original_png: &[u8],
  viewport_pt: [f64; 2],
  parent_scale: [f64; 2],
  dpi: f64,
) -> Option<Bytes> {
  if !dpi.is_finite()
    || dpi <= 0.0
    || viewport_pt
      .into_iter()
      .chain(parent_scale)
      .any(|v| !v.is_finite() || v <= 0.0)
  {
    return None;
  }
  let reader = png::Decoder::new(Cursor::new(original_png))
    .read_info()
    .ok()?;
  let info = reader.info();
  let density = info.pixel_dims?;
  if density.unit != png::Unit::Meter || density.xppu == 0 || density.yppu == 0 {
    return None;
  }
  let source_size = [info.width, info.height];
  if source_size.contains(&0) || u64::from(info.width) * u64::from(info.height) > MAX_RASTER_PIXELS
  {
    return None;
  }
  let target = viewport_pt.map(|points| (points * dpi / 72.0).floor().max(1.0));
  if target[0] * target[1] > MAX_RASTER_PIXELS as f64 {
    return None;
  }
  let target = target.map(|value| value as u32);
  let source_density = [density.xppu, density.yppu];
  let source_pt: [f64; 2] = std::array::from_fn(|axis| {
    f64::from(source_size[axis]) * POINTS_PER_METER / f64::from(source_density[axis])
      * parent_scale[axis]
  });
  if source_pt
    .into_iter()
    .any(|value| !value.is_finite() || value <= 0.0)
  {
    return None;
  }
  let source = image::load_from_memory(data).ok()?.to_rgba8();
  if source.dimensions() != (source_size[0], source_size[1]) {
    return None;
  }
  let horizontal = sampling_axis(source_size[0], source_pt[0], viewport_pt[0], target[0]);
  let vertical = sampling_axis(source_size[1], source_pt[1], viewport_pt[1], target[1]);
  let output = RgbaImage::from_fn(target[0], target[1], |x, y| {
    sample(&source, horizontal[x as usize], vertical[y as usize])
  });
  let mut png = Vec::new();
  PngEncoder::new(&mut png)
    .write_image(
      output.as_raw(),
      output.width(),
      output.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(png.into())
}

#[derive(Clone, Copy)]
struct AxisSample {
  indices: [Option<u32>; 2],
  weights: [f64; 2],
}

fn sampling_axis(source: u32, source_pt: f64, viewport_pt: f64, target: u32) -> Vec<AxisSample> {
  (0..target)
    .map(|destination| {
      // Both the stored PNG and the final viewport use pixel centers. Do not
      // stretch the source to the frame or independently snap its four edges.
      let point = (f64::from(destination) + 0.5) * viewport_pt / f64::from(target);
      let coordinate =
        (point - (viewport_pt - source_pt) / 2.0) * f64::from(source) / source_pt - 0.5;
      let first = coordinate.floor();
      let fraction = coordinate - first;
      AxisSample {
        indices: [first, first + 1.0]
          .map(|index| (index >= 0.0 && index < f64::from(source)).then_some(index as u32)),
        weights: [1.0 - fraction, fraction],
      }
    })
    .collect()
}

fn sample(source: &RgbaImage, horizontal: AxisSample, vertical: AxisSample) -> Rgba<u8> {
  let mut associated = [0.0; 4];
  for (y, wy) in vertical.indices.into_iter().zip(vertical.weights) {
    for (x, wx) in horizontal.indices.into_iter().zip(horizontal.weights) {
      let (Some(x), Some(y)) = (x, y) else { continue };
      let pixel = source.get_pixel(x, y);
      let weight = wx * wy;
      let alpha = f64::from(pixel[3]);
      associated[3] += alpha * weight;
      for (component, channel) in associated[..3].iter_mut().zip(&pixel.0[..3]) {
        *component += f64::from(*channel) * alpha / 255.0 * weight;
      }
    }
  }
  let associated = associated.map(|value| value.round_ties_even().clamp(0.0, 255.0) as u8);
  let alpha = u16::from(associated[3]);
  // The display-list PNG contract is straight alpha. Choose the integer
  // inverse that exactly round-trips each associated byte at PDF encoding;
  // this also keeps hidden RGB from leaking across transparent boundaries.
  let color = [associated[0], associated[1], associated[2]].map(|value| {
    (u16::from(value) * 255 + alpha / 2)
      .checked_div(alpha)
      .unwrap_or(0)
      .min(255) as u8
  });
  Rgba([color[0], color[1], color[2], associated[3]])
}

#[cfg(test)]
mod tests {
  use super::*;

  fn png(density: Option<png::PixelDimensions>) -> Vec<u8> {
    let mut data = Vec::new();
    {
      let mut encoder = png::Encoder::new(&mut data, 144, 72);
      encoder.set_color(png::ColorType::Rgba);
      encoder.set_depth(png::BitDepth::Eight);
      encoder.set_pixel_dims(density);
      let image = RgbaImage::from_pixel(144, 72, Rgba([25, 80, 170, 255]));
      encoder
        .write_header()
        .unwrap()
        .write_image_data(image.as_raw())
        .unwrap();
    }
    data
  }

  fn physical_png() -> Vec<u8> {
    png(Some(png::PixelDimensions {
      xppu: 5669,
      yppu: 5669,
      unit: png::Unit::Meter,
    }))
  }

  #[test]
  fn viewport_owns_output_grid_not_source_pixel_dimensions() {
    let data = physical_png();
    for (dpi, expected) in [(96.0, (192, 192)), (200.0, (400, 400))] {
      let bytes = realize_object_preview(&data, &data, [144.0; 2], [1.0; 2], dpi).unwrap();
      let image = image::load_from_memory(&bytes).unwrap().to_rgba8();
      assert_eq!(image.dimensions(), expected);
      assert_eq!(image.get_pixel(0, 0)[3], 0);
      assert_eq!(
        *image.get_pixel(expected.0 / 2, expected.1 / 2),
        Rgba([25, 80, 170, 255])
      );
    }
  }

  #[test]
  fn group_scale_applies_to_viewport_and_intrinsic_pixels_together() {
    let data = physical_png();
    let first = realize_object_preview(&data, &data, [144.0; 2], [1.0; 2], 96.0).unwrap();
    let second = realize_object_preview(&data, &data, [288.0; 2], [2.0; 2], 48.0).unwrap();
    assert_eq!(first, second);
  }

  #[test]
  fn missing_relative_or_invalid_density_does_not_invent_physical_size() {
    for density in [
      None,
      Some(png::PixelDimensions {
        xppu: 5669,
        yppu: 5669,
        unit: png::Unit::Unspecified,
      }),
      Some(png::PixelDimensions {
        xppu: 0,
        yppu: 5669,
        unit: png::Unit::Meter,
      }),
    ] {
      let data = png(density);
      assert!(realize_object_preview(&data, &data, [144.0; 2], [1.0; 2], 96.0).is_none());
    }
  }

  #[test]
  fn oversized_and_invalid_targets_do_not_silently_lower_density() {
    let data = physical_png();
    for (viewport, scale, dpi) in [
      ([100_000.0; 2], [1.0; 2], 96.0),
      ([f64::INFINITY; 2], [1.0; 2], 96.0),
      ([144.0; 2], [1.0; 2], f64::NAN),
      ([144.0; 2], [0.0, 1.0], 96.0),
    ] {
      assert!(realize_object_preview(&data, &data, viewport, scale, dpi).is_none());
    }
  }

  #[test]
  fn sampling_preserves_pixel_centers_and_associated_alpha() {
    let mut source = RgbaImage::from_pixel(2, 1, Rgba([255, 0, 255, 0]));
    source.put_pixel(0, 0, Rgba([30, 90, 180, 255]));
    let vertical = sampling_axis(1, 1.0, 1.0, 1)[0];
    let identity = sampling_axis(2, 2.0, 2.0, 2);
    assert_eq!(
      sample(&source, identity[0], vertical),
      *source.get_pixel(0, 0)
    );
    assert_eq!(sample(&source, identity[1], vertical), Rgba([0; 4]));
    let blended = sample(&source, sampling_axis(2, 2.0, 2.0, 1)[0], vertical);
    assert_eq!(blended[3], 128);
    for (channel, expected) in blended.0[..3].iter().zip([15, 45, 90]) {
      assert_eq!(
        (u16::from(*channel) * u16::from(blended[3]) + 127) / 255,
        expected
      );
    }
    // The source lies at the viewport center, not stretched over all samples.
    let padded = sampling_axis(2, 2.0, 6.0, 6);
    assert!(padded[0].indices.iter().all(Option::is_none));
    assert!(padded[5].indices.iter().all(Option::is_none));
  }
}
