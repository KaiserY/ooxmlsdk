use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::{Color, PresetColor, SystemColor};
use super::fill::FillProperties;
use super::line::LineProperties;
use super::shape_properties::EffectProperties;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Theme {
  pub(crate) path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ThemeColorScheme {
  pub(crate) name: String,
  pub(crate) entries: Vec<ThemeColorEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ThemeColorEntry {
  pub(crate) token: a::ColorSchemeIndexValues,
  pub(crate) color: Option<Color>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ThemeFormatScheme {
  pub(crate) name: Option<String>,
  pub(crate) fill_styles: Vec<FillProperties>,
  pub(crate) background_fill_styles: Vec<FillProperties>,
  pub(crate) line_styles: Vec<LineProperties>,
  pub(crate) effect_styles: Vec<EffectProperties>,
}

impl ThemeColorScheme {
  pub(crate) fn from_dml(scheme: &a::ColorScheme) -> Self {
    Self {
      name: scheme.name.clone(),
      entries: vec![
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Dark1,
          color_from_dark1(scheme.dark1_color.dark1_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Light1,
          color_from_light1(scheme.light1_color.light1_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Dark2,
          color_from_dark2(scheme.dark2_color.dark2_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Light2,
          color_from_light2(scheme.light2_color.light2_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Accent1,
          color_from_accent1(scheme.accent1_color.accent1_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Accent2,
          color_from_accent2(scheme.accent2_color.accent2_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Accent3,
          color_from_accent3(scheme.accent3_color.accent3_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Accent4,
          color_from_accent4(scheme.accent4_color.accent4_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Accent5,
          color_from_accent5(scheme.accent5_color.accent5_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Accent6,
          color_from_accent6(scheme.accent6_color.accent6_color_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::Hyperlink,
          color_from_hyperlink(scheme.hyperlink.hyperlink_choice.as_ref()),
        ),
        ThemeColorEntry::new(
          a::ColorSchemeIndexValues::FollowedHyperlink,
          color_from_followed_hyperlink(
            scheme
              .followed_hyperlink_color
              .followed_hyperlink_color_choice
              .as_ref(),
          ),
        ),
      ],
    }
  }

  pub(crate) fn get_color(&self, token: a::ColorSchemeIndexValues) -> Option<&Color> {
    self
      .entries
      .iter()
      .find(|entry| entry.token == token)
      .and_then(|entry| entry.color.as_ref())
  }
}

impl ThemeFormatScheme {
  pub(crate) fn from_dml(scheme: &a::FormatScheme) -> Self {
    Self {
      name: scheme.name.clone(),
      fill_styles: scheme
        .fill_style_list
        .fill_style_list_choice
        .iter()
        .map(FillProperties::from_fill_style_choice)
        .collect(),
      background_fill_styles: scheme
        .background_fill_style_list
        .background_fill_style_list_choice
        .iter()
        .map(FillProperties::from_background_fill_style_choice)
        .collect(),
      line_styles: scheme
        .line_style_list
        .outline
        .iter()
        .filter_map(LineProperties::from_dml_outline)
        .collect(),
      effect_styles: scheme
        .effect_style_list
        .effect_style
        .iter()
        .map(effect_properties_from_dml)
        .collect(),
    }
  }

  pub(crate) fn get_fill_style(&self, index: u32) -> Option<&FillProperties> {
    if index >= 1000 {
      style_at(&self.background_fill_styles, index - 1000)
    } else {
      style_at(&self.fill_styles, index)
    }
  }

  pub(crate) fn get_line_style(&self, index: u32) -> Option<&LineProperties> {
    style_at(&self.line_styles, index)
  }

  pub(crate) fn get_effect_style(&self, index: u32) -> Option<&EffectProperties> {
    style_at(&self.effect_styles, index)
  }
}

impl ThemeColorEntry {
  fn new(token: a::ColorSchemeIndexValues, color: Option<Color>) -> Self {
    Self { token, color }
  }
}

fn style_at<T>(styles: &[T], index: u32) -> Option<&T> {
  if styles.is_empty() || index < 1 {
    return None;
  }
  let clamped = usize::try_from(index - 1)
    .ok()
    .map(|index| index.min(styles.len() - 1))?;
  styles.get(clamped)
}

fn effect_properties_from_dml(style: &a::EffectStyle) -> EffectProperties {
  EffectProperties {
    has_effect: style.effect_style_choice.is_some()
      || style.scene3_d_type.is_some()
      || style.shape3_d_type.is_some(),
  }
}

fn color_from_dark1(choice: Option<&a::Dark1ColorChoice>) -> Option<Color> {
  match choice? {
    a::Dark1ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Dark1ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Dark1ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Dark1ColorChoice::RgbColorModelPercentage(_) | a::Dark1ColorChoice::HslColor(_) => None,
  }
}

fn color_from_light1(choice: Option<&a::Light1ColorChoice>) -> Option<Color> {
  match choice? {
    a::Light1ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Light1ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Light1ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Light1ColorChoice::RgbColorModelPercentage(_) | a::Light1ColorChoice::HslColor(_) => None,
  }
}

fn color_from_dark2(choice: Option<&a::Dark2ColorChoice>) -> Option<Color> {
  match choice? {
    a::Dark2ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Dark2ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Dark2ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Dark2ColorChoice::RgbColorModelPercentage(_) | a::Dark2ColorChoice::HslColor(_) => None,
  }
}

fn color_from_light2(choice: Option<&a::Light2ColorChoice>) -> Option<Color> {
  match choice? {
    a::Light2ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Light2ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Light2ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Light2ColorChoice::RgbColorModelPercentage(_) | a::Light2ColorChoice::HslColor(_) => None,
  }
}

fn color_from_accent1(choice: Option<&a::Accent1ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent1ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Accent1ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent1ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Accent1ColorChoice::RgbColorModelPercentage(_) | a::Accent1ColorChoice::HslColor(_) => None,
  }
}

fn color_from_accent2(choice: Option<&a::Accent2ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent2ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Accent2ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent2ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Accent2ColorChoice::RgbColorModelPercentage(_) | a::Accent2ColorChoice::HslColor(_) => None,
  }
}

fn color_from_accent3(choice: Option<&a::Accent3ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent3ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Accent3ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent3ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Accent3ColorChoice::RgbColorModelPercentage(_) | a::Accent3ColorChoice::HslColor(_) => None,
  }
}

fn color_from_accent4(choice: Option<&a::Accent4ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent4ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Accent4ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent4ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Accent4ColorChoice::RgbColorModelPercentage(_) | a::Accent4ColorChoice::HslColor(_) => None,
  }
}

fn color_from_accent5(choice: Option<&a::Accent5ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent5ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Accent5ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent5ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Accent5ColorChoice::RgbColorModelPercentage(_) | a::Accent5ColorChoice::HslColor(_) => None,
  }
}

fn color_from_accent6(choice: Option<&a::Accent6ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent6ColorChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::Accent6ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent6ColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::Accent6ColorChoice::RgbColorModelPercentage(_) | a::Accent6ColorChoice::HslColor(_) => None,
  }
}

fn color_from_hyperlink(choice: Option<&a::HyperlinkChoice>) -> Option<Color> {
  match choice? {
    a::HyperlinkChoice::RgbColorModelHex(color) => Some(Color::RgbHex(color.val.clone())),
    a::HyperlinkChoice::SystemColor(color) => Some(system_color(color)),
    a::HyperlinkChoice::PresetColor(color) => Some(preset_color(color)),
    a::HyperlinkChoice::RgbColorModelPercentage(_) | a::HyperlinkChoice::HslColor(_) => None,
  }
}

fn color_from_followed_hyperlink(
  choice: Option<&a::FollowedHyperlinkColorChoice>,
) -> Option<Color> {
  match choice? {
    a::FollowedHyperlinkColorChoice::RgbColorModelHex(color) => {
      Some(Color::RgbHex(color.val.clone()))
    }
    a::FollowedHyperlinkColorChoice::SystemColor(color) => Some(system_color(color)),
    a::FollowedHyperlinkColorChoice::PresetColor(color) => Some(preset_color(color)),
    a::FollowedHyperlinkColorChoice::RgbColorModelPercentage(_)
    | a::FollowedHyperlinkColorChoice::HslColor(_) => None,
  }
}

fn system_color(color: &a::SystemColor) -> Color {
  Color::System(SystemColor {
    value: color.val,
    last_color: color.last_color.clone(),
  })
}

fn preset_color(color: &a::PresetColor) -> Color {
  Color::Preset(PresetColor { value: color.val })
}
