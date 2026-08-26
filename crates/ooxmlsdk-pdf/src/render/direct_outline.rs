use pdf_writer::{Finish, Pdf, Ref, TextStr};

use super::direct::RefAllocator;
use super::page::{PageSelection, pdf_page_dimension};
use crate::error::{PdfError, Result};
use crate::options::{PdfOptions, PdfStandard};
use ooxmlsdk_layout::common;

/// A checked, backend-independent PDF outline object graph.
///
/// The layout document stores outline entries in preorder. This type resolves
/// that sequence into explicit parent, sibling, and child links before any PDF
/// objects are written. Keeping the graph closed here prevents serializer
/// order from accidentally producing the malformed cross-level `/Last` links
/// found in some real-world producers.
pub(super) struct DirectOutline {
  root_id: Ref,
  root_children: Vec<usize>,
  root_visible_count: i32,
  nodes: Vec<OutlineNode>,
}

struct OutlineNode {
  id: Ref,
  level: u8,
  text: String,
  output_page_index: usize,
  target_x_pt: f32,
  target_y_pt: f32,
  parent: Option<usize>,
  previous: Option<usize>,
  next: Option<usize>,
  children: Vec<usize>,
  open: bool,
  visible_descendants_if_open: i32,
}

struct OutlineSeed {
  level: u8,
  text: String,
  output_page_index: usize,
  target_x_pt: f32,
  target_y_pt: f32,
}

impl DirectOutline {
  pub(super) fn allocate(
    document: &common::LayoutDocument<'static>,
    options: &PdfOptions,
    selection: &PageSelection,
    refs: &mut RefAllocator,
  ) -> Result<Option<Self>> {
    if !options.general.export_bookmarks {
      return Ok(None);
    }

    let mut seeds = document
      .outline_entries
      .iter()
      .filter_map(|entry| {
        let output_page_index = selection.output_index(entry.page_index)?;
        Some(OutlineSeed {
          level: entry.level,
          text: entry.text.to_string(),
          output_page_index,
          target_x_pt: entry.target.x.0,
          target_y_pt: entry.target.y.0,
        })
      })
      .collect::<Vec<_>>();
    let requests_pdf_ua = options.general.pdf_ua_compliance
      || options
        .standards
        .iter()
        .any(|standard| matches!(standard, PdfStandard::PdfUa1));
    if seeds.is_empty() && requests_pdf_ua {
      if selection.source_indices.is_empty() {
        return Err(PdfError::Options(
          "PDF/UA requires at least one selected page for its document outline".to_string(),
        ));
      }
      let title = options
        .metadata
        .title
        .as_deref()
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .ok_or_else(|| {
          PdfError::Options("PDF/UA requires a non-empty document outline title".to_string())
        })?;
      seeds.push(OutlineSeed {
        level: 0,
        text: title.to_string(),
        output_page_index: 0,
        target_x_pt: 0.0,
        target_y_pt: 0.0,
      });
    }
    if seeds.is_empty() {
      return Ok(None);
    }

    Self::build(seeds, options.general.open_bookmark_levels, refs).map(Some)
  }

  fn build(
    seeds: Vec<OutlineSeed>,
    open_levels: Option<i32>,
    refs: &mut RefAllocator,
  ) -> Result<Self> {
    let root_id = refs.alloc()?;
    let mut root_children = Vec::new();
    let mut nodes = Vec::<OutlineNode>::with_capacity(seeds.len());
    let mut ancestors = Vec::<usize>::new();

    for seed in seeds {
      if !seed.target_x_pt.is_finite() || !seed.target_y_pt.is_finite() {
        return Err(PdfError::Writer(
          "outline destination contains a non-finite coordinate".to_string(),
        ));
      }
      while ancestors
        .last()
        .is_some_and(|&index| nodes[index].level >= seed.level)
      {
        ancestors.pop();
      }
      let parent = ancestors.last().copied();
      let siblings = parent.map_or(&root_children, |index| &nodes[index].children);
      let previous = siblings.last().copied();
      let index = nodes.len();
      nodes.push(OutlineNode {
        id: refs.alloc()?,
        level: seed.level,
        text: seed.text,
        output_page_index: seed.output_page_index,
        target_x_pt: seed.target_x_pt,
        target_y_pt: seed.target_y_pt,
        parent,
        previous,
        next: None,
        children: Vec::new(),
        open: open_levels.is_some_and(|levels| levels < 0 || levels >= i32::from(seed.level)),
        visible_descendants_if_open: 0,
      });
      if let Some(previous) = previous {
        nodes[previous].next = Some(index);
      }
      if let Some(parent) = parent {
        nodes[parent].children.push(index);
      } else {
        root_children.push(index);
      }
      ancestors.push(index);
    }

    for index in (0..nodes.len()).rev() {
      let visible_descendants = nodes[index]
        .children
        .iter()
        .try_fold(0i32, |count, &child| {
          let child_count = 1i32
            .checked_add(if nodes[child].open {
              nodes[child].visible_descendants_if_open
            } else {
              0
            })
            .ok_or_else(|| {
              PdfError::Writer("outline count exceeds the PDF integer range".into())
            })?;
          count
            .checked_add(child_count)
            .ok_or_else(|| PdfError::Writer("outline count exceeds the PDF integer range".into()))
        })?;
      nodes[index].visible_descendants_if_open = visible_descendants;
    }
    let root_visible_count = root_children.iter().try_fold(0i32, |count, &child| {
      let child_count = 1i32
        .checked_add(if nodes[child].open {
          nodes[child].visible_descendants_if_open
        } else {
          0
        })
        .ok_or_else(|| PdfError::Writer("outline count exceeds the PDF integer range".into()))?;
      count
        .checked_add(child_count)
        .ok_or_else(|| PdfError::Writer("outline count exceeds the PDF integer range".into()))
    })?;

    Ok(Self {
      root_id,
      root_children,
      root_visible_count,
      nodes,
    })
  }

  pub(super) fn root_id(&self) -> Ref {
    self.root_id
  }

  pub(super) fn write_objects(
    &self,
    pdf: &mut Pdf,
    document: &common::LayoutDocument<'static>,
    selection: &PageSelection,
    page_objects: &[(Ref, Ref)],
  ) -> Result<()> {
    let first = self
      .root_children
      .first()
      .map(|&index| self.nodes[index].id)
      .ok_or_else(|| PdfError::Writer("non-empty outline has no root child".to_string()))?;
    let last = self
      .root_children
      .last()
      .map(|&index| self.nodes[index].id)
      .ok_or_else(|| PdfError::Writer("non-empty outline has no root child".to_string()))?;
    pdf
      .outline(self.root_id)
      .first(first)
      .last(last)
      .count(self.root_visible_count)
      .finish();

    for node in &self.nodes {
      let parent_id = node
        .parent
        .map_or(self.root_id, |index| self.nodes[index].id);
      let page_id = page_objects
        .get(node.output_page_index)
        .map(|objects| objects.0)
        .ok_or_else(|| PdfError::Writer("outline destination page is missing".to_string()))?;
      let source_page_index = *selection
        .source_indices
        .get(node.output_page_index)
        .ok_or_else(|| PdfError::Writer("outline source page is missing".to_string()))?;
      let page_height = pdf_page_dimension(
        document.engine_kind,
        document.pages[source_page_index].setup.size.height.0,
      );

      let mut item = pdf.outline_item(node.id);
      item.title(TextStr(&node.text)).parent(parent_id);
      if let Some(previous) = node.previous {
        item.prev(self.nodes[previous].id);
      }
      if let Some(next) = node.next {
        item.next(self.nodes[next].id);
      }
      if let (Some(&first), Some(&last)) = (node.children.first(), node.children.last()) {
        item.first(self.nodes[first].id).last(self.nodes[last].id);
        let count = if node.open {
          node.visible_descendants_if_open
        } else {
          -node.visible_descendants_if_open
        };
        item.count(count);
      }
      item
        .dest()
        .page(page_id)
        .xyz(node.target_x_pt, page_height - node.target_y_pt, None);
      item.finish();
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn seeds() -> Vec<OutlineSeed> {
    [(0, "A"), (1, "A.1"), (1, "A.2"), (3, "A.2.a"), (0, "B")]
      .into_iter()
      .enumerate()
      .map(|(index, (level, text))| OutlineSeed {
        level,
        text: text.to_string(),
        output_page_index: 0,
        target_x_pt: index as f32,
        target_y_pt: index as f32,
      })
      .collect()
  }

  #[test]
  fn outline_preorder_closes_parent_and_sibling_links() {
    let mut refs = RefAllocator::default();
    let outline = DirectOutline::build(seeds(), Some(0), &mut refs).unwrap();

    assert_eq!(outline.root_children, vec![0, 4]);
    assert_eq!(outline.root_visible_count, 4);
    assert_eq!(outline.nodes[0].parent, None);
    assert_eq!(outline.nodes[0].previous, None);
    assert_eq!(outline.nodes[0].next, Some(4));
    assert_eq!(outline.nodes[0].children, vec![1, 2]);
    assert_eq!(outline.nodes[0].visible_descendants_if_open, 2);
    assert!(outline.nodes[0].open);
    assert_eq!(outline.nodes[1].parent, Some(0));
    assert_eq!(outline.nodes[1].next, Some(2));
    assert_eq!(outline.nodes[2].previous, Some(1));
    assert_eq!(outline.nodes[2].children, vec![3]);
    assert_eq!(outline.nodes[2].visible_descendants_if_open, 1);
    assert!(!outline.nodes[2].open);
    assert_eq!(outline.nodes[3].parent, Some(2));
    assert_eq!(outline.nodes[4].previous, Some(0));
  }

  #[test]
  fn outline_negative_and_fully_open_counts_follow_visible_descendants() {
    let mut refs = RefAllocator::default();
    let collapsed = DirectOutline::build(seeds(), None, &mut refs).unwrap();
    assert_eq!(collapsed.root_visible_count, 2);
    assert!(!collapsed.nodes[0].open);
    assert_eq!(collapsed.nodes[0].visible_descendants_if_open, 2);
    assert_eq!(collapsed.nodes[2].visible_descendants_if_open, 1);

    let mut refs = RefAllocator::default();
    let expanded = DirectOutline::build(seeds(), Some(-1), &mut refs).unwrap();
    assert_eq!(expanded.root_visible_count, 5);
    assert_eq!(expanded.nodes[0].visible_descendants_if_open, 3);
    assert_eq!(expanded.nodes[2].visible_descendants_if_open, 1);
    assert!(expanded.nodes.iter().all(|node| node.open));
  }

  #[test]
  fn outline_rejects_a_non_finite_destination_before_allocating_objects() {
    let mut entries = seeds();
    entries[2].target_y_pt = f32::NAN;
    let mut refs = RefAllocator::default();
    assert!(matches!(
      DirectOutline::build(entries, Some(-1), &mut refs),
      Err(PdfError::Writer(message)) if message.contains("non-finite")
    ));
  }
}
