use std::borrow::Cow;

use emfsdk::emfplus::EmfPlusHatchStyle;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk_fonts::{FontRequest, TextScript, ThemeFontKind};

use crate::common::Pt;

/// Typed DrawingML effect source retained across all host lowerers.
///
/// DOCX, PPTX, XLSX, charts, and diagrams use host-specific `spPr` wrapper
/// types, but the effect payload itself is always the shared DrawingML type.
/// Keeping this enum in the common model prevents non-Presentation hosts from
/// silently dropping ordered DAGs before paint.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DrawingEffectSource {
  List {
    source: Box<a::EffectList>,
    resolved: Option<super::drawingml_image_effects::ImageEffectContainer>,
  },
  Dag {
    source: Box<a::EffectDag>,
    resolved: Option<super::drawingml_image_effects::ImageEffectContainer>,
  },
}

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
  /// Gradient paint for DrawingML outlines, resolved in the same page-space
  /// coordinate system as the owning path.
  pub gradient: Option<GradientFill<'doc>>,
  pub source_style_id: Option<Cow<'doc, str>>,
}

/// One authored DrawingML shape-style component after host color resolution.
///
/// `Unspecified` is deliberately distinct from `NoPaint`: chart series and
/// data points inherit omitted fill/outline components from their parent
/// style, while an explicit `a:noFill` suppresses that inherited paint.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum ShapeStyleValue<T> {
  #[default]
  Unspecified,
  NoPaint,
  Paint(T),
}

impl<T> ShapeStyleValue<T> {
  pub(crate) fn resolve_over<'a>(&'a self, inherited: &'a Self) -> &'a Self {
    if matches!(self, Self::Unspecified) {
      inherited
    } else {
      self
    }
  }
}

/// Resolved, inheritance-aware fill and outline authored on a DrawingML
/// `spPr` wrapper. Hosts retain this pair instead of flattening chart paint
/// to one solid RGB value during import.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ShapeStyle<'doc> {
  pub fill: ShapeStyleValue<Fill<'doc>>,
  pub stroke: ShapeStyleValue<Stroke<'doc>>,
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
    if preset == StrokeDashPreset::SystemDot && self.cap == Some(StrokeCap::Round) {
      // A round cap extends half a line width beyond both ends of every dash
      // segment. PowerPoint therefore lowers a round-capped `sysDot` to a
      // zero-length segment followed by a two-width gap: the cap turns the
      // segment into a one-width circle while preserving the two-width cycle.
      return Some(vec![Pt(0.0), Pt(2.0 * self.width.0)]);
    }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn round_system_dot_compensates_for_the_cap_extension() {
    let stroke = Stroke {
      width: Pt(1.8),
      preset_dash: Some(StrokeDashPreset::SystemDot),
      cap: Some(StrokeCap::Round),
      ..Stroke::default()
    };

    assert_eq!(stroke.resolved_dash(), Some(vec![Pt(0.0), Pt(3.6)]));
  }

  #[test]
  fn flat_system_dot_keeps_the_authored_binary_pattern() {
    let stroke = Stroke {
      width: Pt(1.8),
      preset_dash: Some(StrokeDashPreset::SystemDot),
      cap: Some(StrokeCap::Flat),
      ..Stroke::default()
    };

    assert_eq!(stroke.resolved_dash(), Some(vec![Pt(1.8), Pt(1.8)]));
  }
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
  /// Unresolved DrawingML `rotWithShape` intent. Hosts that lower a shape
  /// before its final page transform retain this until they can resolve the
  /// gradient line/path into page coordinates.
  pub rotate_with_shape: Option<bool>,
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
  /// The authored tile rectangle is smaller than the owning shape, so Office
  /// mirrors successive copies in both axes while covering the shape.
  pub mirror_tile: bool,
}

/// Expands a DrawingML circle-gradient rectangle to its circumscribed circle.
///
/// PowerPoint uses half of the transformed rectangle's diagonal as the outer
/// radius. LibreOffice tracks the smaller-rendering interoperability gap as
/// tdf#166140 in `oox/source/drawingml/fillproperties.cxx`.
pub fn office_circle_gradient_transform(transform: super::Transform) -> super::Transform {
  let width = transform.m11.hypot(transform.m12);
  let height = transform.m21.hypot(transform.m22);
  let diameter = width.hypot(height);
  if width <= f32::EPSILON || height <= f32::EPSILON || diameter <= f32::EPSILON {
    return transform;
  }

  let m11 = transform.m11 / width * diameter;
  let m12 = transform.m12 / width * diameter;
  let m21 = transform.m21 / height * diameter;
  let m22 = transform.m22 / height * diameter;
  let center_x = transform.dx.0 + (transform.m11 + transform.m21) * 0.5;
  let center_y = transform.dy.0 + (transform.m12 + transform.m22) * 0.5;
  super::Transform {
    m11,
    m12,
    m21,
    m22,
    dx: Pt(center_x - (m11 + m21) * 0.5),
    dy: Pt(center_y - (m12 + m22) * 0.5),
  }
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
  /// Microsoft Office's fixed-format path for a two-stop DrawingML gradient
  /// uses the same gamma-correct sigma falloff exposed by the Windows GDI+
  /// linear gradient brush.
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
