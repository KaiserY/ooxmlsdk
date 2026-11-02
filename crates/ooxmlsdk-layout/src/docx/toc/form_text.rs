//! Print a regular text form's complete block-spanning result.
//!
//! Word retains the live cache but prints ffData/textInput's default when
//! print updates are enabled. The result can own table cell/paragraph marks.
//! Native on/off, locked, cell-close and whole-table controls distinguish this
//! from ordinary inline form fields and malformed unclosed fields.
use super::*;

struct Replacement {
  ordinal: usize,
  section: usize,
  container: Vec<StoryPathStep>,
  first_block: usize,
  last_block: usize,
  paragraph: Paragraph,
}

pub(super) fn refresh(
  sections: &mut [ImportedSection],
  scan: &StoryScan,
  styles: &StylesCatalog,
) -> bool {
  if !styles.import_settings.refresh_field_at_print("FORMTEXT") {
    return false;
  }
  let mut replacements = Vec::new();
  for field in &scan.fields {
    let Some(form) = &field.form_text_print else {
      continue;
    };
    // Inline forms retain their existing cache/blank-cell formatter. This
    // owner rebuilds structural results, including a form crossing cell ends.
    if field.locked
      || field.start_ordinal == field.end_ordinal
      || !field.starts_at_paragraph_start
      || !field.ends_at_paragraph_end
      || field_instruction_name(&field.instruction).as_deref() != Some("FORMTEXT")
      || field.separator_location.is_none()
    {
      continue;
    }
    let Some((section, container, first_block, last_block)) = complete_block_range(scan, field)
    else {
      continue;
    };
    let (Some(first), Some(last)) = (
      paragraph(sections, scan, field.start_ordinal),
      paragraph(sections, scan, field.end_ordinal),
    ) else {
      continue;
    };
    let mut result = last.clone();
    // A range replacement preserves its ending paragraph mark. Word also
    // takes the default's character properties from the closing field marker,
    // not the first cached cell or the separator (native 9/11pt controls).
    let mut style = properties::run_style(
      form.end_properties.as_deref(),
      last.base_style.clone(),
      styles,
    );
    let text = if form.default_text.is_empty() {
      style.wordprocessingml_form_text_blank_cell = true;
      "\u{2002}".repeat(5)
    } else {
      form.default_text.clone()
    };
    result.inlines = first
      .inlines
      .iter()
      .filter(|inline| boundary_inline(inline))
      .cloned()
      .collect();
    push_resolved_field_text(&mut result.inlines, text, style, None);
    result.inlines.extend(
      last
        .inlines
        .iter()
        .filter(|inline| boundary_inline(inline))
        .cloned(),
    );
    result.field_events = first
      .field_events
      .iter()
      .filter(|event| boundary_event(event))
      .cloned()
      .chain(std::iter::once(ParagraphFieldEvent::Content))
      .chain(
        last
          .field_events
          .iter()
          .filter(|event| boundary_event(event))
          .cloned(),
      )
      .collect();
    refresh_paragraph_story_derivatives(&mut result);
    replacements.push(Replacement {
      ordinal: field.start_ordinal,
      section,
      container,
      first_block,
      last_block,
      paragraph: result,
    });
  }
  let changed = !replacements.is_empty();
  // Source addresses remain valid when replacing later, independent ranges
  // first. Only an outermost field receives form_text_print metadata.
  replacements.sort_by_key(|replacement| std::cmp::Reverse(replacement.ordinal));
  for replacement in replacements {
    if let Some(blocks) = story_block_container_mut(
      &mut sections[replacement.section].blocks,
      &replacement.container,
    ) {
      blocks.splice(
        replacement.first_block..=replacement.last_block,
        [Block::paragraph(replacement.paragraph)],
      );
    }
  }
  changed
}

fn boundary_inline(inline: &InlineItem) -> bool {
  matches!(
    inline,
    InlineItem::BookmarkStart(_) | InlineItem::FormWidgetStart(_) | InlineItem::FormWidgetEnd(_)
  )
}

fn boundary_event(event: &ParagraphFieldEvent) -> bool {
  matches!(
    event,
    ParagraphFieldEvent::BookmarkStart { .. }
      | ParagraphFieldEvent::BookmarkEnd { .. }
      | ParagraphFieldEvent::BookmarkReferenceUsesCaret(_)
  )
}

fn complete_block_range(
  scan: &StoryScan,
  field: &StoryField,
) -> Option<(usize, Vec<StoryPathStep>, usize, usize)> {
  let start = &scan.paragraphs.get(field.start_ordinal)?.address;
  let end = &scan.paragraphs.get(field.end_ordinal)?.address;
  if start.section_index != end.section_index {
    return None;
  }
  let mut depth = 0;
  while start.path.get(depth..depth + 2) == end.path.get(depth..depth + 2)
    && matches!(
      start.path.get(depth..depth + 2),
      Some([StoryPathStep::Block(_), StoryPathStep::TableCell { .. }])
    )
  {
    depth += 2;
  }
  let container = &start.path[..depth];
  let (StoryPathStep::Block(first), StoryPathStep::Block(last)) =
    (start.path.get(depth)?, end.path.get(depth)?)
  else {
    return None;
  };
  let belongs = |paragraph: &StoryParagraph| {
    paragraph.address.section_index == start.section_index
      && paragraph.address.path.starts_with(container)
      && matches!(paragraph.address.path.get(depth), Some(StoryPathStep::Block(index)) if (first..=last).contains(&index))
  };
  // StoryScan is a preorder traversal, so these two neighboring addresses
  // prove the candidate blocks have no authored paragraph outside the field.
  // This avoids rescanning every paragraph for each independent form field.
  if field
    .start_ordinal
    .checked_sub(1)
    .and_then(|ordinal| scan.paragraphs.get(ordinal))
    .is_some_and(belongs)
    || scan
      .paragraphs
      .get(field.end_ordinal + 1)
      .is_some_and(belongs)
  {
    return None;
  }
  Some((start.section_index, container.to_vec(), *first, *last))
}
