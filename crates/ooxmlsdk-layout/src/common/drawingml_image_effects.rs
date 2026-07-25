use std::io::Cursor;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use ooxmlsdk::units::DrawingmlPercentageValue;

use crate::model::RgbColor;
use crate::render::emf_wmf;

use super::color_math::HslColor;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ImageEffect {
  AlphaBiLevel(u8),
  AlphaCeiling,
  AlphaFloor,
  AlphaInverse(Option<RgbColor>),
  AlphaModulateFixed(f32),
  AlphaReplace(u8),
  BiLevel(u8),
  Blur {
    radius_px: f32,
    grow_bounds: bool,
  },
  ColorChange(ColorChangeEffect),
  ColorReplacement(RgbColor),
  Duotone(RgbColor, RgbColor),
  Grayscale,
  Hsl {
    hue_degrees: f32,
    saturation_offset: f32,
    luminance_offset: f32,
  },
  Luminance {
    watermark: bool,
    brightness: Option<i32>,
    contrast: Option<i32>,
  },
  Tint {
    hue_degrees: f32,
    amount: f32,
  },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ColorChangeEffect {
  pub(crate) from: RgbColor,
  pub(crate) to: RgbColor,
  pub(crate) from_alpha: u8,
  pub(crate) to_alpha: u8,
  pub(crate) use_alpha: bool,
  pub(crate) tolerance: u8,
}

pub(crate) fn apply(
  data: &[u8],
  content_type: Option<&str>,
  effects: &[ImageEffect],
) -> Option<Vec<u8>> {
  let raster_data = emf_wmf::decode_metafile_as_raster(data, content_type)
    .ok()
    .flatten()
    .map(|raster| raster.data);
  let image_data = raster_data.as_deref().unwrap_or(data);
  let mut image = image::load_from_memory(image_data).ok()?.to_rgba8();
  for effect in effects {
    if let ImageEffect::Blur { radius_px, .. } = *effect {
      if radius_px > f32::EPSILON {
        image = image::imageops::blur(&image, radius_px);
      }
      continue;
    }
    for pixel in image.pixels_mut() {
      let [mut r, mut g, mut b, mut a] = pixel.0;
      match *effect {
        ImageEffect::AlphaBiLevel(threshold) => {
          a = if a < threshold { 0 } else { u8::MAX };
        }
        ImageEffect::AlphaCeiling => {
          if a > 0 {
            a = u8::MAX;
          }
        }
        ImageEffect::AlphaFloor => {
          if a < u8::MAX {
            a = 0;
          }
        }
        ImageEffect::AlphaInverse(color) => {
          a = u8::MAX - a;
          if let Some(color) = color {
            r = color.r;
            g = color.g;
            b = color.b;
          }
        }
        ImageEffect::AlphaModulateFixed(amount) => {
          a = (f32::from(a) * amount).round().clamp(0.0, 255.0) as u8;
        }
        ImageEffect::AlphaReplace(alpha) => a = alpha,
        ImageEffect::BiLevel(threshold) => {
          let value = if srgb_luminance(r, g, b) >= threshold {
            u8::MAX
          } else {
            0
          };
          r = value;
          g = value;
          b = value;
        }
        ImageEffect::Blur { .. } => unreachable!("blur handled as a whole-image effect"),
        ImageEffect::ColorChange(effect)
          if channel_within_tolerance(r, effect.from.r, effect.tolerance)
            && channel_within_tolerance(g, effect.from.g, effect.tolerance)
            && channel_within_tolerance(b, effect.from.b, effect.tolerance)
            && (!effect.use_alpha || a == effect.from_alpha) =>
        {
          r = effect.to.r;
          g = effect.to.g;
          b = effect.to.b;
          if effect.use_alpha {
            a = effect.to_alpha;
          }
        }
        ImageEffect::ColorChange(_) => {}
        ImageEffect::ColorReplacement(color) => {
          r = color.r;
          g = color.g;
          b = color.b;
        }
        ImageEffect::Duotone(first, second) => {
          let luminance = libreoffice_luminance(r, g, b);
          r = duotone_component(luminance, first.r, second.r);
          g = duotone_component(luminance, first.g, second.g);
          b = duotone_component(luminance, first.b, second.b);
        }
        ImageEffect::Grayscale => {
          let luminance = srgb_luminance(r, g, b);
          r = luminance;
          g = luminance;
          b = luminance;
        }
        ImageEffect::Hsl {
          hue_degrees,
          saturation_offset,
          luminance_offset,
        } => {
          let mut hsl = HslColor::from_srgb8([r, g, b]);
          hsl.hue_degrees = (hsl.hue_degrees + hue_degrees).rem_euclid(360.0);
          hsl.saturation = (hsl.saturation + saturation_offset).clamp(0.0, 1.0);
          hsl.lightness = (hsl.lightness + luminance_offset).clamp(0.0, 1.0);
          [r, g, b] = hsl.to_srgb8();
        }
        ImageEffect::Luminance {
          watermark,
          brightness,
          contrast,
        } => {
          if watermark {
            r = libreoffice_luminance_contrast_component(r, 0.5, -0.7);
            g = libreoffice_luminance_contrast_component(g, 0.5, -0.7);
            b = libreoffice_luminance_contrast_component(b, 0.5, -0.7);
          } else if brightness.is_some() || contrast.is_some() {
            let brightness = brightness.unwrap_or(0);
            let contrast = contrast.unwrap_or(0);
            r = mso_brightness_contrast_component(r, brightness, contrast);
            g = mso_brightness_contrast_component(g, brightness, contrast);
            b = mso_brightness_contrast_component(b, brightness, contrast);
          }
        }
        ImageEffect::Tint {
          hue_degrees,
          amount,
        } => {
          let mut hsl = HslColor::from_srgb8([r, g, b]);
          let delta = (hue_degrees - hsl.hue_degrees + 540.0).rem_euclid(360.0) - 180.0;
          hsl.hue_degrees = (hsl.hue_degrees + delta * amount).rem_euclid(360.0);
          [r, g, b] = hsl.to_srgb8();
        }
      }
      pixel.0 = [r, g, b, a];
    }
  }

  let mut output = Vec::new();
  PngEncoder::new(Cursor::new(&mut output))
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(output)
}

pub(crate) fn office_alpha_modulate_amount(value: DrawingmlPercentageValue) -> f32 {
  // MS-OI29500 §20.1.8.6: Office wraps authored values beyond 100% while
  // retaining positive exact multiples as the schema default of 100%.
  let authored = value.as_drawingml_percent().max(0);
  let remainder = authored % 100_000;
  let office_value = if authored > 0 && remainder == 0 {
    100_000
  } else {
    remainder
  };
  office_value as f32 / 100_000.0
}

pub(crate) fn color_change_tolerance(content_type: Option<&str>) -> u8 {
  match content_type {
    Some("image/jpeg" | "image/jpg") => 15,
    Some("image/png" | "image/tiff" | "image/tif") => 1,
    Some("image/bmp" | "image/x-bmp") => 0,
    _ => 9,
  }
}

fn channel_within_tolerance(actual: u8, expected: u8, tolerance: u8) -> bool {
  actual.abs_diff(expected) <= tolerance
}

fn srgb_luminance(r: u8, g: u8, b: u8) -> u8 {
  ((u32::from(r) * 2_126 + u32::from(g) * 7_152 + u32::from(b) * 722 + 5_000) / 10_000).min(255)
    as u8
}

fn libreoffice_luminance(r: u8, g: u8, b: u8) -> u8 {
  ((u32::from(b) * 29 + u32::from(g) * 151 + u32::from(r) * 76) >> 8) as u8
}

pub(crate) fn duotone_component(luminance: u8, first: u8, second: u8) -> u8 {
  let luminance = u16::from(luminance);
  ((u16::from(second) * luminance / u16::from(u8::MAX))
    + (u16::from(first) * (u16::from(u8::MAX) - luminance) / u16::from(u8::MAX))) as u8
}

fn mso_brightness_contrast_component(value: u8, brightness: i32, contrast: i32) -> u8 {
  let contrast = contrast.clamp(-100, 100) as f32;
  let slope = if contrast >= 0.0 {
    128.0 / (128.0 - 1.27 * contrast)
  } else {
    (128.0 + 1.27 * contrast) / 128.0
  };
  let offset = brightness.clamp(-100, 100) as f32 * 2.55;
  ((f32::from(value) + offset / 2.0 - 128.0) * slope + 128.0 + offset / 2.0)
    .round()
    .clamp(0.0, 255.0) as u8
}

fn libreoffice_luminance_contrast_component(value: u8, luminance: f32, contrast: f32) -> u8 {
  let luminance = luminance.clamp(-1.0, 1.0);
  let contrast = contrast.clamp(-1.0, 1.0);
  let contrast_offset = if contrast >= 0.0 {
    128.0 / (128.0 - contrast * 127.0)
  } else {
    (128.0 + contrast * 127.0) / 128.0
  };
  let prepared_contrast_offset = (128.0 - contrast_offset * 128.0) / 255.0;
  ((f32::from(value) / 255.0) * contrast_offset + luminance + prepared_contrast_offset)
    .clamp(0.0, 1.0)
    .mul_add(255.0, 0.0)
    .round() as u8
}
