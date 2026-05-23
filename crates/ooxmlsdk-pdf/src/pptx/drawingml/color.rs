use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

const COLOR_PERCENT_MAX: i32 = 100_000;
const DEC_GAMMA: f64 = 2.3;
const INC_GAMMA: f64 = 1.0 / DEC_GAMMA;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Color {
  RgbHex(RgbHexColor),
  RgbPercent(RgbPercentColor),
  Hsl(HslColor),
  Scheme(SchemeColor),
  Preset(PresetColor),
  System(SystemColor),
}

impl Color {
  pub(crate) fn from_solid_fill_choice(choice: &a::SolidFillChoice) -> Option<Self> {
    match choice {
      a::SolidFillChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::SolidFillChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::SolidFillChoice::HslColor(color) => Some(hsl_color(color)),
      a::SolidFillChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::SolidFillChoice::PresetColor(color) => Some(preset_color(color)),
      a::SolidFillChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_diagram_fill_color_choice(choice: &dgm::FillColorListChoice) -> Option<Self> {
    match choice {
      dgm::FillColorListChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      dgm::FillColorListChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      dgm::FillColorListChoice::HslColor(color) => Some(hsl_color(color)),
      dgm::FillColorListChoice::SchemeColor(color) => Some(scheme_color(color)),
      dgm::FillColorListChoice::PresetColor(color) => Some(preset_color(color)),
      dgm::FillColorListChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_fill_reference_choice(choice: &a::FillReferenceChoice) -> Option<Self> {
    match choice {
      a::FillReferenceChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::FillReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::FillReferenceChoice::HslColor(color) => Some(hsl_color(color)),
      a::FillReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::FillReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      a::FillReferenceChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_line_reference_choice(choice: &a::LineReferenceChoice) -> Option<Self> {
    match choice {
      a::LineReferenceChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::LineReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::LineReferenceChoice::HslColor(color) => Some(hsl_color(color)),
      a::LineReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::LineReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      a::LineReferenceChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_effect_reference_choice(choice: &a::EffectReferenceChoice) -> Option<Self> {
    match choice {
      a::EffectReferenceChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::EffectReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::EffectReferenceChoice::HslColor(color) => Some(hsl_color(color)),
      a::EffectReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::EffectReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      a::EffectReferenceChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_font_reference_choice(choice: &a::FontReferenceChoice) -> Option<Self> {
    match choice {
      a::FontReferenceChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::FontReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::FontReferenceChoice::HslColor(color) => Some(hsl_color(color)),
      a::FontReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::FontReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      a::FontReferenceChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_table_cell_text_style_choice(
    choice: &a::TableCellTextStyleChoice2,
  ) -> Option<Self> {
    match choice {
      a::TableCellTextStyleChoice2::RgbColorModelPercentage(color) => {
        Some(rgb_percent_color(color))
      }
      a::TableCellTextStyleChoice2::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::TableCellTextStyleChoice2::HslColor(color) => Some(hsl_color(color)),
      a::TableCellTextStyleChoice2::SchemeColor(color) => Some(scheme_color(color)),
      a::TableCellTextStyleChoice2::PresetColor(color) => Some(preset_color(color)),
      a::TableCellTextStyleChoice2::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_background_style_reference_choice(
    choice: &p::BackgroundStyleReferenceChoice,
  ) -> Option<Self> {
    match choice {
      p::BackgroundStyleReferenceChoice::RgbColorModelPercentage(color) => {
        Some(rgb_percent_color(color))
      }
      p::BackgroundStyleReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      p::BackgroundStyleReferenceChoice::HslColor(color) => Some(hsl_color(color)),
      p::BackgroundStyleReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      p::BackgroundStyleReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      p::BackgroundStyleReferenceChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_color_from_choice(choice: &a::ColorFromChoice) -> Option<Self> {
    match choice {
      a::ColorFromChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::ColorFromChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::ColorFromChoice::HslColor(color) => Some(hsl_color(color)),
      a::ColorFromChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::ColorFromChoice::PresetColor(color) => Some(preset_color(color)),
      a::ColorFromChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_color_to_choice(choice: &a::ColorToChoice) -> Option<Self> {
    match choice {
      a::ColorToChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::ColorToChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::ColorToChoice::HslColor(color) => Some(hsl_color(color)),
      a::ColorToChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::ColorToChoice::PresetColor(color) => Some(preset_color(color)),
      a::ColorToChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_glow_choice(choice: &a::GlowChoice) -> Option<Self> {
    match choice {
      a::GlowChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::GlowChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::GlowChoice::HslColor(color) => Some(hsl_color(color)),
      a::GlowChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::GlowChoice::PresetColor(color) => Some(preset_color(color)),
      a::GlowChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_inner_shadow_choice(choice: &a::InnerShadowChoice) -> Option<Self> {
    match choice {
      a::InnerShadowChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::InnerShadowChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::InnerShadowChoice::HslColor(color) => Some(hsl_color(color)),
      a::InnerShadowChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::InnerShadowChoice::PresetColor(color) => Some(preset_color(color)),
      a::InnerShadowChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn from_outer_shadow_choice(choice: &a::OuterShadowChoice) -> Option<Self> {
    match choice {
      a::OuterShadowChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
      a::OuterShadowChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::OuterShadowChoice::HslColor(color) => Some(hsl_color(color)),
      a::OuterShadowChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::OuterShadowChoice::PresetColor(color) => Some(preset_color(color)),
      a::OuterShadowChoice::SystemColor(color) => Some(system_color(color)),
    }
  }

  pub(crate) fn resolve_rgb<F>(
    &self,
    scheme_resolver: &mut F,
    placeholder_color: Option<&Color>,
  ) -> Option<ResolvedColor>
  where
    F: FnMut(a::SchemeColorValues) -> Option<Color>,
  {
    let (mut color, transformations) = match self {
      Self::RgbHex(color) => (
        ResolvedColor::from_hex(&color.value)?,
        color.transformations.as_slice(),
      ),
      Self::RgbPercent(color) => (
        ResolvedColor::new(
          crgb_percent_to_channel(color.red),
          crgb_percent_to_channel(color.green),
          crgb_percent_to_channel(color.blue),
        ),
        color.transformations.as_slice(),
      ),
      Self::Hsl(color) => (
        hsl_to_rgb(color.hue, color.saturation, color.luminance),
        color.transformations.as_slice(),
      ),
      Self::System(color) => (
        ResolvedColor::from_hex(color.last_color.as_deref()?)?,
        color.transformations.as_slice(),
      ),
      Self::Preset(color) => (
        preset_color_rgb(color.value)?,
        color.transformations.as_slice(),
      ),
      Self::Scheme(color) => {
        let base = if color.value == a::SchemeColorValues::PhColor {
          match placeholder_color {
            Some(Self::Scheme(placeholder))
              if placeholder.value == a::SchemeColorValues::PhColor =>
            {
              None
            }
            _ => placeholder_color.cloned(),
          }
        } else {
          scheme_resolver(color.value)
        }?;
        let mut resolved = base.resolve_rgb(scheme_resolver, placeholder_color)?;
        apply_transformations(&mut resolved, &color.transformations);
        return Some(resolved);
      }
    };
    apply_transformations(&mut color, transformations);
    Some(color)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RgbHexColor {
  pub(crate) value: String,
  pub(crate) transformations: Vec<ColorTransformation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RgbPercentColor {
  pub(crate) red: i32,
  pub(crate) green: i32,
  pub(crate) blue: i32,
  pub(crate) transformations: Vec<ColorTransformation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HslColor {
  pub(crate) hue: i32,
  pub(crate) saturation: i32,
  pub(crate) luminance: i32,
  pub(crate) transformations: Vec<ColorTransformation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SchemeColor {
  pub(crate) value: a::SchemeColorValues,
  pub(crate) transformations: Vec<ColorTransformation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PresetColor {
  pub(crate) value: a::PresetColorValues,
  pub(crate) transformations: Vec<ColorTransformation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SystemColor {
  pub(crate) value: a::SystemColorValues,
  pub(crate) last_color: Option<String>,
  pub(crate) transformations: Vec<ColorTransformation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedColor {
  pub(crate) r: u8,
  pub(crate) g: u8,
  pub(crate) b: u8,
  pub(crate) alpha: i32,
}

impl ResolvedColor {
  pub(crate) fn new(r: u8, g: u8, b: u8) -> Self {
    Self {
      r,
      g,
      b,
      alpha: COLOR_PERCENT_MAX,
    }
  }

  pub(crate) fn from_hex(hex: &str) -> Option<Self> {
    if hex.len() != 6 {
      return None;
    }
    Some(Self::new(
      u8::from_str_radix(&hex[0..2], 16).ok()?,
      u8::from_str_radix(&hex[2..4], 16).ok()?,
      u8::from_str_radix(&hex[4..6], 16).ok()?,
    ))
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ColorTransformation {
  pub(crate) kind: ColorTransformationKind,
  pub(crate) value: Option<i32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ColorTransformationKind {
  Red,
  RedMod,
  RedOff,
  Green,
  GreenMod,
  GreenOff,
  Blue,
  BlueMod,
  BlueOff,
  Alpha,
  AlphaMod,
  AlphaOff,
  Hue,
  HueMod,
  HueOff,
  Sat,
  SatMod,
  SatOff,
  Lum,
  LumMod,
  LumOff,
  Shade,
  Tint,
  Gray,
  Comp,
  Inv,
  Gamma,
  InvGamma,
}

pub(crate) fn rgb_hex_color(color: &a::RgbColorModelHex) -> Color {
  Color::RgbHex(RgbHexColor {
    value: color.val.clone(),
    transformations: transformations_from_rgb_color_model_hex_choices(
      &color.rgb_color_model_hex_choice,
    ),
  })
}

pub(crate) fn rgb_percent_color(color: &a::RgbColorModelPercentage) -> Color {
  Color::RgbPercent(RgbPercentColor {
    red: percent_value(color.red_portion),
    green: percent_value(color.green_portion),
    blue: percent_value(color.blue_portion),
    transformations: transformations_from_rgb_color_model_percentage_choices(
      &color.rgb_color_model_percentage_choice,
    ),
  })
}

pub(crate) fn hsl_color(color: &a::HslColor) -> Color {
  Color::Hsl(HslColor {
    hue: color.hue_value,
    saturation: percent_value(color.sat_value),
    luminance: percent_value(color.lum_value),
    transformations: transformations_from_hsl_color_choices(&color.hsl_color_choice),
  })
}

pub(crate) fn scheme_color(color: &a::SchemeColor) -> Color {
  Color::Scheme(SchemeColor {
    value: color.val,
    transformations: transformations_from_scheme_color_choices(&color.scheme_color_choice),
  })
}

pub(crate) fn preset_color(color: &a::PresetColor) -> Color {
  Color::Preset(PresetColor {
    value: color.val,
    transformations: transformations_from_preset_color_choices(&color.preset_color_choice),
  })
}

pub(crate) fn system_color(color: &a::SystemColor) -> Color {
  Color::System(SystemColor {
    value: color.val,
    last_color: color.last_color.clone(),
    transformations: transformations_from_system_color_choices(&color.system_color_choice),
  })
}

fn percent_value(value: ooxmlsdk::simple_type::DrawingmlPercentageValue) -> i32 {
  value.as_drawingml_percent()
}

macro_rules! color_transformations_from_choices {
  ($fn_name:ident, $choice_ty:ident) => {
    fn $fn_name(choices: &[a::$choice_ty]) -> Vec<ColorTransformation> {
      choices
        .iter()
        .map(|choice| match choice {
          a::$choice_ty::Tint(value) => {
            valued_transform(ColorTransformationKind::Tint, percent_value(value.val))
          }
          a::$choice_ty::Shade(value) => {
            valued_transform(ColorTransformationKind::Shade, percent_value(value.val))
          }
          a::$choice_ty::Complement => empty_transform(ColorTransformationKind::Comp),
          a::$choice_ty::Inverse => empty_transform(ColorTransformationKind::Inv),
          a::$choice_ty::Gray => empty_transform(ColorTransformationKind::Gray),
          a::$choice_ty::Alpha(value) => {
            valued_transform(ColorTransformationKind::Alpha, percent_value(value.val))
          }
          a::$choice_ty::AlphaOffset(value) => {
            valued_transform(ColorTransformationKind::AlphaOff, percent_value(value.val))
          }
          a::$choice_ty::AlphaModulation(value) => {
            valued_transform(ColorTransformationKind::AlphaMod, percent_value(value.val))
          }
          a::$choice_ty::Hue(value) => valued_transform(ColorTransformationKind::Hue, value.val),
          a::$choice_ty::HueOffset(value) => {
            valued_transform(ColorTransformationKind::HueOff, value.val)
          }
          a::$choice_ty::HueModulation(value) => {
            valued_transform(ColorTransformationKind::HueMod, percent_value(value.val))
          }
          a::$choice_ty::Saturation(value) => {
            valued_transform(ColorTransformationKind::Sat, percent_value(value.val))
          }
          a::$choice_ty::SaturationOffset(value) => {
            valued_transform(ColorTransformationKind::SatOff, percent_value(value.val))
          }
          a::$choice_ty::SaturationModulation(value) => {
            valued_transform(ColorTransformationKind::SatMod, percent_value(value.val))
          }
          a::$choice_ty::Luminance(value) => {
            valued_transform(ColorTransformationKind::Lum, percent_value(value.val))
          }
          a::$choice_ty::LuminanceOffset(value) => {
            valued_transform(ColorTransformationKind::LumOff, percent_value(value.val))
          }
          a::$choice_ty::LuminanceModulation(value) => {
            valued_transform(ColorTransformationKind::LumMod, percent_value(value.val))
          }
          a::$choice_ty::Red(value) => {
            valued_transform(ColorTransformationKind::Red, percent_value(value.val))
          }
          a::$choice_ty::RedOffset(value) => {
            valued_transform(ColorTransformationKind::RedOff, percent_value(value.val))
          }
          a::$choice_ty::RedModulation(value) => {
            valued_transform(ColorTransformationKind::RedMod, percent_value(value.val))
          }
          a::$choice_ty::Green(value) => {
            valued_transform(ColorTransformationKind::Green, percent_value(value.val))
          }
          a::$choice_ty::GreenOffset(value) => {
            valued_transform(ColorTransformationKind::GreenOff, percent_value(value.val))
          }
          a::$choice_ty::GreenModulation(value) => {
            valued_transform(ColorTransformationKind::GreenMod, percent_value(value.val))
          }
          a::$choice_ty::Blue(value) => {
            valued_transform(ColorTransformationKind::Blue, percent_value(value.val))
          }
          a::$choice_ty::BlueOffset(value) => {
            valued_transform(ColorTransformationKind::BlueOff, percent_value(value.val))
          }
          a::$choice_ty::BlueModulation(value) => {
            valued_transform(ColorTransformationKind::BlueMod, percent_value(value.val))
          }
          a::$choice_ty::Gamma => empty_transform(ColorTransformationKind::Gamma),
          a::$choice_ty::InverseGamma => empty_transform(ColorTransformationKind::InvGamma),
        })
        .collect()
    }
  };
}

fn valued_transform(kind: ColorTransformationKind, value: i32) -> ColorTransformation {
  ColorTransformation {
    kind,
    value: Some(value),
  }
}

fn empty_transform(kind: ColorTransformationKind) -> ColorTransformation {
  ColorTransformation { kind, value: None }
}

color_transformations_from_choices!(
  transformations_from_rgb_color_model_hex_choices,
  RgbColorModelHexChoice
);
color_transformations_from_choices!(
  transformations_from_rgb_color_model_percentage_choices,
  RgbColorModelPercentageChoice
);
color_transformations_from_choices!(transformations_from_hsl_color_choices, HslColorChoice);
color_transformations_from_choices!(transformations_from_system_color_choices, SystemColorChoice);
color_transformations_from_choices!(transformations_from_scheme_color_choices, SchemeColorChoice);
color_transformations_from_choices!(transformations_from_preset_color_choices, PresetColorChoice);

fn preset_color_rgb(value: a::PresetColorValues) -> Option<ResolvedColor> {
  let hex = match value {
    a::PresetColorValues::AliceBlue => "F0F8FF",
    a::PresetColorValues::AntiqueWhite => "FAEBD7",
    a::PresetColorValues::Aqua => "00FFFF",
    a::PresetColorValues::Aquamarine => "7FFFD4",
    a::PresetColorValues::Azure => "F0FFFF",
    a::PresetColorValues::Beige => "F5F5DC",
    a::PresetColorValues::Bisque => "FFE4C4",
    a::PresetColorValues::Black => "000000",
    a::PresetColorValues::BlanchedAlmond => "FFEBCD",
    a::PresetColorValues::Blue => "0000FF",
    a::PresetColorValues::BlueViolet => "8A2BE2",
    a::PresetColorValues::Brown => "A52A2A",
    a::PresetColorValues::BurlyWood => "DEB887",
    a::PresetColorValues::CadetBlue => "5F9EA0",
    a::PresetColorValues::Chartreuse => "7FFF00",
    a::PresetColorValues::Chocolate => "D2691E",
    a::PresetColorValues::Coral => "FF7F50",
    a::PresetColorValues::CornflowerBlue => "6495ED",
    a::PresetColorValues::Cornsilk => "FFF8DC",
    a::PresetColorValues::Crimson => "DC143C",
    a::PresetColorValues::Cyan => "00FFFF",
    a::PresetColorValues::DarkBlue | a::PresetColorValues::DarkBlue2010 => "00008B",
    a::PresetColorValues::DarkCyan | a::PresetColorValues::DarkCyan2010 => "008B8B",
    a::PresetColorValues::DarkGoldenrod | a::PresetColorValues::DarkGoldenrod2010 => "B8860B",
    a::PresetColorValues::DarkGray
    | a::PresetColorValues::DarkGray2010
    | a::PresetColorValues::DarkGrey
    | a::PresetColorValues::DarkGrey2010 => "A9A9A9",
    a::PresetColorValues::DarkGreen | a::PresetColorValues::DarkGreen2010 => "006400",
    a::PresetColorValues::DarkKhaki | a::PresetColorValues::DarkKhaki2010 => "BDB76B",
    a::PresetColorValues::DarkMagenta | a::PresetColorValues::DarkMagenta2010 => "8B008B",
    a::PresetColorValues::DarkOliveGreen | a::PresetColorValues::DarkOliveGreen2010 => "556B2F",
    a::PresetColorValues::DarkOrange | a::PresetColorValues::DarkOrange2010 => "FF8C00",
    a::PresetColorValues::DarkOrchid | a::PresetColorValues::DarkOrchid2010 => "9932CC",
    a::PresetColorValues::DarkRed | a::PresetColorValues::DarkRed2010 => "8B0000",
    a::PresetColorValues::DarkSalmon | a::PresetColorValues::DarkSalmon2010 => "E9967A",
    a::PresetColorValues::DarkSeaGreen | a::PresetColorValues::DarkSeaGreen2010 => "8FBC8F",
    a::PresetColorValues::DarkSlateBlue | a::PresetColorValues::DarkSlateBlue2010 => "483D8B",
    a::PresetColorValues::DarkSlateGray
    | a::PresetColorValues::DarkSlateGray2010
    | a::PresetColorValues::DarkSlateGrey
    | a::PresetColorValues::DarkSlateGrey2010 => "2F4F4F",
    a::PresetColorValues::DarkTurquoise | a::PresetColorValues::DarkTurquoise2010 => "00CED1",
    a::PresetColorValues::DarkViolet | a::PresetColorValues::DarkViolet2010 => "9400D3",
    a::PresetColorValues::DeepPink => "FF1493",
    a::PresetColorValues::DeepSkyBlue => "00BFFF",
    a::PresetColorValues::DimGray | a::PresetColorValues::DimGrey => "696969",
    a::PresetColorValues::DodgerBlue => "1E90FF",
    a::PresetColorValues::Firebrick => "B22222",
    a::PresetColorValues::FloralWhite => "FFFAF0",
    a::PresetColorValues::ForestGreen => "228B22",
    a::PresetColorValues::Fuchsia => "FF00FF",
    a::PresetColorValues::Gainsboro => "DCDCDC",
    a::PresetColorValues::GhostWhite => "F8F8FF",
    a::PresetColorValues::Gold => "FFD700",
    a::PresetColorValues::Goldenrod => "DAA520",
    a::PresetColorValues::Gray | a::PresetColorValues::Grey => "808080",
    a::PresetColorValues::Green => "008000",
    a::PresetColorValues::GreenYellow => "ADFF2F",
    a::PresetColorValues::Honeydew => "F0FFF0",
    a::PresetColorValues::HotPink => "FF69B4",
    a::PresetColorValues::IndianRed => "CD5C5C",
    a::PresetColorValues::Indigo => "4B0082",
    a::PresetColorValues::Ivory => "FFFFF0",
    a::PresetColorValues::Khaki => "F0E68C",
    a::PresetColorValues::Lavender => "E6E6FA",
    a::PresetColorValues::LavenderBlush => "FFF0F5",
    a::PresetColorValues::LawnGreen => "7CFC00",
    a::PresetColorValues::LemonChiffon => "FFFACD",
    a::PresetColorValues::LightBlue | a::PresetColorValues::LightBlue2010 => "ADD8E6",
    a::PresetColorValues::LightCoral | a::PresetColorValues::LightCoral2010 => "F08080",
    a::PresetColorValues::LightCyan | a::PresetColorValues::LightCyan2010 => "E0FFFF",
    a::PresetColorValues::LightGoldenrodYellow | a::PresetColorValues::LightGoldenrodYellow2010 => {
      "FAFAD2"
    }
    a::PresetColorValues::LightGray
    | a::PresetColorValues::LightGray2010
    | a::PresetColorValues::LightGrey
    | a::PresetColorValues::LightGrey2010 => "D3D3D3",
    a::PresetColorValues::LightGreen | a::PresetColorValues::LightGreen2010 => "90EE90",
    a::PresetColorValues::LightPink | a::PresetColorValues::LightPink2010 => "FFB6C1",
    a::PresetColorValues::LightSalmon | a::PresetColorValues::LightSalmon2010 => "FFA07A",
    a::PresetColorValues::LightSeaGreen | a::PresetColorValues::LightSeaGreen2010 => "20B2AA",
    a::PresetColorValues::LightSkyBlue | a::PresetColorValues::LightSkyBlue2010 => "87CEFA",
    a::PresetColorValues::LightSlateGray
    | a::PresetColorValues::LightSlateGray2010
    | a::PresetColorValues::LightSlateGrey
    | a::PresetColorValues::LightSlateGrey2010 => "778899",
    a::PresetColorValues::LightSteelBlue | a::PresetColorValues::LightSteelBlue2010 => "B0C4DE",
    a::PresetColorValues::LightYellow | a::PresetColorValues::LightYellow2010 => "FFFFE0",
    a::PresetColorValues::Lime => "00FF00",
    a::PresetColorValues::LimeGreen => "32CD32",
    a::PresetColorValues::Linen => "FAF0E6",
    a::PresetColorValues::Magenta => "FF00FF",
    a::PresetColorValues::Maroon => "800000",
    a::PresetColorValues::MedAquamarine | a::PresetColorValues::MediumAquamarine2010 => "66CDAA",
    a::PresetColorValues::MediumBlue | a::PresetColorValues::MediumBlue2010 => "0000CD",
    a::PresetColorValues::MediumOrchid | a::PresetColorValues::MediumOrchid2010 => "BA55D3",
    a::PresetColorValues::MediumPurple | a::PresetColorValues::MediumPurple2010 => "9370DB",
    a::PresetColorValues::MediumSeaGreen | a::PresetColorValues::MediumSeaGreen2010 => "3CB371",
    a::PresetColorValues::MediumSlateBlue | a::PresetColorValues::MediumSlateBlue2010 => "7B68EE",
    a::PresetColorValues::MediumSpringGreen | a::PresetColorValues::MediumSpringGreen2010 => {
      "00FA9A"
    }
    a::PresetColorValues::MediumTurquoise | a::PresetColorValues::MediumTurquoise2010 => "48D1CC",
    a::PresetColorValues::MediumVioletRed | a::PresetColorValues::MediumVioletRed2010 => "C71585",
    a::PresetColorValues::MidnightBlue => "191970",
    a::PresetColorValues::MintCream => "F5FFFA",
    a::PresetColorValues::MistyRose => "FFE4E1",
    a::PresetColorValues::Moccasin => "FFE4B5",
    a::PresetColorValues::NavajoWhite => "FFDEAD",
    a::PresetColorValues::Navy => "000080",
    a::PresetColorValues::OldLace => "FDF5E6",
    a::PresetColorValues::Olive => "808000",
    a::PresetColorValues::OliveDrab => "6B8E23",
    a::PresetColorValues::Orange => "FFA500",
    a::PresetColorValues::OrangeRed => "FF4500",
    a::PresetColorValues::Orchid => "DA70D6",
    a::PresetColorValues::PaleGoldenrod => "EEE8AA",
    a::PresetColorValues::PaleGreen => "98FB98",
    a::PresetColorValues::PaleTurquoise => "AFEEEE",
    a::PresetColorValues::PaleVioletRed => "DB7093",
    a::PresetColorValues::PapayaWhip => "FFEFD5",
    a::PresetColorValues::PeachPuff => "FFDAB9",
    a::PresetColorValues::Peru => "CD853F",
    a::PresetColorValues::Pink => "FFC0CB",
    a::PresetColorValues::Plum => "DDA0DD",
    a::PresetColorValues::PowderBlue => "B0E0E6",
    a::PresetColorValues::Purple => "800080",
    a::PresetColorValues::Red => "FF0000",
    a::PresetColorValues::RosyBrown => "BC8F8F",
    a::PresetColorValues::RoyalBlue => "4169E1",
    a::PresetColorValues::SaddleBrown => "8B4513",
    a::PresetColorValues::Salmon => "FA8072",
    a::PresetColorValues::SandyBrown => "F4A460",
    a::PresetColorValues::SeaGreen => "2E8B57",
    a::PresetColorValues::SeaShell => "FFF5EE",
    a::PresetColorValues::Sienna => "A0522D",
    a::PresetColorValues::Silver => "C0C0C0",
    a::PresetColorValues::SkyBlue => "87CEEB",
    a::PresetColorValues::SlateBlue => "6A5ACD",
    a::PresetColorValues::SlateGray | a::PresetColorValues::SlateGrey => "708090",
    a::PresetColorValues::Snow => "FFFAFA",
    a::PresetColorValues::SpringGreen => "00FF7F",
    a::PresetColorValues::SteelBlue => "4682B4",
    a::PresetColorValues::Tan => "D2B48C",
    a::PresetColorValues::Teal => "008080",
    a::PresetColorValues::Thistle => "D8BFD8",
    a::PresetColorValues::Tomato => "FF6347",
    a::PresetColorValues::Turquoise => "40E0D0",
    a::PresetColorValues::Violet => "EE82EE",
    a::PresetColorValues::Wheat => "F5DEB3",
    a::PresetColorValues::White => "FFFFFF",
    a::PresetColorValues::WhiteSmoke => "F5F5F5",
    a::PresetColorValues::Yellow => "FFFF00",
    a::PresetColorValues::YellowGreen => "9ACD32",
  };
  ResolvedColor::from_hex(hex)
}

fn apply_transformations(color: &mut ResolvedColor, transformations: &[ColorTransformation]) {
  for transformation in transformations {
    let value = transformation.value.unwrap_or(0);
    match transformation.kind {
      ColorTransformationKind::Red => color.r = crgb_percent_to_channel(value),
      ColorTransformationKind::RedMod => color.r = mod_crgb_channel(color.r, value),
      ColorTransformationKind::RedOff => color.r = off_crgb_channel(color.r, value),
      ColorTransformationKind::Green => color.g = crgb_percent_to_channel(value),
      ColorTransformationKind::GreenMod => color.g = mod_crgb_channel(color.g, value),
      ColorTransformationKind::GreenOff => color.g = off_crgb_channel(color.g, value),
      ColorTransformationKind::Blue => color.b = crgb_percent_to_channel(value),
      ColorTransformationKind::BlueMod => color.b = mod_crgb_channel(color.b, value),
      ColorTransformationKind::BlueOff => color.b = off_crgb_channel(color.b, value),
      ColorTransformationKind::Alpha => color.alpha = clamp_percent(value),
      ColorTransformationKind::AlphaMod => {
        color.alpha = mod_value(color.alpha, value, COLOR_PERCENT_MAX);
      }
      ColorTransformationKind::AlphaOff => {
        color.alpha = offset_value(color.alpha, value, COLOR_PERCENT_MAX)
      }
      ColorTransformationKind::Shade => apply_shade(color, value),
      ColorTransformationKind::Tint => apply_tint(color, value),
      ColorTransformationKind::Gray => apply_gray(color),
      ColorTransformationKind::Comp => {
        let (h, s, l) = rgb_to_hsl(*color);
        set_rgb_preserve_alpha(color, hsl_to_rgb(wrap_hue(h, 180 * 60_000), s, l));
      }
      ColorTransformationKind::Inv => {
        apply_inverse(color);
      }
      ColorTransformationKind::Gamma => apply_gamma(color, INC_GAMMA),
      ColorTransformationKind::InvGamma => apply_gamma(color, DEC_GAMMA),
      ColorTransformationKind::Hue
      | ColorTransformationKind::HueMod
      | ColorTransformationKind::HueOff
      | ColorTransformationKind::Sat
      | ColorTransformationKind::SatMod
      | ColorTransformationKind::SatOff
      | ColorTransformationKind::Lum
      | ColorTransformationKind::LumMod
      | ColorTransformationKind::LumOff => apply_hsl_transform(color, transformation.kind, value),
    }
    color.alpha = clamp_percent(color.alpha);
  }
}

fn apply_hsl_transform(color: &mut ResolvedColor, kind: ColorTransformationKind, value: i32) {
  let (mut h, mut s, mut l) = rgb_to_hsl(*color);
  match kind {
    ColorTransformationKind::Hue => h = value.rem_euclid(360 * 60_000),
    ColorTransformationKind::HueMod => h = mod_value(h, value, 360 * 60_000),
    ColorTransformationKind::HueOff => h = wrap_hue(h, value),
    ColorTransformationKind::Sat => s = clamp_percent(value),
    ColorTransformationKind::SatMod => s = mod_value(s, value, COLOR_PERCENT_MAX),
    ColorTransformationKind::SatOff => s = offset_value(s, value, COLOR_PERCENT_MAX),
    ColorTransformationKind::Lum => l = clamp_percent(value),
    ColorTransformationKind::LumMod => l = mod_value(l, value, COLOR_PERCENT_MAX),
    ColorTransformationKind::LumOff => l = offset_value(l, value, COLOR_PERCENT_MAX),
    _ => {}
  }
  if l == 0 || l == COLOR_PERCENT_MAX {
    s = 0;
  }
  set_rgb_preserve_alpha(color, hsl_to_rgb(h, s, l));
}

fn apply_shade(color: &mut ResolvedColor, value: i32) {
  let value = clamp_percent(value);
  color.r = crgb_percent_to_channel(mod_value(
    channel_to_crgb_percent(color.r),
    value,
    COLOR_PERCENT_MAX,
  ));
  color.g = crgb_percent_to_channel(mod_value(
    channel_to_crgb_percent(color.g),
    value,
    COLOR_PERCENT_MAX,
  ));
  color.b = crgb_percent_to_channel(mod_value(
    channel_to_crgb_percent(color.b),
    value,
    COLOR_PERCENT_MAX,
  ));
}

fn apply_tint(color: &mut ResolvedColor, value: i32) {
  let value = clamp_percent(value);
  color.r = tint_crgb_channel(color.r, value);
  color.g = tint_crgb_channel(color.g, value);
  color.b = tint_crgb_channel(color.b, value);
}

fn apply_gray(color: &mut ResolvedColor) {
  let gray = (u32::from(color.r) * 22 + u32::from(color.g) * 72 + u32::from(color.b) * 6) / 100;
  color.r = gray as u8;
  color.g = gray as u8;
  color.b = gray as u8;
}

fn apply_gamma(color: &mut ResolvedColor, gamma: f64) {
  color.r = gamma_channel(color.r, gamma);
  color.g = gamma_channel(color.g, gamma);
  color.b = gamma_channel(color.b, gamma);
}

fn apply_inverse(color: &mut ResolvedColor) {
  color.r = crgb_percent_to_channel(COLOR_PERCENT_MAX - channel_to_crgb_percent(color.r));
  color.g = crgb_percent_to_channel(COLOR_PERCENT_MAX - channel_to_crgb_percent(color.g));
  color.b = crgb_percent_to_channel(COLOR_PERCENT_MAX - channel_to_crgb_percent(color.b));
}

fn set_rgb_preserve_alpha(color: &mut ResolvedColor, rgb: ResolvedColor) {
  color.r = rgb.r;
  color.g = rgb.g;
  color.b = rgb.b;
}

fn gamma_channel(value: u8, gamma: f64) -> u8 {
  crgb_percent_to_channel(gamma_percent(channel_to_crgb_percent(value), gamma))
}

fn crgb_percent_to_channel(value: i32) -> u8 {
  let gamma = gamma_percent(clamp_percent(value), INC_GAMMA);
  ((gamma * 255) / COLOR_PERCENT_MAX).clamp(0, 255) as u8
}

fn channel_to_crgb_percent(value: u8) -> i32 {
  gamma_percent((i32::from(value) * COLOR_PERCENT_MAX) / 255, DEC_GAMMA)
}

fn gamma_percent(value: i32, gamma: f64) -> i32 {
  ((clamp_percent(value) as f64 / COLOR_PERCENT_MAX as f64).powf(gamma) * COLOR_PERCENT_MAX as f64
    + 0.5) as i32
}

fn mod_crgb_channel(channel: u8, value: i32) -> u8 {
  crgb_percent_to_channel(mod_value(
    channel_to_crgb_percent(channel),
    value,
    COLOR_PERCENT_MAX,
  ))
}

fn off_crgb_channel(channel: u8, value: i32) -> u8 {
  crgb_percent_to_channel(offset_value(
    channel_to_crgb_percent(channel),
    value,
    COLOR_PERCENT_MAX,
  ))
}

fn tint_crgb_channel(channel: u8, value: i32) -> u8 {
  let channel = channel_to_crgb_percent(channel);
  let tinted = i64::from(COLOR_PERCENT_MAX)
    - (i64::from(COLOR_PERCENT_MAX - channel) * i64::from(value)) / i64::from(COLOR_PERCENT_MAX);
  crgb_percent_to_channel(tinted.clamp(0, i64::from(COLOR_PERCENT_MAX)) as i32)
}

fn mod_value(value: i32, modifier: i32, max: i32) -> i32 {
  (i64::from(value) * i64::from(modifier) / i64::from(COLOR_PERCENT_MAX)).clamp(0, i64::from(max))
    as i32
}

fn offset_value(value: i32, offset: i32, max: i32) -> i32 {
  (i64::from(value) + i64::from(offset)).clamp(0, i64::from(max)) as i32
}

fn wrap_hue(value: i32, offset: i32) -> i32 {
  (i64::from(value) + i64::from(offset)).rem_euclid(i64::from(360 * 60_000)) as i32
}

fn clamp_percent(value: i32) -> i32 {
  value.clamp(0, COLOR_PERCENT_MAX)
}

fn rgb_to_hsl(color: ResolvedColor) -> (i32, i32, i32) {
  let r = f64::from(color.r) / 255.0;
  let g = f64::from(color.g) / 255.0;
  let b = f64::from(color.b) / 255.0;
  let max = r.max(g).max(b);
  let min = r.min(g).min(b);
  let l = (max + min) / 2.0;
  if (max - min).abs() < f64::EPSILON {
    return (0, 0, (l * COLOR_PERCENT_MAX as f64).round() as i32);
  }
  let d = max - min;
  let s = if l > 0.5 {
    d / (2.0 - max - min)
  } else {
    d / (max + min)
  };
  let h = if (max - r).abs() < f64::EPSILON {
    ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
  } else if (max - g).abs() < f64::EPSILON {
    ((b - r) / d + 2.0) / 6.0
  } else {
    ((r - g) / d + 4.0) / 6.0
  };
  (
    (h * (360 * 60_000) as f64).round() as i32,
    (s * COLOR_PERCENT_MAX as f64).round() as i32,
    (l * COLOR_PERCENT_MAX as f64).round() as i32,
  )
}

fn hsl_to_rgb(hue: i32, saturation: i32, luminance: i32) -> ResolvedColor {
  let h = hue.rem_euclid(360 * 60_000) as f64 / (360 * 60_000) as f64;
  let s = clamp_percent(saturation) as f64 / COLOR_PERCENT_MAX as f64;
  let l = clamp_percent(luminance) as f64 / COLOR_PERCENT_MAX as f64;
  if s == 0.0 {
    let gray = (l * 255.0).round() as u8;
    return ResolvedColor::new(gray, gray, gray);
  }
  let q = if l < 0.5 {
    l * (1.0 + s)
  } else {
    l + s - l * s
  };
  let p = 2.0 * l - q;
  ResolvedColor::new(
    hue_to_channel(p, q, h + 1.0 / 3.0),
    hue_to_channel(p, q, h),
    hue_to_channel(p, q, h - 1.0 / 3.0),
  )
}

fn hue_to_channel(p: f64, q: f64, mut t: f64) -> u8 {
  if t < 0.0 {
    t += 1.0;
  }
  if t > 1.0 {
    t -= 1.0;
  }
  let value = if t < 1.0 / 6.0 {
    p + (q - p) * 6.0 * t
  } else if t < 1.0 / 2.0 {
    q
  } else if t < 2.0 / 3.0 {
    p + (q - p) * (2.0 / 3.0 - t) * 6.0
  } else {
    p
  };
  (value * 255.0).round().clamp(0.0, 255.0) as u8
}
