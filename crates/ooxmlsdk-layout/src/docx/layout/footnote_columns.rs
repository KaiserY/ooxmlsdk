//! Word 2013 page footnotes have their own balanced column frame.
//!
//! The first reference on a page supplies its automatic column geometry;
//! later continuous sections do not change that frame. Keep immutable plans
//! through body checkpoints, then materialize the selected plan once. Replacing
//! already painted notes during each reference would lose checkpoint ownership.

use super::*;

#[derive(Debug)]
pub(super) struct Plan {
  ids: Vec<i64>,
  pub height_pt: f32,
  items: Vec<PageItem>,
  fragments: Vec<FrameFragment>,
  influences: Vec<FrameInfluence>,
}

fn columns(document: &DocxDocument, page: &Page, flow: FlowContext) -> SectionColumns {
  if flow.compatibility_mode < 15 {
    return SectionColumns::default();
  }
  // MS-DOCX 2.5.1.8: zero/omitted means the displayed section layout. Native
  // independent first/last-reference controls identify the first reference's
  // section, rather than the first body section or the page's last section.
  let owner = page.footnote_column_owner.unwrap_or(flow.section_index);
  let automatic = document
    .sections
    .get(owner)
    .map_or(flow.columns, |section| section.columns);
  let explicit = document
    .sections
    .get(owner)
    .and_then(|section| section.section_properties.as_ref())
    .and_then(|properties| properties.footnote_columns.as_ref())
    .map_or(0, |columns| columns.val);
  let mut columns = if explicit > 0 {
    // Native LayoutColumns 2/3/4 controls use the standard 36pt gap, not
    // the body columns' authored spacing. Like automatic geometry, the first
    // reference's section owns the option. Other continuous sections cannot
    // replace it, even when their COM FootnoteOptions reads a document value.
    SectionColumns {
      count: (explicit as usize).min(45),
      ..SectionColumns::default()
    }
  } else {
    automatic
  };
  columns.separator = false;
  columns.unbalanced = false;
  columns.balanced_height_pt = None;
  columns.pending_balance = None;
  columns.completed_content_height_pt = 0.0;
  columns.completed_column_max_height_pt = 0.0;
  columns.authored_column_break_on_page = false;
  columns
}

pub(super) fn uses_columns(document: &DocxDocument, page: &Page, flow: FlowContext) -> bool {
  columns(document, page, flow).count > 1
}

fn all_ids(page: &Page, pending: &[i64]) -> Vec<i64> {
  let mut ids = page
    .footnote_column_plans
    .last()
    .map_or_else(Vec::new, |plan| plan.ids.clone());
  for &id in pending {
    if !ids.contains(&id) {
      ids.push(id);
    }
  }
  ids
}

pub(super) fn prepare(
  document: &DocxDocument,
  page: &mut Page,
  block: &Block,
  emitted: &HashSet<i64>,
  flow: FlowContext,
  metrics: &mut TextMetrics,
) {
  page.prepared_column_footnotes = None;
  if document.footnotes.is_empty() || !uses_columns(document, page, flow) {
    return;
  }
  let pending = pending_block_footnote_ids(block, document, emitted);
  if pending.is_empty() {
    return;
  }
  page.prepared_column_footnotes = build(
    document,
    all_ids(page, &pending),
    columns(document, page, flow),
    flow,
    metrics,
  );
}

pub(super) fn commit(
  pending: &[i64],
  document: &DocxDocument,
  page: &mut Page,
  flow: FlowContext,
  metrics: &mut TextMetrics,
) -> bool {
  if !uses_columns(document, page, flow) {
    return false;
  }
  let ids = all_ids(page, pending);
  let plan = page
    .prepared_column_footnotes
    .take()
    .filter(|plan| plan.ids == ids)
    .or_else(|| build(document, ids, columns(document, page, flow), flow, metrics));
  let Some(plan) = plan else { return false };
  page.footnote_column_owner.get_or_insert(flow.section_index);
  page.footnote_reserved_height_pt = plan.height_pt;
  page.footnote_reclaimable_spacing_after_pt = 0.0;
  page.footnote_area_top_pt = Some(flow.body_content_bottom_pt - plan.height_pt);
  page.footnote_flow_bottom_pt = Some(flow.body_content_bottom_pt);
  page.footnote_column_plans.push(plan);
  true
}

struct Trial {
  page: Page,
  height_pt: f32,
  fits: bool,
}

fn trial(
  document: &DocxDocument,
  ids: &[i64],
  columns: SectionColumns,
  flow: FlowContext,
  limit: f32,
  metrics: &mut TextMetrics,
) -> Trial {
  let setup = PageSetup {
    height_pt: MEASURE_SCRATCH_PAGE_HEIGHT_PT,
    margin_top_pt: 0.0,
    margin_bottom_pt: 0.0,
    ..flow.setup
  };
  let mut page = empty_section_page(setup, flow.section_index, flow.section_page_index);
  let mut pages = Vec::new();
  let mut note_flow = flow_with_column(
    footnote_story_flow(FlowContext {
      setup,
      columns,
      column_index: 0,
      content_top_pt: 0.0,
      content_bottom: limit,
      body_content_bottom_pt: limit,
      repeating_slots: RepeatingSlotState::default(),
      ..flow
    }),
    0,
  );
  let mut y = 0.0;
  let mut height = 0.0_f32;
  let mut previous = None;
  let mut spacing_previous = None;
  for id in ids {
    let blocks = &document.footnotes[id];
    for (index, block) in blocks.iter().enumerate() {
      let mut neighbors = BlockNeighbors::for_index(blocks, index).with_boundaries(previous, None);
      neighbors.spacing_previous = neighbors.spacing_previous.or(spacing_previous);
      let (next_flow, next_y) = layout_document_block(
        neighbors,
        block,
        note_flow,
        LayoutBlockTarget {
          current: &mut page,
          pages: &mut pages,
          anchor_pages: None,
          text_metrics: metrics,
          paragraph_decoration_outer_bottom_pt: None,
        },
        y,
      );
      note_flow = next_flow;
      y = next_y;
      height = height.max(y);
      if !pages.is_empty() {
        return Trial {
          page,
          height_pt: height,
          fits: false,
        };
      }
      previous = Some(block);
      if !block_is_paragraph_frame(block) {
        spacing_previous = Some(block);
      }
    }
  }
  // A wrapped paragraph can cross several columns in one block. Its source
  // line origins/advances retain earlier columns' true ends, independently of
  // glyph baseline shifts and the final column's shorter cursor.
  for fragment in &page.frame_fragments {
    if let (Some(top), Some(advance)) = (fragment.flow_top_pt, fragment.content_advance_pt) {
      height = height.max(top + advance + fragment.paragraph_spacing_after_pt);
    }
  }
  Trial {
    page,
    height_pt: height,
    fits: height <= limit + LAYOUT_EPSILON_PT,
  }
}

fn build(
  document: &DocxDocument,
  ids: Vec<i64>,
  columns: SectionColumns,
  flow: FlowContext,
  metrics: &mut TextMetrics,
) -> Option<Arc<Plan>> {
  let measured = trial(
    document,
    &ids,
    columns,
    flow,
    MEASURE_SCRATCH_PAGE_HEIGHT_PT * 0.5,
    metrics,
  );
  if !measured.fits || !measured.height_pt.is_finite() || measured.height_pt <= 0.0 {
    return None;
  }
  // As in FormatWidthCols, test ordinary layout between finite fit bounds.
  // The ordinary formatter owns wrapping/keep/widow admission. No per-note
  // font-height approximation or generated-text translation chooses breaks.
  let mut lower = 0.0;
  let mut upper = measured.height_pt;
  let mut best = measured;
  for _ in 0..32 {
    let middle = lower + (upper - lower) * 0.5;
    if middle <= lower || middle >= upper {
      break;
    }
    let candidate = trial(document, &ids, columns, flow, middle, metrics);
    if candidate.fits {
      upper = candidate.height_pt.min(middle);
      best = candidate;
    } else {
      lower = middle;
    }
  }
  let separator_height = footnote_separator_layout_height(document, flow, metrics);
  let height = best.height_pt + separator_height;
  if height > flow.body_content_bottom_pt - flow.setup.margin_top_pt {
    // Independent physical-page note follows retain their established owner.
    // This balanced page plan only replaces complete areas that fit a page.
    return None;
  }
  let mut separator = empty_section_page(flow.setup, flow.section_index, flow.section_page_index);
  let mut unused_pages = Vec::new();
  let separator_flow = flow_with_column(
    footnote_story_flow(FlowContext {
      columns,
      column_index: 0,
      content_top_pt: 0.0,
      content_bottom: MEASURE_SCRATCH_PAGE_HEIGHT_PT,
      body_content_bottom_pt: MEASURE_SCRATCH_PAGE_HEIGHT_PT,
      ..flow
    }),
    0,
  );
  if document.footnote_separator_stories.separator.is_empty() {
    let setup = PageSetup {
      width_pt: separator_flow.content_left_pt
        + separator_flow.content_width
        + flow.setup.margin_right_pt,
      ..flow.setup
    };
    layout_footnote_separator(
      setup,
      &mut separator,
      &mut unused_pages,
      0.0,
      separator_flow.content_bottom,
    );
  } else {
    layout_note_story_blocks(
      &document.footnote_separator_stories.separator,
      separator_flow,
      &mut separator,
      &mut unused_pages,
      metrics,
      0.0,
    );
  }
  let offset = separator.items.len();
  for mut item in best.page.items {
    shift_page_item_y(&mut item, separator_height);
    separator.items.push(item);
  }
  separator
    .frame_fragments
    .extend(best.page.frame_fragments.into_iter().map(|fragment| {
      let mut fragment = translate_frame_fragment(fragment, 0.0, separator_height);
      fragment.item_start += offset;
      fragment.item_end += offset;
      fragment
    }));
  separator
    .frame_influences
    .extend(best.page.frame_influences.into_iter().map(|mut influence| {
      influence.item_start += offset;
      influence.item_end += offset;
      influence.bounds = influence
        .bounds
        .map(|bounds| translate_frame_bounds(bounds, 0.0, separator_height));
      influence
    }));
  Some(Arc::new(Plan {
    ids,
    height_pt: height,
    items: separator.items,
    fragments: separator.frame_fragments,
    influences: separator.frame_influences,
  }))
}

pub(super) fn materialize(document: &DocxDocument, pages: &mut [Page], metrics: &mut TextMetrics) {
  for (index, page) in pages.iter_mut().enumerate() {
    page.prepared_column_footnotes = None;
    let Some(plan) = page.footnote_column_plans.last().cloned() else {
      continue;
    };
    let slots = repeating_slot_state(document, page.section_index, metrics);
    let (_, bottom) =
      body_content_limits_for_page(page.setup, slots, index + 1, page.section_page_index);
    let top = bottom - plan.height_pt;
    let offset = page.items.len();
    for mut item in plan.items.clone() {
      shift_page_item_y(&mut item, top);
      page.items.push(item);
    }
    page.footnote_item_indices.extend(offset..page.items.len());
    page
      .frame_fragments
      .extend(plan.fragments.iter().cloned().map(|fragment| {
        let mut fragment = translate_frame_fragment(fragment, 0.0, top);
        fragment.item_start += offset;
        fragment.item_end += offset;
        if fragment.kind == FrameFragmentKind::ParagraphLine {
          fragment.kind = FrameFragmentKind::NoteLine;
        }
        fragment
      }));
    page
      .frame_influences
      .extend(plan.influences.iter().cloned().map(|mut influence| {
        influence.item_start += offset;
        influence.item_end += offset;
        influence.bounds = influence
          .bounds
          .map(|bounds| translate_frame_bounds(bounds, 0.0, top));
        influence
      }));
    page.footnote_area_top_pt = Some(top);
    page.footnote_flow_bottom_pt = Some(bottom);
    // All body checkpoint replay is complete before materialization. Earlier
    // immutable revisions are no longer needed by the returned page.
    page.footnote_column_plans.clear();
    page.prepared_column_footnotes = None;
  }
}

#[cfg(test)]
mod tests {
  use super::super::footnote_follow::tests::{document, paragraph};
  use super::*;

  fn section(page: PageSetup, count: usize, gap_pt: f32) -> crate::docx::ImportedSection {
    crate::docx::ImportedSection {
      break_kind: SectionBreakKind::Continuous,
      discarded_carrier_spacing_after_pt: None,
      section_properties: None,
      page,
      columns: SectionColumns {
        count,
        gap_pt,
        ..Default::default()
      },
      title_page: false,
      header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      even_header_blocks: Vec::new(),
      even_footer_blocks: Vec::new(),
      blocks: Vec::new(),
    }
  }

  #[test]
  fn footnote_columns_follow_the_first_reference_and_explicit_layout_option() {
    let mut document = document();
    document.sections = vec![
      section(document.page, 1, 14.0),
      section(document.page, 3, 18.0),
    ];
    let mut flow = flow_context(
      document.page,
      1,
      document.sections[1].columns,
      0,
      0,
      DEFAULT_TAB_STOP_PT,
    );
    flow.compatibility_mode = 15;
    let mut page = empty_page(document.page, 0);
    // The body starts in a single-column section, but the first reference is
    // in the later three-column section. A subsequent callout cannot change it.
    assert_eq!(columns(&document, &page, flow).count, 3);
    page.footnote_column_owner = Some(1);
    let later = FlowContext {
      section_index: 0,
      columns: document.sections[0].columns,
      ..flow
    };
    assert_eq!(columns(&document, &page, later).count, 3);
    assert_eq!(columns(&document, &page, later).gap_pt, 18.0);
    document.sections[0].section_properties = Some(w::SectionProperties {
      footnote_columns: Some(ooxmlsdk::schemas::w15::FootnoteColumns { val: 2 }),
      ..Default::default()
    });
    assert_eq!(columns(&document, &page, later).count, 3);
    assert_eq!(columns(&document, &page, later).gap_pt, 18.0);
    // Native first=2/owner=1 retains one note column; first=1/owner=omitted
    // retains the owner's automatic three columns, on the same or a new page.
    for (value, count, gap) in [(1, 1, 36.0), (2, 2, 36.0), (0, 3, 18.0)] {
      document.sections[1].section_properties = Some(w::SectionProperties {
        footnote_columns: Some(ooxmlsdk::schemas::w15::FootnoteColumns { val: value }),
        ..Default::default()
      });
      assert_eq!(columns(&document, &page, later).count, count);
      assert_eq!(columns(&document, &page, later).gap_pt, gap);
    }
    assert_eq!(
      columns(
        &document,
        &page,
        FlowContext {
          compatibility_mode: 14,
          ..later
        }
      )
      .count,
      1
    );
  }

  #[test]
  fn footnote_columns_balance_complete_note_stories_before_materialization() {
    let mut document = document();
    document.sections = vec![section(document.page, 3, 18.0)];
    for id in 1..=10 {
      document
        .footnotes
        .insert(id, vec![paragraph(&format!("note-{id}"), false, false)]);
    }
    let mut flow = flow_context(
      document.page,
      0,
      document.sections[0].columns,
      0,
      0,
      DEFAULT_TAB_STOP_PT,
    );
    flow.compatibility_mode = 15;
    let mut metrics = TextMetrics::new();
    let mut page = empty_page(document.page, 0);
    assert!(commit(
      &(1..=10).collect::<Vec<_>>(),
      &document,
      &mut page,
      flow,
      &mut metrics
    ));
    assert!(
      page.items.is_empty(),
      "body checkpoints must not retain rewritten note paint"
    );
    assert!((page.footnote_reserved_height_pt - 90.0).abs() < 0.01);
    materialize(&document, std::slice::from_mut(&mut page), &mut metrics);
    let positions = page
      .items
      .iter()
      .filter_map(|item| match item {
        PageItem::Text(text) if text.text.starts_with("note-") => {
          Some((text.text.clone(), text.x_pt, text.y_pt))
        }
        _ => None,
      })
      .collect::<Vec<_>>();
    assert_eq!(positions.len(), 10);
    for (index, (_, x, y)) in positions.iter().enumerate() {
      let column = index / 4;
      assert!((*x - (36.0 + column as f32 * 102.0)).abs() < 0.01);
      assert!((*y - positions[0].2 - (index % 4) as f32 * 18.0).abs() < 0.01);
    }
    assert!(
      page.footnote_column_plans.is_empty(),
      "finished pages release obsolete plans"
    );
    assert_eq!(page.footnote_item_indices.len(), page.items.len());
  }

  #[test]
  fn footnote_columns_restore_committed_prefix_without_rewriting_paint() {
    let mut document = document();
    document.sections = vec![section(document.page, 3, 18.0)];
    for id in 1..=5 {
      document
        .footnotes
        .insert(id, vec![paragraph("NOTE", false, false)]);
    }
    let mut flow = flow_context(
      document.page,
      0,
      document.sections[0].columns,
      0,
      0,
      DEFAULT_TAB_STOP_PT,
    );
    flow.compatibility_mode = 15;
    let mut metrics = TextMetrics::new();
    let mut page = empty_page(document.page, 0);
    let empty = page_checkpoint(&page);
    assert!(commit(&[1, 2], &document, &mut page, flow, &mut metrics));
    let checkpoint = page_checkpoint(&page);
    let prefix = page.footnote_column_plans.last().unwrap().clone();
    assert!(commit(&[3, 4, 5], &document, &mut page, flow, &mut metrics));
    assert!(page.footnote_reserved_height_pt > checkpoint.footnote_reserved_height_pt);
    restore_page_checkpoint(&mut page, checkpoint);
    assert!(Arc::ptr_eq(
      page.footnote_column_plans.last().unwrap(),
      &prefix
    ));
    assert_eq!(page.footnote_column_plans.last().unwrap().ids, [1, 2]);
    assert!(page.items.is_empty());
    assert_eq!(
      page.footnote_reserved_height_pt,
      checkpoint.footnote_reserved_height_pt
    );
    restore_page_checkpoint(&mut page, empty);
    assert!(page.footnote_column_owner.is_none());
    assert!(page.footnote_column_plans.is_empty());
    assert!(page.prepared_column_footnotes.is_none());
  }

  #[test]
  fn modern_uniform_column_sections_keep_complete_rows_and_following_boundary() {
    // Native independent no-note controls retain 3/3/1, 3/3/2 and 4/4/2,
    // including a larger font. Use exact 18/27pt lines here to distinguish
    // measured rows from the generic default-line rounding and legacy mean.
    for (count, columns, height, expected) in [
      (7_usize, 3, 18.0, vec![3, 3, 1]),
      (8, 3, 18.0, vec![3, 3, 2]),
      (10, 3, 18.0, vec![4, 4, 2]),
      (5, 2, 18.0, vec![3, 2]),
      (7, 3, 27.0, vec![3, 3, 1]),
    ] {
      let mut document = document();
      let mut first = section(document.page, columns, 18.0);
      first.blocks = (1..=count)
        .map(|index| {
          let mut block = paragraph(&format!("A{index}"), false, false);
          if let Block::Paragraph(paragraph) = &mut block {
            paragraph.format.line_height_pt = Some(height);
          }
          block
        })
        .collect();
      let geometry = column_geometry(document.page, first.columns, 0);
      let mut next = section(document.page, 1, 36.0);
      next.columns.unbalanced = true;
      let mut marker = paragraph("FOLLOWING", false, false);
      if let Block::Paragraph(paragraph) = &mut marker {
        paragraph.format.line_height_pt = Some(height);
      }
      next.blocks = vec![marker];
      document.blocks = first.blocks.iter().chain(&next.blocks).cloned().collect();
      document.sections = vec![first, next];
      let layout = RootFrameLayout::new(&document, &LayoutOptions::default()).format();
      assert_eq!(layout.pages.len(), 1);
      let mut actual = vec![0; columns];
      let mut first_y = None;
      let mut following_y = None;
      for item in &layout.pages[0].items {
        if let PageItem::Text(text) = item {
          if text.text.starts_with('A') {
            let column = geometry
              .lefts
              .iter()
              .position(|left| (text.x_pt - left).abs() < 0.01)
              .unwrap();
            actual[column] += 1;
            first_y.get_or_insert(text.y_pt);
          } else if text.text == "FOLLOWING" {
            following_y = Some(text.y_pt);
          }
        }
      }
      assert_eq!(
        actual, expected,
        "count={count}, columns={columns}, height={height}"
      );
      assert!(
        (following_y.unwrap() - first_y.unwrap() - count.div_ceil(columns) as f32 * height).abs()
          < 0.01
      );
    }
  }
}
