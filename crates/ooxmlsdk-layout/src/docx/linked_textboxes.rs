//! WPS story identities are independent of the order of their shape anchors.
//!
//! Resolve zero-area leading frames without inventing an overflow estimate.
//! The ordinary formatter supplies source cursors for both the blank line in
//! a zero-area head and the last admitted line in an ordinary fixed frame.

use super::*;

pub(super) fn advance_fixed_height_heads(document: &mut DocxDocument) {
  let layout = layout::LinkedTextboxLayout::new(document);
  let mut frames = Vec::new();
  let mut chains = std::collections::BTreeMap::<u16, Vec<(usize, PageSetup)>>::new();
  if document.sections.is_empty() {
    collect_frames(&mut document.blocks, &mut frames);
    for (index, frame) in frames.iter().enumerate() {
      let link = frame.text_box_link.expect("collected linked frame");
      chains
        .entry(link.story_id)
        .or_default()
        .push((index, document.page));
    }
  } else {
    for section in &mut document.sections {
      let first = frames.len();
      collect_frames(&mut section.blocks, &mut frames);
      for (index, frame) in frames.iter().enumerate().skip(first) {
        let link = frame.text_box_link.expect("collected linked frame");
        chains
          .entry(link.story_id)
          .or_default()
          .push((index, section.page));
      }
    }
  }
  let mut metrics = None;
  for chain in chains.values_mut() {
    chain.sort_unstable_by_key(|&(index, _)| frames[index].text_box_link.unwrap().sequence);
    if frames[chain[0].0].text_box_link.unwrap().sequence != 0
      || chain.windows(2).any(|pair| {
        Some(frames[pair[1].0].text_box_link.unwrap().sequence)
          != frames[pair[0].0]
            .text_box_link
            .unwrap()
            .sequence
            .checked_add(1)
      })
    {
      continue;
    }
    for pair in chain.windows(2) {
      let ((head, setup), (next, _)) = (pair[0], pair[1]);
      if !frames[next].text_box_blocks.is_empty()
        || !layout::fixed_linked_textbox_can_flow(frames[head])
      {
        continue;
      }
      let Some((prefix, following)) = layout.split(
        frames[head],
        setup,
        metrics.get_or_insert_with(crate::text_metrics::TextMetrics::new),
      ) else {
        continue;
      };
      frames[head].text_box_blocks = prefix;
      frames[next].text_box_blocks = following;
    }
  }
}

pub(super) fn advance_zero_area_heads<'a>(
  stories: impl IntoIterator<Item = &'a mut [Block]>,
  default_tab_stop_pt: f32,
) {
  let mut frames = Vec::new();
  for blocks in stories {
    collect_frames(blocks, &mut frames);
  }
  let mut chains = std::collections::BTreeMap::<u16, Vec<usize>>::new();
  for (index, frame) in frames.iter().enumerate() {
    let link = frame.text_box_link.expect("collected linked frame");
    chains.entry(link.story_id).or_default().push(index);
  }
  let mut metrics = None;
  for chain in chains.values_mut() {
    chain.sort_unstable_by_key(|&index| frames[index].text_box_link.unwrap().sequence);
    if frames[chain[0]].text_box_link.unwrap().sequence != 0
      || chain.windows(2).any(|pair| {
        Some(frames[pair[1]].text_box_link.unwrap().sequence)
          != frames[pair[0]]
            .text_box_link
            .unwrap()
            .sequence
            .checked_add(1)
      })
    {
      // Missing heads, sequence gaps and duplicate sequence owners do not
      // authorize moving another shape's rich content.
      continue;
    }
    for pair in chain.windows(2) {
      let (head_index, next_index) = (pair[0], pair[1]);
      if !frames[next_index].text_box_blocks.is_empty()
        || !layout::linked_textbox_has_zero_height(frames[head_index])
      {
        continue;
      }
      let Some((prefix, following)) = layout::zero_area_linked_textbox_split(
        frames[head_index],
        default_tab_stop_pt,
        metrics.get_or_insert_with(crate::text_metrics::TextMetrics::new),
      ) else {
        continue;
      };
      frames[head_index].text_box_blocks = prefix;
      frames[next_index].text_box_blocks = following;
    }
  }
}

fn collect_frames<'a>(blocks: &'a mut [Block], frames: &mut Vec<&'a mut InlineShape>) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => collect_inlines(&mut paragraph.inlines, frames),
      Block::Table(table) => {
        for row in &mut table.rows {
          for cell in &mut row.cells {
            collect_frames(&mut cell.blocks, frames);
          }
        }
      }
      Block::Frame(frame) => collect_frames(&mut frame.blocks, frames),
    }
  }
}

fn collect_inlines<'a>(inlines: &'a mut [InlineItem], frames: &mut Vec<&'a mut InlineShape>) {
  for inline in inlines {
    let InlineItem::Shape(shape) = inline else {
      continue;
    };
    if shape.text_box_link.is_some() {
      frames.push(shape);
    } else if let Some(children) = &mut shape.canvas_children {
      collect_inlines(children, frames);
    }
  }
}
