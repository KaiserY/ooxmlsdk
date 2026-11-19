//! Textbox story identities are independent of their shape anchor order.
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

pub(super) fn advance_vml_fixed_height_heads(document: &mut DocxDocument) {
  let layout = layout::LinkedTextboxLayout::new(document);
  let mut frames = Vec::new();
  let mut setups = Vec::new();
  let is_vml = |shape: &InlineShape| shape.vml_shape_id.is_some() && shape.vml_text_box.is_some();
  if document.sections.is_empty() {
    collect_frames_matching(&mut document.blocks, &mut frames, is_vml);
    setups.resize(frames.len(), document.page);
  } else {
    for section in &mut document.sections {
      collect_frames_matching(&mut section.blocks, &mut frames, is_vml);
      setups.resize(frames.len(), section.page);
    }
  }
  if !frames.iter().any(|frame| frame.vml_next_textbox.is_some()) {
    return;
  }
  // Borrow the existing identities only while constructing the index graph.
  // Text movement needs no new shared strings, global cache or synchronization.
  let chains = {
    let links = frames
      .iter()
      .map(|frame| {
        (
          frame
            .vml_shape_id
            .as_deref()
            .expect("collected VML identity"),
          frame.vml_next_textbox.as_deref(),
        )
      })
      .collect::<Vec<_>>();
    resolve_vml_chains(&links)
  };
  let mut metrics = None;
  for chain in chains {
    for pair in chain.windows(2) {
      let (head, next) = (pair[0], pair[1]);
      if !frames[next].text_box_blocks.is_empty()
        || !layout::fixed_linked_textbox_can_flow(frames[head])
      {
        continue;
      }
      let Some((prefix, following)) = layout.split(
        frames[head],
        setups[head],
        metrics.get_or_insert_with(crate::text_metrics::TextMetrics::new),
      ) else {
        continue;
      };
      frames[head].text_box_blocks = prefix;
      frames[next].text_box_blocks = following;
    }
  }
}

fn resolve_vml_chains(links: &[(&str, Option<&str>)]) -> Vec<Vec<usize>> {
  let mut names = std::collections::BTreeMap::new();
  for (index, &(name, _)) in links.iter().enumerate() {
    names
      .entry(name)
      .and_modify(|owner| *owner = None)
      .or_insert(Some(index));
  }
  let next = links
    .iter()
    .enumerate()
    .map(|(index, &(name, next))| {
      if names.get(name).copied() != Some(Some(index)) {
        return Err(());
      }
      // Office and Writer both treat a self-reference as the terminal mark.
      match next {
        None => Ok(None),
        Some(next) if next == name => Ok(None),
        Some(next) => names.get(next).copied().flatten().map(Some).ok_or(()),
      }
    })
    .collect::<Vec<_>>();
  let mut incoming = vec![0_usize; links.len()];
  for &index in next.iter().flatten().flatten() {
    incoming[index] += 1;
  }
  let mut visited = vec![false; links.len()];
  let mut chains = Vec::new();
  for start in 0..links.len() {
    if incoming[start] != 0 || !matches!(next[start], Ok(Some(_))) {
      continue;
    }
    let mut chain = Vec::new();
    let mut current = Some(start);
    let mut valid = true;
    while let Some(index) = current {
      if visited[index] || (index != start && incoming[index] != 1) {
        valid = false;
        break;
      }
      visited[index] = true;
      chain.push(index);
      match next[index] {
        Ok(next) => current = next,
        Err(()) => {
          valid = false;
          break;
        }
      }
    }
    if valid && chain.len() > 1 {
      chains.push(chain);
    }
  }
  chains
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
  collect_frames_matching(blocks, frames, |shape| shape.text_box_link.is_some());
}

fn collect_frames_matching<'a>(
  blocks: &'a mut [Block],
  frames: &mut Vec<&'a mut InlineShape>,
  matches: fn(&InlineShape) -> bool,
) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => collect_inlines(&mut paragraph.inlines, frames, matches),
      Block::Table(table) => {
        for row in &mut table.rows {
          for cell in &mut row.cells {
            collect_frames_matching(&mut cell.blocks, frames, matches);
          }
        }
      }
      Block::Frame(frame) => collect_frames_matching(&mut frame.blocks, frames, matches),
    }
  }
}

fn collect_inlines<'a>(
  inlines: &'a mut [InlineItem],
  frames: &mut Vec<&'a mut InlineShape>,
  matches: fn(&InlineShape) -> bool,
) {
  for inline in inlines {
    let InlineItem::Shape(shape) = inline else {
      continue;
    };
    if matches(shape) {
      frames.push(shape);
    } else if let Some(children) = &mut shape.canvas_children {
      collect_inlines(children, frames, matches);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vml_string_links_follow_ids_instead_of_anchor_order_and_accept_terminal_self() {
    let links = [
      ("last", Some("last")),
      ("middle", Some("last")),
      ("Text Box 2", Some("middle")),
    ];
    assert_eq!(resolve_vml_chains(&links), [vec![2, 1, 0]]);
    assert_eq!(
      resolve_vml_chains(&[("head", Some("tail")), ("tail", None)]),
      [vec![0, 1]]
    );
  }

  #[test]
  fn vml_ambiguous_and_cyclic_components_do_not_move_another_story() {
    for links in [
      vec![("a", Some("b")), ("b", Some("a"))],
      vec![("a", Some("b")), ("b", Some("missing"))],
      vec![("a", Some("b")), ("b", None), ("other", Some("b"))],
      vec![("a", Some("b")), ("b", None), ("b", None)],
      vec![("a", Some("b")), ("b", Some("c")), ("c", Some("b"))],
    ] {
      assert!(resolve_vml_chains(&links).is_empty(), "{links:?}");
    }
    assert_eq!(
      resolve_vml_chains(&[
        ("bad", Some("missing")),
        ("head", Some("tail")),
        ("tail", None)
      ]),
      [vec![1, 2]],
    );
  }
}
