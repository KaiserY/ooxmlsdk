use std::borrow::Cow;

use ooxmlsdk_fonts::{FontRequest, TextScript, ThemeFontKind};

use crate::common::Pt;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stroke<'doc> {
  pub width: Pt,
  pub color: Color,
  pub dash: Option<Vec<Pt>>,
  pub source_style_id: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Fill<'doc> {
  #[default]
  None,
  Solid(Color),
  Theme(Cow<'doc, str>),
  Gradient(GradientFill<'doc>),
  Image {
    relationship_id: Option<Cow<'doc, str>>,
    tile: bool,
  },
  Pattern {
    foreground: Color,
    background: Color,
  },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GradientFill<'doc> {
  pub stops: Vec<GradientStop<'doc>>,
  pub angle_degrees: Option<f32>,
  /// Coordinate space used to define the gradient. When absent, the painted
  /// path bounds are used. Slide-background fills set this to the page bounds
  /// so clipping a shape does not restart the gradient inside that shape.
  pub definition_bounds: Option<super::Rect>,
  /// Resolved page-space endpoints for a transformed linear gradient. This
  /// keeps the gradient in the same local-to-page transform as its shape
  /// without forcing PDF backends to reconstruct DrawingML shape transforms.
  pub line: Option<(super::Point, super::Point)>,
  pub interpolation: GradientInterpolation,
  pub scaled: bool,
  pub path: Option<Cow<'doc, str>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GradientInterpolation {
  #[default]
  LinearSrgb,
  /// PowerPoint's fixed-format path for transformed DrawingML gradients uses
  /// the same gamma-correct sigma falloff exposed by the Windows GDI+ linear
  /// gradient brush.
  PowerPointGammaSigma,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GradientStop<'doc> {
  pub position: f32,
  pub color: Color,
  pub scheme: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutFontRequest<'doc> {
  pub base: FontRequest<'doc>,
  pub families: ScriptFontFamilies<'doc>,
  pub small_caps: bool,
  pub character_spacing: Pt,
}

impl<'doc> LayoutFontRequest<'doc> {
  pub fn from_font_request(base: FontRequest<'doc>) -> Self {
    Self {
      families: ScriptFontFamilies {
        latin: base.family.clone(),
        high_ansi: base.family.clone(),
        ..ScriptFontFamilies::default()
      },
      base,
      small_caps: false,
      character_spacing: Pt::default(),
    }
  }

  pub fn for_script(&self, script: TextScript) -> FontRequest<'doc> {
    let mut request = self.base.clone();
    request.script = Some(script);
    if let Some(family) = self.families.family_for_script(script).cloned() {
      request.family = Some(family);
      request.theme_family = None;
    } else if let Some(theme_family) = self.families.theme_for_script(script) {
      request.theme_family = Some(theme_family);
    }
    request
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScriptFontFamilies<'doc> {
  pub latin: Option<Cow<'doc, str>>,
  pub high_ansi: Option<Cow<'doc, str>>,
  pub east_asian: Option<Cow<'doc, str>>,
  pub complex_script: Option<Cow<'doc, str>>,
  pub symbol: Option<Cow<'doc, str>>,
  pub latin_theme: Option<ThemeFontKind>,
  pub east_asian_theme: Option<ThemeFontKind>,
  pub complex_script_theme: Option<ThemeFontKind>,
}

impl<'doc> ScriptFontFamilies<'doc> {
  pub fn family_for_script(&self, script: TextScript) -> Option<&Cow<'doc, str>> {
    match script {
      TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul => self
        .east_asian
        .as_ref()
        .or(self.high_ansi.as_ref())
        .or(self.latin.as_ref()),
      TextScript::Arabic | TextScript::Hebrew | TextScript::Devanagari | TextScript::Thai => self
        .complex_script
        .as_ref()
        .or(self.high_ansi.as_ref())
        .or(self.latin.as_ref()),
      TextScript::Common => self
        .symbol
        .as_ref()
        .or(self.high_ansi.as_ref())
        .or(self.latin.as_ref()),
      _ => self.high_ansi.as_ref().or(self.latin.as_ref()),
    }
  }

  pub fn theme_for_script(&self, script: TextScript) -> Option<ThemeFontKind> {
    match script {
      TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul => {
        self.east_asian_theme.or(self.latin_theme)
      }
      TextScript::Arabic | TextScript::Hebrew | TextScript::Devanagari | TextScript::Thai => {
        self.complex_script_theme.or(self.latin_theme)
      }
      _ => self.latin_theme,
    }
  }
}
