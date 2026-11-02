//! Resolve Word's script size and device baseline after the font/style cascade.
use std::cell::RefCell;
use std::rc::Rc;

use ooxmlsdk_fonts::FontCharset;
use rustc_hash::FxHashMap;
use skrifa::raw::TableProvider;

use crate::fonts::{
  FontFaceCacheKey, FontFaceData, FontResolver, materialize_wordprocessingml_source_font_slot,
  script_scan_options,
};
use crate::model::TextStyle;

use super::{
  InlineItem, ParagraphFieldEvent, TextRun, WORD_DEFAULT_ESCAPEMENT_HEIGHT_PERCENT, properties,
};

const RETAINED_DEFAULT_CHARSET: u8 = 4;

// Import is sequential; only this document's style-catalog clones share the
// cache. Ordinary styles return before borrowing or loading fonts. The resolver
// caches physical faces and their already parsed OS/2 metrics, including
// document-embedded fonts. Keep the mutable cache local to the import, as in
// Parley's FontContext, rather than adding synchronization for a single owner.
#[derive(Clone, Debug, Default)]
pub(super) struct Fonts(Rc<RefCell<Cache>>);

#[derive(Debug, Default)]
struct Cache {
  fonts: FontResolver,
  faces: FxHashMap<(FontFaceCacheKey, u8, bool, bool), Option<DeviceFace>>,
}

impl Fonts {
  pub(super) fn is_variable_face(&self, style: &TextStyle) -> bool {
    // Reuse the document-owned import resolver and its face cache. No global
    // scan or new synchronization is needed for revision-owned bitmap runs.
    self
      .0
      .borrow_mut()
      .fonts
      .with_cached_text_face(style, |face| {
        skrifa::FontRef::from_index(face.data.as_slice(), face.index)
          .is_ok_and(|font| font.fvar().is_ok())
      })
      .unwrap_or(false)
  }

  pub(super) fn has_exact_ascii_face(&self, style: &TextStyle) -> bool {
    // Generated field resources also query the document's Normal font. Reuse
    // the import-owned resolver rather than loading a fresh font registry or
    // introducing another shared cache for those uncommon diagnostics.
    self.0.borrow_mut().fonts.has_exact_ascii_face(style)
  }

  pub(super) fn has_exact_east_asia_face_for_char(
    &self,
    style: &TextStyle,
    character: char,
  ) -> bool {
    self
      .0
      .borrow_mut()
      .fonts
      .has_exact_east_asia_face_for_char(style, character)
  }

  pub(super) fn resolve(&self, style: &mut TextStyle) {
    let Some(original_size) = style
      .automatic_escapement_font_size_pt
      .filter(|_| style.wordprocessingml_font_slots)
    else {
      return;
    };
    let mut cache = self.0.borrow_mut();
    let mut selection = style.clone();
    selection.complex_script = Some(false);
    selection.right_to_left = Some(false);
    let legacy = style.wordprocessingml_legacy_escapement_rounding;
    style.font_size_pt = cache.glyph_size(
      &selection,
      original_size,
      style.automatic_escapement_is_subscript,
      legacy,
    );
    style.complex_font_size_pt = style.automatic_escapement_complex_font_size_pt.map(|size| {
      selection.complex_script = Some(true);
      selection.right_to_left = Some(true);
      cache.glyph_size(
        &selection,
        size,
        style.automatic_escapement_is_subscript,
        legacy,
      )
    });
    let complex = style.complex_script == Some(true) || style.right_to_left == Some(true);
    selection.complex_script = Some(complex);
    selection.right_to_left = Some(complex);
    let original_size = if complex {
      style
        .automatic_escapement_complex_font_size_pt
        .unwrap_or(original_size)
    } else {
      original_size
    };
    let reduced_size = if complex {
      style.complex_font_size_pt.unwrap_or(style.font_size_pt)
    } else {
      style.font_size_pt
    };
    if let Some(shift) = cache.baseline(
      &selection,
      original_size,
      reduced_size,
      style.automatic_escapement_is_subscript,
    ) {
      style.baseline_shift_pt = shift + style.wordprocessingml_position_pt.unwrap_or(0.0);
    }
  }

  pub(super) fn resolve_inlines(
    &self,
    inlines: &mut Vec<InlineItem>,
    field_events: &mut [ParagraphFieldEvent],
  ) {
    if !inlines.iter().any(|item| {
      matches!(item, InlineItem::Text(run)
      if run.style.automatic_escapement_font_size_pt.is_some()
        && run.style.wordprocessingml_font_slots && !run.text.is_ascii())
    }) {
      return;
    }
    let mut output = Vec::with_capacity(inlines.len());
    let mut offsets = Vec::with_capacity(inlines.len() + 1);
    for item in std::mem::take(inlines) {
      offsets.push(output.len());
      if let InlineItem::Text(run) = &item
        && let Some(portions) = self.resolve_east_asian_run(run)
      {
        output.extend(portions.into_iter().map(InlineItem::Text));
      } else {
        output.push(item);
      }
    }
    offsets.push(output.len());
    for event in field_events {
      match event {
        ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
        | ParagraphFieldEvent::DeferredReferenceParagraphBreak { inline_offset, .. }
        | ParagraphFieldEvent::GeneratedParagraphBreak { inline_offset, .. }
        | ParagraphFieldEvent::PageReferenceResultStyle { inline_offset, .. } => {
          *inline_offset = offsets[*inline_offset];
        }
        ParagraphFieldEvent::ReferenceResultSpan {
          inline_start,
          inline_end,
          ..
        } => {
          *inline_start = offsets[*inline_start];
          *inline_end = offsets[*inline_end];
        }
        _ => {}
      }
    }
    *inlines = output;
  }

  fn resolve_east_asian_run(&self, run: &TextRun) -> Option<Vec<TextRun>> {
    use ooxmlsdk_fonts::{
      FontSize, TextScript, WordprocessingFontSlot, script_direction_runs_with_options,
    };
    let original = run.style.automatic_escapement_font_size_pt?;
    if !run.style.wordprocessingml_font_slots || run.style.hidden || run.text.is_ascii() {
      return None;
    }
    let source_runs = script_direction_runs_with_options(
      &run.text,
      FontSize(run.style.font_size_pt),
      script_scan_options(&run.style, false),
    );
    // Word's documented Times New Roman exception selects ASCII even for
    // Han. The linked Chinese face still owns its physical script size.
    let uses_east_asian_face = |part: &ooxmlsdk_fonts::FontScriptRun| {
      part.wordprocessingml_font_slot == Some(WordprocessingFontSlot::EastAsia)
        || (part.wordprocessingml_font_slot == Some(WordprocessingFontSlot::Ascii)
          && part.script == TextScript::Han)
    };
    if !source_runs.iter().any(uses_east_asian_face) {
      return None;
    }
    let mut output = Vec::with_capacity(source_runs.len());
    let mut changed = false;
    for source_run in source_runs {
      let east_asian = uses_east_asian_face(&source_run);
      let text = &run.text[source_run.text_range];
      if !east_asian {
        let mut portion = run.clone();
        portion.text = text.to_owned();
        output.push(portion);
        continue;
      }
      // The selected rFonts slot owns baseline realization. A linked face
      // can own a different script size: Native TNR->SimSun keeps TNR's
      // baseline, whereas an explicit EA SimSun uses SimSun's baseline.
      let mut logical_style =
        materialize_wordprocessingml_source_font_slot(&run.style, text.chars().next()?);
      self.resolve(&mut logical_style);
      let mut cache = self.0.borrow_mut();
      let physical_runs = cache.fonts.shape_text_runs(text, &logical_style)?;
      for physical in physical_runs {
        let face = cache.fonts.font_face_data(&physical.font_id)?;
        let size = cache.chinese_glyph_size(
          &face,
          original,
          run.style.automatic_escapement_is_subscript,
          run.style.wordprocessingml_legacy_escapement_rounding,
        );
        let mut portion = run.clone();
        portion.text = text[physical.text_range].to_owned();
        if let Some(size) = size {
          portion.style = logical_style.clone();
          portion.style.font_size_pt = size;
          changed |= portion.style != run.style;
        }
        output.push(portion);
      }
    }
    changed.then_some(output)
  }
}

impl Cache {
  fn script_face(&mut self, face: &FontFaceData) -> Option<&mut DeviceFace> {
    self
      .faces
      .entry((
        face.cache_key(),
        0,
        face.synthetic_bold,
        face.synthetic_italic,
      ))
      .or_insert_with(|| DeviceFace::new(face, 0))
      .as_mut()
  }

  fn glyph_size(&mut self, style: &TextStyle, original: f32, subscript: bool, legacy: bool) -> f32 {
    let Self { fonts, faces } = self;
    if let Some(size) = fonts
      .with_cached_text_face(style, |face| {
        faces
          .entry((
            face.cache_key(),
            0,
            face.synthetic_bold,
            face.synthetic_italic,
          ))
          .or_insert_with(|| DeviceFace::new(face, 0))
          .as_ref()?
          .glyph_size(original, subscript, legacy)
      })
      .flatten()
    {
      return size;
    }
    properties::automatic_escapement_glyph_size(
      original,
      height_percent(fonts.script_metrics(style), subscript),
      legacy,
    )
  }

  fn chinese_glyph_size(
    &mut self,
    data: &FontFaceData,
    original: f32,
    subscript: bool,
    legacy: bool,
  ) -> Option<f32> {
    let face = self.script_face(data)?;
    if !face.chinese_charset {
      // Japanese and other charset policies have separate Native counters;
      // retain their established realization rather than generalizing the
      // Chinese font rule from the shared CJK capability flag.
      return None;
    }
    let percent = face.height_percent(subscript);
    if percent != 50 {
      return None;
    }
    face.glyph_size(original, subscript, legacy)
  }

  fn baseline(
    &mut self,
    style: &TextStyle,
    original: f32,
    reduced: f32,
    subscript: bool,
  ) -> Option<f32> {
    // VDMX distinguishes ANSI from the other realized charsets. The default
    // charset is resolved to the physical face by the existing font query.
    let charset = match if style.complex_script == Some(true) {
      style.complex_font_charset.or(style.font_charset)
    } else {
      style.font_charset
    } {
      Some(FontCharset::Other(1))
        if self
          .fonts
          .wordprocessingml_default_charset_line_metrics(style, "A")
          .is_some() =>
      {
        RETAINED_DEFAULT_CHARSET
      }
      None | Some(FontCharset::Ansi | FontCharset::Other(1)) => 0,
      Some(FontCharset::Symbol) => 2,
      Some(
        FontCharset::ShiftJis
        | FontCharset::Hangul
        | FontCharset::Gb2312
        | FontCharset::ChineseBig5,
      ) => 3,
      Some(_) => 1,
    };
    let Self { fonts, faces } = self;
    fonts
      .with_cached_text_face(style, |face| {
        let key = (
          face.cache_key(),
          charset,
          face.synthetic_bold,
          face.synthetic_italic,
        );
        faces
          .entry(key)
          .or_insert_with(|| DeviceFace::new(face, charset))
          .as_mut()?
          .baseline(original, reduced, subscript)
      })
      .flatten()
  }
}

#[derive(Clone, Copy, Debug)]
struct DeviceMetrics {
  ascent: i64,
  descent: i64,
  leading: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScriptParameters {
  super_size: i64,
  sub_size: i64,
  super_offset: i64,
  sub_offset: i64,
}

impl ScriptParameters {
  fn valid(self, original: i64) -> bool {
    // Word validates all four recommendations together, after conversion to
    // half points. Native font-cache captures and independent embedded-font
    // controls establish these integer bounds; OS/2 values are recommendations,
    // not unconditional placement instructions.
    let minimum_size = original / 3 + 1;
    let half = original / 2;
    let quarter = half / 2;
    (minimum_size..=original).contains(&self.super_size)
      && (minimum_size..=original).contains(&self.sub_size)
      && (0..=half).contains(&self.sub_offset)
      && (quarter..=quarter + half).contains(&self.super_offset)
  }

  fn fallback(original: i64) -> Self {
    Self {
      super_size: ((original * 6 + 5) / 10).max(2),
      sub_size: ((original * 6 + 5) / 10).max(2),
      super_offset: ((original * 4 + 5) / 10).max(1),
      sub_offset: ((original * 2 + 5) / 10).max(1),
    }
  }
}

#[derive(Debug)]
struct DeviceFace {
  units_per_em: i64,
  ascent: i64,
  descent: i64,
  leading: i64,
  cjk_line_metrics: bool,
  script: ScriptParameters,
  chinese_charset: bool,
  charset: u8,
  vdmx: Option<Vec<u8>>,
  metrics: FxHashMap<u16, DeviceMetrics>,
  baselines: FxHashMap<(u32, u32, bool), f32>,
}

impl DeviceFace {
  fn new(data: &FontFaceData, charset: u8) -> Option<Self> {
    let face = skrifa::FontRef::from_index(data.data.as_slice(), data.index).ok()?;
    // Variable-font instancing and synthetic bold retain their existing
    // fallback until their device metric policies are independently resolved.
    if face.fvar().is_ok() || data.synthetic_bold {
      return None;
    }
    let units_per_em = i64::from(face.head().ok()?.units_per_em());
    if units_per_em == 0 {
      return None;
    }
    let os2 = face.os2().ok()?;
    let hhea = face.hhea().ok()?;
    let ascent = i64::from(os2.us_win_ascent());
    let descent = i64::from(os2.us_win_descent());
    let leading = (i64::from(hhea.ascender().to_i16()) - i64::from(hhea.descender().to_i16())
      + i64::from(hhea.line_gap().to_i16())
      - ascent
      - descent)
      .max(0);
    Some(Self {
      units_per_em,
      ascent,
      descent,
      leading,
      // Installed faces supply their CJK classification. For private embedded
      // faces Word additionally requires a CJK font-table charset; native
      // identical-font controls distinguish absent/ANSI from Shift-JIS.
      // OS/2 code-page bits17..20 match the ordinary Word line metric policy.
      // A retained DEFAULT_CHARSET record uses this same side-leading cell.
      // Native Liberation Serif 12/36pt captures distinguish it from ANSI;
      // reuse the ordinary line-metric eligibility policy above so normalized
      // Arial/Times and mismatched font-table metadata remain on their path.
      cjk_line_metrics: charset == RETAINED_DEFAULT_CHARSET
        || ((!data.document_embedded || charset == 3)
          && os2
            .ul_code_page_range_1()
            .is_some_and(|range| range & 0x001e_0000 != 0)),
      script: ScriptParameters {
        super_size: i64::from(os2.y_superscript_y_size()),
        sub_size: i64::from(os2.y_subscript_y_size()),
        super_offset: i64::from(os2.y_superscript_y_offset()),
        sub_offset: i64::from(os2.y_subscript_y_offset()),
      },
      chinese_charset: os2
        .ul_code_page_range_1()
        .is_some_and(|range| range & (1 << 18) != 0),
      charset: match charset {
        3 => 1,
        RETAINED_DEFAULT_CHARSET => 0,
        _ => charset,
      },
      vdmx: face
        .table_data(skrifa::Tag::new(b"VDMX"))
        .map(|table| table.as_bytes().to_vec()),
      metrics: FxHashMap::default(),
      baselines: FxHashMap::default(),
    })
  }

  fn height_percent(&self, subscript: bool) -> u32 {
    let size = if subscript {
      self.script.sub_size
    } else {
      self.script.super_size
    };
    rounded_ratio(size * 100, self.units_per_em).max(0) as u32
  }

  fn script_parameters(&self, original: f32) -> Option<(ScriptParameters, bool)> {
    let original = (f64::from(original) * 2.0).round();
    if !(1.0..=f64::from(u16::MAX)).contains(&original) {
      return None;
    }
    let original = original as i64;
    let scale = |value| rounded_ratio(original * value, self.units_per_em);
    let parameters = ScriptParameters {
      super_size: scale(self.script.super_size),
      sub_size: scale(self.script.sub_size),
      super_offset: scale(self.script.super_offset),
      sub_offset: scale(self.script.sub_offset),
    };
    let recommended = parameters.valid(original);
    Some((
      if recommended {
        parameters
      } else {
        ScriptParameters::fallback(original)
      },
      recommended,
    ))
  }

  fn glyph_size(&self, original: f32, subscript: bool, legacy: bool) -> Option<f32> {
    let (parameters, recommended) = self.script_parameters(original)?;
    if !recommended {
      let half_points = if subscript {
        parameters.sub_size
      } else {
        parameters.super_size
      };
      return Some(half_points as f32 * 0.5);
    }
    let percent = self.height_percent(subscript);
    Some(properties::automatic_escapement_glyph_size(
      original,
      percent,
      legacy || (percent == 50 && self.chinese_charset),
    ))
  }

  fn metrics(&mut self, size: f32) -> Option<DeviceMetrics> {
    let ppem = (f64::from(size) * 600.0 / 72.0).round();
    if !ppem.is_finite() || ppem < 1.0 || ppem > f64::from(u16::MAX) {
      return None;
    }
    let ppem = ppem as u16;
    if let Some(metrics) = self.metrics.get(&ppem) {
      return Some(*metrics);
    }
    let hinted = self
      .vdmx
      .as_deref()
      .and_then(|table| emfsdk::font::vdmx_vertical_device_metrics(table, ppem, self.charset));
    let scaled = |value| rounded_ratio(value * i64::from(ppem), self.units_per_em);
    let (ascent, mut descent) = hinted.map_or_else(
      || (scaled(self.ascent), scaled(self.descent)),
      |metrics| (i64::from(metrics.ascent), i64::from(metrics.descent)),
    );
    let leading = if self.cjk_line_metrics {
      // This printer cache rounds side leading after realizing ascent and
      // descent, unlike the ordinary layout design-grid calculation. MS
      // Gothic6pt distinguishes them (8 printer units, not7). Keep physical
      // ascent; put one side below and one above, replacing hhea leading.
      let side = rounded_ratio((ascent + descent) * 15, 100);
      descent += side;
      side
    } else {
      scaled(self.leading)
    };
    let metrics = DeviceMetrics {
      ascent,
      descent,
      leading,
    };
    self.metrics.insert(ppem, metrics);
    Some(metrics)
  }

  fn baseline(&mut self, original: f32, reduced: f32, subscript: bool) -> Option<f32> {
    let key = (original.to_bits(), reduced.to_bits(), subscript);
    if let Some(shift) = self.baselines.get(&key) {
      return Some(*shift);
    }
    let original_metrics = self.metrics(original)?;
    let reduced_metrics = self.metrics(reduced)?;
    let to_grid = |value| rounded_ratio(value * 144, 600);
    let ascent = to_grid(original_metrics.ascent + original_metrics.leading);
    let descent = to_grid(original_metrics.descent);
    let (parameters, recommended) = self.script_parameters(original)?;
    let initial = if subscript {
      -parameters.sub_offset
    } else {
      parameters.super_offset
    };
    // Word's native font cache keeps the original external leading, rounds
    // each printer metric independently to 144 DPI, then limits the font's
    // recommendation. A superscript rises at least one third of the original
    // ascent within that limit; a subscript cannot lower the line's descent.
    let chinese_half_size =
      recommended && self.chinese_charset && self.height_percent(subscript) == 50;
    let shift = if chinese_half_size {
      // Native SimSun/SimHei/KaiTi controls use the face's recommendation
      // directly on the half-point grid, including 12pt subscript: -1pt,
      // where the Western descent clamp would incorrectly produce -0.5pt.
      initial
    } else if initial > 0 {
      initial.min((ascent / 3).max(ascent - to_grid(reduced_metrics.ascent)))
    } else if initial < 0 {
      initial.max(to_grid(reduced_metrics.descent) - descent)
    } else {
      0
    };
    let shift = shift as f32 * 0.5;
    self.baselines.insert(key, shift);
    Some(shift)
  }
}

fn rounded_ratio(numerator: i64, denominator: i64) -> i64 {
  (numerator + numerator.signum() * (denominator / 2)) / denominator
}

fn height_percent(metrics: Option<ooxmlsdk_fonts::ScriptMetrics>, subscript: bool) -> u32 {
  let scale = metrics.map(|metrics| {
    if subscript {
      metrics.subscript_scale
    } else {
      metrics.superscript_scale
    }
  });
  scale_height_percent(scale)
}

fn scale_height_percent(scale: Option<f32>) -> u32 {
  // The font layer uses 1.0 when script sizing is absent or unusable. Keep
  // Word's default in that case. Native six-family body/textbox controls
  // distinguish 65% from DejaVu/Miriam's 70%; the source tables supply the
  // recommendation rather than a list of special-cased family names.
  scale
    .filter(|scale| scale.is_finite() && *scale > 0.0 && *scale < 1.0)
    .map_or(WORD_DEFAULT_ESCAPEMENT_HEIGHT_PERCENT, |scale| {
      (scale * 100.0).round().max(1.0) as u32
    })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::w;

  #[test]
  fn script_recommendations_validate_coupled_half_point_bounds() {
    let parameters = ScriptParameters {
      super_size: 18,
      sub_size: 18,
      super_offset: 10,
      sub_offset: 4,
    };
    // Actual 12pt embedded-font controls straddle each native boundary. A
    // valid superscript cannot hide an invalid subscript recommendation.
    for size in [9, 24] {
      assert!(
        ScriptParameters {
          super_size: size,
          ..parameters
        }
        .valid(24)
      );
      assert!(
        ScriptParameters {
          sub_size: size,
          ..parameters
        }
        .valid(24)
      );
    }
    for size in [0, 8, 25] {
      assert!(
        !ScriptParameters {
          super_size: size,
          ..parameters
        }
        .valid(24)
      );
      assert!(
        !ScriptParameters {
          sub_size: size,
          ..parameters
        }
        .valid(24)
      );
    }
    for offset in [6, 18] {
      assert!(
        ScriptParameters {
          super_offset: offset,
          ..parameters
        }
        .valid(24)
      );
    }
    for offset in [0, 5, 19] {
      assert!(
        !ScriptParameters {
          super_offset: offset,
          ..parameters
        }
        .valid(24)
      );
    }
    for offset in [0, 12] {
      assert!(
        ScriptParameters {
          sub_offset: offset,
          ..parameters
        }
        .valid(24)
      );
    }
    for offset in [-1, 13] {
      assert!(
        !ScriptParameters {
          sub_offset: offset,
          ..parameters
        }
        .valid(24)
      );
    }
    assert_eq!(
      ScriptParameters::fallback(24),
      ScriptParameters {
        super_size: 14,
        sub_size: 14,
        super_offset: 10,
        sub_offset: 5,
      }
    );
    assert_eq!(
      ScriptParameters::fallback(1),
      ScriptParameters {
        super_size: 2,
        sub_size: 2,
        super_offset: 1,
        sub_offset: 1,
      }
    );
    // The accepted interval changes with the authored size, rather than
    // using fixed percentage cutoffs before half-point quantization.
    assert!(
      !ScriptParameters {
        super_offset: 5,
        ..parameters
      }
      .valid(24)
    );
    assert!(
      ScriptParameters {
        super_offset: 5,
        ..parameters
      }
      .valid(20)
    );
  }

  #[test]
  fn invalid_font_script_metrics_fall_back_without_a_family_or_charset_switch() {
    let fonts = Fonts::default();
    for (family, reduced, superscript, subscript) in [
      ("BIZ UDGothic", 7.0, 5.0, -1.0),
      ("MS Gothic", 7.0, 5.0, -1.5),
      ("Yu Gothic", 7.0, 5.0, -2.5),
      ("Meiryo", 8.0, 5.5, -1.5),
    ] {
      for (alignment, shift) in [
        (w::VerticalPositionValues::Superscript, superscript),
        (w::VerticalPositionValues::Subscript, subscript),
      ] {
        let mut style = TextStyle {
          font_family: Some(family.into()),
          high_ansi_font_family: Some(family.into()),
          complex_font_family: Some(family.into()),
          font_size_pt: 12.0,
          complex_font_size_pt: Some(12.0),
          wordprocessingml_font_slots: true,
          wordprocessingml_position_pt: Some(2.0),
          baseline_shift_pt: 2.0,
          ..Default::default()
        };
        properties::apply_vertical_text_alignment(&mut style, alignment);
        for _ in 0..2 {
          fonts.resolve(&mut style);
          assert_eq!(style.font_size_pt, reduced, "{family}/{alignment:?}");
          assert_eq!(style.complex_font_size_pt, Some(reduced));
          assert_eq!(
            style.baseline_shift_pt,
            shift + 2.0,
            "{family}/{alignment:?}"
          );
        }
        properties::apply_vertical_text_alignment(&mut style, w::VerticalPositionValues::Baseline);
        assert_eq!(style.font_size_pt, 12.0);
        assert_eq!(style.baseline_shift_pt, 2.0);
      }
    }
  }

  #[test]
  fn private_script_faces_require_cjk_charset_for_native_side_leading() {
    let mut fonts = FontResolver::default();
    for (family, plain_super, plain_sub, cjk_super, cjk_sub) in [
      ("BIZ UDGothic", 4.5, -0.5, 5.0, -1.0),
      ("MS Gothic", 4.5, -0.5, 5.0, -1.5),
      ("Meiryo", 4.0, -1.5, 5.5, -1.5),
    ] {
      let style = TextStyle {
        font_family: Some(family.into()),
        wordprocessingml_font_slots: true,
        ..Default::default()
      };
      let mut data = fonts.cached_text_face(&style).expect("native control face");
      for embedded in [false, true] {
        data.document_embedded = embedded;
        for charset in [0, 3] {
          let mut face = DeviceFace::new(&data, charset).unwrap();
          let cjk = !embedded || charset == 3;
          for (subscript, expected) in [
            (false, if cjk { cjk_super } else { plain_super }),
            (true, if cjk { cjk_sub } else { plain_sub }),
          ] {
            let size = face.glyph_size(12.0, subscript, false).unwrap();
            assert_eq!(
              face.baseline(12.0, size, subscript),
              Some(expected),
              "{family}/{embedded}/{charset}/{subscript}"
            );
          }
        }
      }
    }
    let style = TextStyle {
      font_family: Some("MS Gothic".into()),
      ..Default::default()
    };
    let data = fonts.cached_text_face(&style).unwrap();
    let mut face = DeviceFace::new(&data, 3).unwrap();
    let reduced = face.metrics(6.0).unwrap();
    assert_eq!(
      (reduced.ascent, reduced.descent, reduced.leading),
      (43, 15, 8)
    );
    assert_eq!(face.baseline(10.0, 6.0, true), Some(-1.0));
  }

  #[test]
  fn retained_default_charset_scripts_keep_the_logical_printer_cell() {
    use ooxmlsdk_fonts::{FontFamilyClass, FontPitch};
    let fonts = Fonts::default();
    // Independent Word charset00/01 controls, 12/24/36pt, both script kinds.
    // Arial normalizes the default charset and is the negative control.
    for (family, class, pitch, plain, retained) in [
      (
        "Liberation Serif",
        FontFamilyClass::Serif,
        FontPitch::Variable,
        [(4.0, -1.0), (8.5, -1.5), (12.5, -3.0)],
        [(5.5, -1.5), (11.0, -3.0), (16.5, -5.0)],
      ),
      (
        "Calibri",
        FontFamilyClass::SansSerif,
        FontPitch::Variable,
        [(4.0, -1.0), (8.0, -2.5), (12.0, -3.0)],
        [(5.5, -1.5), (11.5, -3.5), (17.0, -5.0)],
      ),
      (
        "Arial",
        FontFamilyClass::SansSerif,
        FontPitch::Variable,
        [(4.0, -0.5), (8.5, -2.0), (13.0, -2.5)],
        [(4.0, -0.5), (8.5, -2.0), (13.0, -2.5)],
      ),
      (
        "Courier New",
        FontFamilyClass::Fixed,
        FontPitch::Fixed,
        [(3.0, -1.0), (6.5, -2.5), (11.0, -4.5)],
        [(5.0, -1.5), (10.0, -3.5), (15.0, -5.0)],
      ),
    ] {
      for (charset, shifts) in [
        (FontCharset::Ansi, plain),
        (FontCharset::Other(1), retained),
      ] {
        for (index, size) in [12.0, 24.0, 36.0].into_iter().enumerate() {
          for (align, shift) in [
            (w::VerticalPositionValues::Superscript, shifts[index].0),
            (w::VerticalPositionValues::Subscript, shifts[index].1),
          ] {
            let mut style = TextStyle {
              font_family: Some(family.into()),
              font_family_class: Some(class),
              font_pitch: Some(pitch),
              font_charset: Some(charset),
              font_size_pt: size,
              wordprocessingml_font_slots: true,
              wordprocessingml_legacy_escapement_rounding: true,
              ..Default::default()
            };
            properties::apply_vertical_text_alignment(&mut style, align);
            fonts.resolve(&mut style);
            assert_eq!(style.font_size_pt, [8.0, 15.5, 23.5][index]);
            assert_eq!(
              style.baseline_shift_pt, shift,
              "{family}/{charset:?}/{size}/{align:?}"
            );
            fonts.resolve(&mut style);
            assert_eq!(style.baseline_shift_pt, shift, "cached resolution");
          }
        }
      }
    }
  }

  #[test]
  fn physical_script_sizes_keep_authored_line_sizes_and_baseline_cancellation() {
    let fonts = Fonts::default();
    for (family, expected, superscript, subscript) in [
      (
        "Arial",
        [2.5, 6.5, 9.5],
        [1.5, 3.0, 5.5],
        [-0.5, -0.5, -1.0],
      ),
      (
        "Times New Roman",
        [2.5, 6.5, 9.5],
        [1.5, 3.5, 5.5],
        [-0.5, -1.0, -1.0],
      ),
      (
        "Calibri",
        [2.5, 6.5, 9.5],
        [1.0, 3.5, 5.5],
        [-0.5, -0.5, -1.5],
      ),
      (
        "DejaVu Sans",
        [3.0, 7.0, 10.5],
        [1.0, 3.0, 4.5],
        [-0.5, -1.0, -1.0],
      ),
      (
        "DejaVu Serif",
        [3.0, 7.0, 10.5],
        [1.0, 3.0, 4.5],
        [-0.5, -1.0, -1.0],
      ),
      (
        "Miriam Mono CLM",
        [3.0, 7.0, 10.5],
        [1.0, 2.5, 4.0],
        [0.0, -0.5, -1.0],
      ),
    ] {
      for (index, (size, expected)) in [4.0, 10.0, 15.0].into_iter().zip(expected).enumerate() {
        for align in [
          w::VerticalPositionValues::Superscript,
          w::VerticalPositionValues::Subscript,
        ] {
          let mut style = TextStyle {
            font_family: Some(family.into()),
            high_ansi_font_family: Some(family.into()),
            complex_font_family: Some(family.into()),
            font_size_pt: size,
            complex_font_size_pt: Some(size),
            wordprocessingml_font_slots: true,
            ..Default::default()
          };
          properties::apply_vertical_text_alignment(&mut style, align);
          fonts.resolve(&mut style);
          assert_eq!(style.font_size_pt, expected, "{family}/{size}/{align:?}");
          assert_eq!(style.complex_font_size_pt, Some(expected));
          assert_eq!(style.automatic_escapement_font_size_pt, Some(size));
          let shift = if align == w::VerticalPositionValues::Subscript {
            subscript[index]
          } else {
            superscript[index]
          };
          assert_eq!(style.baseline_shift_pt, shift, "{family}/{size}/{align:?}");
          assert!(!style.automatic_escapement_baseline_normalized);
          fonts.resolve(&mut style);
          assert_eq!(style.font_size_pt, expected, "resolution is idempotent");
          assert_eq!(style.baseline_shift_pt, shift, "resolution is idempotent");
          properties::apply_vertical_text_alignment(
            &mut style,
            w::VerticalPositionValues::Baseline,
          );
          assert_eq!(style.font_size_pt, size);
          assert_eq!(style.complex_font_size_pt, Some(size));
        }
      }
    }
  }

  #[test]
  fn explicit_position_adds_to_the_device_script_shift_and_survives_cancellation() {
    let fonts = Fonts::default();
    for position in [-3.0, 0.0, 3.0] {
      for (alignment, shift) in [
        (w::VerticalPositionValues::Superscript, 5.5),
        (w::VerticalPositionValues::Subscript, -1.0),
      ] {
        let mut style = TextStyle {
          font_family: Some("Arial".into()),
          font_size_pt: 15.0,
          wordprocessingml_font_slots: true,
          wordprocessingml_position_pt: Some(position),
          baseline_shift_pt: position,
          ..Default::default()
        };
        properties::apply_vertical_text_alignment(&mut style, alignment);
        fonts.resolve(&mut style);
        assert_eq!(style.font_size_pt, 9.5);
        assert_eq!(style.baseline_shift_pt, shift + position);
        fonts.resolve(&mut style);
        assert_eq!(style.baseline_shift_pt, shift + position);
        properties::apply_vertical_text_alignment(&mut style, w::VerticalPositionValues::Baseline);
        assert_eq!(style.font_size_pt, 15.0);
        assert_eq!(style.baseline_shift_pt, position);
      }
    }
  }

  #[test]
  fn chinese_script_metrics_follow_physical_face_recommendations() {
    let fonts = Fonts::default();
    // Native controls cover three independent physical faces, both alignments,
    // and half-point ties. Glyph size and the original line size are distinct.
    for family in ["SimSun", "SimHei", "KaiTi"] {
      for (size, reduced, subscript) in [
        (4.0, 2.0, -0.5),
        (8.0, 4.0, -0.5),
        (9.0, 4.5, -0.5),
        (10.0, 5.0, -0.5),
        (10.5, 5.5, -0.5),
        (11.0, 5.5, -0.5),
        (12.0, 6.0, -1.0),
        (15.0, 7.5, -1.0),
        (18.0, 9.0, -1.0),
      ] {
        for (alignment, shift) in [
          (w::VerticalPositionValues::Superscript, reduced),
          (w::VerticalPositionValues::Subscript, subscript),
        ] {
          let mut style = TextStyle {
            font_family: Some(family.into()),
            high_ansi_font_family: Some(family.into()),
            east_asia_font_family: Some(family.into()),
            font_size_pt: size,
            wordprocessingml_font_slots: true,
            ..Default::default()
          };
          properties::apply_vertical_text_alignment(&mut style, alignment);
          fonts.resolve(&mut style);
          assert_eq!(style.font_size_pt, reduced, "{family}/{size}/{alignment:?}");
          assert_eq!(
            style.baseline_shift_pt, shift,
            "{family}/{size}/{alignment:?}"
          );
          assert_eq!(style.automatic_escapement_font_size_pt, Some(size));
        }
      }
    }
  }

  #[test]
  fn mixed_script_baselines_follow_source_slots_but_sizes_follow_linked_faces() {
    let fonts = Fonts::default();
    for (east_asia, alignment, latin_shift, han_shift) in [
      ("SimSun", w::VerticalPositionValues::Superscript, 3.5, 5.5),
      ("SimSun", w::VerticalPositionValues::Subscript, -1.0, -0.5),
      (
        "Times New Roman",
        w::VerticalPositionValues::Superscript,
        3.5,
        3.5,
      ),
      (
        "Times New Roman",
        w::VerticalPositionValues::Subscript,
        -1.0,
        -1.0,
      ),
    ] {
      let mut style = TextStyle {
        font_family: Some("Times New Roman".into()),
        high_ansi_font_family: Some("Times New Roman".into()),
        east_asia_font_family: Some(east_asia.into()),
        font_size_pt: 10.5,
        wordprocessingml_font_slots: true,
        wordprocessingml_han_fallback: true,
        ..Default::default()
      };
      properties::apply_vertical_text_alignment(&mut style, alignment);
      fonts.resolve(&mut style);
      let run = TextRun {
        text: "2正".into(),
        style,
        hyperlink_url: Some("https://example.com".into()),
        dynamic_field: None,
        style_ref_keys: vec!["source-style".into()],
        style_ref_text: Some("2正".into()),
        style_ref_numbering_text: None,
        preserve_text_portion: true,
      };
      let mut inlines = vec![InlineItem::Text(run.clone()), InlineItem::Text(run)];
      let mut events = [
        ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 1 },
        ParagraphFieldEvent::DeferredReferenceParagraphBreak {
          field_id: 1,
          inline_offset: 2,
        },
        ParagraphFieldEvent::ReferenceResultSpan {
          field_id: 2,
          bookmark_name: "source".into(),
          inline_start: 0,
          inline_end: 2,
          merge_format: true,
          copy_text: true,
          separator_style: None,
        },
      ];
      fonts.resolve_inlines(&mut inlines, &mut events);
      assert_eq!(inlines.len(), 4, "{east_asia}/{alignment:?}");
      for (index, inline) in inlines.iter().enumerate() {
        let InlineItem::Text(run) = inline else {
          panic!("text expected")
        };
        let latin = index % 2 == 0;
        assert_eq!(run.text, if latin { "2" } else { "正" });
        assert_eq!(
          run.style.font_size_pt,
          if latin { 7.0 } else { 5.5 },
          "{east_asia}/{alignment:?}/{}",
          run.text
        );
        assert_eq!(
          run.style.baseline_shift_pt,
          if latin { latin_shift } else { han_shift }
        );
        assert_eq!(run.style.automatic_escapement_font_size_pt, Some(10.5));
        assert_eq!(run.hyperlink_url.as_deref(), Some("https://example.com"));
        assert_eq!(run.style_ref_keys[0].as_ref(), "source-style");
        assert_eq!(run.style_ref_text.as_deref(), Some("2正"));
        assert!(run.preserve_text_portion);
      }
      assert!(matches!(
        events[0],
        ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 2 }
      ));
      assert!(matches!(
        events[1],
        ParagraphFieldEvent::DeferredReferenceParagraphBreak {
          inline_offset: 4,
          ..
        }
      ));
      assert!(matches!(
        events[2],
        ParagraphFieldEvent::ReferenceResultSpan {
          inline_start: 0,
          inline_end: 4,
          ..
        }
      ));
      fonts.resolve_inlines(&mut inlines, &mut events);
      assert_eq!(inlines.len(), 4, "resolution is idempotent");
    }
  }

  #[test]
  fn repeated_script_resolution_reuses_owned_device_metrics() {
    let fonts = Fonts::default();
    let mut style = TextStyle {
      font_family: Some("Arial".into()),
      font_size_pt: 15.0,
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    properties::apply_vertical_text_alignment(&mut style, w::VerticalPositionValues::Superscript);
    for _ in 0..100 {
      fonts.resolve(&mut style);
      assert_eq!(style.baseline_shift_pt, 5.5);
    }
    let cache = fonts.0.borrow();
    assert_eq!(cache.faces.len(), 1);
    let face = cache.faces.values().next().unwrap().as_ref().unwrap();
    assert_eq!(face.metrics.len(), 2);
    assert_eq!(face.baselines.len(), 1);
  }
}
