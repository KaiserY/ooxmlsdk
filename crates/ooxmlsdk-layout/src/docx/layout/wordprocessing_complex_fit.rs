//! Word's incremental complex-text measurements are distinct from final paint.
//!
//! Native MSLS observations show a nominal-width read-ahead, followed by
//! shaping and another read-ahead if necessary. Extending an unfinished word
//! reshapes that word, but does not replace already committed word and blank
//! widths. Contextual GPOS on a preceding blank can therefore affect
//! fitting even though the final line is painted with fresh glyph positions.

use super::{InlineItem, Script, TextMetrics, TextStyle, UnicodeScript};
use crate::docx::Paragraph;
use skrifa::{
  FontRef, MetadataProvider,
  instance::{LocationRef, Size},
};

const IDEAL_UNITS_PER_POINT: f32 = 4096.0;
const SOURCE_BUFFER_CHARACTERS: usize = 512;
const SOURCE_ITEM_CHARACTERS: usize = 256;
const READ_AHEAD_CHARACTERS: i64 = 12;

pub(super) struct Plan {
  capacity_pt: f32,
  changed_lines: Vec<ChangedLine>,
  inline_ranges: Vec<Option<std::ops::Range<usize>>>,
}

struct ChangedLine {
  start: usize,
  end: usize,
  limit: usize,
  deltas: Vec<(usize, i64)>,
}

struct SourceObject {
  character: usize,
  advance: i64,
}

struct Profile {
  face: crate::fonts::FontFaceData,
  size_pt: f32,
  reference: i64,
  width_pt: f32,
  contextual_blank: bool,
}

impl Plan {
  pub(super) fn for_paragraph(
    paragraph: &Paragraph,
    capacity_pt: f32,
    metrics: &mut TextMetrics,
  ) -> Option<Self> {
    if let [InlineItem::Text(run)] = paragraph.inlines.as_slice() {
      return Self::new(
        &run.text,
        &run.style,
        &paragraph.format.complex_source_restarts,
        capacity_pt,
        metrics,
      );
    }
    let mut text = String::new();
    let mut objects = Vec::new();
    let mut inline_ranges = Vec::with_capacity(paragraph.inlines.len());
    let mut ordinary_style = None;
    let mut profile: Option<Profile> = None;
    let mut width_pt = 0.0;
    let mut contextual_blank = false;
    let mut character = 0;
    for inline in &paragraph.inlines {
      let InlineItem::Text(run) = inline else {
        return None;
      };
      if run.dynamic_field.is_some()
        || !run.style_ref_keys.is_empty()
        || run.style_ref_text.is_some()
      {
        return None;
      }
      if run
        .hyperlink_url
        .as_deref()
        .and_then(super::page_note_link_parts)
        .is_some_and(|(_, backlink, _)| !backlink)
      {
        let width = metrics.measure_text(&run.text, &run.style);
        if !width.is_finite() || width <= 0.0 {
          return None;
        }
        // A generated reference occupies one source CP, independently of
        // the number of printed label characters. Never shape the internal
        // placeholder or interpret an authored U+FFFC as a reference.
        text.push('\u{fffc}');
        objects.push(SourceObject {
          character,
          advance: ideal(width),
        });
        inline_ranges.push(None);
        character += 1;
        width_pt += width;
      } else {
        if run.hyperlink_url.is_some()
          || !eligible_style(&run.style)
          || !run.text.chars().all(supported_character)
          || ordinary_style.is_some_and(|style| style != &run.style)
        {
          return None;
        }
        ordinary_style = Some(&run.style);
        let start = text.len();
        text.push_str(&run.text);
        inline_ranges.push(Some(start..text.len()));
        character += run.text.chars().count();
        if run.text.is_empty() {
          continue;
        }
        let current = Profile::new(&run.text, &run.style, metrics)?;
        if profile.as_ref().is_some_and(|previous| {
          previous.face != current.face || previous.size_pt != current.size_pt
        }) {
          return None;
        }
        width_pt += current.width_pt;
        contextual_blank |= current.contextual_blank;
        profile = Some(current);
      }
    }
    if objects.is_empty()
      || !capacity_pt.is_finite()
      || capacity_pt <= 0.0
      || width_pt <= capacity_pt
      || !contextual_blank
    {
      return None;
    }
    let mut plan = Self::build(
      &text,
      ordinary_style?,
      &paragraph.format.complex_source_restarts,
      &objects,
      capacity_pt,
      profile?.reference,
      metrics,
    )?;
    plan.inline_ranges = inline_ranges;
    Some(plan)
  }

  fn new(
    text: &str,
    style: &TextStyle,
    source_restarts: &[usize],
    capacity_pt: f32,
    metrics: &mut TextMetrics,
  ) -> Option<Self> {
    if !eligible_style(style)
      || !capacity_pt.is_finite()
      || capacity_pt <= 0.0
      || !text.contains(' ')
      || !text.chars().any(|ch| ch.script() == Script::Arabic)
      || !text.chars().all(supported_character)
    {
      return None;
    }
    // Avoid constructing a staging plan for ordinary fonts or a single line.
    // A contextual blank must actually differ from its nominal face advance;
    // a family name or the presence of a GPOS table does not establish this.
    let profile = Profile::new(text, style, metrics)?;
    if profile.width_pt <= capacity_pt || !profile.contextual_blank {
      return None;
    }
    Self::build(
      text,
      style,
      source_restarts,
      &[],
      capacity_pt,
      profile.reference,
      metrics,
    )
  }

  fn build(
    text: &str,
    style: &TextStyle,
    source_restarts: &[usize],
    objects: &[SourceObject],
    capacity_pt: f32,
    reference: i64,
    metrics: &mut TextMetrics,
  ) -> Option<Self> {
    let characters = text.chars().collect::<Vec<_>>();
    let mut offsets = text.char_indices().map(|(at, _)| at).collect::<Vec<_>>();
    offsets.push(text.len());
    let source_ends = source_item_ends(&characters, source_restarts);
    let mut measured_ends = vec![0; source_ends.len()];
    let capacity = ideal(capacity_pt);
    let step = (capacity / 16).max(1);
    let mut changed_lines = Vec::new();
    let mut cursor = 0;
    let mut line = 0;
    let mut nominal_tail = 0;
    while line < characters.len() {
      let mut attempt = 1_i64;
      let mut budget = capacity + step - nominal_tail;
      let mut frozen = line;
      let mut retained = Vec::<i64>::new();
      loop {
        while cursor < characters.len() {
          if let Ok(index) = objects.binary_search_by_key(&cursor, |object| object.character) {
            cursor += 1;
            let fresh = advances(text, &offsets, objects, line, cursor, style, metrics)?;
            update_staged_advances(&mut retained, &fresh, frozen - line);
            frozen = cursor;
            // The styled reference realizes the entire prefix. The next
            // text fetch subtracts that measured prefix, not one nominal
            // source advance per printed digit (or per preceding character).
            debug_assert_eq!(retained.last(), Some(&objects[index].advance));
            budget = capacity
              .checked_add(attempt.checked_mul(step)?)?
              .checked_sub(retained.iter().sum::<i64>())?;
            if budget <= 0 {
              break;
            }
            continue;
          }
          let source_index = source_ends.partition_point(|&end| end <= cursor);
          let source_end = source_ends[source_index];
          let nominal = if matches!(characters[cursor], '\u{200e}' | '\u{200f}') {
            0
          } else {
            reference
          };
          let count = if nominal == 0 {
            1
          } else {
            let estimated = (budget / nominal + READ_AHEAD_CHARACTERS).max(1) as usize;
            let mut count = estimated.min(source_end - cursor);
            // The source estimator includes an adjacent SP sequence in the
            // same portion, rather than leaving it at the next fetch start.
            while cursor + count < source_end && characters[cursor + count] == ' ' {
              count += 1;
            }
            count
          };
          if nominal != 0 && (budget > 0 || characters[cursor + count - 1] == ' ') {
            measured_ends[source_index] = cursor + count;
          }
          cursor += count;
          budget -= nominal * count as i64;
          // A source-limited fetch or terminal SP still needs its following
          // text. Reuse the same estimator even with a negative budget: the
          // follow-up can contain several characters, not just one scalar.
          // A zero-width direction item completes this pending source peek.
          if budget <= 0 && (nominal == 0 || (cursor < source_end && characters[cursor - 1] != ' '))
          {
            break;
          }
        }
        let fresh = advances(text, &offsets, objects, line, cursor, style, metrics)?;
        update_staged_advances(&mut retained, &fresh, frozen - line);
        // Word commits completed words and their following SP. Later source
        // pieces reshape the entire unfinished word: both its final joining
        // glyph and earlier cursive positioning can change. Neither the last
        // fetch fragment nor the last glyph owns that editable range.
        frozen = open_word_start(&characters, line, cursor);
        let width = retained.iter().sum::<i64>();
        if width > capacity || cursor == characters.len() {
          let mut prefix = 0;
          let mut end = None;
          for (i, &advance) in retained.iter().enumerate() {
            // Fitting elides the trailing blank's actual shaped advance.
            if characters[line + i] == ' ' && prefix <= capacity {
              end = Some(line + i + 1);
            }
            prefix += advance;
          }
          let end = if cursor == characters.len() && width <= capacity {
            cursor
          } else {
            end.filter(|&end| end > line)?
          };
          let mut delta = 0;
          let mut deltas = Vec::new();
          for (i, (&cached, &current)) in retained.iter().zip(&fresh).enumerate() {
            if cached != current {
              delta += cached - current;
              deltas.push((offsets[line + i + 1], delta));
            }
          }
          if !deltas.is_empty() {
            changed_lines.push(ChangedLine {
              start: offsets[line],
              end: offsets[end],
              limit: offsets[cursor],
              deltas,
            });
          }
          // The source containing the accepted break owns the reusable
          // measured prefix. Later prefetched sources remain nominal: the
          // last source inspected while rejecting a longer candidate does
          // not replace that owner. Native ordinary-text and styled-note
          // callbacks distinguish these ranges independently of final paint.
          let measured_index = source_ends.partition_point(|&source_end| source_end < end);
          let measured_start = measured_index
            .checked_sub(1)
            .map_or(0, |index| source_ends[index]);
          let measured_source = measured_start..measured_ends[measured_index];
          nominal_tail = characters[end..cursor]
            .iter()
            .enumerate()
            .filter(|(index, ch)| {
              !measured_source.contains(&(end + index))
                && !matches!(ch, '\u{200e}' | '\u{200f}' | '\u{fffc}')
            })
            .count() as i64
            * reference;
          line = end;
          break;
        }
        // The formatter doubles the retry allowance (1,2,4,...). This is
        // independent of whether a source portion ends at a direction item.
        attempt = attempt.checked_mul(2)?;
        budget = capacity
          .checked_add(attempt.checked_mul(step)?)?
          .checked_sub(width)?;
      }
    }
    (!changed_lines.is_empty()).then_some(Self {
      capacity_pt,
      changed_lines,
      inline_ranges: Vec::new(),
    })
  }

  pub(super) fn adjustment(
    &self,
    inline_index: usize,
    mut start: usize,
    mut end: usize,
    capacity_pt: f32,
  ) -> f32 {
    if (capacity_pt - self.capacity_pt).abs() > 1.0 / IDEAL_UNITS_PER_POINT {
      return 0.0;
    }
    let index = if self.inline_ranges.is_empty() {
      let Ok(index) = self
        .changed_lines
        .binary_search_by_key(&start, |line| line.start)
      else {
        return 0.0;
      };
      index
    } else {
      let Some(Some(range)) = self.inline_ranges.get(inline_index) else {
        return 0.0;
      };
      if start > end || end > range.len() {
        return 0.0;
      }
      start += range.start;
      end += range.start;
      let Some(index) = self
        .changed_lines
        .partition_point(|line| line.start <= start)
        .checked_sub(1)
      else {
        return 0.0;
      };
      if start >= self.changed_lines[index].end {
        return 0.0;
      }
      index
    };
    let line = &self.changed_lines[index];
    if end > line.limit {
      return 0.0;
    }
    let index = line.deltas.partition_point(|&(at, _)| at <= end);
    index.checked_sub(1).map_or(0.0, |index| {
      line.deltas[index].1 as f32 / IDEAL_UNITS_PER_POINT
    })
  }
}

impl Profile {
  fn new(text: &str, style: &TextStyle, metrics: &mut TextMetrics) -> Option<Self> {
    let shaped = metrics.shape_text(text, style)?;
    // Repeated shaping entries can name the same resolved face across RLM
    // boundaries. A different face or size needs an independent fit policy.
    let face = shaped.font_faces.first()?;
    if shaped.font_faces.iter().any(|other| other != face)
      || face.synthetic_bold
      || face.synthetic_italic
    {
      return None;
    }
    let size_pt = shaped.glyphs.first()?.font_size_pt;
    if shaped
      .glyphs
      .iter()
      .any(|glyph| glyph.font_size_pt != size_pt)
    {
      return None;
    }
    let font = FontRef::from_index(face.data.as_ref(), face.index).ok()?;
    let glyph_metrics = font.glyph_metrics(Size::new(size_pt), LocationRef::default());
    let space = glyph_metrics.advance_width(font.charmap().map(' ')?)?;
    let contextual_blank = shaped.glyphs.iter().any(|glyph| {
      text.get(glyph.text_range.clone()) == Some(" ")
        && ((glyph.x_advance_em * glyph.font_size_pt - space) * IDEAL_UNITS_PER_POINT).abs() > 1.0
    });
    // Native Word source callbacks use the selected face's x, not OS/2
    // xAvgCharWidth, before the measured run-construction path.
    let reference = ideal(glyph_metrics.advance_width(font.charmap().map('x')?)?);
    (reference > 0).then(|| Self {
      face: face.clone(),
      size_pt,
      reference,
      width_pt: shaped.width_pt,
      contextual_blank,
    })
  }
}

fn eligible_style(style: &TextStyle) -> bool {
  style.wordprocessingml_font_slots
    && style.shaping_context.is_none()
    && style.character_spacing_pt == 0.0
    && style.horizontal_scale.is_none_or(|scale| scale == 1.0)
    && style.wordprocessing_font_width_percent.is_none()
    && style.wordprocessing_kashida.is_none()
}

fn ideal(value: f32) -> i64 {
  (value * IDEAL_UNITS_PER_POINT).round() as i64
}

fn open_word_start(characters: &[char], line: usize, cursor: usize) -> usize {
  characters[line..cursor]
    .iter()
    .rposition(|&ch| matches!(ch, ' ' | '\u{fffc}'))
    .map_or(line, |index| line + index + 1)
}

fn update_staged_advances(retained: &mut Vec<i64>, fresh: &[i64], committed: usize) {
  retained.truncate(committed);
  retained.extend_from_slice(&fresh[committed..]);
}

fn supported_character(ch: char) -> bool {
  // Arabic combining marks belong to the unfinished word and remain editable.
  // The observed source path uses Arabic/Common BMP text with SP word breaks.
  // Mixed scripts, digits, explicit breaks, scopes and other
  // formatting controls keep their existing independent line-fit policies.
  ch.len_utf16() == 1
    && !ch.is_numeric()
    && (matches!(ch, '\u{200e}' | '\u{200f}' | '\u{200c}')
      || (!ch.is_control()
        && !matches!(
          ch,
          '\u{00ad}' | '\u{061c}' | '\u{200b}'..='\u{206f}' | '\u{fffc}'
        )
        && (!ch.is_whitespace() || ch == ' ')
        && (matches!(ch.script(), Script::Arabic | Script::Common)
          || crate::fonts::arabic_nonspacing_mark(ch))))
}

fn source_item_ends(characters: &[char], restarts: &[usize]) -> Vec<usize> {
  let mut ends = Vec::new();
  let mut cursor = 0;
  while cursor < characters.len() {
    if matches!(
      characters[cursor],
      '\u{200e}' | '\u{200f}' | '\u{200c}' | '\u{fffc}'
    ) {
      cursor += 1;
      ends.push(cursor);
      continue;
    }
    let script_end = characters[cursor..]
      .iter()
      .position(|ch| matches!(ch, '\u{200e}' | '\u{200f}' | '\u{200c}' | '\u{fffc}'))
      .map_or(characters.len(), |offset| cursor + offset);
    let source_end = restarts
      .get(restarts.partition_point(|&start| start <= cursor))
      .copied()
      .unwrap_or(characters.len())
      .min(script_end);
    while cursor < source_end {
      cursor = (cursor + SOURCE_ITEM_CHARACTERS)
        .min((cursor / SOURCE_BUFFER_CHARACTERS + 1) * SOURCE_BUFFER_CHARACTERS)
        .min(source_end);
      ends.push(cursor);
    }
  }
  ends
}

fn advances(
  text: &str,
  offsets: &[usize],
  objects: &[SourceObject],
  start: usize,
  end: usize,
  style: &TextStyle,
  metrics: &mut TextMetrics,
) -> Option<Vec<i64>> {
  let mut advances = vec![0; end - start];
  let mut shape_advances = |part_start: usize, part_end: usize, result: &mut [i64]| {
    if part_start == part_end {
      return Some(());
    }
    let byte_start = offsets[part_start];
    let shaped = metrics.shape_text(&text[byte_start..offsets[part_end]], style)?;
    for glyph in shaped.glyphs {
      let index = offsets[part_start..part_end]
        .binary_search(&(byte_start + glyph.text_range.start))
        .ok()?;
      result[part_start - start + index] += ideal(glyph.x_advance_em * glyph.font_size_pt);
    }
    Some(())
  };
  let mut text_start = start;
  for object in objects
    .iter()
    .skip_while(|object| object.character < start)
    .take_while(|object| object.character < end)
  {
    shape_advances(text_start, object.character, &mut advances)?;
    advances[object.character - start] = object.advance;
    text_start = object.character + 1;
  }
  shape_advances(text_start, end, &mut advances)?;
  Some(advances)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn source_items_respect_controls_and_transport_windows() {
    let text = format!("رساله\u{200f}{}", "ا".repeat(600));
    assert_eq!(
      source_item_ends(&text.chars().collect::<Vec<_>>(), &[]),
      [5, 6, 262, 512, 606]
    );
  }

  #[test]
  fn contextual_blank_really_changes_when_the_partial_word_is_extended() {
    let style = TextStyle {
      font_family: Some("Traditional Arabic".into()),
      complex_font_family: Some("Traditional Arabic".into()),
      font_size_pt: 15.0,
      complex_font_size_pt: Some(15.0),
      right_to_left: Some(true),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let mut metrics = TextMetrics::new();
    let blank = |text: &str, metrics: &mut TextMetrics| {
      let shaped = metrics.shape_text(text, &style).unwrap();
      let glyph = shaped
        .glyphs
        .iter()
        .find(|glyph| glyph.text_range.start == text.find(' ').unwrap())
        .unwrap();
      ideal(glyph.x_advance_em * glyph.font_size_pt)
    };
    // Native ideal placement arrays: the first stage ends with a partial
    // Kaf; the extended stage changes the preceding SP from500 to200 units.
    assert_eq!(blank("نوشت ك", &mut metrics), 15000);
    assert_eq!(blank("نوشت كه از ", &mut metrics), 6000);
  }

  #[test]
  fn styled_note_labels_keep_one_source_position_and_their_actual_width() {
    let style = TextStyle {
      font_family: Some("Traditional Arabic".into()),
      complex_font_family: Some("Traditional Arabic".into()),
      font_size_pt: 15.0,
      complex_font_size_pt: Some(15.0),
      right_to_left: Some(true),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let mut note_style = style.clone();
    note_style.font_size_pt = 9.5;
    note_style.complex_font_size_pt = Some(9.5);
    let text = "نوشت\u{fffc} كه ";
    let mut offsets = text.char_indices().map(|(at, _)| at).collect::<Vec<_>>();
    offsets.push(text.len());
    let mut metrics = TextMetrics::new();
    // Actual Word source callbacks for three differently numbered references
    // all consume one source CP, but debit these distinct ideal advances.
    for (label, native_width) in [("1", 19076), ("17", 38152), ("101", 57228)] {
      let width = ideal(metrics.measure_text(label, &note_style));
      assert_eq!(width, native_width);
      let objects = [SourceObject {
        character: 4,
        advance: width,
      }];
      let widths = advances(text, &offsets, &objects, 0, 9, &style, &mut metrics).unwrap();
      assert_eq!(widths.len(), 9);
      assert_eq!(widths[4], native_width);
      assert_eq!(offsets[5] - offsets[4], '\u{fffc}'.len_utf8());
    }
    assert!(!supported_character('\u{fffc}'));
  }

  #[test]
  fn staged_widths_commit_completed_words_and_reshape_the_whole_open_word() {
    let style = TextStyle {
      font_family: Some("Traditional Arabic".into()),
      complex_font_family: Some("Traditional Arabic".into()),
      font_size_pt: 15.0,
      complex_font_size_pt: Some(15.0),
      right_to_left: Some(true),
      bidi_language: Some("fa-IR".into()),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let mut metrics = TextMetrics::new();
    for (before, after, native_delta) in [
      ("نوشت ك", "نوشت كه از ", 9000),
      ("ارح", "ارحم الراحمينى.", 0),
    ] {
      let characters = after.chars().collect::<Vec<_>>();
      let mut offsets = after.char_indices().map(|(at, _)| at).collect::<Vec<_>>();
      offsets.push(after.len());
      let before_count = before.chars().count();
      let old = advances(after, &offsets, &[], 0, before_count, &style, &mut metrics).unwrap();
      let fresh = advances(
        after,
        &offsets,
        &[],
        0,
        characters.len(),
        &style,
        &mut metrics,
      )
      .unwrap();
      // Both native counterexamples have an earlier advance that changes when
      // the word is extended. The first belongs to a completed word's blank;
      // the second belongs to the still-open word's cursive positioning.
      assert!(old[..old.len() - 1].iter().zip(&fresh).any(|(a, b)| a != b));
      let mut cached = old;
      update_staged_advances(
        &mut cached,
        &fresh,
        open_word_start(&characters, 0, before_count),
      );
      assert_eq!(
        cached.iter().sum::<i64>() - fresh.iter().sum::<i64>(),
        native_delta,
        "{before} -> {after}"
      );
    }
    assert!(supported_character('\u{0651}'));
    assert!(supported_character('\u{064e}'));
    assert!(!supported_character('a'));
  }
}
