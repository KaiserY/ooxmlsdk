use color::{ColorSpace, Hsl, OpaqueColor, Srgb};
use ooxmlsdk::units::DRAWINGML_PERCENT_SCALE;

/// Shared color-space representation used only after OOXML integer values have
/// crossed their schema/Office compatibility boundary.
///
/// DrawingML stores percentages in thousandths of one percent and angles in
/// 1/60000 degree units. Those integers remain authoritative in the import
/// layer. This type uses `f32` because `color`, the layout display lists, image
/// processing, and the PDF backend all use `f32`; converting once at this
/// boundary avoids repeated `f32`/`f64` round trips.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HslColor {
  pub(crate) hue_degrees: f32,
  pub(crate) saturation: f32,
  pub(crate) lightness: f32,
}

impl HslColor {
  pub(crate) fn from_srgb8(rgb: [u8; 3]) -> Self {
    let [r, g, b] = rgb;
    let [hue_degrees, saturation_percent, lightness_percent] =
      OpaqueColor::<Srgb>::from_rgb8(r, g, b)
        .convert::<Hsl>()
        .components;
    Self {
      hue_degrees,
      saturation: saturation_percent * 0.01,
      lightness: lightness_percent * 0.01,
    }
  }

  pub(crate) fn to_srgb8(self) -> [u8; 3] {
    let rgb = OpaqueColor::<Hsl>::new([
      self.hue_degrees,
      self.saturation.clamp(0.0, 1.0) * 100.0,
      self.lightness.clamp(0.0, 1.0) * 100.0,
    ])
    .convert::<Srgb>()
    .components;
    rgb.map(normalized_to_u8)
  }

  pub(crate) fn apply_tint(&mut self, amount: f32) {
    self.lightness = (self.lightness * (1.0 - amount) + amount).clamp(0.0, 1.0);
  }

  pub(crate) fn apply_shade(&mut self, amount: f32) {
    self.lightness = (self.lightness * amount).clamp(0.0, 1.0);
  }

  pub(crate) fn apply_saturation_mod(&mut self, amount: f32) {
    self.saturation = (self.saturation * amount).clamp(0.0, 1.0);
  }

  pub(crate) fn apply_luminance_mod(&mut self, amount: f32) {
    self.lightness = (self.lightness * amount).clamp(0.0, 1.0);
  }

  pub(crate) fn apply_luminance_offset(&mut self, amount: f32) {
    self.lightness = (self.lightness + amount).clamp(0.0, 1.0);
  }
}

pub(crate) fn srgb_to_linear_channel(value: f32) -> f32 {
  Srgb::to_linear_srgb([value, 0.0, 0.0])[0]
}

pub(crate) fn linear_to_srgb_channel(value: f32) -> f32 {
  Srgb::from_linear_srgb([value, 0.0, 0.0])[0]
}

pub(crate) fn drawingml_srgb8_to_scrgb(value: u8) -> i32 {
  normalized_to_drawingml_percent(srgb_to_linear_channel(f32::from(value) / 255.0))
}

pub(crate) fn drawingml_scrgb_to_srgb8(value: i32) -> u8 {
  normalized_to_u8(linear_to_srgb_channel(drawingml_percent_to_normalized(
    value,
  )))
}

pub(crate) fn drawingml_linear_to_srgb_percent(value: i32) -> i32 {
  normalized_to_drawingml_percent(linear_to_srgb_channel(drawingml_percent_to_normalized(
    value,
  )))
}

pub(crate) fn drawingml_srgb_to_linear_percent(value: i32) -> i32 {
  normalized_to_drawingml_percent(srgb_to_linear_channel(drawingml_percent_to_normalized(
    value,
  )))
}

pub(crate) fn relative_luminance(rgb: [u8; 3]) -> f32 {
  let [r, g, b] = rgb;
  OpaqueColor::<Srgb>::from_rgb8(r, g, b).relative_luminance()
}

fn normalized_to_u8(value: f32) -> u8 {
  (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn drawingml_percent_to_normalized(value: i32) -> f32 {
  value.clamp(0, DRAWINGML_PERCENT_SCALE) as f32 / DRAWINGML_PERCENT_SCALE as f32
}

fn normalized_to_drawingml_percent(value: f32) -> i32 {
  (value.clamp(0.0, 1.0) * DRAWINGML_PERCENT_SCALE as f32).round() as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn srgb_linear_channel_round_trips_every_u8_value() {
    for value in u8::MIN..=u8::MAX {
      assert_eq!(
        drawingml_scrgb_to_srgb8(drawingml_srgb8_to_scrgb(value)),
        value
      );
    }
  }

  #[test]
  fn hsl_round_trips_primary_and_achromatic_colors() {
    for rgb in [
      [0, 0, 0],
      [255, 255, 255],
      [128, 128, 128],
      [255, 0, 0],
      [0, 255, 0],
      [0, 0, 255],
      [13, 44, 64],
    ] {
      assert_eq!(HslColor::from_srgb8(rgb).to_srgb8(), rgb);
    }
  }
}
