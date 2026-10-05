use icu_properties::{
  CodePointMapData,
  props::{JoiningGroup, JoiningType, Script},
};

use super::{
  FontRef, FontStyleRef, LocationRef, MetadataProvider, ShapedGlyph, ShapedGlyphBounds, ShapedText,
  Size,
};
use crate::common::{KashidaExpansion, WordprocessingKashida, wordprocessing_device};
use skrifa::raw::TableProvider;

const DEVICE_PIXEL_PT: f32 = 72.0 / 600.0;

pub(crate) struct KashidaOpportunity {
  pub byte_index: usize,
  pub minimum_width_pt: f32,
  pub glyph_width_pt: f32,
}

/// Word's completed Arabic runs use whole device advances, including GPOS,
/// before allocating justification. Ideal document widths still own fitting.
/// Native Word captures compare all source advances in three complete lines.
pub(super) fn realize_device(
  text: &str,
  mut shaped: ShapedText,
  scale: f32,
  device: &WordprocessingKashida,
  character_spacing_pt: f32,
  natural_metrics: &mut super::WordNaturalMetricCache,
) -> Option<ShapedText> {
  if !scale.is_finite()
    || !character_spacing_pt.is_finite()
    || (scale - f32::from(device.font_width_percent) / 100.0).abs() > 0.00001
  {
    return None;
  }
  let pitch_pixels = wordprocessing_device::source_coordinate(character_spacing_pt) as f64;
  let mut width = 0.0;
  for glyph in &mut shaped.glyphs {
    let face = shaped.font_faces.get(glyph.font_index)?;
    let font = FontRef::from_index(face.data.as_slice(), face.index).ok()?;
    font.glyf().ok()?;
    if font.fvar().is_ok() {
      return None;
    }
    let upem = f64::from(font.head().ok()?.units_per_em());
    let size = f64::from(glyph.font_size_pt);
    let pixels = (size / f64::from(DEVICE_PIXEL_PT)).round();
    let ratio =
      wordprocessing_device::font_width_ratio(face, glyph.font_size_pt, device.font_width_percent)?;
    if upem == 0.0 || !(1.0..=f64::from(u16::MAX)).contains(&pixels) {
      return None;
    }
    let design = (f64::from(glyph.x_advance_em) * upem / f64::from(scale)).round();
    let cluster = text.get(glyph.text_range.clone());
    let mut advance =
      if cluster == Some("\u{00a0}") || (device.unshaped_blanks && cluster == Some(" ")) {
        // Word sends NBSP as an independent unshaped blank, outside the Arabic
        // GetGlyphPlacements arrays. Its GDI Natural whole-pixel space width
        // precedes the LOGFONT width transform. Native lowKashida observations
        // cover three fonts, three sizes, both weights and three width ratios;
        // applying the Arabic shaped-glyph rounding changes the join budget.
        let space = font.charmap().map(' ')?;
        let natural = natural_metrics.advance(face, pixels as u16, space.to_u32())?;
        wordprocessing_device::glyph_advance(f64::from(natural) * ratio)
      } else {
        wordprocessing_device::glyph_advance(design * pixels / upem * ratio)
      };
    if pitch_pixels != 0.0 {
      // Native Word keeps a mark's zero advance when condensed, while a
      // positive pitch can enlarge it independently of the adjacent base.
      // Do not merge their pitch merely because HarfBuzz shares a cluster.
      // Preserve signed cursive placement advances and their GPOS owner.
      advance = if advance >= 0.0 {
        (advance + pitch_pixels).max(0.0)
      } else {
        advance + pitch_pixels
      };
    }
    if !advance.is_finite() || !(f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&advance) {
      return None;
    }
    if let Some((_, extra)) = device
      .space_expansions
      .iter()
      .find(|(byte_index, _)| *byte_index == glyph.text_range.start)
    {
      advance += f64::from(*extra);
    }
    let advance_pt = advance as f32 * DEVICE_PIXEL_PT;
    glyph.x_advance_em = advance_pt / glyph.font_size_pt;
    width += advance_pt;
  }
  shaped.width_pt = width;
  Some(shaped)
}

fn device_extender_widths(
  shaped: &ShapedText,
  glyph: &ShapedGlyph,
  percent: u16,
) -> Option<(f32, f32)> {
  let face = shaped.font_faces.get(glyph.font_index)?;
  let font = FontRef::from_index(face.data.as_slice(), face.index).ok()?;
  let id = font.charmap().map('\u{0640}')?;
  let advance_em = font
    .glyph_metrics(Size::new(1.0), LocationRef::default())
    .advance_width(id)?;
  let pixels = (glyph.font_size_pt / DEVICE_PIXEL_PT).round();
  let width = advance_em * pixels;
  let ratio = wordprocessing_device::font_width_ratio(face, glyph.font_size_pt, percent)?;
  Some((
    (f64::from(width) * ratio).round() as f32 * DEVICE_PIXEL_PT,
    width * DEVICE_PIXEL_PT,
  ))
}

fn extender(shaped: &ShapedText, glyph: &ShapedGlyph, scale: f32) -> Option<ShapedGlyph> {
  let face = shaped.font_faces.get(glyph.font_index)?;
  let font = FontRef::from_index(face.data.as_ref(), face.index).ok()?;
  let id = font.charmap().map('\u{0640}')?;
  let metrics = font.glyph_metrics(Size::new(1.0), LocationRef::default());
  let width = metrics.advance_width(id)? * scale;
  if !width.is_finite() || width <= 0.0 {
    return None;
  }
  Some(ShapedGlyph {
    font_index: glyph.font_index,
    font_size_pt: glyph.font_size_pt,
    glyph_id: id.to_u32(),
    text_range: glyph.text_range.clone(),
    safe_to_insert_tatweel: false,
    x_advance_em: width,
    x_offset_em: 0.0,
    y_offset_em: 0.0,
    y_advance_em: 0.0,
    bounds_em: metrics.bounds(id).map(|bounds| ShapedGlyphBounds {
      x_min_em: bounds.x_min * scale,
      y_min_em: bounds.y_min,
      x_max_em: bounds.x_max * scale,
      y_max_em: bounds.y_max,
    }),
  })
}

pub(super) fn opportunities(
  text: &str,
  shaped: &ShapedText,
  style: &(impl FontStyleRef + ?Sized),
) -> Vec<KashidaOpportunity> {
  let joining = CodePointMapData::<JoiningType>::new();
  let groups = CodePointMapData::<JoiningGroup>::new();
  let scripts = CodePointMapData::<Script>::new();
  let mut result = Vec::new();
  let mut word_start = 0;
  let word_device = style.wordprocessing_kashida().is_some();
  for word in text.split_inclusive(|ch: char| {
    // Native Word selects one connection across these embedded punctuation
    // marks (e.g. قطاع/موضوع). Spaces, hyphens and zero-width spaces still
    // separate its selection domains. Keep unprobed delimiters and other
    // consumers on the existing path.
    if word_device {
      match ch {
        '\u{200b}' => return true,
        '/' | ',' | '\u{060c}' | ':' | '.' => return false,
        _ => {}
      }
    }
    !ch.is_alphanumeric()
      && joining.get(ch) != JoiningType::Transparent
      && !matches!(ch, '\u{0640}' | '\u{200c}' | '\u{200d}')
  }) {
    let end = word_start + word.len();
    let selected = shaped
      .glyphs
      .iter()
      .filter_map(|glyph| {
        let byte_index = glyph.text_range.start;
        if !glyph.safe_to_insert_tatweel || byte_index <= word_start || byte_index >= end {
          return None;
        }
        let previous = text
          .get(word_start..byte_index)?
          .chars()
          .rev()
          .find(|&ch| joining.get(ch) != JoiningType::Transparent)?;
        if scripts.get(previous) != Script::Arabic && previous != '\u{0640}' {
          return None;
        }
        let cluster = text.get(glyph.text_range.clone())?;
        let mut cluster_characters = cluster
          .char_indices()
          .filter(|&(_, ch)| joining.get(ch) != JoiningType::Transparent);
        let (first_offset, first) = cluster_characters.next()?;
        // Word's glyph properties describe the character at the join's
        // entry, including a medial Yeh in a Yeh+Noon ligature. Its final
        // source character does not describe that entry shape. Native Word
        // controls give both Beh/Reh and Teh/Reh font ligatures the BA class.
        // Retain their entry character's priority: promoting the compound to
        // standalone Reh's priority outranks a later Yeh/Noon BA connection.
        let (current, next_start) = if word_device {
          (first, byte_index + first_offset + first.len_utf8())
        } else {
          (
            cluster
              .chars()
              .rev()
              .find(|&ch| joining.get(ch) != JoiningType::Transparent)?,
            glyph.text_range.end,
          )
        };
        let next = text
          .get(next_start..end)?
          .chars()
          .find(|&ch| joining.get(ch) != JoiningType::Transparent);
        let connects_next = matches!(
          joining.get(current),
          JoiningType::DualJoining | JoiningType::JoinCausing
        ) && next.is_some_and(|ch| {
          matches!(
            joining.get(ch),
            JoiningType::DualJoining | JoiningType::RightJoining | JoiningType::JoinCausing
          )
        });
        // Uniscribe marks medial U+0649 (alef maksura) as
        // SCRIPT_JUSTIFY_NONE, even though its shaped glyph is otherwise a
        // safe HarfBuzz insertion boundary. Word instead extends another
        // join in words such as "ولىس"; final U+0649 remains eligible.
        if current == '\u{0649}' && connects_next {
          return None;
        }
        // Native Word's connection priorities. These are priorities of
        // eligible shaped joins, not permission to split arbitrary letters.
        // Seen/Sad extend after the letter; the remaining classes extend
        // before the selected final/medial shape. Equal priority prefers the
        // last logical position in the word.
        let priority = if previous == '\u{0640}' {
          8
        } else if matches!(groups.get(previous), JoiningGroup::Seen | JoiningGroup::Sad) {
          7
        } else {
          match (groups.get(current), connects_next) {
            (JoiningGroup::Heh | JoiningGroup::TehMarbuta | JoiningGroup::Dal, false) => 6,
            (
              JoiningGroup::Alef
              | JoiningGroup::Beh
              | JoiningGroup::Kaf
              | JoiningGroup::Gaf
              | JoiningGroup::Lam,
              false,
            ) => 5,
            (
              JoiningGroup::Waw | JoiningGroup::Ain | JoiningGroup::Feh | JoiningGroup::Qaf,
              false,
            ) => 4,
            (JoiningGroup::Reh | JoiningGroup::Yeh | JoiningGroup::FarsiYeh, false) => 3,
            (
              JoiningGroup::Beh | JoiningGroup::Noon | JoiningGroup::Yeh | JoiningGroup::FarsiYeh,
              true,
            ) => 2,
            _ => 1,
          }
        };
        let width =
          extender(shaped, glyph, style.horizontal_scale())?.x_advance_em * glyph.font_size_pt;
        let (minimum_width_pt, glyph_width_pt) =
          if let Some(device) = style.wordprocessing_kashida() {
            device_extender_widths(shaped, glyph, device.font_width_percent)?
          } else {
            (width, width)
          };
        if minimum_width_pt <= 0.0 || glyph_width_pt <= 0.0 {
          return None;
        }
        Some((priority, byte_index, minimum_width_pt, glyph_width_pt))
      })
      .max_by_key(|&(priority, byte_index, _, _)| (priority, byte_index));
    if let Some((_, byte_index, minimum_width_pt, glyph_width_pt)) = selected {
      result.push(KashidaOpportunity {
        byte_index,
        minimum_width_pt,
        glyph_width_pt,
      });
    }
    word_start = end;
  }
  result
}

pub(super) fn expand(mut shaped: ShapedText, style: &(impl FontStyleRef + ?Sized)) -> ShapedText {
  let expansions = style.kashida_expansions();
  if expansions.is_empty() {
    return shaped;
  }
  let mut glyphs = Vec::with_capacity(shaped.glyphs.len());
  for (index, glyph) in shaped.glyphs.iter().enumerate() {
    glyphs.push(glyph.clone());
    // Marks in the same cluster stay with their base before adding extenders.
    if shaped
      .glyphs
      .get(index + 1)
      .is_some_and(|next| next.text_range == glyph.text_range)
    {
      continue;
    }
    let Some(KashidaExpansion {
      advance,
      glyph_count,
      ..
    }) = expansions
      .iter()
      .find(|extension| extension.byte_index == glyph.text_range.start)
    else {
      continue;
    };
    if !advance.0.is_finite() || advance.0 <= 0.0 || *glyph_count == 0 {
      continue;
    }
    if !shaped
      .glyphs
      .iter()
      .any(|candidate| candidate.text_range == glyph.text_range && candidate.safe_to_insert_tatweel)
    {
      continue;
    }
    let Some(mut extension) = extender(&shaped, glyph, style.horizontal_scale()) else {
      continue;
    };
    let native_width = style.wordprocessing_kashida().and_then(|device| {
      device_extender_widths(&shaped, glyph, device.font_width_percent)
        .map(|(_, width)| width / glyph.font_size_pt)
    });
    let width_em = native_width.unwrap_or(extension.x_advance_em);
    let total_em = advance.0 / glyph.font_size_pt;
    let step_em = if native_width.is_some() {
      width_em
    } else if *glyph_count > 1 {
      (total_em - width_em) / (*glyph_count - 1) as f32
    } else {
      total_em
    };
    for copy in 0..*glyph_count {
      extension.x_advance_em = if copy + 1 == *glyph_count {
        total_em - step_em * copy as f32
      } else {
        step_em
      };
      extension.x_offset_em = if native_width.is_some() && copy + 1 == *glyph_count {
        (total_em - width_em * *glyph_count as f32).min(0.0)
      } else if *glyph_count == 1 {
        (total_em - width_em).min(0.0)
      } else {
        0.0
      };
      glyphs.push(extension.clone());
    }
    shaped.width_pt += advance.0;
  }
  shaped.glyphs = glyphs;
  shaped
}

#[cfg(test)]
mod tests {
  use crate::{
    common::{KashidaExpansion, Pt, TextStyle},
    text_metrics::TextMetrics,
  };
  use std::{borrow::Cow, sync::Arc};

  fn arabic_style() -> TextStyle<'static> {
    TextStyle {
      font_family: Some(Cow::Borrowed("Arial")),
      complex_font_family: Some(Cow::Borrowed("Arial")),
      font_size: Pt(14.0),
      complex_font_size: Some(Pt(14.0)),
      right_to_left: Some(true),
      resolved_bidi_level: Some(1),
      ..Default::default()
    }
  }

  #[test]
  fn word_device_character_spacing_matches_native_numeric_and_mark_advances() {
    let mut metrics = TextMetrics::new();
    let mut style = arabic_style();
    style.font_family = Some(Cow::Borrowed("Traditional Arabic"));
    style.complex_font_family = style.font_family.clone();
    style.font_size = Pt(15.0);
    style.complex_font_size = Some(Pt(15.0));
    style.wordprocessingml_font_slots = true;
    style.wordprocessing_kashida = Some(Arc::new(crate::common::WordprocessingKashida {
      retain_trailing_blank: false,
      unshaped_blanks: false,
      font_width_percent: 100,
      space_expansions: Vec::new(),
    }));
    // Read-only Word expansion inputs for source numeral 4 and fathatan.
    // The mark's condensation floor is independent of the numeric advance.
    for (spacing, numeric_pixels, mark_pixels) in [
      (-0.5, 54.0, 0.0),
      (-0.1, 57.0, 0.0),
      (-0.05, 58.0, 0.0),
      (0.0, 58.0, 0.0),
      (0.05, 58.0, 0.0),
      (0.1, 59.0, 1.0),
      (0.5, 62.0, 4.0),
    ] {
      style.character_spacing = Pt(spacing);
      let numeric = metrics.shape_text("٤", &style).expect("source numeral");
      assert_eq!(numeric.glyphs.len(), 1);
      assert_eq!(numeric.glyphs[0].glyph_id, 209);
      assert!(
        (numeric.width_pt / super::DEVICE_PIXEL_PT - numeric_pixels).abs() < 0.001,
        "spacing={spacing}: numeral {}",
        numeric.width_pt
      );
      let marked = metrics
        .shape_text("أيضاً", &style)
        .expect("marked source word");
      let mark = marked
        .glyphs
        .iter()
        .find(|glyph| glyph.glyph_id == 197)
        .expect("native fathatan glyph");
      assert!(
        (mark.x_advance_em * mark.font_size_pt / super::DEVICE_PIXEL_PT - mark_pixels).abs()
          < 0.001,
        "spacing={spacing}: mark {}",
        mark.x_advance_em
      );
    }
  }

  #[test]
  fn word_device_kashida_uses_nominal_steps_and_overlaps_the_last_copy() {
    let mut metrics = TextMetrics::new();
    let mut style = arabic_style();
    style.font_family = Some(Cow::Borrowed("Traditional Arabic"));
    style.complex_font_family = style.font_family.clone();
    style.font_size = Pt(15.0);
    style.complex_font_size = Some(Pt(15.0));
    style.horizontal_scale = Some(1.03);
    style.wordprocessing_font_width_percent = Some(103);
    style.wordprocessing_legacy_font_measurement = Some(true);
    style.wordprocessingml_font_slots = true;
    let text = "السياسات";
    let ideal = metrics.measure_text(text, &style);
    style.wordprocessing_kashida = Some(Arc::new(crate::common::WordprocessingKashida {
      retain_trailing_blank: false,
      unshaped_blanks: false,
      font_width_percent: 103,
      space_expansions: Vec::new(),
    }));
    let source = metrics.shape_text(text, &style).unwrap();
    let opportunity = metrics.kashida_opportunities(text, &style).remove(0);
    // Configured native Word: em125, gid186, design width110/upem2048.
    assert!((opportunity.minimum_width_pt - 0.84).abs() < 0.00001);
    assert!((opportunity.glyph_width_pt - 0.80566406).abs() < 0.00001);
    for (pixels, copies) in [(28.0, 5), (7.0, 2), (16.0, 3)] {
      style.kashida_expansions = Some(Arc::from([KashidaExpansion {
        byte_index: opportunity.byte_index,
        advance: Pt(pixels * 0.12),
        glyph_count: copies,
      }]));
      let expanded = metrics.shape_text(text, &style).unwrap();
      let extenders = expanded
        .glyphs
        .iter()
        .filter(|glyph| glyph.glyph_id == 186)
        .collect::<Vec<_>>();
      assert_eq!(extenders.len(), copies);
      assert!((expanded.width_pt - source.width_pt - pixels * 0.12).abs() < 0.00001);
      for extender in &extenders[..copies - 1] {
        assert!((extender.x_advance_em * 15.0 - 0.80566406).abs() < 0.00001);
        assert_eq!(extender.x_offset_em, 0.0);
      }
      let last = extenders.last().unwrap();
      assert!(last.x_offset_em < 0.0);
      assert_eq!(last.text_range.start, opportunity.byte_index);
    }
    style.kashida_expansions = None;
    style.wordprocessing_kashida = None;
    assert_eq!(metrics.measure_text(text, &style), ideal);
  }

  #[test]
  fn word_device_kashida_keeps_embedded_punctuation_in_one_word() {
    let mut metrics = TextMetrics::new();
    let mut style = arabic_style();
    style.font_family = Some(Cow::Borrowed("Traditional Arabic"));
    style.complex_font_family = style.font_family.clone();
    style.font_size = Pt(15.0);
    style.complex_font_size = Some(Pt(15.0));
    style.horizontal_scale = Some(1.03);
    style.wordprocessing_font_width_percent = Some(103);
    style.wordprocessingml_font_slots = true;
    style.wordprocessing_kashida = Some(Arc::new(crate::common::WordprocessingKashida {
      retain_trailing_blank: false,
      unshaped_blanks: false,
      font_width_percent: 103,
      space_expansions: Vec::new(),
    }));
    // Eight native controls: the five embedded punctuation variants select
    // the DAD connection in موضوع; whitespace, hyphen and ZWSP select one
    // connection in each of the two words.
    for separator in ['/', ',', '\u{060c}', ':', '.'] {
      let text = format!("قطاع{separator}موضوع ");
      let positions = metrics.kashida_opportunities(&text, &style);
      assert_eq!(positions.len(), 1, "{text}");
      // SCRIPT_VISATTR extends after DAD (character7); HarfRust marks the
      // same connection at the following WAW's source boundary (character8).
      assert_eq!(
        positions[0].byte_index,
        text.char_indices().nth(8).unwrap().0
      );
      let ordinary = TextStyle {
        wordprocessing_kashida: None,
        ..style.clone()
      };
      assert_eq!(metrics.kashida_opportunities(&text, &ordinary).len(), 2);
    }
    for separator in [' ', '-', '\u{200b}'] {
      let text = format!("قطاع{separator}موضوع ");
      assert_eq!(
        metrics.kashida_opportunities(&text, &style).len(),
        2,
        "{text}"
      );
    }
  }

  #[test]
  fn office_connection_priority_matrix() {
    let mut metrics = TextMetrics::new();
    let style = arabic_style();
    // Word LowKashida, Arial14, fixed120pt frame. Each four-letter word
    // has a natural control; glyph traces identify the single extended join.
    let letters: Vec<char> = "ابدرسصطعفقكلمهو ي".replace(' ', "").chars().collect();
    let positions = [
      "3331111111331311",
      "3333222333332333",
      "1131111111111311",
      "3333111333331333",
      "2222222222222222",
      "2222222222222222",
      "3333222333332333",
      "3333222333332333",
      "3333222333332333",
      "3333222333332333",
      "3333222333332333",
      "3333222333332333",
      "3333222333332333",
      "3333222333332333",
      "3331111333331331",
      "3333222333332333",
    ];
    for (i, &a) in letters.iter().enumerate() {
      for (j, &b) in letters.iter().enumerate() {
        let text = format!("ب{a}ب{b}");
        let opportunities = metrics.kashida_opportunities(&text, &style);
        assert_eq!(opportunities.len(), 1, "{text}");
        let expected = usize::from(positions[i].as_bytes()[j] - b'0') * 2;
        assert_eq!(opportunities[0].byte_index, expected, "{text}");
      }
    }
  }

  #[test]
  fn office_arabic_extensions_use_safe_joins_and_keep_source_clusters() {
    let mut metrics = TextMetrics::new();
    let style = arabic_style();
    // Native Word controls: final connected join, Seen/Sad priority, last
    // equal-priority join, and disconnected final characters.
    for (text, character) in [
      ("التي", 3),
      ("دينه", 3),
      ("ببي", 2),
      ("بري", 1),
      ("سبي", 1),
      ("سشي", 2),
      ("بصي", 2),
      ("سلي", 1),
    ] {
      let byte_index = text.char_indices().nth(character).unwrap().0;
      let positions = metrics.kashida_opportunities(text, &style);
      assert_eq!(positions.len(), 1, "{text}");
      assert_eq!(positions[0].byte_index, byte_index, "{text}");
      let natural = metrics.shape_text(text, &style).unwrap();
      let natural_width = metrics.measure_text(text, &style);
      let glyph_count = (50.0 / positions[0].minimum_width_pt).ceil() as usize;
      let justified_style = TextStyle {
        kashida_expansions: Some(Arc::from([KashidaExpansion {
          byte_index,
          advance: Pt(50.0),
          glyph_count,
        }])),
        ..style.clone()
      };
      let justified = metrics.shape_text(text, &justified_style).unwrap();
      assert!((justified.width_pt - natural.width_pt - 50.0).abs() < 0.001);
      assert_eq!(justified.glyphs.len(), natural.glyphs.len() + glyph_count);
      assert!(justified.glyphs.iter().all(|glyph| {
        natural
          .glyphs
          .iter()
          .any(|source| source.text_range == glyph.text_range)
      }));
      assert!((metrics.measure_text(text, &justified_style) - natural_width - 50.0).abs() < 0.001);
      assert_eq!(metrics.measure_text(text, &style), natural_width);
      let advance: f32 = justified
        .glyphs
        .iter()
        .map(|glyph| glyph.x_advance_em * glyph.font_size_pt)
        .sum();
      assert!((advance - justified.width_pt).abs() < 0.001);
    }
    for text in ["لا", "ب\u{200c}ب", "אב", "abc"] {
      assert!(
        metrics.kashida_opportunities(text, &style).is_empty(),
        "{text}"
      );
    }
  }

  #[test]
  fn word_device_kashida_uses_native_ligature_entry_priorities() {
    let mut metrics = TextMetrics::new();
    // Native Word LowKashida controls: two fonts,32 words. Each authored
    // soft-break line has an unextended final-line control. C# captures
    // identify the advance owner; the following glyph owns our boundary.
    let cases = [
      ("المغتربين", 14, 10),
      ("المغتربون", 14, 14),
      ("المغتربات", 8, 14),
      ("المغبربين", 14, 10),
      ("المغيربين", 14, 10),
      ("المتربين", 12, 8),
      ("لتمكين", 8, 8),
      ("يمكن", 6, 6),
      ("تمكين", 6, 6),
      ("الدين", 4, 4),
      ("والدين", 6, 6),
      ("المادة", 6, 6),
      ("المرأة", 6, 6),
      ("الطفل", 8, 8),
      ("والأطفال", 12, 12),
      ("الطفولة", 12, 12),
      ("وعلى", 6, 6),
      ("المساهمة", 8, 8),
      ("العالم", 6, 6),
      ("النظام", 8, 8),
      ("سما", 4, 2),
      ("سلام", 2, 2),
      ("سليم", 2, 2),
      ("لكي", 4, 4),
      ("الذين", 4, 4),
      ("بدونه", 8, 8),
      ("تكوين", 4, 4),
      ("قوانين", 2, 2),
      ("المواطنين", 6, 6),
      ("العمل", 8, 8),
      ("بلا", 2, 2),
      ("ببلا", 2, 2),
      ("بلالا", 2, 2),
      ("برا", 2, 2),
      ("ببرا", 2, 4),
      ("البر", 4, 6),
      ("بريم", 2, 2),
      ("ببريا", 2, 8),
    ];
    for (font, traditional) in [("Traditional Arabic", true), ("Arial", false)] {
      let mut style = arabic_style();
      style.font_family = Some(Cow::Borrowed(font));
      style.complex_font_family = style.font_family.clone();
      style.font_size = Pt(15.0);
      style.complex_font_size = Some(Pt(15.0));
      style.horizontal_scale = Some(1.03);
      style.wordprocessing_kashida = Some(Arc::new(crate::common::WordprocessingKashida {
        retain_trailing_blank: false,
        unshaped_blanks: false,
        font_width_percent: 103,
        space_expansions: Vec::new(),
      }));
      for (text, traditional_byte, arial_byte) in cases {
        let positions = metrics.kashida_opportunities(text, &style);
        assert_eq!(positions.len(), 1, "{font}: {text}");
        assert_eq!(
          positions[0].byte_index,
          if traditional {
            traditional_byte
          } else {
            arial_byte
          },
          "{font}: {text}",
        );
      }
    }
  }

  #[test]
  fn office_medial_alef_maksura_has_no_kashida_opportunity() {
    let mut metrics = TextMetrics::new();
    let style = arabic_style();
    // Native Word lowKashida controls and Uniscribe SCRIPT_VISATTR classes:
    // medial alef maksura has class NONE, while medial yeh remains eligible.
    for (text, selected_character) in [
      ("ولىس", 3),
      ("وليس", 2),
      ("بلىس", 3),
      ("بلىر", 3),
      ("بلى", 2),
      ("سلىس", 1),
    ] {
      let opportunities = metrics.kashida_opportunities(text, &style);
      assert_eq!(opportunities.len(), 1, "{text}");
      assert_eq!(
        opportunities[0].byte_index,
        text.char_indices().nth(selected_character).unwrap().0,
        "{text}"
      );
    }
  }
}
