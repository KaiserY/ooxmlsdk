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
    let primary_percent = height_percent(
      cache.fonts.script_metrics(&selection),
      style.automatic_escapement_is_subscript,
    );
    let primary_ties_up = primary_percent == 50 && cache.chinese_script_font(&selection);
    let complex_percent = style.automatic_escapement_complex_font_size_pt.map(|_| {
      selection.complex_script = Some(true);
      selection.right_to_left = Some(true);
      height_percent(
        cache.fonts.script_metrics(&selection),
        style.automatic_escapement_is_subscript,
      )
    });
    let legacy = style.wordprocessingml_legacy_escapement_rounding;
    style.font_size_pt = properties::automatic_escapement_glyph_size(
      original_size,
      primary_percent,
      legacy || primary_ties_up,
    );
    style.complex_font_size_pt = style
      .automatic_escapement_complex_font_size_pt
      .zip(complex_percent)
      .map(|(size, percent)| properties::automatic_escapement_glyph_size(size, percent, legacy));
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

  fn chinese_script_font(&mut self, style: &TextStyle) -> bool {
    self
      .fonts
      .cached_text_face(style)
      .and_then(|face| self.script_face(&face).map(|face| face.chinese_charset))
      .unwrap_or(false)
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
    let percent = if subscript {
      face.sub_height_percent
    } else {
      face.super_height_percent
    };
    if percent != 50 {
      return None;
    }
    Some(properties::automatic_escapement_glyph_size(
      original,
      percent,
      legacy || percent == 50,
    ))
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
      None | Some(FontCharset::Ansi | FontCharset::Other(1)) => 0,
      Some(FontCharset::Symbol) => 2,
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

#[derive(Debug)]
struct DeviceFace {
  units_per_em: i64,
  ascent: i64,
  descent: i64,
  leading: i64,
  super_offset: i64,
  sub_offset: i64,
  chinese_charset: bool,
  super_height_percent: u32,
  sub_height_percent: u32,
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
      super_offset: i64::from(os2.y_superscript_y_offset()),
      sub_offset: i64::from(os2.y_subscript_y_offset()),
      chinese_charset: os2
        .ul_code_page_range_1()
        .is_some_and(|range| range & (1 << 18) != 0),
      super_height_percent: scale_height_percent(Some(
        f32::from(os2.y_superscript_y_size()) / units_per_em as f32,
      )),
      sub_height_percent: scale_height_percent(Some(
        f32::from(os2.y_subscript_y_size()) / units_per_em as f32,
      )),
      charset,
      vdmx: face
        .table_data(skrifa::Tag::new(b"VDMX"))
        .map(|table| table.as_bytes().to_vec()),
      metrics: FxHashMap::default(),
      baselines: FxHashMap::default(),
    })
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
    let (ascent, descent) = hinted.map_or_else(
      || (scaled(self.ascent), scaled(self.descent)),
      |metrics| (i64::from(metrics.ascent), i64::from(metrics.descent)),
    );
    let metrics = DeviceMetrics {
      ascent,
      descent,
      leading: scaled(self.leading),
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
    let recommendation = if subscript {
      -self.sub_offset
    } else {
      self.super_offset
    };
    let initial =
      (f64::from(original) * 2.0 * recommendation as f64 / self.units_per_em as f64).round() as i64;
    // Word's native font cache keeps the original external leading, rounds
    // each printer metric independently to 144 DPI, then limits the font's
    // recommendation. A superscript rises at least one third of the original
    // ascent within that limit; a subscript cannot lower the line's descent.
    let chinese_half_size = self.chinese_charset
      && if subscript {
        self.sub_height_percent == 50
      } else {
        self.super_height_percent == 50
      };
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
