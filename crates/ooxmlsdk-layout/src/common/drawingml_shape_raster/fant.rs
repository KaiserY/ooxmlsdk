use image::{Rgba, RgbaImage};
use std::ops::Range;

struct SampleSpan {
  first: u32,
  weights: Range<usize>,
}

struct SampleAxis {
  spans: Vec<SampleSpan>,
  weights: Vec<f64>,
}

impl SampleAxis {
  fn new(source: u32, target: u32) -> Option<Self> {
    if target == 0 || target > source {
      return None;
    }
    let step = f64::from(source) / f64::from(target);
    let mut spans = Vec::with_capacity(target as usize);
    let mut weights = Vec::new();
    for destination in 0..target {
      let left = f64::from(destination) * step;
      let right = f64::from(destination + 1) * step;
      let first = left.floor() as u32;
      let last = (right.ceil() as u32).min(source);
      let start = weights.len();
      let fraction = left - f64::from(first);
      let carried = (right.min(f64::from(first + 1)) - left) * fraction;
      for sample in first..last {
        let mut weight = right.min(f64::from(sample + 1)) - left.max(f64::from(sample));
        // WIC Fant interpolates the retained first source sample before
        // integrating the next span. An ordinary box integral omits this
        // carry and changes both impulses and reduced checkerboards.
        if sample == first {
          weight -= carried;
        } else if sample == first + 1 {
          weight += carried;
        }
        weights.push(weight / step);
      }
      spans.push(SampleSpan {
        first,
        weights: start..weights.len(),
      });
    }
    Some(Self { spans, weights })
  }
}

/// Word's composed picture material uses WIC Fant before texture painting.
/// Retain both axes until the final byte resolve; rounding the horizontal
/// result first changes opaque RGB. Associated color keeps hidden source RGB
/// out of translucent texels without an intermediate full-size allocation.
pub(super) fn resize_premultiplied_rgba_fant(
  source: &RgbaImage,
  width: u32,
  height: u32,
) -> Option<RgbaImage> {
  let horizontal = SampleAxis::new(source.width(), width)?;
  let vertical = SampleAxis::new(source.height(), height)?;
  Some(RgbaImage::from_fn(width, height, |x, y| {
    let sx = &horizontal.spans[x as usize];
    let sy = &vertical.spans[y as usize];
    let mut alpha = 0.0;
    let mut associated = [0.0; 3];
    for (dy, &wy) in vertical.weights[sy.weights.clone()].iter().enumerate() {
      for (dx, &wx) in horizontal.weights[sx.weights.clone()].iter().enumerate() {
        let pixel = source.get_pixel(sx.first + dx as u32, sy.first + dy as u32);
        let weight = wx * wy * f64::from(pixel[3]);
        alpha += weight;
        for (sum, &channel) in associated.iter_mut().zip(&pixel.0[..3]) {
          *sum += weight * f64::from(channel);
        }
      }
    }
    if alpha <= f64::EPSILON {
      return Rgba([0; 4]);
    }
    let color = associated.map(|value| (value / alpha).round().clamp(0.0, 255.0) as u8);
    Rgba([
      color[0],
      color[1],
      color[2],
      alpha.round().clamp(0.0, 255.0) as u8,
    ])
  }))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn word_picture_fant_retains_native_boundary_impulse_weights() {
    // Actual public WIC Fant 17-to-5 response, independently identical to
    // the API used by Word's observed material preparation.
    let expected = [
      [75, 0, 0, 0, 0],
      [75, 0, 0, 0, 0],
      [75, 0, 0, 0, 0],
      [30, 27, 0, 0, 0],
      [0, 93, 0, 0, 0],
      [0, 75, 0, 0, 0],
      [0, 60, 3, 0, 0],
      [0, 0, 87, 0, 0],
      [0, 0, 75, 0, 0],
      [0, 0, 75, 0, 0],
      [0, 0, 15, 48, 0],
      [0, 0, 0, 87, 0],
      [0, 0, 0, 75, 0],
      [0, 0, 0, 45, 12],
      [0, 0, 0, 0, 93],
      [0, 0, 0, 0, 75],
      [0, 0, 0, 0, 75],
    ];
    for (sample, response) in expected.into_iter().enumerate() {
      let mut source = RgbaImage::from_pixel(17, 1, Rgba([0, 0, 0, 255]));
      source.put_pixel(sample as u32, 0, Rgba([255; 4]));
      let result = resize_premultiplied_rgba_fant(&source, 5, 1).unwrap();
      assert_eq!(result.pixels().map(|p| p[0]).collect::<Vec<_>>(), response);
      assert!(result.pixels().all(|p| p[3] == 255));
    }
  }

  #[test]
  fn word_picture_fant_resolves_minification_and_associated_alpha() {
    let source = RgbaImage::from_fn(420, 280, |x, y| {
      let value = ((x + y) % 2 * 255) as u8;
      Rgba([value, value, value, 255])
    });
    let result = resize_premultiplied_rgba_fant(&source, 210, 140).unwrap();
    assert!(result.pixels().all(|p| *p == Rgba([128, 128, 128, 255])));
    let source = RgbaImage::from_fn(2, 1, |x, _| {
      if x == 0 {
        Rgba([255, 0, 0, 255])
      } else {
        Rgba([0, 0, 255, 0])
      }
    });
    assert_eq!(
      resize_premultiplied_rgba_fant(&source, 1, 1).unwrap()[(0, 0)],
      Rgba([255, 0, 0, 128])
    );
    assert!(resize_premultiplied_rgba_fant(&source, 0, 1).is_none());
    assert!(resize_premultiplied_rgba_fant(&source, 3, 1).is_none());
  }
}
