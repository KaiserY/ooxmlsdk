//! Allocate a kept body suffix before committing the newly referenced notes.
//!
//! Notes use the ordinary block formatter, including its keep/widow decisions.
//! Their follow frames are installed before the next body's first line; they
//! are not extra physical pages inserted into an already formatted body.

use super::*;

#[derive(Clone, Debug)]
pub(super) struct FootnotePlan {
  pub ids: Vec<i64>,
  pub master: Arc<FootnotePage>,
}

#[derive(Clone, Debug)]
pub(super) struct FootnotePage {
  pub height_pt: f32,
  top_pt: f32,
  body_bottom_pt: f32,
  items: Vec<PageItem>,
  fragments: Vec<FrameFragment>,
  influences: Vec<FrameInfluence>,
  follow: Option<Arc<FootnotePage>>,
}

pub(super) fn plan_for_block<'a>(
  page: &'a Page,
  block: &Block,
  document: &DocxDocument,
  emitted: &HashSet<i64>,
) -> Option<&'a FootnotePlan> {
  if page.footnote_plans.is_empty() {
    return None;
  }
  let ids = pending_block_footnote_ids(block, document, emitted);
  page.footnote_plans.iter().find(|plan| plan.ids == ids)
}

pub(super) fn plan_kept_body_suffix(
  root: &mut RootFrameLayout<'_>,
  index: usize,
  ids: &[i64],
  flow: FlowContext,
) {
  if flow.columns.count != 1
    || flow.text_segmentation != TextSegmentation::Body
    || !root.current.pending_footnote_follows.is_empty()
    || root
      .current
      .footnote_plans
      .iter()
      .any(|plan| plan.ids == ids)
  {
    return;
  }
  let blocks = root
    .document
    .sections
    .get(flow.section_index)
    .map_or(root.document.blocks.as_slice(), |section| {
      section.blocks.as_slice()
    });
  let mut end = index;
  while matches!((blocks.get(end), blocks.get(end + 1)),
    (Some(Block::Paragraph(p)), Some(Block::Paragraph(_)))
      if p.format.keep_with_next && p.format.frame.is_none())
  {
    end += 1;
  }
  if end == index {
    return;
  }
  let mut height = 0.0;
  for i in index..end {
    let Block::Paragraph(paragraph) = &blocks[i] else {
      return;
    };
    height +=
      paragraph_spacing_before(
        i.checked_sub(1).and_then(|j| blocks.get(j)),
        paragraph,
        flow,
      ) + estimated_keep_chain_paragraph_content_height(paragraph, flow, &mut root.text_metrics)
        + paragraph_spacing_after(paragraph, blocks.get(i + 1), flow);
  }
  let Block::Paragraph(terminal) = &blocks[end] else {
    return;
  };
  height += (estimated_paragraph_keep_start_height(terminal, flow, &mut root.text_metrics)
    - paragraph_upper_space(terminal, flow.setup.doc_grid_line_pitch_pt)
    + paragraph_spacing_before(blocks.get(end - 1), terminal, flow))
  .max(0.0);
  let available =
    flow.body_content_bottom_pt - root.current.footnote_reserved_height_pt - root.y - height;
  // An overfull keep chain can flow across pages. It must not sacrifice notes
  // for a suffix that still cannot fit on this page (ECMA-376 17.3.1.15).
  if available < 0.0 {
    return;
  }
  let include_separator = root.current.footnote_reserved_height_pt <= LAYOUT_EPSILON_PT;
  let preceding = (!include_separator)
    .then(|| last_emitted_footnote_block(root.document, &root.emitted_footnote_order))
    .flatten();
  let stories = ids
    .iter()
    .map(|id| root.document.footnotes[id].as_slice())
    .collect::<Vec<_>>();
  let full_height = measured_note_stories_height(&stories, preceding, flow, &mut root.text_metrics)
    + if include_separator {
      footnote_separator_layout_height(root.document, flow, &mut root.text_metrics)
    } else {
      0.0
    };
  if full_height <= available + LAYOUT_EPSILON_PT {
    return;
  }
  if let Some(master) = format_pages(
    root.document,
    &stories,
    preceding,
    flow,
    FootnoteMasterSpace {
      page_index: root.pages.len(),
      available,
      include_separator,
    },
    &mut root.text_metrics,
  ) {
    root.current.footnote_plans.push(FootnotePlan {
      ids: ids.to_vec(),
      master,
    });
  }
}

pub(super) fn plan_reference_prefix(
  root: &mut RootFrameLayout<'_>,
  ids: &[i64],
  checkpoint: LayoutCheckpoint,
  body_prefix_bottom_pt: f32,
) -> Option<FootnotePlan> {
  let flow = checkpoint.flow;
  let existing = checkpoint.current.footnote_reserved_height_pt;
  let available = flow.body_content_bottom_pt - existing - body_prefix_bottom_pt;
  if available <= LAYOUT_EPSILON_PT {
    return None;
  }
  let include_separator = existing <= LAYOUT_EPSILON_PT;
  let preceding = (!include_separator)
    .then(|| {
      last_emitted_footnote_block(
        root.document,
        &root.emitted_footnote_order[..checkpoint.emitted_footnotes_len],
      )
    })
    .flatten();
  let stories = ids
    .iter()
    .map(|id| root.document.footnotes[id].as_slice())
    .collect::<Vec<_>>();
  let full_height = measured_note_stories_height(&stories, preceding, flow, &mut root.text_metrics)
    + if include_separator {
      footnote_separator_layout_height(root.document, flow, &mut root.text_metrics)
    } else {
      0.0
    };
  if full_height <= available + LAYOUT_EPSILON_PT {
    return None;
  }
  // Use the ordinary note formatter's keep/widow rules to determine whether
  // a real note prefix fits beside the callout. An empty master is not proof
  // that the callout may stay; its ordinary paragraph ownership path decides.
  let master = format_pages(
    root.document,
    &stories,
    preceding,
    flow,
    FootnoteMasterSpace {
      page_index: checkpoint.page_index,
      available,
      include_separator,
    },
    &mut root.text_metrics,
  )?;
  (master.height_pt > LAYOUT_EPSILON_PT).then(|| FootnotePlan {
    ids: ids.to_vec(),
    master,
  })
}

struct FootnoteMasterSpace {
  page_index: usize,
  available: f32,
  include_separator: bool,
}

fn format_pages<'a>(
  document: &DocxDocument,
  stories: &[&'a [Block]],
  mut preceding: Option<&'a Block>,
  flow: FlowContext,
  space: FootnoteMasterSpace,
  metrics: &mut TextMetrics,
) -> Option<Arc<FootnotePage>> {
  let FootnoteMasterSpace {
    page_index,
    available,
    include_separator,
  } = space;
  let separator_height = footnote_separator_layout_height(document, flow, metrics);
  let first_top = flow.body_content_bottom_pt - available;
  let left = flow.setup.margin_left_pt;
  let width = flow.setup.width_pt - left - flow.setup.margin_right_pt;
  let mut note_flow = note_story_flow(FlowContext {
    content_left_pt: left,
    content_width: width,
    content_bottom: flow.body_content_bottom_pt,
    columns: SectionColumns::default(),
    column_index: 0,
    note_continuation_top_inset_pt: separator_height,
    ..flow
  });
  let mut current = empty_section_page(flow.setup, flow.section_index, flow.section_page_index);
  // Retain the physical page number for odd/even repeating-slot geometry.
  let mut pages = (0..page_index)
    .map(|_| empty_page(flow.setup, flow.section_index))
    .collect::<Vec<_>>();
  let mut y = first_top;
  if include_separator {
    y = format_separator(document, note_flow, &mut current, &mut pages, metrics, y);
  }
  let separator_items = current.items.len();
  for blocks in stories {
    for (i, block) in blocks.iter().enumerate() {
      let previous = i.checked_sub(1).and_then(|j| blocks.get(j)).or(preceding);
      let first_page = pages.len();
      let first_fragment = current.frame_fragments.len();
      let (next_flow, next_y) = layout_document_block(
        previous,
        block,
        blocks.get(i + 1),
        note_flow,
        LayoutBlockTarget {
          current: &mut current,
          pages: &mut pages,
          anchor_pages: None,
          text_metrics: metrics,
          paragraph_decoration_outer_bottom_pt: None,
        },
        y,
      );
      if let Block::Paragraph(paragraph) = block {
        for (index, page) in pages.iter_mut().enumerate().skip(first_page) {
          let fragment_start = if index == first_page {
            first_fragment
          } else {
            0
          };
          let advance = page.frame_fragments[fragment_start..]
            .iter()
            .filter(|fragment| fragment.kind == FrameFragmentKind::ParagraphLine)
            .filter_map(|fragment| {
              fragment
                .content_advance_pt
                .or(fragment.bounds.map(|b| b.height_pt))
            })
            .sum::<f32>();
          let top = if index == first_page {
            y + paragraph_spacing_before(previous, paragraph, note_flow)
              + paragraph_border_layout_extent(
                ParagraphBorderContext::for_blocks(previous, paragraph, blocks.get(i + 1))
                  .top(paragraph),
              )
          } else {
            body_flow_for_page(
              FlowContext {
                section_page_index: page.section_page_index,
                ..flow
              },
              index + 1,
            )
            .content_top_pt
              + separator_height
          };
          // Exact-line glyph alignment can shift fragment bounds without
          // consuming flow. Keep the allocated line advances as the cursor.
          page.footnote_flow_bottom_pt = Some(top + advance);
        }
      }
      note_flow = FlowContext {
        content_left_pt: left,
        content_width: width,
        ..next_flow
      };
      y = next_y;
      current.footnote_flow_bottom_pt = Some(y);
      preceding = Some(block);
    }
  }
  pages.push(current);
  let formatted = pages.split_off(page_index);
  if formatted.len() <= 1 {
    return None;
  }
  let mut follow = None;
  for (i, mut page) in formatted.into_iter().enumerate().rev() {
    let page_flow = body_flow_for_page(
      FlowContext {
        section_page_index: page.section_page_index,
        ..flow
      },
      page_index + i + 1,
    );
    let top = if i == 0 {
      first_top
    } else {
      page_flow.content_top_pt
    };
    let bottom = page.footnote_flow_bottom_pt.unwrap_or_else(|| {
      page
        .frame_fragments
        .iter()
        .filter_map(|fragment| {
          fragment
            .bounds
            .map(|bounds| bounds.y_pt + fragment.content_advance_pt.unwrap_or(bounds.height_pt))
        })
        .fold(top, f32::max)
    });
    let empty_master = i == 0 && page.items.len() == separator_items;
    if empty_master {
      page.items.clear();
      page.frame_fragments.clear();
      page.frame_influences.clear();
    } else if i > 0 {
      // Native keep-chain controls repeat the ordinary footnote separator,
      // including its formatting, on this independently allocated note boss.
      let mut discarded = Vec::new();
      format_separator(
        document,
        note_story_flow(page_flow),
        &mut page,
        &mut discarded,
        metrics,
        top,
      );
      if !discarded.is_empty() {
        return None;
      }
    }
    let height = if empty_master {
      0.0
    } else {
      (bottom - top).max(0.0)
    };
    if i == 0 && height > available + LAYOUT_EPSILON_PT {
      return None;
    }
    follow = Some(Arc::new(FootnotePage {
      height_pt: height,
      top_pt: top,
      body_bottom_pt: page_flow.body_content_bottom_pt,
      items: page.items,
      fragments: page.frame_fragments,
      influences: page.frame_influences,
      follow,
    }));
  }
  follow
}

fn format_separator(
  document: &DocxDocument,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  metrics: &mut TextMetrics,
  y: f32,
) -> f32 {
  if document.footnote_separator_stories.separator.is_empty() {
    layout_footnote_separator(flow.setup, current, pages, y, flow.body_content_bottom_pt)
  } else {
    layout_note_story_blocks(
      &document.footnote_separator_stories.separator,
      flow,
      current,
      pages,
      metrics,
      y,
    )
  }
}

pub(super) fn append_page(current: &mut Page, note: &FootnotePage, bottom: f32) {
  if note.height_pt > LAYOUT_EPSILON_PT {
    shift_existing_page_footnotes(current, -note.height_pt);
    let top = bottom - note.height_pt;
    if current.footnote_reserved_height_pt <= LAYOUT_EPSILON_PT {
      current.footnote_area_top_pt = Some(top);
    }
    let offset = current.items.len();
    let dy = top - note.top_pt;
    for mut item in note.items.clone() {
      shift_page_item_y(&mut item, dy);
      current.items.push(item);
    }
    current
      .footnote_item_indices
      .extend(offset..current.items.len());
    current
      .frame_fragments
      .extend(note.fragments.iter().cloned().map(|fragment| {
        let mut fragment = translate_frame_fragment(fragment, 0.0, dy);
        fragment.item_start += offset;
        fragment.item_end += offset;
        if fragment.kind == FrameFragmentKind::ParagraphLine {
          fragment.kind = FrameFragmentKind::NoteLine;
        }
        fragment
      }));
    current
      .frame_influences
      .extend(note.influences.iter().cloned().map(|mut influence| {
        influence.item_start += offset;
        influence.item_end += offset;
        influence.bounds = influence
          .bounds
          .map(|bounds| translate_frame_bounds(bounds, 0.0, dy));
        influence
      }));
    current.footnote_reserved_height_pt += note.height_pt;
    // A planned continuation owns its complete fragment; an earlier note
    // no longer supplies the terminal paragraph spacing on this page.
    current.footnote_reclaimable_spacing_after_pt = 0.0;
    current.footnote_flow_bottom_pt = Some(bottom);
  }
  if let Some(follow) = &note.follow {
    current.pending_footnote_follows.push(follow.clone());
  }
}

pub(super) fn activate(current: &mut Page, pages: &[Page], bottom: f32) {
  let Some(previous) = pages.last() else { return };
  for follow in &previous.pending_footnote_follows {
    append_page(current, follow, bottom);
  }
}

pub(super) fn finish_pending_pages(pages: &mut Vec<Page>) {
  while let Some(last) = pages
    .last()
    .filter(|page| !page.pending_footnote_follows.is_empty())
  {
    let mut next = empty_section_page(last.setup, last.section_index, last.section_page_index + 1);
    for note in &last.pending_footnote_follows {
      append_page(&mut next, note, note.body_bottom_pt);
    }
    pages.push(next);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::{Paragraph, ParagraphFormat};

  fn paragraph(text: &str, keep: bool, widow: bool) -> Block {
    let run = TextRun {
      text: text.into(),
      style: TextStyle::default(),
      hyperlink_url: Some("ooxmlsdk-pdf:footnote-backlink:8".into()),
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    };
    Block::paragraph(Paragraph {
      inlines: vec![InlineItem::Text(run.clone())],
      runs: vec![run],
      field_events: Vec::new(),
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: TextStyle::default(),
      format: Box::new(ParagraphFormat {
        line_height_rule: LineHeightRule::Exact,
        line_height_pt: Some(18.0),
        keep_lines: keep,
        widow_control: Some(widow),
        ..Default::default()
      }),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_image: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    })
  }

  fn document() -> DocxDocument {
    DocxDocument {
      page: PageSetup {
        width_pt: 360.0,
        height_pt: 300.0,
        margin_top_pt: 36.0,
        margin_bottom_pt: 36.0,
        margin_left_pt: 36.0,
        margin_right_pt: 36.0,
        ..Default::default()
      },
      page_background_pattern: None,
      page_background_texture: None,
      line_number_style: TextStyle::default(),
      note_separator_style: TextStyle::default(),
      footnote_separator_stories: crate::docx::NoteSeparatorStories {
        separator: vec![paragraph("SEP", false, false)],
        ..Default::default()
      },
      endnote_separator_stories: Default::default(),
      default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
      hyphenation: Default::default(),
      compatibility_mode: 15,
      justify_lines_with_shrinking: false,
      do_not_expand_shift_return: false,
      suppress_top_spacing: false,
      even_and_odd_headers: false,
      split_page_break_and_paragraph_mark: false,
      form_widgets: Vec::new(),
      sections: Vec::new(),
      header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      footnote_blocks: Vec::new(),
      footnotes: Default::default(),
      footnote_numbering: Vec::new(),
      footnote_positions: Vec::new(),
      endnotes: Default::default(),
      endnote_numbering: Vec::new(),
      endnote_position: w::EndnotePositionValues::DocumentEnd,
      title_page: false,
      blocks: Vec::new(),
    }
  }

  fn text(items: &[PageItem]) -> String {
    items
      .iter()
      .filter_map(|item| match item {
        PageItem::Text(text) => Some(text.text.as_str()),
        _ => None,
      })
      .collect()
  }

  #[test]
  fn footnote_follow_retains_paragraph_indents_and_wrapped_text() {
    let document = document();
    let flow = flow_context(
      document.page,
      0,
      SectionColumns::default(),
      0,
      0,
      DEFAULT_TAB_STOP_PT,
    );
    let words = "alpha beta gamma delta epsilon zeta eta theta iota kappa lambda ".repeat(12);
    for (left, right) in [(0.0, 18.0), (18.0, 36.0), (54.0, 54.0)] {
      let mut note = paragraph(&words, false, true);
      if let Block::Paragraph(paragraph) = &mut note {
        paragraph.format.indent_left_pt = left;
        paragraph.format.indent_right_pt = right;
      }
      let story = vec![note];
      let mut metrics = TextMetrics::new();
      let plan = format_pages(
        &document,
        &[&story],
        None,
        flow,
        FootnoteMasterSpace {
          page_index: 0,
          available: 54.0,
          include_separator: true,
        },
        &mut metrics,
      )
      .expect("wrapped note has a continuation");
      assert!(plan.height_pt > 0.0);
      let mut current = Some(plan.as_ref());
      let mut combined = String::new();
      let mut note_pages = 0;
      while let Some(page) = current {
        note_pages += 1;
        for item in &page.items {
          if let PageItem::Text(text) = item {
            if text.text == "SEP" {
              continue;
            }
            combined.push_str(&text.text);
            let bounds = item_bounds(item, &mut metrics).expect("text has bounds");
            assert!(bounds.0 >= document.page.margin_left_pt + left - LAYOUT_EPSILON_PT);
            assert!(
              bounds.2
                <= document.page.width_pt - document.page.margin_right_pt - right
                  + LAYOUT_EPSILON_PT,
              "indents=({left}, {right}), page={note_pages}, right={}",
              bounds.2
            );
          }
        }
        current = page.follow.as_deref();
      }
      assert!(note_pages >= 2);
      assert_eq!(
        combined
          .chars()
          .filter(|ch| !ch.is_whitespace())
          .collect::<String>(),
        words
          .chars()
          .filter(|ch| !ch.is_whitespace())
          .collect::<String>(),
      );
    }
  }

  #[test]
  fn footnote_follow_preserves_legal_line_splits_and_reserves_separator() {
    let document = document();
    let flow = flow_context(
      document.page,
      0,
      SectionColumns::default(),
      0,
      0,
      DEFAULT_TAB_STOP_PT,
    );
    let mut metrics = TextMetrics::new();
    for (lines, keep, widow, available, master_lines) in [
      (2, false, false, 27.0, 1),
      (2, true, false, 27.0, 0),
      (2, false, true, 27.0, 0),
      (3, false, false, 27.0, 1),
      (4, false, true, 45.0, 2),
      (4, false, true, 27.0, 0),
    ] {
      let words = (0..lines).map(|i| format!("NOTE{i}")).collect::<Vec<_>>();
      let story = vec![paragraph(&words.join("\n"), keep, widow)];
      let plan = format_pages(
        &document,
        &[&story],
        None,
        flow,
        FootnoteMasterSpace {
          page_index: 0,
          available,
          include_separator: false,
        },
        &mut metrics,
      )
      .unwrap();
      assert_eq!(text(&plan.items), words[..master_lines].join(""));
      assert!(
        (plan.height_pt - master_lines as f32 * 18.0).abs() < 0.01,
        "master: lines={lines} keep={keep} widow={widow} height={}",
        plan.height_pt
      );
      let follow = plan.follow.as_ref().unwrap();
      assert_eq!(
        text(&follow.items),
        format!("{}SEP", words[master_lines..].join(""))
      );
      assert!(
        (follow.height_pt - (lines - master_lines + 1) as f32 * 18.0).abs() < 0.01,
        "follow: lines={lines} keep={keep} widow={widow} height={}",
        follow.height_pt
      );
      assert!(follow.follow.is_none());
      let mut current = empty_page(document.page, 0);
      append_page(&mut current, &plan, flow.body_content_bottom_pt);
      let mut pages = Vec::new();
      let (next, _) = advance_section_flow(flow, &mut current, &mut pages);
      assert!(
        (next.content_bottom - (next.body_content_bottom_pt - follow.height_pt)).abs() < 0.01
      );
      assert_eq!(text(&current.items), text(&follow.items));
    }
  }

  #[test]
  fn footnote_follow_survives_body_replay_and_terminal_page_materialization() {
    let document = document();
    let flow = flow_context(
      document.page,
      0,
      SectionColumns::default(),
      0,
      0,
      DEFAULT_TAB_STOP_PT,
    );
    let story = vec![paragraph("MASTER\nFOLLOW", false, false)];
    let plan = format_pages(
      &document,
      &[&story],
      None,
      flow,
      FootnoteMasterSpace {
        page_index: 0,
        available: 27.0,
        include_separator: false,
      },
      &mut TextMetrics::new(),
    )
    .unwrap();
    let mut current = empty_page(document.page, 0);
    append_page(&mut current, &plan, flow.body_content_bottom_pt);
    let checkpoint = page_checkpoint(&current);
    let first_y = item_vertical_bounds(&current.items[0]).0;
    append_page(&mut current, &plan, flow.body_content_bottom_pt);
    restore_page_checkpoint(&mut current, checkpoint);
    assert!((item_vertical_bounds(&current.items[0]).0 - first_y).abs() < 0.001);
    assert_eq!(current.pending_footnote_follows.len(), 1);
    let mut pages = Vec::new();
    advance_section_flow(flow, &mut current, &mut pages);
    let expected = text(&current.items);
    restore_text_frame_start_page(&mut current, &mut pages, 0, checkpoint, None, None);
    assert_eq!(text(&current.items), "MASTER");
    force_page_break(flow, &mut current, &mut pages);
    assert_eq!(text(&current.items), expected);
    assert_eq!(current.footnote_reserved_height_pt, 36.0);
    let mut terminal = pages;
    finish_pending_pages(&mut terminal);
    assert_eq!(terminal.len(), 2);
    assert_eq!(text(&terminal[1].items), expected);
    finish_pending_pages(&mut terminal);
    assert_eq!(terminal.len(), 2);
  }
}
