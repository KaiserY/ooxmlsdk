//! Word Print's opaque RGB bitmap representation decision.
//!
//! Native Print observations separate the photographic histogram score, the
//! bounded BGR compression estimate, and the JPEG counting stream. Screen's
//! scanline chunks and a comparison of final PDF stream lengths implement a
//! different policy. The estimate never allocates a complete RGB color plane.

use image::RgbaImage;

use super::{RasterImageFormat, office_deflate_size};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Selection {
  Lossless,
  Jpeg,
  JpegTrial { limit: usize },
}

pub(super) fn select(image: &RgbaImage) -> Selection {
  // The photographic shortcut precedes the 64 KiB small-bitmap check.
  if photographic_score(image) >= 0.5 {
    return Selection::Jpeg;
  }
  let raw_bytes = u64::from(image.width()) * u64::from(image.height()) * 3;
  if raw_bytes < 64 * 1024 {
    return Selection::Lossless;
  }
  let Some(estimated) = estimated_lossless_size(image) else {
    return Selection::Jpeg;
  };
  if estimated < 20 * 1024 {
    return Selection::Lossless;
  }
  // The native conversion divides in single precision and truncates. A
  // successful JPEG trial must be strictly smaller than this counting limit.
  Selection::JpegTrial {
    limit: (estimated as f32 / 2.5) as usize,
  }
}

fn photographic_score(image: &RgbaImage) -> f32 {
  let count = u64::from(image.width()) * u64::from(image.height());
  if count == 0 {
    return 0.0;
  }
  let mut histograms = [[0_u64; 256]; 3];
  for pixel in image.pixels() {
    for (histogram, &value) in histograms.iter_mut().zip(&pixel.0[..3]) {
      histogram[usize::from(value)] += 1;
    }
  }
  let mean = count as f64 / 256.0;
  let mut transitions = 0_u32;
  let mut frequent = 0_u32;
  let mut top_ten = 0_u64;
  for histogram in &mut histograms {
    frequent += histogram
      .iter()
      .filter(|&&value| value as f64 > count as f64 * 0.1)
      .count() as u32;
    for pair in histogram.windows(2) {
      let before = pair[0] as f64;
      let after = pair[1] as f64;
      if mean > before && mean > after {
        continue;
      }
      if before == 0.0 || after == 0.0 || after / before > 10.0 || before / after > 10.0 {
        transitions += 1;
      }
    }
    histogram.sort_unstable();
    top_ten += histogram[246..].iter().sum::<u64>();
  }
  let coverage = top_ten as f64 / (3.0 * count as f64);
  let coverage_term = if coverage < 0.2 {
    1.0
  } else if coverage > 0.8 {
    0.0
  } else {
    (0.8 - coverage) / 0.6
  };
  let transition_mean = f64::from(transitions) / 3.0;
  let transition_term = if transition_mean <= 1.0 {
    1.0
  } else {
    ((1.0 - transition_mean) * 0.5).exp()
  };
  (0.25 * transition_term + 0.7 * coverage_term + 0.05 * (-f64::from(frequent) / 3.0 * 0.5).exp())
    as f32
}

// Win32 MulDiv rounds the 64-bit product to the nearest signed 32-bit result,
// including rounding a positive half upward. Invalid/overflow estimates take
// the native failure branch rather than inventing a zero compression cost.
fn mul_div(number: usize, numerator: usize, denominator: usize) -> Option<usize> {
  if denominator == 0
    || number > i32::MAX as usize
    || numerator > i32::MAX as usize
    || denominator > i32::MAX as usize
  {
    return None;
  }
  let value = (number as u64 * numerator as u64 + denominator as u64 / 2) / denominator as u64;
  (value <= i32::MAX as u64).then_some(value as usize)
}

fn bgr_sample(image: &RgbaImage) -> Option<Vec<u8>> {
  let row_bytes = (image.width() as usize).checked_mul(3)?;
  let sample_width = row_bytes.min(512);
  let groups = (image.height() as usize / 8).min(16);
  if sample_width == 0 || groups == 0 {
    return None;
  }
  let mut sample = Vec::with_capacity(sample_width * groups * 8);
  let pixels = image.as_raw();
  for group in 0..groups {
    let y = mul_div(group, image.height() as usize, groups)?;
    let x = if groups == 1 {
      0
    } else {
      let step = if group % 2 == 0 {
        group
      } else {
        groups - group
      };
      mul_div(step, row_bytes - sample_width, groups - 1)?
    };
    for row in y..y + 8 {
      for byte in x..x + sample_width {
        let pixel = row * image.width() as usize + byte / 3;
        sample.push(*pixels.get(pixel * 4 + 2 - byte % 3)?);
      }
    }
  }
  Some(sample)
}

pub(super) fn estimated_lossless_size(image: &RgbaImage) -> Option<usize> {
  let sample = bgr_sample(image)?;
  let size = office_deflate_size::compressed_size_mem7(&sample);
  let raw_bytes = (image.width() as usize)
    .checked_mul(image.height() as usize)?
    .checked_mul(3)?;
  mul_div(size, raw_bytes, sample.len())
}

/// Count the source properties retained by GDI+'s JPEG trial, independently
/// of the final PDF JPEG (which drops them). Uncovered metadata and pixel
/// models retain the existing owner policy until their native contract is
/// established; in particular this is not a blanket TIFF-lossless exception.
pub(super) fn trial_metadata_size(data: &[u8], format: RasterImageFormat) -> Option<usize> {
  match format {
    RasterImageFormat::Png => png_trial_metadata_size(data),
    RasterImageFormat::Tiff => tiff_trial_metadata_size(data),
    _ => None,
  }
}

fn png_trial_metadata_size(data: &[u8]) -> Option<usize> {
  if data.get(..8)? != b"\x89PNG\r\n\x1a\n" {
    return None;
  }
  let mut position = 8_usize;
  let mut compatible_header = false;
  let mut gamma = false;
  let mut srgb = false;
  loop {
    let header = data.get(position..position.checked_add(8)?)?;
    let length = u32::from_be_bytes(header[..4].try_into().ok()?) as usize;
    let end = position.checked_add(12)?.checked_add(length)?;
    let chunk = data.get(position..end)?;
    match &header[4..8] {
      b"IHDR"
        if length == 13
          && ((chunk[16] == 8 && matches!(chunk[17], 0 | 2))
            || (matches!(chunk[16], 1 | 2 | 4 | 8) && chunk[17] == 3)) =>
      {
        compatible_header = true;
      }
      // The decoded opaque palette has the same Print decision as its RGB
      // counterpart. The palette itself is not a GDI+ JPEG property item.
      b"PLTE" if length > 0 && length <= 768 && length.is_multiple_of(3) => {}
      b"IDAT" => {}
      b"pHYs" if length == 9 => {}
      b"gAMA" if length == 4 => gamma = true,
      b"sRGB" if length == 1 && chunk[8] <= 3 => srgb = true,
      // GDI+ supplies pixel-unit and X/Y density properties even when pHYs is
      // absent. Their three inline IFD values form a 60-byte Exif segment.
      b"IEND" if compatible_header && length == 0 => {
        // Gamma adds an IFD entry and an external eight-byte rational.
        // GDI+ also synthesizes that gamma property from sRGB alone; the
        // rendering-intent byte adds one further inline IFD entry. Native
        // PNG/property/JPEG trials therefore cost60/80/92, not60/80/72.
        return Some(60 + usize::from(gamma || srgb) * 20 + usize::from(srgb) * 12);
      }
      _ => return None,
    }
    position = end;
  }
}

fn tiff_trial_metadata_size(data: &[u8]) -> Option<usize> {
  let little_endian = match data.get(..4)? {
    b"II\x2a\0" => true,
    b"MM\0\x2a" => false,
    _ => return None,
  };
  let read_u16 = |offset: usize| -> Option<u16> {
    let value = data.get(offset..offset.checked_add(2)?)?.try_into().ok()?;
    Some(if little_endian {
      u16::from_le_bytes(value)
    } else {
      u16::from_be_bytes(value)
    })
  };
  let read_u32 = |offset: usize| -> Option<u32> {
    let value = data.get(offset..offset.checked_add(4)?)?.try_into().ok()?;
    Some(if little_endian {
      u32::from_le_bytes(value)
    } else {
      u32::from_be_bytes(value)
    })
  };
  let directory = read_u32(4)? as usize;
  let count = usize::from(read_u16(directory)?);
  let entries = directory.checked_add(2)?;
  let next = entries.checked_add(count.checked_mul(12)?)?;
  if read_u32(next)? != 0 {
    return None;
  }
  // APP1 marker/length + Exif signature + TIFF header + IFD count/next link.
  let mut size = 24_usize.checked_add(count.checked_mul(12)?)?;
  for index in 0..count {
    let entry = entries + index * 12;
    let tag = read_u16(entry)?;
    if !matches!(tag, 0xfe | 0x100..=0x103 | 0x106 | 0x111 | 0x115..=0x117 | 0x11a..=0x11c | 0x128 | 0x13d)
    {
      return None;
    }
    let component_bytes = match read_u16(entry + 2)? {
      3 => 2,
      4 => 4,
      5 => 8,
      _ => return None,
    };
    let length = (read_u32(entry + 4)? as usize).checked_mul(component_bytes)?;
    if length > 4 {
      let offset = read_u32(entry + 8)? as usize;
      data.get(offset..offset.checked_add(length)?)?;
      size = size.checked_add(length)?;
    }
  }
  (size <= u16::MAX as usize + 2).then_some(size)
}

#[cfg(test)]
mod tests {
  use image::{ImageEncoder, Rgba, codecs::png::PngEncoder};

  use super::super::{DirectRasterEncoding, RasterExportOptions, decode_image};
  use super::*;
  use crate::PdfOptions;
  use crate::options::{PdfDocumentKind, PdfImageOptimizationPolicy, PdfStandard};

  fn line_art(width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_fn(width, height, |x, y| {
      let index = x + y * width;
      let value = index.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
      Rgba([
        if index.is_multiple_of(10) {
          value as u8
        } else {
          32
        },
        if (index / 10).is_multiple_of(10) {
          (value >> 8) as u8
        } else {
          96
        },
        (value >> 16) as u8,
        255,
      ])
    })
  }

  #[test]
  fn print_line_art_matches_native_score_sample_cost_and_small_bitmap_boundary() {
    // Independently captured GDI+ scores/estimates, including a non-palette
    // image just above 64 KiB which the former histogram-only gate rejected.
    let small = line_art(154, 141);
    let large = line_art(155, 142);
    assert_eq!(photographic_score(&small).to_bits(), 1_055_644_360);
    assert_eq!(photographic_score(&large).to_bits(), 1_055_639_687);
    let sample = bgr_sample(&large).unwrap();
    assert_eq!(sample.len(), 59_520);
    assert_eq!(office_deflate_size::compressed_size_mem7(&sample), 17_502);
    assert_eq!(estimated_lossless_size(&large), Some(19_416));
    assert_eq!(select(&small), Selection::Lossless);
    assert_eq!(select(&large), Selection::Lossless);
  }

  #[test]
  fn print_sample_bounds_follow_rounded_byte_offsets_and_eight_row_groups() {
    assert_eq!(mul_div(1, 5, 2), Some(3));
    assert_eq!(mul_div(1, 1, 0), None);
    assert_eq!(mul_div(i32::MAX as usize, 2, 1), None);
    assert!(bgr_sample(&line_art(4096, 7)).is_none());
    for (width, height, expected_bytes) in [(2, 8, 48), (193, 114, 57_344), (301, 200, 65_536)] {
      let image = line_art(width, height);
      let sampled = bgr_sample(&image).unwrap();
      assert_eq!(sampled.len(), expected_bytes);
      assert_eq!(&sampled[..3], &[110, 243, 95]);
    }
  }

  #[test]
  fn rgb_png_source_dispatch_preserves_native_line_art_in_both_print_qualities() {
    for archival in [false, true] {
      let mut options = PdfOptions::default();
      options.images.optimization_policy =
        PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
      if archival {
        options.standards.push(PdfStandard::PdfA3a);
      }
      for (width, height) in [(154, 141), (155, 142)] {
        let source = image::DynamicImage::ImageRgba8(line_art(width, height)).into_rgb8();
        let mut encoded = Vec::new();
        PngEncoder::new(&mut encoded)
          .write_image(
            source.as_raw(),
            width,
            height,
            image::ColorType::Rgb8.into(),
          )
          .unwrap();
        assert_eq!(
          trial_metadata_size(&encoded, RasterImageFormat::Png),
          Some(60)
        );
        let actual = decode_image(
          &encoded,
          Some("image/png"),
          RasterExportOptions::new(
            &options,
            width as f32 * 72.0 / 220.0,
            height as f32 * 72.0 / 220.0,
          ),
          None,
        )
        .unwrap();
        let DirectRasterEncoding::Sampled { pixels } = &actual.direct().encoding else {
          panic!("native Print line art must retain its exact color samples");
        };
        assert_eq!(pixels.rgb.as_slice(), source.as_raw().as_slice());
        assert!(!actual.direct().interpolate);
      }
    }
  }

  #[test]
  fn print_palette_and_rgb_metadata_keep_native_codec_and_trial_properties() {
    // Native same-pixel PNG quartets retain the photo/line-art decision
    // independently of PLTE, gAMA and sRGB. Public GDI+ trials retain an
    // implicit gamma property for sRGB, including without an authored gAMA.
    let palette: Vec<u8> = (0_u8..=255)
      .flat_map(|value| [value, value.wrapping_mul(73), value.wrapping_mul(151)])
      .collect();
    for photograph in [false, true] {
      let indices: Vec<u8> = (0..64 * 64)
        .map(|index| {
          if photograph {
            ((index % 64 + 31 * (index / 64)) % 256) as u8
          } else {
            ((index / 8) % 2) as u8
          }
        })
        .collect();
      let rgb: Vec<u8> = indices
        .iter()
        .flat_map(|&index| {
          palette[usize::from(index) * 3..usize::from(index) * 3 + 3]
            .iter()
            .copied()
        })
        .collect();
      for indexed in [false, true] {
        for (gamma, srgb, property_bytes) in [
          (false, false, 60),
          (true, false, 80),
          (false, true, 92),
          (true, true, 92),
        ] {
          let mut encoded = Vec::new();
          {
            let mut encoder = png::Encoder::new(&mut encoded, 64, 64);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_color(if indexed {
              png::ColorType::Indexed
            } else {
              png::ColorType::Rgb
            });
            if indexed {
              encoder.set_palette(palette.clone());
            }
            if gamma {
              encoder.set_source_gamma(png::ScaledFloat::from_scaled(45_455));
            }
            if srgb {
              encoder.set_source_srgb(png::SrgbRenderingIntent::Perceptual);
            }
            encoder
              .write_header()
              .unwrap()
              .write_image_data(if indexed { &indices } else { &rgb })
              .unwrap();
          }
          assert_eq!(
            trial_metadata_size(&encoded, RasterImageFormat::Png),
            Some(property_bytes)
          );
          let mut options = PdfOptions::default();
          options.images.optimization_policy =
            PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
          let actual = decode_image(
            &encoded,
            Some("image/png"),
            RasterExportOptions::new(&options, 64.0 * 72.0 / 220.0, 64.0 * 72.0 / 220.0),
            None,
          )
          .unwrap();
          assert_eq!(actual.size(), (64, 64));
          if photograph {
            assert!(matches!(
              actual.direct().encoding,
              DirectRasterEncoding::Dct { .. }
            ));
          } else {
            let DirectRasterEncoding::Sampled { pixels } = &actual.direct().encoding else {
              panic!("line art must preserve exact RGB samples");
            };
            assert_eq!(pixels.rgb.as_slice(), rgb.as_slice());
          }
        }
      }
    }
  }

  #[test]
  fn print_gray8_and_rgb_use_the_same_retained_and_reduced_codec_surface() {
    // Native unreduced Gray8/RGB pairs retain lossless RGB; reduced pairs
    // select JPEG. Route the actual realized samples through one classifier,
    // retaining its photographic and line-art opposites.
    for photograph in [false, true] {
      let gray: Vec<u8> = (0..64 * 64)
        .map(|index| {
          if photograph {
            ((index % 64 + 31 * (index / 64)) % 256) as u8
          } else if (index / 8) % 2 == 0 {
            0
          } else {
            255
          }
        })
        .collect();
      let rgb: Vec<u8> = gray.iter().flat_map(|&value| [value; 3]).collect();
      let mut sources = Vec::new();
      for (color, data) in [
        (png::ColorType::Grayscale, gray.as_slice()),
        (png::ColorType::Rgb, rgb.as_slice()),
      ] {
        let mut encoded = Vec::new();
        {
          let mut encoder = png::Encoder::new(&mut encoded, 64, 64);
          encoder.set_depth(png::BitDepth::Eight);
          encoder.set_color(color);
          encoder.set_source_gamma(png::ScaledFloat::from_scaled(45_000));
          encoder
            .write_header()
            .unwrap()
            .write_image_data(data)
            .unwrap();
        }
        assert_eq!(
          trial_metadata_size(&encoded, RasterImageFormat::Png),
          Some(80)
        );
        sources.push(encoded);
      }
      for dpi in [220.0, 440.0] {
        let mut options = PdfOptions::default();
        options.images.optimization_policy =
          PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
        let export = RasterExportOptions::new(&options, 64.0 * 72.0 / dpi, 64.0 * 72.0 / dpi);
        let results: Vec<_> = sources
          .iter()
          .map(|data| decode_image(data, Some("image/png"), export, None).unwrap())
          .collect();
        assert_eq!(results[0].size(), results[1].size());
        match (&results[0].direct().encoding, &results[1].direct().encoding) {
          (
            DirectRasterEncoding::Dct { data: left, .. },
            DirectRasterEncoding::Dct { data: right, .. },
          ) => {
            assert!(photograph, "line art must keep its lossless color samples");
            assert_eq!(left, right);
          }
          (
            DirectRasterEncoding::Sampled { pixels: left },
            DirectRasterEncoding::Sampled { pixels: right },
          ) => {
            assert_eq!(left.rgb, right.rgb);
            assert!(
              !photograph,
              "photographic samples must enter the common JPEG owner"
            );
          }
          _ => panic!("Gray8 and equal RGB must have the same codec and color samples"),
        }
      }
    }
  }

  #[test]
  fn reduced_print_rgb_pngs_use_native_bitmap_codec_with_empty_trial_properties() {
    // Actual native 600x400 RGB line-art/noise controls. The realized
    // 600x264 and 236x131 surfaces select all three Print decision paths:
    // rejected JPEG trial, accepted JPEG trial, and photographic shortcut.
    // Source sRGB metadata disappears with the GDI+ resize surface. Pin the
    // complete native lossless plane as well as representation and dimensions.
    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    options.standards.push(PdfStandard::PdfA3a);
    for photographic in [false, true] {
      let source = image::RgbImage::from_fn(600, 400, |x, y| {
        let index = x + y * 600;
        let value = index.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        if !photographic && index % 20 < 17 {
          image::Rgb([32; 3])
        } else {
          image::Rgb([value as u8, (value >> 8) as u8, (value >> 16) as u8])
        }
      });
      for srgb in [false, true] {
        let mut encoded = Vec::new();
        {
          let mut encoder = png::Encoder::new(&mut encoded, 600, 400);
          encoder.set_color(png::ColorType::Rgb);
          encoder.set_depth(png::BitDepth::Eight);
          if srgb {
            encoder.set_source_srgb(png::SrgbRenderingIntent::Perceptual);
          }
          encoder
            .write_header()
            .unwrap()
            .write_image_data(source.as_raw())
            .unwrap();
        }
        for (display_width, display_height, expected_size, lossless) in [
          (216.05, 95.05, (600, 264), !photographic),
          (85.3, 47.525, (236, 131), false),
        ] {
          let actual = decode_image(
            &encoded,
            Some("image/png"),
            RasterExportOptions::new(&options, display_width, display_height),
            None,
          )
          .unwrap();
          let image = actual.direct();
          assert_eq!((image.width, image.height), expected_size);
          match &image.encoding {
            DirectRasterEncoding::Sampled { pixels } => {
              assert!(lossless);
              let mut crc = flate2::Crc::new();
              crc.update(&pixels.rgb);
              assert_eq!(crc.sum(), 0x1046_46af);
              assert!(pixels.icc_profile.is_none());
            }
            DirectRasterEncoding::Dct {
              alpha, icc_profile, ..
            } => {
              assert!(!lossless);
              assert!(alpha.is_none() && icc_profile.is_none());
            }
            _ => panic!("native reduced RGB PNG is either sampled RGB or JPEG"),
          }
        }
      }
    }
  }

  #[test]
  fn tiff_trial_metadata_counts_native_external_values_and_rejects_uncovered_owners() {
    // Native 15-property, six-strip TIFF: Exif cost274. A different number of
    // strips changes the cost even when the rendered color samples agree.
    let properties = [
      (0xfe, 4, 1),
      (0x100, 4, 1),
      (0x101, 4, 1),
      (0x102, 3, 3),
      (0x103, 3, 1),
      (0x106, 3, 1),
      (0x111, 4, 6),
      (0x115, 3, 1),
      (0x116, 4, 1),
      (0x117, 4, 6),
      (0x11a, 5, 1),
      (0x11b, 5, 1),
      (0x11c, 3, 1),
      (0x128, 3, 1),
      (0x13d, 3, 1),
    ];
    for little in [false, true] {
      let u16_bytes = |value: u16| {
        if little {
          value.to_le_bytes()
        } else {
          value.to_be_bytes()
        }
      };
      let u32_bytes = |value: u32| {
        if little {
          value.to_le_bytes()
        } else {
          value.to_be_bytes()
        }
      };
      let mut data = if little {
        b"II".to_vec()
      } else {
        b"MM".to_vec()
      };
      data.extend(u16_bytes(42));
      data.extend(u32_bytes(8));
      data.extend(u16_bytes(15));
      let mut external = 8 + 2 + properties.len() * 12 + 4;
      for (tag, kind, count) in properties {
        data.extend(u16_bytes(tag));
        data.extend(u16_bytes(kind));
        data.extend(u32_bytes(count));
        let length = count as usize
          * match kind {
            3 => 2,
            4 => 4,
            _ => 8,
          };
        if length > 4 {
          data.extend(u32_bytes(external as u32));
          external += length;
        } else {
          data.extend(u32_bytes(1));
        }
      }
      data.extend(u32_bytes(0));
      data.resize(external, 0);
      assert_eq!(
        trial_metadata_size(&data, RasterImageFormat::Tiff),
        Some(274)
      );
      assert_eq!(
        trial_metadata_size(&data[..data.len() - 1], RasterImageFormat::Tiff),
        None
      );
      let mut multi_frame = data.clone();
      multi_frame[190..194].copy_from_slice(&u32_bytes(8));
      assert_eq!(
        trial_metadata_size(&multi_frame, RasterImageFormat::Tiff),
        None
      );
      data[10..12].copy_from_slice(&u16_bytes(0x9999));
      assert_eq!(trial_metadata_size(&data, RasterImageFormat::Tiff), None);
    }
  }
}
