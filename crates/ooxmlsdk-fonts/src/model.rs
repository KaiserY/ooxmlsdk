use std::borrow::Cow;
use std::collections::BTreeSet;
use std::ops::Range;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use rustybuzz::{
  Direction as BuzzDirection, Face as BuzzFace, Language as BuzzLanguage, Script as BuzzScript,
  UnicodeBuffer, script,
};
use ttf_parser::Face as TtfFace;

use crate::{FontError, Result};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct FontId(pub Arc<str>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontRegistry<'a> {
  pub sources: Vec<FontSource<'a>>,
  pub faces: Vec<RegisteredFontFace<'a>>,
  pub book: FontBook<'a>,
}

impl<'a> FontRegistry<'a> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn register_face(&mut self, source: FontSource<'a>, face: FontFaceInfo<'a>) {
    self.sources.push(source.clone());
    self.faces.push(RegisteredFontFace {
      source,
      family_names: face.family_names.clone(),
      style_name: face.style_name.clone(),
      weight: face.weight,
      slant: face.slant,
      stretch: face.stretch,
      pitch: face.pitch,
      charset: None,
      face_index: face.face_index,
      origin_priority: 0,
    });
    self.book.faces.push(face);
  }

  pub fn register_memory_font(
    &mut self,
    id: impl Into<Cow<'a, str>>,
    data: impl Into<Cow<'a, [u8]>>,
  ) -> Result<FontId> {
    let id = id.into();
    let data = data.into();
    let face = FontFaceInfo::from_ttf_bytes(id.as_ref(), data.as_ref(), 0)?;
    let font_id = face.font_id.clone();
    self.register_face(FontSource::Memory { id, data }, face);
    Ok(font_id)
  }

  pub fn resolve(&self, request: &FontRequest<'a>) -> Result<ResolvedFont<'a>> {
    self.book.resolve(request, &self.faces)
  }

  pub fn shape_text(
    &self,
    request: &FontRequest<'a>,
    text: impl Into<Cow<'a, str>>,
    direction: TextDirection,
  ) -> Result<ShapedRun<'a>> {
    let resolved = self.resolve(request)?;
    match &resolved.source {
      FontSource::Memory { data, .. }
      | FontSource::EmbeddedOoxml { data, .. }
      | FontSource::TestFixture { data, .. } => resolved.shape_with_ttf_bytes(
        text,
        data.as_ref(),
        request.size_pt,
        direction,
        request.script,
        request.language.clone(),
      ),
      FontSource::System | FontSource::Path(_) => Ok(resolved.shape_approximate(
        text,
        request.size_pt,
        direction,
        request.script,
        request.language.clone(),
      )),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontBook<'a> {
  pub faces: Vec<FontFaceInfo<'a>>,
  pub family_aliases: Vec<FontFamilyAlias<'a>>,
  pub substitutions: Vec<FontSubstitutionRule<'a>>,
  pub fallback_chains: Vec<FontFallbackChain<'a>>,
  pub fallback_cache: Vec<GlyphFallbackCacheEntry<'a>>,
}

impl<'a> FontBook<'a> {
  pub fn resolve(
    &self,
    request: &FontRequest<'a>,
    registered_faces: &[RegisteredFontFace<'a>],
  ) -> Result<ResolvedFont<'a>> {
    let requested_family = request.family.clone();
    let aliased_family = request
      .family
      .as_ref()
      .map(|family| Cow::Owned(resolve_family_alias(self, family).into_owned()));
    let substituted_family = aliased_family
      .as_ref()
      .map(|family| substitute_family(self, family).into_owned());
    let target_family = substituted_family
      .as_deref()
      .or(aliased_family.as_deref())
      .or(requested_family.as_deref());

    let mut candidates = Vec::new();
    for face in &self.faces {
      let family = primary_family(face);
      let mut rejected = false;
      let mut reason = None;

      if let Some(target) = target_family {
        if family_matches(face, target) {
          reason = Some(FontMatchReason::Family);
        } else {
          rejected = true;
          reason = Some(FontMatchReason::Family);
        }
      }

      let requested_weight = requested_weight(request);
      let requested_slant = requested_slant(request);
      let requested_stretch = request.stretch.unwrap_or(FontStretch::Normal);
      let slant_mismatch = face.slant != requested_slant;
      let stretch_distance = stretch_distance(face.stretch, requested_stretch);
      let weight_distance = weight_distance(face.weight, requested_weight);
      let pitch_mismatch = request.pitch.is_some_and(|pitch| pitch != face.pitch);
      if slant_mismatch && !rejected {
        reason = Some(FontMatchReason::Slant);
      }
      if stretch_distance != 0 && !rejected && reason == Some(FontMatchReason::Family) {
        reason = Some(FontMatchReason::Stretch);
      }
      if pitch_mismatch && !rejected && reason == Some(FontMatchReason::Family) {
        reason = Some(FontMatchReason::Pitch);
      }

      let rank = FontMatchRank {
        rejected,
        slant_mismatch,
        stretch_distance,
        weight_distance,
        pitch_mismatch,
      };
      candidates.push((
        rank,
        FontMatchCandidate {
          font_id: face.font_id.clone(),
          family: family.into_owned().into(),
          score: -rank.distance(),
          rejected,
          reason,
        },
      ));
    }

    candidates.sort_by(|left, right| {
      left
        .0
        .cmp(&right.0)
        .then_with(|| left.1.family.cmp(&right.1.family))
    });
    let candidates = candidates
      .into_iter()
      .map(|(_, candidate)| candidate)
      .collect::<Vec<_>>();

    let Some(winner) = candidates.iter().find(|candidate| !candidate.rejected) else {
      return Err(FontError::NoMatch);
    };
    let Some(face) = self
      .faces
      .iter()
      .find(|face| face.font_id == winner.font_id)
    else {
      return Err(FontError::NoMatch);
    };
    let registered = registered_faces.iter().find(|registered| {
      registered.face_index == face.face_index && family_overlaps(registered, face)
    });

    let synthetic_bold =
      request.bold && font_weight_number(face.weight) < font_weight_number(FontWeight::Bold);
    let synthetic_italic = request.italic && face.slant == FontSlant::Upright;
    let substitution = requested_family
      .as_ref()
      .zip(substituted_family.as_ref())
      .and_then(|(requested, substituted)| {
        (requested.as_ref() != substituted.as_str()).then(|| FontSubstitution {
          requested_family: requested.clone(),
          substituted_family: Cow::Owned(substituted.clone()),
          reason: FontSubstitutionReason::Alias,
        })
      });

    Ok(ResolvedFont {
      font_id: face.font_id.clone(),
      requested_family: request.family.clone(),
      resolved_family: primary_family(face),
      source: registered
        .map(|face| face.source.clone())
        .unwrap_or(FontSource::System),
      face_index: face.face_index,
      synthetic_bold,
      synthetic_italic,
      variation_values: request.variations.clone(),
      metrics: face.metrics.clone(),
      substitution,
      match_diagnostics: FontMatchDiagnostics {
        candidates,
        fallback_level: None,
      },
    })
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontFaceInfo<'a> {
  pub font_id: FontId,
  pub family_names: Vec<Cow<'a, str>>,
  pub postscript_name: Option<Cow<'a, str>>,
  pub style_name: Option<Cow<'a, str>>,
  pub weight: FontWeight,
  pub slant: FontSlant,
  pub stretch: FontStretch,
  pub pitch: FontPitch,
  pub coverage: FontCoverage,
  pub flags: FontFlags,
  pub axes: Vec<VariationAxis<'a>>,
  pub features: Vec<OpenTypeFeature<'a>>,
  pub metrics: FontMetrics,
  pub face_index: u32,
}

impl<'a> FontFaceInfo<'a> {
  pub fn synthetic(id: impl Into<Arc<str>>, family: impl Into<Cow<'a, str>>) -> Self {
    Self {
      font_id: FontId(id.into()),
      family_names: vec![family.into()],
      postscript_name: None,
      style_name: None,
      weight: FontWeight::Normal,
      slant: FontSlant::Upright,
      stretch: FontStretch::Normal,
      pitch: FontPitch::Variable,
      coverage: FontCoverage::default(),
      flags: FontFlags::default(),
      axes: Vec::new(),
      features: Vec::new(),
      metrics: FontMetrics::default(),
      face_index: 0,
    }
  }

  pub fn from_ttf_bytes(id: &str, data: &[u8], face_index: u32) -> Result<Self> {
    let face = TtfFace::parse(data, face_index).map_err(|_| FontError::InvalidFace)?;
    let mut family_names = Vec::new();
    let mut postscript_name = None;
    let mut style_name = None;
    for name in face.names() {
      let Some(value) = name.to_string() else {
        continue;
      };
      match name.name_id {
        ttf_parser::name_id::FAMILY => push_unique_string(&mut family_names, value),
        ttf_parser::name_id::TYPOGRAPHIC_FAMILY => push_unique_string(&mut family_names, value),
        ttf_parser::name_id::POST_SCRIPT_NAME => postscript_name = Some(Cow::Owned(value)),
        ttf_parser::name_id::SUBFAMILY => style_name = Some(Cow::Owned(value)),
        _ => {}
      }
    }
    if family_names.is_empty() {
      family_names.push(Cow::Owned(id.to_string()));
    }

    let pitch = if face.is_monospaced() {
      FontPitch::Fixed
    } else {
      FontPitch::Variable
    };
    let flags = FontFlags {
      monospace: face.is_monospaced(),
      ..FontFlags::default()
    };
    let metrics = font_metrics_from_ttf(&face, 1.0);

    Ok(Self {
      font_id: FontId(Arc::from(id)),
      family_names,
      postscript_name,
      style_name,
      weight: font_weight_from_ttf(face.weight().to_number()),
      slant: font_slant_from_ttf(face.style()),
      stretch: font_stretch_from_ttf(face.width().to_number()),
      pitch,
      coverage: font_coverage_from_ttf(&face),
      flags,
      axes: Vec::new(),
      features: Vec::new(),
      metrics,
      face_index,
    })
  }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FontCoverage {
  pub unicode_ranges: Vec<Range<u32>>,
  pub scripts: BTreeSet<TextScript>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontFlags {
  pub symbolic: bool,
  pub serif: bool,
  pub monospace: bool,
  pub color_glyphs: bool,
  pub vertical: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontFamilyAlias<'a> {
  pub from: Cow<'a, str>,
  pub to: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontSubstitutionRule<'a> {
  pub requested_family: Cow<'a, str>,
  pub substitute_family: Cow<'a, str>,
  pub reason: FontSubstitutionReason,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FontFallbackChain<'a> {
  pub script: Option<TextScript>,
  pub language: Option<Cow<'a, str>>,
  pub families: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphFallbackCacheEntry<'a> {
  pub codepoint: u32,
  pub request: FontRequestKey<'a>,
  pub fallback_font_id: Option<FontId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontRequestKey<'a> {
  pub family: Option<Cow<'a, str>>,
  pub weight: Option<FontWeight>,
  pub slant: Option<FontSlant>,
  pub stretch: Option<FontStretch>,
  pub script: Option<TextScript>,
  pub language: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariationAxis<'a> {
  pub tag: Cow<'a, str>,
  pub name: Option<Cow<'a, str>>,
  pub min: f32,
  pub default: f32,
  pub max: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenTypeFeature<'a> {
  pub tag: Cow<'a, str>,
  pub enabled_by_default: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FontSource<'a> {
  System,
  Path(PathBuf),
  Memory {
    id: Cow<'a, str>,
    data: Cow<'a, [u8]>,
  },
  EmbeddedOoxml {
    id: Cow<'a, str>,
    data: Cow<'a, [u8]>,
  },
  TestFixture {
    id: Cow<'a, str>,
    data: Cow<'a, [u8]>,
  },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredFontFace<'a> {
  pub source: FontSource<'a>,
  pub family_names: Vec<Cow<'a, str>>,
  pub style_name: Option<Cow<'a, str>>,
  pub weight: FontWeight,
  pub slant: FontSlant,
  pub stretch: FontStretch,
  pub pitch: FontPitch,
  pub charset: Option<FontCharset>,
  pub face_index: u32,
  pub origin_priority: u16,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ThemeFontMap<'a> {
  pub major_latin: Option<Cow<'a, str>>,
  pub minor_latin: Option<Cow<'a, str>>,
  pub major_east_asian: Option<Cow<'a, str>>,
  pub minor_east_asian: Option<Cow<'a, str>>,
  pub major_complex_script: Option<Cow<'a, str>>,
  pub minor_complex_script: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontRequest<'a> {
  pub family: Option<Cow<'a, str>>,
  pub theme_family: Option<ThemeFontKind>,
  pub bold: bool,
  pub italic: bool,
  pub weight: Option<FontWeight>,
  pub slant: Option<FontSlant>,
  pub stretch: Option<FontStretch>,
  pub size_pt: FontSize,
  pub script: Option<TextScript>,
  pub language: Option<Cow<'a, str>>,
  pub region: Option<Cow<'a, str>>,
  pub charset: Option<FontCharset>,
  pub pitch: Option<FontPitch>,
  pub variations: Vec<VariationValue<'a>>,
  pub features: Vec<FeatureValue<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFont<'a> {
  pub font_id: FontId,
  pub requested_family: Option<Cow<'a, str>>,
  pub resolved_family: Cow<'a, str>,
  pub source: FontSource<'a>,
  pub face_index: u32,
  pub synthetic_bold: bool,
  pub synthetic_italic: bool,
  pub variation_values: Vec<VariationValue<'a>>,
  pub metrics: FontMetrics,
  pub substitution: Option<FontSubstitution<'a>>,
  pub match_diagnostics: FontMatchDiagnostics<'a>,
}

impl<'a> ResolvedFont<'a> {
  pub fn metrics_at_size(&self, size: FontSize) -> FontMetrics {
    self.metrics.scaled(size.0)
  }

  pub fn shape_approximate(
    &self,
    text: impl Into<Cow<'a, str>>,
    size: FontSize,
    direction: TextDirection,
    script: Option<TextScript>,
    language: Option<Cow<'a, str>>,
  ) -> ShapedRun<'a> {
    let text = text.into();
    let safe_breaks = text_safe_breaks(text.as_ref());
    let glyphs = approximate_glyphs(text.as_ref(), size);
    let advance_pt = glyphs.iter().map(|glyph| glyph.x_advance_pt).sum();
    ShapedRun {
      font_id: self.font_id.clone(),
      text_range: 0..text.len(),
      text,
      glyphs: Cow::Owned(glyphs),
      advance_pt,
      direction,
      script,
      language,
      safe_breaks,
      approximate: true,
      decorations: Vec::new(),
    }
  }

  pub fn shape_with_ttf_bytes(
    &self,
    text: impl Into<Cow<'a, str>>,
    data: &[u8],
    size: FontSize,
    direction: TextDirection,
    script: Option<TextScript>,
    language: Option<Cow<'a, str>>,
  ) -> Result<ShapedRun<'a>> {
    let text = text.into();
    let face = BuzzFace::from_slice(data, self.face_index).ok_or(FontError::InvalidFace)?;
    let units_per_em = face.units_per_em() as f32;
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text.as_ref());
    if let Some(direction) = buzz_direction(direction) {
      buffer.set_direction(direction);
    }
    if let Some(script) = script.and_then(buzz_script) {
      buffer.set_script(script);
    }
    if let Some(language) = language
      .as_deref()
      .and_then(|language| BuzzLanguage::from_str(language).ok())
    {
      buffer.set_language(language);
    }
    let output = rustybuzz::shape(&face, &[], buffer);
    let infos = output.glyph_infos();
    let positions = output.glyph_positions();
    let safe_breaks = text_safe_breaks(text.as_ref());
    let glyphs = infos
      .iter()
      .zip(positions.iter())
      .enumerate()
      .map(|(index, (info, position))| {
        let start = info.cluster as usize;
        let end = infos
          .get(index + 1)
          .map(|next| next.cluster as usize)
          .filter(|next| *next > start)
          .unwrap_or_else(|| next_char_boundary(text.as_ref(), start));
        ShapedGlyph {
          glyph_id: info.glyph_id,
          cluster: info.cluster,
          text_range: start..end,
          x_advance_pt: position.x_advance as f32 / units_per_em * size.0,
          y_advance_pt: position.y_advance as f32 / units_per_em * size.0,
          x_offset_pt: position.x_offset as f32 / units_per_em * size.0,
          y_offset_pt: position.y_offset as f32 / units_per_em * size.0,
          safe_to_break: text
            .get(start..end)
            .is_some_and(|cluster| cluster.chars().all(char::is_whitespace)),
          source_char: text
            .get(start..end)
            .and_then(|cluster| cluster.chars().next()),
          justifiable: text
            .get(start..end)
            .is_some_and(|cluster| cluster.chars().any(char::is_whitespace)),
        }
      })
      .collect::<Vec<_>>();
    let advance_pt = glyphs.iter().map(|glyph| glyph.x_advance_pt).sum();

    Ok(ShapedRun {
      font_id: self.font_id.clone(),
      text_range: 0..text.len(),
      text,
      glyphs: Cow::Owned(glyphs),
      advance_pt,
      direction,
      script,
      language,
      safe_breaks,
      approximate: false,
      decorations: Vec::new(),
    })
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontMatchDiagnostics<'a> {
  pub candidates: Vec<FontMatchCandidate<'a>>,
  pub fallback_level: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontMatchCandidate<'a> {
  pub font_id: FontId,
  pub family: Cow<'a, str>,
  pub score: i32,
  pub rejected: bool,
  pub reason: Option<FontMatchReason>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FontMatchRank {
  rejected: bool,
  slant_mismatch: bool,
  stretch_distance: i32,
  weight_distance: i32,
  pitch_mismatch: bool,
}

impl FontMatchRank {
  fn distance(self) -> i32 {
    i32::from(self.rejected)
      + i32::from(self.slant_mismatch)
      + i32::from(self.stretch_distance != 0)
      + i32::from(self.weight_distance != 0)
      + i32::from(self.pitch_mismatch)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontMatchReason {
  Family,
  StyleName,
  Pitch,
  Weight,
  Slant,
  Stretch,
  Coverage,
  SourcePriority,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariationValue<'a> {
  pub tag: Cow<'a, str>,
  pub value: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureValue<'a> {
  pub tag: Cow<'a, str>,
  pub value: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontSubstitution<'a> {
  pub requested_family: Cow<'a, str>,
  pub substituted_family: Cow<'a, str>,
  pub reason: FontSubstitutionReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontSubstitutionReason {
  ThemeResolved,
  Alias,
  MissingFamily,
  MissingStyle,
  MissingGlyph,
  LastResort,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontMetrics {
  pub vertical: VerticalMetrics,
  pub decoration: DecorationMetrics,
  pub script: ScriptMetrics,
  pub em_size: f32,
}

impl Default for FontMetrics {
  fn default() -> Self {
    Self {
      vertical: VerticalMetrics::default(),
      decoration: DecorationMetrics::default(),
      script: ScriptMetrics::default(),
      em_size: 1.0,
    }
  }
}

impl FontMetrics {
  pub fn scaled(&self, size_pt: f32) -> Self {
    let scale = if self.em_size > 0.0 {
      size_pt / self.em_size
    } else {
      size_pt
    };
    Self {
      vertical: self.vertical.scaled(scale),
      decoration: self.decoration.scaled(scale),
      script: self.script.scaled(scale),
      em_size: size_pt,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VerticalMetrics {
  pub ascent_pt: f32,
  pub descent_pt: f32,
  pub internal_leading_pt: f32,
  pub external_leading_pt: f32,
  pub line_gap_pt: f32,
  pub ink_height_pt: f32,
  pub baseline_offset_pt: f32,
  pub hanging_baseline_pt: f32,
  pub cjk_horizontal_advance_pt: f32,
  pub cjk_vertical_advance_pt: f32,
}

impl VerticalMetrics {
  fn scaled(self, scale: f32) -> Self {
    Self {
      ascent_pt: self.ascent_pt * scale,
      descent_pt: self.descent_pt * scale,
      internal_leading_pt: self.internal_leading_pt * scale,
      external_leading_pt: self.external_leading_pt * scale,
      line_gap_pt: self.line_gap_pt * scale,
      ink_height_pt: self.ink_height_pt * scale,
      baseline_offset_pt: self.baseline_offset_pt * scale,
      hanging_baseline_pt: self.hanging_baseline_pt * scale,
      cjk_horizontal_advance_pt: self.cjk_horizontal_advance_pt * scale,
      cjk_vertical_advance_pt: self.cjk_vertical_advance_pt * scale,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DecorationMetrics {
  pub underline_offset_pt: f32,
  pub underline_thickness_pt: f32,
  pub strikeout_offset_pt: f32,
  pub strikeout_thickness_pt: f32,
}

impl DecorationMetrics {
  fn scaled(self, scale: f32) -> Self {
    Self {
      underline_offset_pt: self.underline_offset_pt * scale,
      underline_thickness_pt: self.underline_thickness_pt * scale,
      strikeout_offset_pt: self.strikeout_offset_pt * scale,
      strikeout_thickness_pt: self.strikeout_thickness_pt * scale,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScriptMetrics {
  pub superscript_scale: f32,
  pub subscript_scale: f32,
  pub superscript_offset_pt: f32,
  pub subscript_offset_pt: f32,
  pub small_caps_scale: f32,
}

impl ScriptMetrics {
  fn scaled(self, scale: f32) -> Self {
    Self {
      superscript_scale: self.superscript_scale,
      subscript_scale: self.subscript_scale,
      superscript_offset_pt: self.superscript_offset_pt * scale,
      subscript_offset_pt: self.subscript_offset_pt * scale,
      small_caps_scale: self.small_caps_scale,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapedRun<'a> {
  pub font_id: FontId,
  pub text: Cow<'a, str>,
  pub text_range: Range<usize>,
  pub glyphs: Cow<'a, [ShapedGlyph]>,
  pub advance_pt: f32,
  pub direction: TextDirection,
  pub script: Option<TextScript>,
  pub language: Option<Cow<'a, str>>,
  pub safe_breaks: Vec<usize>,
  pub approximate: bool,
  pub decorations: Vec<TextDecoration>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShapedGlyph {
  pub glyph_id: u32,
  pub cluster: u32,
  pub text_range: Range<usize>,
  pub x_advance_pt: f32,
  pub y_advance_pt: f32,
  pub x_offset_pt: f32,
  pub y_offset_pt: f32,
  pub safe_to_break: bool,
  pub source_char: Option<char>,
  pub justifiable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDecoration {
  Underline,
  DoubleUnderline,
  Strikeout,
  Overline,
  WaveUnderline,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FontUsage {
  pub font_id: FontId,
  pub glyph_ids: BTreeSet<u32>,
  pub unicode_ranges: Vec<Range<u32>>,
  pub needs_embedding: bool,
  pub subset_policy: FontSubsetPolicy,
  pub color_glyph_usage: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontSubsetPolicy {
  #[default]
  Subset,
  EmbedFull,
  DoNotEmbed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FontSize(pub f32);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ThemeFontKind {
  MajorLatin,
  #[default]
  MinorLatin,
  MajorEastAsian,
  MinorEastAsian,
  MajorComplexScript,
  MinorComplexScript,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontWeight {
  Thin,
  ExtraLight,
  Light,
  #[default]
  Normal,
  Medium,
  SemiBold,
  Bold,
  ExtraBold,
  Black,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontSlant {
  #[default]
  Upright,
  Italic,
  Oblique,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontStretch {
  UltraCondensed,
  ExtraCondensed,
  Condensed,
  SemiCondensed,
  #[default]
  Normal,
  SemiExpanded,
  Expanded,
  ExtraExpanded,
  UltraExpanded,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontPitch {
  Fixed,
  #[default]
  Variable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontCharset {
  Ansi,
  Symbol,
  ShiftJis,
  Hangul,
  Gb2312,
  ChineseBig5,
  Greek,
  Turkish,
  Vietnamese,
  Hebrew,
  Arabic,
  Baltic,
  Russian,
  Thai,
  EastEurope,
  Oem,
  Other(u8),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextDirection {
  #[default]
  LeftToRight,
  RightToLeft,
  TopToBottom,
  BottomToTop,
  Mixed,
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum TextScript {
  #[default]
  Common,
  Latin,
  Cyrillic,
  Greek,
  Han,
  Hiragana,
  Katakana,
  Hangul,
  Arabic,
  Hebrew,
  Devanagari,
  Thai,
  Other,
}

fn push_unique_string<'a>(values: &mut Vec<Cow<'a, str>>, value: String) {
  if !values.iter().any(|existing| existing.as_ref() == value) {
    values.push(Cow::Owned(value));
  }
}

fn primary_family<'a>(face: &FontFaceInfo<'a>) -> Cow<'a, str> {
  face
    .family_names
    .first()
    .cloned()
    .unwrap_or_else(|| Cow::Owned(face.font_id.0.to_string()))
}

fn normalize_family(value: &str) -> String {
  value
    .chars()
    .filter(|ch| !ch.is_ascii_whitespace() && *ch != '-' && *ch != '_')
    .flat_map(char::to_lowercase)
    .collect()
}

fn family_matches(face: &FontFaceInfo<'_>, family: &str) -> bool {
  let target = normalize_family(family);
  face
    .family_names
    .iter()
    .any(|candidate| normalize_family(candidate) == target)
}

fn family_overlaps(registered: &RegisteredFontFace<'_>, face: &FontFaceInfo<'_>) -> bool {
  registered
    .family_names
    .iter()
    .any(|registered| family_matches(face, registered))
}

fn resolve_family_alias<'a>(book: &FontBook<'a>, family: &Cow<'a, str>) -> Cow<'a, str> {
  book
    .family_aliases
    .iter()
    .find(|alias| normalize_family(&alias.from) == normalize_family(family))
    .map(|alias| alias.to.clone())
    .unwrap_or_else(|| family.clone())
}

fn substitute_family<'a>(book: &FontBook<'a>, family: &Cow<'a, str>) -> Cow<'a, str> {
  book
    .substitutions
    .iter()
    .find(|rule| normalize_family(&rule.requested_family) == normalize_family(family))
    .map(|rule| rule.substitute_family.clone())
    .unwrap_or_else(|| family.clone())
}

fn requested_weight(request: &FontRequest<'_>) -> FontWeight {
  request.weight.unwrap_or(if request.bold {
    FontWeight::Bold
  } else {
    FontWeight::Normal
  })
}

fn requested_slant(request: &FontRequest<'_>) -> FontSlant {
  request.slant.unwrap_or(if request.italic {
    FontSlant::Italic
  } else {
    FontSlant::Upright
  })
}

fn weight_distance(left: FontWeight, right: FontWeight) -> i32 {
  (font_weight_number(left) - font_weight_number(right)).abs()
}

fn stretch_distance(left: FontStretch, right: FontStretch) -> i32 {
  (font_stretch_number(left) - font_stretch_number(right)).abs()
}

fn font_metrics_from_ttf(face: &TtfFace<'_>, em_size: f32) -> FontMetrics {
  let units_per_em = f32::from(face.units_per_em());
  let to_em = |value: i32| value as f32 / units_per_em * em_size;
  let ascent_units = face.ascender().max(0);
  let descent_units = (-face.descender()).max(0);
  let line_gap_units = face.line_gap().max(0);
  let fallback_gap_units =
    (i32::from(face.units_per_em()) - i32::from(ascent_units) - i32::from(descent_units)).max(0);
  let ascender = to_em(i32::from(ascent_units));
  let descender = to_em(i32::from(descent_units));
  let line_gap = if line_gap_units > 0 {
    to_em(i32::from(line_gap_units))
  } else {
    to_em(fallback_gap_units)
  };
  let underline = face.underline_metrics();
  let strikeout = face.strikeout_metrics();
  FontMetrics {
    vertical: VerticalMetrics {
      ascent_pt: ascender,
      descent_pt: descender,
      line_gap_pt: line_gap,
      ink_height_pt: ascender + descender,
      ..VerticalMetrics::default()
    },
    decoration: DecorationMetrics {
      underline_offset_pt: underline
        .map(|metrics| -to_em(i32::from(metrics.position)))
        .unwrap_or_default(),
      underline_thickness_pt: underline
        .map(|metrics| to_em(i32::from(metrics.thickness)).abs())
        .unwrap_or_default(),
      strikeout_offset_pt: strikeout
        .map(|metrics| to_em(i32::from(metrics.position)))
        .unwrap_or_default(),
      strikeout_thickness_pt: strikeout
        .map(|metrics| to_em(i32::from(metrics.thickness)).abs())
        .unwrap_or_default(),
    },
    script: ScriptMetrics {
      superscript_scale: 1.0,
      subscript_scale: 1.0,
      small_caps_scale: 1.0,
      ..ScriptMetrics::default()
    },
    em_size,
  }
}

fn font_coverage_from_ttf(face: &TtfFace<'_>) -> FontCoverage {
  let mut ranges = Vec::new();
  let mut range_start = None;
  let mut last = 0u32;
  for codepoint in 0..=0xFFFF {
    let covered = char::from_u32(codepoint)
      .and_then(|ch| face.glyph_index(ch))
      .is_some();
    match (range_start, covered) {
      (None, true) => {
        range_start = Some(codepoint);
        last = codepoint;
      }
      (Some(_), true) => {
        last = codepoint;
      }
      (Some(start), false) => {
        ranges.push(start..last + 1);
        range_start = None;
      }
      (None, false) => {}
    }
  }
  if let Some(start) = range_start {
    ranges.push(start..last + 1);
  }
  FontCoverage {
    unicode_ranges: ranges,
    scripts: BTreeSet::new(),
  }
}

fn font_weight_number(weight: FontWeight) -> i32 {
  match weight {
    FontWeight::Thin => 100,
    FontWeight::ExtraLight => 200,
    FontWeight::Light => 300,
    FontWeight::Normal => 400,
    FontWeight::Medium => 500,
    FontWeight::SemiBold => 600,
    FontWeight::Bold => 700,
    FontWeight::ExtraBold => 800,
    FontWeight::Black => 900,
  }
}

fn approximate_glyphs(text: &str, _size: FontSize) -> Vec<ShapedGlyph> {
  text
    .char_indices()
    .map(|(start, ch)| {
      let end = start + ch.len_utf8();
      ShapedGlyph {
        glyph_id: 0,
        cluster: start as u32,
        text_range: start..end,
        x_advance_pt: 0.0,
        y_advance_pt: 0.0,
        x_offset_pt: 0.0,
        y_offset_pt: 0.0,
        safe_to_break: ch.is_whitespace(),
        source_char: Some(ch),
        justifiable: ch.is_whitespace(),
      }
    })
    .collect()
}

fn text_safe_breaks(text: &str) -> Vec<usize> {
  text
    .char_indices()
    .filter_map(|(index, ch)| ch.is_whitespace().then_some(index + ch.len_utf8()))
    .collect()
}

fn next_char_boundary(text: &str, start: usize) -> usize {
  text
    .get(start..)
    .and_then(|tail| tail.char_indices().nth(1).map(|(offset, _)| start + offset))
    .unwrap_or(text.len())
}

fn font_stretch_number(stretch: FontStretch) -> i32 {
  match stretch {
    FontStretch::UltraCondensed => 1,
    FontStretch::ExtraCondensed => 2,
    FontStretch::Condensed => 3,
    FontStretch::SemiCondensed => 4,
    FontStretch::Normal => 5,
    FontStretch::SemiExpanded => 6,
    FontStretch::Expanded => 7,
    FontStretch::ExtraExpanded => 8,
    FontStretch::UltraExpanded => 9,
  }
}

fn font_weight_from_ttf(weight: u16) -> FontWeight {
  match weight {
    0..=149 => FontWeight::Thin,
    150..=249 => FontWeight::ExtraLight,
    250..=349 => FontWeight::Light,
    350..=449 => FontWeight::Normal,
    450..=549 => FontWeight::Medium,
    550..=649 => FontWeight::SemiBold,
    650..=749 => FontWeight::Bold,
    750..=849 => FontWeight::ExtraBold,
    _ => FontWeight::Black,
  }
}

fn font_slant_from_ttf(style: ttf_parser::Style) -> FontSlant {
  match style {
    ttf_parser::Style::Italic => FontSlant::Italic,
    ttf_parser::Style::Oblique => FontSlant::Oblique,
    ttf_parser::Style::Normal => FontSlant::Upright,
  }
}

fn font_stretch_from_ttf(width: u16) -> FontStretch {
  match width {
    1 => FontStretch::UltraCondensed,
    2 => FontStretch::ExtraCondensed,
    3 => FontStretch::Condensed,
    4 => FontStretch::SemiCondensed,
    5 => FontStretch::Normal,
    6 => FontStretch::SemiExpanded,
    7 => FontStretch::Expanded,
    8 => FontStretch::ExtraExpanded,
    _ => FontStretch::UltraExpanded,
  }
}

fn buzz_direction(direction: TextDirection) -> Option<BuzzDirection> {
  match direction {
    TextDirection::LeftToRight => Some(BuzzDirection::LeftToRight),
    TextDirection::RightToLeft => Some(BuzzDirection::RightToLeft),
    TextDirection::TopToBottom => Some(BuzzDirection::TopToBottom),
    TextDirection::BottomToTop => Some(BuzzDirection::BottomToTop),
    TextDirection::Mixed => None,
  }
}

fn buzz_script(script: TextScript) -> Option<BuzzScript> {
  match script {
    TextScript::Common => Some(script::COMMON),
    TextScript::Latin => Some(script::LATIN),
    TextScript::Cyrillic => Some(script::CYRILLIC),
    TextScript::Greek => Some(script::GREEK),
    TextScript::Han => Some(script::HAN),
    TextScript::Hiragana => Some(script::HIRAGANA),
    TextScript::Katakana => Some(script::KATAKANA),
    TextScript::Hangul => Some(script::HANGUL),
    TextScript::Arabic => Some(script::ARABIC),
    TextScript::Hebrew => Some(script::HEBREW),
    TextScript::Devanagari => Some(script::DEVANAGARI),
    TextScript::Thai => Some(script::THAI),
    TextScript::Other => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn resolves_exact_family_and_records_candidates() {
    let mut registry = FontRegistry::new();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("regular", "Example"),
    );
    let mut bold = FontFaceInfo::synthetic("bold", "Example");
    bold.weight = FontWeight::Bold;
    registry.register_face(FontSource::System, bold);

    let resolved = registry
      .resolve(&FontRequest {
        family: Some(Cow::Borrowed("Example")),
        bold: true,
        ..FontRequest::default()
      })
      .unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("bold")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("Example"));
    assert!(!resolved.synthetic_bold);
    assert_eq!(resolved.match_diagnostics.candidates.len(), 2);
  }

  #[test]
  fn applies_alias_before_matching() {
    let mut registry = FontRegistry::new();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("liberation", "Liberation Serif"),
    );
    registry.book.family_aliases.push(FontFamilyAlias {
      from: Cow::Borrowed("Times New Roman"),
      to: Cow::Borrowed("Liberation Serif"),
    });

    let resolved = registry
      .resolve(&FontRequest {
        family: Some(Cow::Borrowed("Times New Roman")),
        ..FontRequest::default()
      })
      .unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("liberation")));
    assert_eq!(
      resolved.substitution.unwrap().reason,
      FontSubstitutionReason::Alias
    );
  }

  #[test]
  fn resolved_font_scales_face_metrics() {
    let mut registry = FontRegistry::new();
    let mut face = FontFaceInfo::synthetic("example", "Example");
    face.metrics = FontMetrics {
      vertical: VerticalMetrics {
        ascent_pt: 1.0,
        descent_pt: 0.25,
        ..VerticalMetrics::default()
      },
      em_size: 1.0,
      ..FontMetrics::default()
    };
    registry.register_face(FontSource::System, face);

    let resolved = registry
      .resolve(&FontRequest {
        family: Some(Cow::Borrowed("Example")),
        ..FontRequest::default()
      })
      .unwrap();
    let metrics = resolved.metrics_at_size(FontSize(12.0));

    assert_eq!(metrics.vertical.ascent_pt, 12.0);
    assert_eq!(metrics.vertical.descent_pt, 3.0);
  }

  #[test]
  fn approximate_shaping_preserves_text_ranges_without_fake_advances() {
    let resolved = ResolvedFont {
      font_id: FontId(Arc::from("example")),
      requested_family: Some(Cow::Borrowed("Example")),
      resolved_family: Cow::Borrowed("Example"),
      source: FontSource::System,
      face_index: 0,
      synthetic_bold: false,
      synthetic_italic: false,
      variation_values: Vec::new(),
      metrics: FontMetrics::default(),
      substitution: None,
      match_diagnostics: FontMatchDiagnostics::default(),
    };

    let shaped = resolved.shape_approximate(
      Cow::Borrowed("A B"),
      FontSize(12.0),
      TextDirection::LeftToRight,
      Some(TextScript::Latin),
      None,
    );

    assert!(shaped.approximate);
    assert_eq!(shaped.glyphs.len(), 3);
    assert_eq!(shaped.glyphs[0].text_range, 0..1);
    assert_eq!(shaped.glyphs[0].x_advance_pt, 0.0);
    assert_eq!(shaped.safe_breaks, vec![2]);
  }

  #[test]
  fn maps_ooxml_text_context_to_rustybuzz_context() {
    assert_eq!(
      buzz_direction(TextDirection::RightToLeft),
      Some(BuzzDirection::RightToLeft)
    );
    assert_eq!(buzz_script(TextScript::Arabic), Some(script::ARABIC));
    assert_eq!(buzz_script(TextScript::Other), None);
  }
}
