use std::borrow::Cow;

use emfsdk::emfplus::EmfPlusHatchStyle;
use ooxmlsdk_fonts::{FontRequest, TextScript, ThemeFontKind};

use crate::common::Pt;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stroke<'doc> {
  pub width: Pt,
  pub color: Color,
  /// Explicit physical dash lengths. DrawingML preset dash names are retained
  /// separately because their expansion depends on line width and cap style.
  pub dash: Option<Vec<Pt>>,
  pub preset_dash: Option<StrokeDashPreset>,
  pub dash_offset: Pt,
  pub cap: Option<StrokeCap>,
  pub join: Option<StrokeJoin>,
  pub compound: Option<StrokeCompound>,
  pub alignment: Option<StrokeAlignment>,
  pub head_end: Option<StrokeEnd>,
  pub tail_end: Option<StrokeEnd>,
  /// Pattern paint for DrawingML outlines. The solid `color` remains the
  /// fallback used by consumers that cannot paint a tiling pattern.
  pub pattern: Option<PatternFill>,
  pub source_style_id: Option<Cow<'doc, str>>,
}

impl Stroke<'_> {
  /// Returns the authored physical dash array, expanding a DrawingML preset
  /// relative to the resolved line width when necessary.
  ///
  /// The preset multipliers follow LibreOffice
  /// `oox/source/drawingml/lineproperties.cxx`; keeping the expansion here
  /// gives vector paint and effect-mask geometry one source of truth.
  pub fn resolved_dash(&self) -> Option<Vec<Pt>> {
    if let Some(dash) = &self.dash {
      return Some(dash.clone());
    }
    let preset = self.preset_dash?;
    let multipliers: &[f32] = match preset {
      StrokeDashPreset::Solid => return None,
      StrokeDashPreset::Dot => &[1.0, 3.0],
      StrokeDashPreset::Dash => &[4.0, 3.0],
      StrokeDashPreset::LargeDash => &[8.0, 3.0],
      StrokeDashPreset::DashDot => &[4.0, 3.0, 1.0, 3.0],
      StrokeDashPreset::LargeDashDot => &[8.0, 3.0, 1.0, 3.0],
      StrokeDashPreset::LargeDashDotDot => &[8.0, 3.0, 1.0, 3.0, 1.0, 3.0],
      StrokeDashPreset::SystemDash => &[3.0, 1.0],
      StrokeDashPreset::SystemDot => &[1.0, 1.0],
      StrokeDashPreset::SystemDashDot => &[3.0, 1.0, 1.0, 1.0],
      StrokeDashPreset::SystemDashDotDot => &[3.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    };
    Some(
      multipliers
        .iter()
        .map(|multiplier| Pt(multiplier * self.width.0))
        .collect(),
    )
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrokeCap {
  Round,
  Square,
  Flat,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StrokeJoin {
  Round,
  Bevel,
  Miter { limit: Option<f32> },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrokeCompound {
  Single,
  Double,
  ThickThin,
  ThinThick,
  Triple,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrokeAlignment {
  Center,
  Inside,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrokeDashPreset {
  Solid,
  Dot,
  Dash,
  LargeDash,
  DashDot,
  LargeDashDot,
  LargeDashDotDot,
  SystemDash,
  SystemDot,
  SystemDashDot,
  SystemDashDotDot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StrokeEnd {
  pub kind: StrokeEndKind,
  pub width: StrokeEndSize,
  pub length: StrokeEndSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrokeEndKind {
  None,
  Triangle,
  Stealth,
  Diamond,
  Oval,
  Arrow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrokeEndSize {
  Small,
  Medium,
  Large,
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
  Pattern(PatternFill),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatternFill {
  pub hatch_style: EmfPlusHatchStyle,
  pub foreground: Color,
  pub background: Color,
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
  /// Resolved path-gradient geometry. DrawingML defines this independently
  /// from the painted shape path; the latter remains on [`super::PathItem`]
  /// and acts as the final clip.
  pub path: Option<GradientPath>,
}

/// Static DrawingML path-gradient geometry after host inheritance and defaults
/// have been resolved.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientPath {
  pub kind: GradientPathKind,
  /// Insets from the corresponding sides of the shape bounds, expressed as
  /// ratios. Positive values inset and negative values outset the focus path.
  pub fill_to: RelativeRect,
  /// Maps the unit shape bounds to page space. Keeping the full affine retains
  /// `rotWithShape`, flips, and non-square circle gradients without asking the
  /// PDF backend to reconstruct host shape transforms.
  pub transform: super::Transform,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GradientPathKind {
  #[default]
  Shape,
  Circle,
  Rectangle,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RelativeRect {
  pub left: f32,
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
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
