use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Color {
  RgbHex(RgbHexColor),
  Scheme(SchemeColor),
  Preset(PresetColor),
  System(SystemColor),
}

impl Color {
  pub(crate) fn from_solid_fill_choice(choice: &a::SolidFillChoice) -> Option<Self> {
    match choice {
      a::SolidFillChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::SolidFillChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::SolidFillChoice::PresetColor(color) => Some(preset_color(color)),
      a::SolidFillChoice::SystemColor(color) => Some(system_color(color)),
      a::SolidFillChoice::RgbColorModelPercentage(_) | a::SolidFillChoice::HslColor(_) => None,
    }
  }

  pub(crate) fn from_fill_reference_choice(choice: &a::FillReferenceChoice) -> Option<Self> {
    match choice {
      a::FillReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::FillReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::FillReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      a::FillReferenceChoice::SystemColor(color) => Some(system_color(color)),
      a::FillReferenceChoice::RgbColorModelPercentage(_) | a::FillReferenceChoice::HslColor(_) => {
        None
      }
    }
  }

  pub(crate) fn from_line_reference_choice(choice: &a::LineReferenceChoice) -> Option<Self> {
    match choice {
      a::LineReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::LineReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::LineReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      a::LineReferenceChoice::SystemColor(color) => Some(system_color(color)),
      a::LineReferenceChoice::RgbColorModelPercentage(_) | a::LineReferenceChoice::HslColor(_) => {
        None
      }
    }
  }

  pub(crate) fn from_effect_reference_choice(choice: &a::EffectReferenceChoice) -> Option<Self> {
    match choice {
      a::EffectReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::EffectReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::EffectReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      a::EffectReferenceChoice::SystemColor(color) => Some(system_color(color)),
      a::EffectReferenceChoice::RgbColorModelPercentage(_)
      | a::EffectReferenceChoice::HslColor(_) => None,
    }
  }

  pub(crate) fn from_font_reference_choice(choice: &a::FontReferenceChoice) -> Option<Self> {
    match choice {
      a::FontReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      a::FontReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      a::FontReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      a::FontReferenceChoice::SystemColor(color) => Some(system_color(color)),
      a::FontReferenceChoice::RgbColorModelPercentage(_) | a::FontReferenceChoice::HslColor(_) => {
        None
      }
    }
  }

  pub(crate) fn from_background_style_reference_choice(
    choice: &p::BackgroundStyleReferenceChoice,
  ) -> Option<Self> {
    match choice {
      p::BackgroundStyleReferenceChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
      p::BackgroundStyleReferenceChoice::SchemeColor(color) => Some(scheme_color(color)),
      p::BackgroundStyleReferenceChoice::PresetColor(color) => Some(preset_color(color)),
      p::BackgroundStyleReferenceChoice::SystemColor(color) => Some(system_color(color)),
      p::BackgroundStyleReferenceChoice::RgbColorModelPercentage(_)
      | p::BackgroundStyleReferenceChoice::HslColor(_) => None,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RgbHexColor {
  pub(crate) value: String,
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
color_transformations_from_choices!(transformations_from_system_color_choices, SystemColorChoice);
color_transformations_from_choices!(transformations_from_scheme_color_choices, SchemeColorChoice);
color_transformations_from_choices!(transformations_from_preset_color_choices, PresetColorChoice);
