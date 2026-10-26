use crate::localization::OfficeNumberSeparators;

use super::*;

/// The first separator wins across character runs. Without one, Word's
/// numeric scanner accepts ASCII digits and the format's grouping symbol;
/// other characters end that first number. Native controls distinguish
/// `12abc34`, `12, 34`, and non-ASCII numerals from a whole-span right tab.
pub(super) fn anchor<'a>(
  items: impl Iterator<Item = &'a PageItem>,
  symbols: OfficeNumberSeparators,
  text_metrics: &mut TextMetrics,
) -> Option<f32> {
  let mut number_started = false;
  let mut number_finished = false;
  let mut number_end: Option<(&TextItem, usize)> = None;
  for item in items {
    let PageItem::Text(text) = item else {
      number_finished |= number_started;
      continue;
    };
    for (index, character) in text.text.char_indices() {
      if character == symbols.decimal {
        return Some(prefix_end(text, index, text_metrics));
      }
      if number_finished {
        continue;
      }
      if character.is_ascii_digit() {
        number_started = true;
        number_end = Some((text, index + character.len_utf8()));
      } else if number_started {
        if character == symbols.grouping {
          number_end = Some((text, index + character.len_utf8()));
        } else {
          number_finished = true;
        }
      }
    }
  }
  number_end.map(|(text, end)| prefix_end(text, end, text_metrics))
}

fn prefix_end(text: &TextItem, end: usize, text_metrics: &mut TextMetrics) -> f32 {
  let prefix = &text.text[..end];
  text.x_pt
    + text_metrics.measure_text(prefix, &text.style)
    + prefix.matches(' ').count() as f32 * text.word_spacing_pt
}

pub(super) struct ImplicitLine {
  pub active: ActiveTextFrame,
  pub item_start: usize,
  pub frame: LineFrame,
}

pub(super) fn settle_implicit(
  paragraph: &crate::docx::Paragraph,
  page: &mut Page,
  state: &mut TextFrameState,
  line: ImplicitLine,
  text_metrics: &mut TextMetrics,
  at_paragraph_end: bool,
) -> f32 {
  if state.implicit_decimal_tab_done
    || line.active.flow.text_segmentation != TextSegmentation::TableCell
  {
    return 0.0;
  }
  let [stop] = paragraph.format.tab_stops.as_slice() else {
    return 0.0;
  };
  if !matches!(stop.alignment, TabStopAlignment::Decimal(_)) {
    return 0.0;
  }
  // [MS-OI29500] §2.1.556(d): every cell line receives the implicit tab,
  // even before an authored leading tab. Keep source inline/bookmark/field
  // identities intact and settle the measured initial span once per line.
  state.implicit_decimal_tab_done = true;
  if paragraph.format.bidi {
    // Retain the virtual boundary until the completed line reflects its
    // independently ordered tab spans around the paragraph's logical axis.
    state.bidi_tab_starts.push(line.item_start);
  }
  let first_right = state
    .first_line_alignment_right
    .filter(|_| state.line_fragments.is_empty());
  let offset = if paragraph.format.bidi {
    rtl_tab_coordinate_offset(line.active.frame, first_right)
  } else {
    0.0
  };
  let x_pt = line.active.flow.content_left_pt - offset + stop.position_pt;
  if line.item_start == page.items.len() {
    // An explicit leading tab follows this empty virtual span and therefore
    // selects the next stop, rather than consuming the decimal stop twice.
    return (x_pt - line.frame.x_pt).max(0.0);
  }
  let mut pending = Some(PendingAlignedTab {
    stop: ResolvedTabStop {
      x_pt,
      alignment: stop.alignment,
      // Native implicit tabs do not paint leaders. Writer independently
      // gives SwAutoTabDecimalPortion::Paint an empty implementation.
      leader: TabLeader::None,
      automatic: false,
    },
    leader_start_x_pt: line.frame.left_pt,
    leader_layout: aligned_tab_leader_layout(line.active.frame.compatibility_mode),
    y_pt: line.frame.y_pt,
    line_height_pt: line.frame.height_pt,
    style: paragraph.base_style.clone(),
    item_start_index: line.item_start,
    field_result_tab: false,
    paragraph_bidi: paragraph.format.bidi,
  });
  apply_pending_aligned_tab_in_context(
    page,
    &mut pending,
    text_metrics,
    line.frame.y_pt,
    line.frame.left_pt,
    line.frame.right_pt,
    at_paragraph_end,
  )
}
