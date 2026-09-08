//! Independent-axis extension of the existing `image` Gaussian contract.
//!
//! `image` 0.25.10 exposes arbitrary sigma only for equal axes. Its
//! anisotropic constructor derives sigma from an integer kernel size, so it
//! cannot preserve authored, continuously scaled deviations. Retain its
//! support selection, normalized f32 taps, symmetric accumulation order,
//! clamped border and floating intermediate here. This supplies a mapping
//! capability; it does not establish a different Office kernel policy.

use image::RgbaImage;

fn kernel(sigma: f32) -> Vec<f32> {
  if !sigma.is_finite() || sigma <= f32::EPSILON {
    return vec![1.0];
  }
  let size = (((((sigma - 0.8) / 0.3) + 1.0) * 2.0) + 1.0).max(3.0) as usize;
  let size = size | 1;
  let centre = (size / 2) as f32;
  let scale = 1.0 / ((2.0 * std::f32::consts::PI).sqrt() * sigma);
  let mut weights = Vec::with_capacity(size);
  let mut sum = 0.0_f32;
  for x in 0..size {
    let weight = (-0.5 * ((x as f32 - centre) / sigma).powf(2.0)).exp() * scale;
    weights.push(weight);
    sum += weight;
  }
  if sum != 0.0 {
    let reciprocal = 1.0 / sum;
    for weight in &mut weights {
      *weight *= reciprocal;
    }
  }
  let zero_pairs = weights[..size / 2]
    .iter()
    .take_while(|&&v| v == 0.0)
    .count();
  weights[zero_pairs..size - zero_pairs].to_vec()
}

#[inline]
fn accumulate(sum: f32, value: f32, weight: f32) -> f32 {
  // Match the portable image backend's target-feature decision, not a
  // runtime-dependent reordering of the taps.
  #[cfg(any(
    all(
      any(target_arch = "x86", target_arch = "x86_64"),
      target_feature = "fma"
    ),
    all(target_arch = "aarch64", target_feature = "neon")
  ))]
  {
    value.mul_add(weight, sum)
  }
  #[cfg(not(any(
    all(
      any(target_arch = "x86", target_arch = "x86_64"),
      target_feature = "fma"
    ),
    all(target_arch = "aarch64", target_feature = "neon")
  )))]
  {
    sum + value * weight
  }
}

/// The input is already associated RGBA; neither pass changes association.
pub(super) fn blur_associated_rgba_xy(source: &RgbaImage, sigma_x: f32, sigma_y: f32) -> RgbaImage {
  RgbaImage::from_raw(
    source.width(),
    source.height(),
    blur_channels_xy::<4>(
      source.as_raw(),
      source.width() as usize,
      source.height() as usize,
      sigma_x,
      sigma_y,
    ),
  )
  .expect("Gaussian preserves RGBA dimensions")
}

pub(super) fn blur_gray_xy(
  source: &image::GrayImage,
  sigma_x: f32,
  sigma_y: f32,
) -> image::GrayImage {
  image::GrayImage::from_raw(
    source.width(),
    source.height(),
    blur_channels_xy::<1>(
      source.as_raw(),
      source.width() as usize,
      source.height() as usize,
      sigma_x,
      sigma_y,
    ),
  )
  .expect("Gaussian preserves alpha dimensions")
}

fn blur_channels_xy<const N: usize>(
  source: &[u8],
  width: usize,
  height: usize,
  sigma_x: f32,
  sigma_y: f32,
) -> Vec<u8> {
  if width == 0 || height == 0 {
    return source.to_vec();
  }
  let kx = kernel(sigma_x);
  let ky = kernel(sigma_y);
  let rx = kx.len() / 2;
  let ry = ky.len() / 2;
  let mut horizontal = vec![[0.0_f32; N]; width * height];
  for (y, row) in horizontal.chunks_exact_mut(width).enumerate() {
    for (x, output) in row.iter_mut().enumerate() {
      let centre = (y * width + x) * N;
      *output = std::array::from_fn(|c| f32::from(source[centre + c]) * kx[rx]);
      for (i, &weight) in kx[..rx].iter().enumerate() {
        let distance = rx - i;
        let left = (y * width + x.saturating_sub(distance)) * N;
        let right = (y * width + x.saturating_add(distance).min(width - 1)) * N;
        for channel in 0..N {
          output[channel] = accumulate(
            output[channel],
            f32::from(source[left + channel]) + f32::from(source[right + channel]),
            weight,
          );
        }
      }
    }
  }
  let mut bytes = vec![0_u8; source.len()];
  for (index, pixel) in bytes.chunks_exact_mut(N).enumerate() {
    let (x, y) = (index % width, index / width);
    let mut output = horizontal[y * width + x].map(|v| v * ky[ry]);
    for (i, &weight) in ky[..ry].iter().enumerate() {
      let distance = ry - i;
      let top = horizontal[y.saturating_sub(distance) * width + x];
      let bottom = horizontal[y.saturating_add(distance).min(height - 1) * width + x];
      for channel in 0..N {
        output[channel] = accumulate(output[channel], top[channel] + bottom[channel], weight);
      }
    }
    pixel.copy_from_slice(&output.map(|v| v.round().clamp(0.0, 255.0) as u8));
  }
  bytes
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gaussian_xy_isotropic_matches_existing_image_backend_exactly() {
    for (width, height) in [(1, 1), (1, 23), (23, 1), (2, 3), (17, 19), (67, 71)] {
      let source = RgbaImage::from_fn(width, height, |x, y| {
        let a = (x * 79 + y * 47 + 61) as u8;
        image::Rgba([(x * 31 + y * 7) as u8, (x * 3 + y * 43) as u8, 255 - a, a])
      });
      for sigma in [0.1, 0.8, 1.1, 2.4, 8.5, 17.0] {
        assert_eq!(
          blur_associated_rgba_xy(&source, sigma, sigma),
          image::imageops::blur(&source, sigma),
          "{width}x{height}, sigma={sigma}"
        );
      }
    }
  }

  #[test]
  fn gaussian_xy_zero_axis_does_not_spread_across_that_axis() {
    let mut source = RgbaImage::new(21, 21);
    source.put_pixel(10, 10, image::Rgba([255; 4]));
    for (sx, sy) in [(0.0, 2.4), (2.4, 0.0)] {
      let result = blur_associated_rgba_xy(&source, sx, sy);
      assert!(result.get_pixel(10, 10)[3] > 0);
      let mut support = 0;
      for (x, y, pixel) in result.enumerate_pixels() {
        if pixel[3] != 0 {
          assert!(if sx == 0.0 { x == 10 } else { y == 10 });
          support += 1;
        }
      }
      assert!(support > 1);
    }
    assert_eq!(blur_associated_rgba_xy(&source, 0.0, 0.0), source);
    // At this sigma the off-centre f32 taps are exactly zero. image
    // 0.25.10's trimmed one-tap ring-queue path skips destination row zero;
    // preserve the mathematical identity instead of reproducing that bug.
    assert_eq!(blur_associated_rgba_xy(&source, 0.01, 0.01), source);
  }

  #[test]
  fn gaussian_gray_matches_existing_and_rgba_channels() {
    for (width, height) in [(1, 1), (1, 23), (23, 1), (2, 3), (17, 19)] {
      let source = image::GrayImage::from_fn(width, height, |x, y| {
        image::Luma([(x * 79 + y * 47 + 61) as u8])
      });
      for sigma in [0.1, 0.8, 1.1, 2.4, 8.5, 17.0] {
        assert_eq!(
          blur_gray_xy(&source, sigma, sigma),
          image::imageops::blur(&source, sigma)
        );
      }
      let rgba = RgbaImage::from_fn(width, height, |x, y| {
        image::Rgba([source.get_pixel(x, y)[0]; 4])
      });
      for (sx, sy) in [(0.0, 2.4), (2.4, 0.0), (1.1, 2.4), (2.4, 1.1), (0.01, 0.01)] {
        let gray = blur_gray_xy(&source, sx, sy);
        let four = blur_associated_rgba_xy(&rgba, sx, sy);
        for (g, r) in gray.pixels().zip(four.pixels()) {
          assert_eq!([g[0]; 4], r.0);
        }
      }
    }
  }
}
