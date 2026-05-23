use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::{
  Color, hsl_color, preset_color, rgb_hex_color, rgb_percent_color, system_color,
};
use super::fill::FillProperties;
use super::line::LineProperties;
use super::shape_properties::EffectProperties;

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ThemeFontScheme {
  pub(crate) name: String,
  pub(crate) major_latin: Option<String>,
  pub(crate) minor_latin: Option<String>,
  pub(crate) major_east_asian: Option<String>,
  pub(crate) minor_east_asian: Option<String>,
  pub(crate) major_complex_script: Option<String>,
  pub(crate) minor_complex_script: Option<String>,
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

impl ThemeFontScheme {
  pub(crate) fn from_dml(scheme: &a::FontScheme) -> Self {
    Self {
      name: scheme.name.clone(),
      major_latin: text_font_typeface(&scheme.major_font.latin_font.typeface),
      minor_latin: text_font_typeface(&scheme.minor_font.latin_font.typeface),
      major_east_asian: text_font_typeface(&scheme.major_font.east_asian_font.typeface),
      minor_east_asian: text_font_typeface(&scheme.minor_font.east_asian_font.typeface),
      major_complex_script: text_font_typeface(&scheme.major_font.complex_script_font.typeface),
      minor_complex_script: text_font_typeface(&scheme.minor_font.complex_script_font.typeface),
    }
  }

  pub(crate) fn latin_font(&self, index: a::FontCollectionIndexValues) -> Option<&str> {
    match index {
      a::FontCollectionIndexValues::Major => self.major_latin.as_deref(),
      a::FontCollectionIndexValues::Minor => self.minor_latin.as_deref(),
      a::FontCollectionIndexValues::None => None,
    }
  }
}

impl ThemeColorEntry {
  fn new(token: a::ColorSchemeIndexValues, color: Option<Color>) -> Self {
    Self { token, color }
  }
}

fn text_font_typeface(typeface: &Option<String>) -> Option<String> {
  typeface.as_ref().filter(|value| !value.is_empty()).cloned()
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
  EffectProperties::from_effect_style(style)
}

fn color_from_dark1(choice: Option<&a::Dark1ColorChoice>) -> Option<Color> {
  match choice? {
    a::Dark1ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Dark1ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Dark1ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Dark1ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Dark1ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_light1(choice: Option<&a::Light1ColorChoice>) -> Option<Color> {
  match choice? {
    a::Light1ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Light1ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Light1ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Light1ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Light1ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_dark2(choice: Option<&a::Dark2ColorChoice>) -> Option<Color> {
  match choice? {
    a::Dark2ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Dark2ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Dark2ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Dark2ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Dark2ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_light2(choice: Option<&a::Light2ColorChoice>) -> Option<Color> {
  match choice? {
    a::Light2ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Light2ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Light2ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Light2ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Light2ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_accent1(choice: Option<&a::Accent1ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent1ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Accent1ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Accent1ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Accent1ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent1ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_accent2(choice: Option<&a::Accent2ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent2ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Accent2ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Accent2ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Accent2ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent2ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_accent3(choice: Option<&a::Accent3ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent3ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Accent3ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Accent3ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Accent3ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent3ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_accent4(choice: Option<&a::Accent4ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent4ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Accent4ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Accent4ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Accent4ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent4ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_accent5(choice: Option<&a::Accent5ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent5ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Accent5ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Accent5ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Accent5ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent5ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_accent6(choice: Option<&a::Accent6ColorChoice>) -> Option<Color> {
  match choice? {
    a::Accent6ColorChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::Accent6ColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::Accent6ColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::Accent6ColorChoice::SystemColor(color) => Some(system_color(color)),
    a::Accent6ColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_hyperlink(choice: Option<&a::HyperlinkChoice>) -> Option<Color> {
  match choice? {
    a::HyperlinkChoice::RgbColorModelPercentage(color) => Some(rgb_percent_color(color)),
    a::HyperlinkChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::HyperlinkChoice::HslColor(color) => Some(hsl_color(color)),
    a::HyperlinkChoice::SystemColor(color) => Some(system_color(color)),
    a::HyperlinkChoice::PresetColor(color) => Some(preset_color(color)),
  }
}

fn color_from_followed_hyperlink(
  choice: Option<&a::FollowedHyperlinkColorChoice>,
) -> Option<Color> {
  match choice? {
    a::FollowedHyperlinkColorChoice::RgbColorModelPercentage(color) => {
      Some(rgb_percent_color(color))
    }
    a::FollowedHyperlinkColorChoice::RgbColorModelHex(color) => Some(rgb_hex_color(color)),
    a::FollowedHyperlinkColorChoice::HslColor(color) => Some(hsl_color(color)),
    a::FollowedHyperlinkColorChoice::SystemColor(color) => Some(system_color(color)),
    a::FollowedHyperlinkColorChoice::PresetColor(color) => Some(preset_color(color)),
  }
}
