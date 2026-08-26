use color::{ColorSpace, Hsl, OpaqueColor, Srgb};
use ooxmlsdk::units::{DRAWINGML_PERCENT_SCALE, DRAWINGML_POSITIVE_FIXED_ANGLE_MAX_EXCLUSIVE};

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
}

/// DrawingML's integer HSL transform state.
///
/// OOXML stores hue in 1/60000-degree units and saturation/luminance in
/// thousandths of one percent. Office-compatible transform pipelines quantize
/// each HSL conversion and each following modulation in that integer domain;
/// retaining a floating HSL value until the final RGB conversion can move an
/// 8-bit channel by one at non-primary colors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DrawingmlHslColor {
  pub(crate) hue: i32,
  pub(crate) saturation: i32,
  pub(crate) lightness: i32,
}

impl DrawingmlHslColor {
  pub(crate) fn from_srgb8(rgb: [u8; 3]) -> Self {
    let [r, g, b] = rgb.map(|channel| f64::from(channel) / 255.0);
    let minimum = r.min(g).min(b);
    let maximum = r.max(g).max(b);
    let spread = maximum - minimum;
    let lightness = (((minimum + maximum) * 0.5 * f64::from(DRAWINGML_PERCENT_SCALE)) + 0.5) as i32;
    let hue_degrees = if spread == 0.0 {
      0.0
    } else if maximum == r {
      ((g - b) / spread * 60.0 + 360.0) % 360.0
    } else if maximum == g {
      (b - r) / spread * 60.0 + 120.0
    } else {
      (r - g) / spread * 60.0 + 240.0
    };
    let hue = ((hue_degrees * 60_000.0 + 0.5) as i32)
      .rem_euclid(DRAWINGML_POSITIVE_FIXED_ANGLE_MAX_EXCLUSIVE);
    let saturation = if lightness == 0 || lightness == DRAWINGML_PERCENT_SCALE {
      0
    } else if lightness <= DRAWINGML_PERCENT_SCALE / 2 {
      (spread / (minimum + maximum) * f64::from(DRAWINGML_PERCENT_SCALE) + 0.5) as i32
    } else {
      (spread / (2.0 - maximum - minimum) * f64::from(DRAWINGML_PERCENT_SCALE) + 0.5) as i32
    };
    Self {
      hue,
      saturation,
      lightness,
    }
  }

  pub(crate) fn to_srgb8(self) -> [u8; 3] {
    let mut rgb = [0.0; 3];
    if self.saturation == 0 || self.lightness == DRAWINGML_PERCENT_SCALE {
      rgb.fill(f64::from(self.lightness) / f64::from(DRAWINGML_PERCENT_SCALE));
    } else if self.lightness > 0 {
      let hue = f64::from(self.hue) / f64::from(DRAWINGML_POSITIVE_FIXED_ANGLE_MAX_EXCLUSIVE) * 6.0;
      rgb = if hue <= 1.0 {
        [1.0, hue, 0.0]
      } else if hue <= 2.0 {
        [2.0 - hue, 1.0, 0.0]
      } else if hue <= 3.0 {
        [0.0, 1.0, hue - 2.0]
      } else if hue <= 4.0 {
        [0.0, 4.0 - hue, 1.0]
      } else if hue <= 5.0 {
        [hue - 4.0, 0.0, 1.0]
      } else {
        [1.0, 0.0, 6.0 - hue]
      };
      let saturation = f64::from(self.saturation) / f64::from(DRAWINGML_PERCENT_SCALE);
      for channel in &mut rgb {
        *channel = (*channel - 0.5) * saturation + 0.5;
      }
      let luminance = 2.0 * f64::from(self.lightness) / f64::from(DRAWINGML_PERCENT_SCALE) - 1.0;
      if luminance < 0.0 {
        let shade = luminance + 1.0;
        for channel in &mut rgb {
          *channel *= shade;
        }
      } else if luminance > 0.0 {
        let tint = 1.0 - luminance;
        for channel in &mut rgb {
          *channel = 1.0 - (1.0 - *channel) * tint;
        }
      }
    }
    rgb.map(drawingml_srgb_unit_to_u8)
  }

  pub(crate) fn apply_hue_mod(&mut self, value: i32) {
    self.hue = mod_drawingml_value(
      self.hue,
      value,
      DRAWINGML_POSITIVE_FIXED_ANGLE_MAX_EXCLUSIVE,
    );
  }

  pub(crate) fn set_saturation(&mut self, value: i32) {
    self.saturation = value.clamp(0, DRAWINGML_PERCENT_SCALE);
  }

  pub(crate) fn offset_saturation(&mut self, value: i32) {
    self.saturation = (self.saturation + value).clamp(0, DRAWINGML_PERCENT_SCALE);
  }

  pub(crate) fn modulate_saturation(&mut self, value: i32) {
    self.saturation = mod_drawingml_value(self.saturation, value, DRAWINGML_PERCENT_SCALE);
  }

  pub(crate) fn set_lightness(&mut self, value: i32) {
    self.lightness = value.clamp(0, DRAWINGML_PERCENT_SCALE);
    self.clear_saturation_at_luminance_extreme();
  }

  pub(crate) fn offset_lightness(&mut self, value: i32) {
    self.lightness = (self.lightness + value).clamp(0, DRAWINGML_PERCENT_SCALE);
    self.clear_saturation_at_luminance_extreme();
  }

  pub(crate) fn modulate_lightness(&mut self, value: i32) {
    self.lightness = mod_drawingml_value(self.lightness, value, DRAWINGML_PERCENT_SCALE);
    self.clear_saturation_at_luminance_extreme();
  }

  fn clear_saturation_at_luminance_extreme(&mut self) {
    if self.lightness == 0 || self.lightness == DRAWINGML_PERCENT_SCALE {
      self.saturation = 0;
    }
  }
}

fn mod_drawingml_value(value: i32, modulation: i32, maximum: i32) -> i32 {
  (i64::from(value) * i64::from(modulation) / i64::from(DRAWINGML_PERCENT_SCALE))
    .clamp(0, i64::from(maximum)) as i32
}

pub(crate) fn drawingml_srgb_unit_to_u8(channel: f64) -> u8 {
  let scaled = channel.clamp(0.0, 1.0) * 255.0;
  let nearest_integer = scaled.round();
  let mathematical_value = if (scaled - nearest_integer).abs() <= 1.0e-9 {
    nearest_integer
  } else {
    scaled.floor()
  };
  mathematical_value as u8
}

pub(crate) fn srgb_to_linear_channel(value: f32) -> f32 {
  Srgb::to_linear_srgb([value, 0.0, 0.0])[0]
}

pub(crate) fn linear_to_srgb_channel(value: f32) -> f32 {
  Srgb::from_linear_srgb([value, 0.0, 0.0])[0]
}

pub(crate) fn apply_linear_saturation_mod(rgb: [u8; 3], amount: f32) -> [u8; 3] {
  let linear = rgb.map(|channel| srgb_to_linear_channel(f32::from(channel) / 255.0));
  let maximum = linear.iter().copied().fold(f32::NEG_INFINITY, f32::max);
  let minimum = linear.iter().copied().fold(f32::INFINITY, f32::min);
  let lightness = (maximum + minimum) * 0.5;
  let spread = maximum - minimum;
  let saturation = if spread <= f32::EPSILON {
    0.0
  } else {
    spread / (1.0 - (2.0 * lightness - 1.0).abs()).max(f32::EPSILON)
  };
  let saturation = (saturation * amount).clamp(0.0, 1.0);
  if spread <= f32::EPSILON || saturation <= f32::EPSILON {
    let channel = normalized_to_u8(linear_to_srgb_channel(lightness));
    return [channel; 3];
  }
  let hue = if maximum == linear[0] {
    60.0 * ((linear[1] - linear[2]) / spread).rem_euclid(6.0)
  } else if maximum == linear[1] {
    60.0 * ((linear[2] - linear[0]) / spread + 2.0)
  } else {
    60.0 * ((linear[0] - linear[1]) / spread + 4.0)
  };
  let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
  let secondary = chroma * (1.0 - ((hue / 60.0).rem_euclid(2.0) - 1.0).abs());
  let offset = lightness - chroma * 0.5;
  let linear = match hue {
    value if value < 60.0 => [chroma, secondary, 0.0],
    value if value < 120.0 => [secondary, chroma, 0.0],
    value if value < 180.0 => [0.0, chroma, secondary],
    value if value < 240.0 => [0.0, secondary, chroma],
    value if value < 300.0 => [secondary, 0.0, chroma],
    _ => [chroma, 0.0, secondary],
  };
  linear.map(|channel| normalized_to_u8(linear_to_srgb_channel(channel + offset)))
}

pub(crate) fn drawingml_srgb8_to_scrgb(value: u8) -> i32 {
  normalized_to_drawingml_percent(srgb_to_linear_channel(f32::from(value) / 255.0))
}

pub(crate) fn drawingml_scrgb_to_srgb8(value: i32) -> u8 {
  normalized_to_u8(linear_to_srgb_channel(drawingml_percent_to_normalized(
    value,
  )))
}

pub(crate) fn drawingml_shade_srgb8(rgb: [u8; 3], retention: i32) -> [u8; 3] {
  let retention = retention.clamp(0, DRAWINGML_PERCENT_SCALE);
  rgb.map(|channel| {
    let linear = drawingml_srgb8_to_scrgb(channel);
    let shaded = i64::from(linear) * i64::from(retention) / i64::from(DRAWINGML_PERCENT_SCALE);
    drawingml_scrgb_to_srgb8(shaded as i32)
  })
}

pub(crate) fn drawingml_tint_srgb8(rgb: [u8; 3], retention: i32) -> [u8; 3] {
  let retention = retention.clamp(0, DRAWINGML_PERCENT_SCALE);
  rgb.map(|channel| {
    let linear = drawingml_srgb8_to_scrgb(channel);
    let tinted = i64::from(DRAWINGML_PERCENT_SCALE)
      - i64::from(DRAWINGML_PERCENT_SCALE - linear) * i64::from(retention)
        / i64::from(DRAWINGML_PERCENT_SCALE);
    drawingml_scrgb_to_srgb8(tinted as i32)
  })
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

  #[test]
  fn drawingml_integer_hsl_preserves_transform_step_quantization() {
    let mut ecma_green = DrawingmlHslColor::from_srgb8([0, 255, 0]);
    ecma_green.modulate_saturation(20_000);
    assert_eq!(ecma_green.to_srgb8(), [0x66, 0x99, 0x66]);

    let tinted = drawingml_tint_srgb8([0xE7, 0xE6, 0xE6], 85_000);
    let mut fill = DrawingmlHslColor::from_srgb8(tinted);
    fill.modulate_saturation(155_000);
    assert_eq!(fill.to_srgb8(), [235, 233, 233]);

    let mut outline = DrawingmlHslColor::from_srgb8([0x44, 0x54, 0x6A]);
    outline.modulate_saturation(155_000);
    assert_eq!(outline.to_srgb8(), [57, 82, 116]);
  }

  #[test]
  fn linear_saturation_mod_matches_word_group_glow_color_management() {
    assert_eq!(
      apply_linear_saturation_mod([0xF7, 0x96, 0x46], 1.75),
      [0xFE, 0x90, 0x00]
    );
  }

  #[test]
  fn drawingml_shade_and_tint_operate_in_scrgb() {
    let accent = [0x5B, 0x9B, 0xD5];
    assert_eq!(drawingml_shade_srgb8(accent, 58_000), [0x46, 0x79, 0xA7]);
    assert_eq!(drawingml_shade_srgb8(accent, 86_000), [0x55, 0x91, 0xC7]);
    assert_eq!(drawingml_tint_srgb8(accent, 86_000), [0x84, 0xAE, 0xDC]);
  }
}
