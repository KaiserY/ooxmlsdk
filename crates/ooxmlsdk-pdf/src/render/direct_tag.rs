use std::collections::BTreeMap;
use std::ops::Range;

use pdf_writer::types::{ArtifactType, StructRole};
use pdf_writer::writers::{StructElement, StructTreeRoot};
use pdf_writer::{Content, Finish, Name, Pdf, Ref, TextStr};

use super::direct::RefAllocator;
use super::direct_link::DirectPageLinks;
use super::paint::{PaintItem, PaintPage, PaintText};
use crate::error::{PdfError, Result};
use ooxmlsdk_layout::common;

/// A page-local marked-content classification.
///
/// A semantic item receives an MCID and exactly one parent in the structure
/// tree. Layout-only paint is marked as an artifact and deliberately receives
/// no MCID. `None` is reserved for items which emit no content of their own.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContentClass {
  None,
  Artifact,
  Text,
  Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TaggedPaintRecord {
  item_index: usize,
  mcid: Option<i32>,
  annotation_range: Range<usize>,
}

/// Collects the MCIDs for one page while its content stream is written.
#[derive(Debug, Default)]
pub(super) struct DirectPageTags {
  next_mcid: i32,
  records: Vec<TaggedPaintRecord>,
}

impl DirectPageTags {
  pub(super) fn begin_prepared(
    &mut self,
    content: &mut Content,
    item_index: usize,
    item: &PaintItem<'_>,
  ) -> Result<bool> {
    self.begin(content, item_index, prepared_content_class(item))
  }

  pub(super) fn begin_display(
    &mut self,
    content: &mut Content,
    item_index: usize,
    item: &common::DisplayItem<'_>,
  ) -> Result<bool> {
    self.begin(content, item_index, display_content_class(item))
  }

  fn begin(
    &mut self,
    content: &mut Content,
    item_index: usize,
    class: ContentClass,
  ) -> Result<bool> {
    match class {
      ContentClass::None => Ok(false),
      ContentClass::Artifact => {
        let mut marked = content.begin_marked_content_with_properties(Name(b"Artifact"));
        marked.properties().artifact().kind(ArtifactType::Layout);
        Ok(true)
      }
      ContentClass::Text | ContentClass::Other => {
        let mcid = self.next_mcid;
        self.next_mcid = self
          .next_mcid
          .checked_add(1)
          .ok_or_else(|| PdfError::Writer("page MCID space exhausted".to_string()))?;
        let tag = match class {
          ContentClass::Text => Name(b"Span"),
          ContentClass::Other => Name(b"P"),
          ContentClass::None | ContentClass::Artifact => unreachable!(),
        };
        content
          .begin_marked_content_with_properties(tag)
          .properties()
          .identify(mcid);
        self.records.push(TaggedPaintRecord {
          item_index,
          mcid: Some(mcid),
          annotation_range: 0..0,
        });
        Ok(true)
      }
    }
  }

  pub(super) fn finish_item(
    &mut self,
    item_index: usize,
    annotation_start: usize,
    annotation_end: usize,
  ) -> Result<()> {
    if annotation_end < annotation_start {
      return Err(PdfError::Writer(
        "page link annotation range is reversed".to_string(),
      ));
    }
    if annotation_start == annotation_end {
      return Ok(());
    }
    if let Some(record) = self
      .records
      .iter_mut()
      .rev()
      .find(|record| record.item_index == item_index)
    {
      if !record.annotation_range.is_empty() {
        return Err(PdfError::Writer(format!(
          "paint item {item_index} has more than one link annotation range"
        )));
      }
      record.annotation_range = annotation_start..annotation_end;
    } else {
      self.records.push(TaggedPaintRecord {
        item_index,
        mcid: None,
        annotation_range: annotation_start..annotation_end,
      });
    }
    Ok(())
  }

  fn parent_count(&self) -> Result<usize> {
    usize::try_from(self.next_mcid)
      .map_err(|_| PdfError::Writer("page MCID count is negative".to_string()))
  }
}

fn prepared_content_class(item: &PaintItem<'_>) -> ContentClass {
  match item {
    PaintItem::Text(text) if !text.item.text.is_empty() && text.emits_page_content() => {
      ContentClass::Text
    }
    PaintItem::Image(image)
      if image
        .alt_text
        .as_deref()
        .is_some_and(|alt| !alt.trim().is_empty()) =>
    {
      ContentClass::Other
    }
    PaintItem::Group { .. } if paint_item_alt_text(item).is_some() => ContentClass::Other,
    PaintItem::Image(_)
    | PaintItem::Group { .. }
    | PaintItem::Rect(_)
    | PaintItem::Line(_)
    | PaintItem::Polyline(_) => ContentClass::Artifact,
    PaintItem::Text(_) | PaintItem::LinkArea(_) => ContentClass::None,
  }
}

fn display_content_class(item: &common::DisplayItem<'_>) -> ContentClass {
  match item {
    common::DisplayItem::Rect(_)
    | common::DisplayItem::Line(_)
    | common::DisplayItem::Path(_)
    | common::DisplayItem::Group(_) => ContentClass::Artifact,
    common::DisplayItem::Text(text) if !text.text.is_empty() => ContentClass::Text,
    common::DisplayItem::Image(image)
      if image
        .alt_text
        .as_deref()
        .is_some_and(|alt| !alt.trim().is_empty()) =>
    {
      ContentClass::Other
    }
    common::DisplayItem::Image(_) => ContentClass::Artifact,
    common::DisplayItem::Text(_)
    | common::DisplayItem::Glyphs(_)
    | common::DisplayItem::LinkArea(_)
    | common::DisplayItem::AnnotationHint(_)
    | common::DisplayItem::Clip(_)
    | common::DisplayItem::Transform(_) => ContentClass::None,
  }
}

fn paint_item_alt_text<'a>(item: &'a PaintItem<'_>) -> Option<&'a str> {
  match item {
    PaintItem::Image(image) => image
      .alt_text
      .as_deref()
      .filter(|alt| !alt.trim().is_empty()),
    PaintItem::Group { items, .. } => items.iter().find_map(paint_item_alt_text),
    PaintItem::Text(_)
    | PaintItem::LinkArea(_)
    | PaintItem::Rect(_)
    | PaintItem::Line(_)
    | PaintItem::Polyline(_) => None,
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TagRole {
  Standard(StructRole),
  Custom { name: Vec<u8>, fallback: StructRole },
}

impl TagRole {
  fn write(&self, element: &mut StructElement<'_>) {
    match self {
      Self::Standard(role) => {
        element.kind(*role);
      }
      Self::Custom { name, .. } => {
        element.custom_kind(Name(name));
      }
    }
  }

  fn collect_mapping(&self, mappings: &mut BTreeMap<Vec<u8>, StructRole>) {
    if let Self::Custom { name, fallback } = self {
      mappings.insert(name.clone(), *fallback);
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum LogicalChild {
  Node(LogicalNode),
  MarkedContent(i32),
  ObjectRef {
    annotation_id: Ref,
    parent_tree_key: i32,
  },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LogicalNode {
  role: TagRole,
  alt_text: Option<String>,
  children: Vec<LogicalChild>,
}

impl LogicalNode {
  fn standard(role: StructRole, children: Vec<LogicalChild>) -> Self {
    Self {
      role: TagRole::Standard(role),
      alt_text: None,
      children,
    }
  }

  fn custom(name: &'static [u8], fallback: StructRole, child: LogicalChild) -> Self {
    Self {
      role: TagRole::Custom {
        name: name.to_vec(),
        fallback,
      },
      alt_text: None,
      children: vec![child],
    }
  }

  fn figure(alt_text: String, mcid: i32) -> Self {
    Self {
      role: TagRole::Standard(StructRole::Figure),
      alt_text: Some(alt_text),
      children: vec![LogicalChild::MarkedContent(mcid)],
    }
  }

  fn link(children: Vec<LogicalChild>) -> Self {
    Self::standard(StructRole::Link, children)
  }
}

#[derive(Debug)]
enum TagChild {
  Node(Box<TagNode>),
  MarkedContent(i32),
  ObjectRef { annotation_id: Ref },
}

#[derive(Debug)]
struct TagNode {
  id: Ref,
  role: TagRole,
  alt_text: Option<String>,
  children: Vec<TagChild>,
}

impl TagNode {
  fn allocate(
    logical: LogicalNode,
    refs: &mut RefAllocator,
    mcid_parents: &mut [Option<Ref>],
    annotation_parents: &mut BTreeMap<i32, Ref>,
  ) -> Result<Self> {
    let id = refs.alloc()?;
    let mut children = Vec::with_capacity(logical.children.len());
    for child in logical.children {
      match child {
        LogicalChild::Node(node) => children.push(TagChild::Node(Box::new(Self::allocate(
          node,
          refs,
          mcid_parents,
          annotation_parents,
        )?))),
        LogicalChild::MarkedContent(mcid) => {
          let index = usize::try_from(mcid)
            .map_err(|_| PdfError::Writer(format!("negative page MCID {mcid}")))?;
          let parent_count = mcid_parents.len();
          let parent = mcid_parents.get_mut(index).ok_or_else(|| {
            PdfError::Writer(format!(
              "structure tree MCID {mcid} exceeds the page marked-content count {}",
              parent_count
            ))
          })?;
          if parent.replace(id).is_some() {
            return Err(PdfError::Writer(format!(
              "page MCID {mcid} has more than one structure parent"
            )));
          }
          children.push(TagChild::MarkedContent(mcid));
        }
        LogicalChild::ObjectRef {
          annotation_id,
          parent_tree_key,
        } => {
          if annotation_parents.insert(parent_tree_key, id).is_some() {
            return Err(PdfError::Writer(format!(
              "annotation ParentTree key {parent_tree_key} has more than one structure parent"
            )));
          }
          children.push(TagChild::ObjectRef { annotation_id });
        }
      }
    }
    Ok(Self {
      id,
      role: logical.role,
      alt_text: logical.alt_text,
      children,
    })
  }

  fn collect_mappings(&self, mappings: &mut BTreeMap<Vec<u8>, StructRole>) {
    self.role.collect_mapping(mappings);
    for child in &self.children {
      if let TagChild::Node(node) = child {
        node.collect_mappings(mappings);
      }
    }
  }

  fn write(&self, pdf: &mut Pdf, parent_id: Ref, page_id: Ref) {
    for child in &self.children {
      if let TagChild::Node(node) = child {
        node.write(pdf, self.id, page_id);
      }
    }

    let mut element = pdf.indirect(self.id).start::<StructElement>();
    self.role.write(&mut element);
    element.parent(parent_id);
    if self
      .children
      .iter()
      .any(|child| matches!(child, TagChild::MarkedContent(_)))
    {
      element.page(page_id);
    }
    if let Some(alt_text) = &self.alt_text {
      element.alt(TextStr(alt_text));
    }
    let mut children = element.children();
    for child in &self.children {
      match child {
        TagChild::Node(node) => {
          children.struct_element(node.id);
        }
        TagChild::MarkedContent(mcid) => {
          children.marked_content_id(*mcid);
        }
        TagChild::ObjectRef { annotation_id } => {
          children.object_ref().page(page_id).object(*annotation_id);
        }
      }
    }
    children.finish();
    element.finish();
  }
}

#[derive(Debug)]
struct TaggedPage {
  page_id: Ref,
  part: TagNode,
  parent_tree_key: Option<i32>,
  parent_array_id: Option<Ref>,
  mcid_parents: Vec<Ref>,
  annotation_parents: BTreeMap<i32, Ref>,
}

/// Owns the complete direct structure tree and its reverse ParentTree index.
#[derive(Debug)]
pub(super) struct DirectTagging {
  root_id: Ref,
  document_id: Ref,
  pages: Vec<TaggedPage>,
  next_parent_tree_key: i32,
}

pub(super) struct DirectTaggedPageSource<'a> {
  pub(super) document: &'a common::LayoutDocument<'static>,
  pub(super) page_index: usize,
  pub(super) page_id: Ref,
}

impl DirectTagging {
  pub(super) fn allocate(refs: &mut RefAllocator) -> Result<Self> {
    Ok(Self {
      root_id: refs.alloc()?,
      document_id: refs.alloc()?,
      pages: Vec::new(),
      next_parent_tree_key: 0,
    })
  }

  pub(super) fn root_id(&self) -> Ref {
    self.root_id
  }

  pub(super) fn finish_page(
    &mut self,
    source: DirectTaggedPageSource<'_>,
    paint_page: Option<&PaintPage<'_>>,
    page_tags: DirectPageTags,
    page_links: &mut DirectPageLinks,
    refs: &mut RefAllocator,
  ) -> Result<Option<i32>> {
    let DirectTaggedPageSource {
      document,
      page_index: source_page_index,
      page_id,
    } = source;
    let mut annotations = Vec::with_capacity(page_links.len());
    for index in 0..page_links.len() {
      let key = self.next_parent_tree_key;
      self.next_parent_tree_key = self
        .next_parent_tree_key
        .checked_add(1)
        .ok_or_else(|| PdfError::Writer("structure ParentTree key space exhausted".to_string()))?;
      let annotation_id = page_links.assign_struct_parent(index, key)?;
      annotations.push((annotation_id, key));
    }
    let logical = if let Some(page) = paint_page {
      build_page_part(
        document,
        source_page_index,
        page,
        &page_tags.records,
        &annotations,
      )?
    } else {
      debug_assert!(page_tags.records.is_empty());
      LogicalNode::standard(StructRole::Part, Vec::new())
    };
    let mut mcid_parents = vec![None; page_tags.parent_count()?];
    let mut annotation_parents = BTreeMap::new();
    let part = TagNode::allocate(logical, refs, &mut mcid_parents, &mut annotation_parents)?;
    if annotation_parents.len() != annotations.len() {
      return Err(PdfError::Writer(format!(
        "{} tagged link annotations have {} structure parents",
        annotations.len(),
        annotation_parents.len()
      )));
    }
    let mcid_parents = mcid_parents
      .into_iter()
      .enumerate()
      .map(|(mcid, parent)| {
        parent.ok_or_else(|| {
          PdfError::Writer(format!(
            "page MCID {mcid} has no parent in the structure tree"
          ))
        })
      })
      .collect::<Result<Vec<_>>>()?;
    let (parent_tree_key, parent_array_id) = if mcid_parents.is_empty() {
      (None, None)
    } else {
      let key = self.next_parent_tree_key;
      self.next_parent_tree_key = self
        .next_parent_tree_key
        .checked_add(1)
        .ok_or_else(|| PdfError::Writer("structure ParentTree key space exhausted".to_string()))?;
      (Some(key), Some(refs.alloc()?))
    };
    self.pages.push(TaggedPage {
      page_id,
      part,
      parent_tree_key,
      parent_array_id,
      mcid_parents,
      annotation_parents,
    });
    Ok(parent_tree_key)
  }

  pub(super) fn write_objects(&self, pdf: &mut Pdf) {
    for page in &self.pages {
      page.part.write(pdf, self.document_id, page.page_id);
      if let Some(parent_array_id) = page.parent_array_id {
        pdf
          .indirect(parent_array_id)
          .array()
          .items(page.mcid_parents.iter().copied());
      }
    }

    let mut document = pdf.indirect(self.document_id).start::<StructElement>();
    document.kind(StructRole::Document).parent(self.root_id);
    let mut children = document.children();
    for page in &self.pages {
      children.struct_element(page.part.id);
    }
    children.finish();
    document.finish();

    let mut mappings = BTreeMap::new();
    for page in &self.pages {
      page.part.collect_mappings(&mut mappings);
    }
    let mut root = pdf.indirect(self.root_id).start::<StructTreeRoot>();
    root.children().item(self.document_id);
    if !mappings.is_empty() {
      let mut role_map = root.role_map();
      for (name, role) in &mappings {
        role_map.insert(Name(name), *role);
      }
      role_map.finish();
    }
    if self.next_parent_tree_key != 0 {
      let mut parent_tree = root.parent_tree();
      let mut numbers = parent_tree.nums();
      let mut entries = BTreeMap::new();
      for page in &self.pages {
        if let (Some(key), Some(parent_array_id)) = (page.parent_tree_key, page.parent_array_id) {
          entries.insert(key, parent_array_id);
        }
        for (&key, &parent_id) in &page.annotation_parents {
          entries.insert(key, parent_id);
        }
      }
      for (key, id) in entries {
        numbers.insert(key, id);
      }
      numbers.finish();
      parent_tree.finish();
      root.parent_tree_next_key(self.next_parent_tree_key);
    }
    root.finish();
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParagraphTagKey {
  Frame(usize),
  Source(Vec<usize>),
  Loose(usize),
}

#[derive(Debug)]
struct ParagraphTagBuilder {
  key: ParagraphTagKey,
  is_note: bool,
  text: String,
  children: Vec<LogicalChild>,
}

#[derive(Debug)]
struct TableCellTagBuilder {
  cell_index: usize,
  header: bool,
  children: Vec<LogicalChild>,
}

#[derive(Debug)]
struct TableRowTagBuilder {
  row_index: usize,
  header: bool,
  cells: Vec<TableCellTagBuilder>,
}

#[derive(Debug)]
struct TableTagBuilder {
  frame_index: usize,
  rows: Vec<TableRowTagBuilder>,
}

#[derive(Debug)]
enum PageTagBlock {
  Paragraph(ParagraphTagBuilder),
  Table(TableTagBuilder),
  Node(LogicalNode),
}

fn build_page_part(
  document: &common::LayoutDocument<'static>,
  source_page_index: usize,
  page: &PaintPage<'_>,
  records: &[TaggedPaintRecord],
  annotation_refs: &[(Ref, i32)],
) -> Result<LogicalNode> {
  let mut blocks = Vec::new();
  for record in records {
    let Some(item) = page.items.get(record.item_index) else {
      continue;
    };
    let mut annotations = annotation_children(record, annotation_refs)?;
    match item {
      PaintItem::Text(text) if !text.item.text.is_empty() && text.emits_page_content() => {
        let mut child = tagged_leaf(record.mcid, annotations).ok_or_else(|| {
          PdfError::Writer(format!(
            "tagged text paint item {} has no marked-content identifier",
            record.item_index
          ))
        })?;
        if text.item.style.italic {
          child = LogicalChild::Node(LogicalNode::custom(b"Em", StructRole::Span, child));
        }
        if text.item.style.bold {
          child = LogicalChild::Node(LogicalNode::custom(b"Strong", StructRole::Span, child));
        }
        if let Some(frame_index) = text.tag_source_frame_index()
          && document
            .frames
            .get(frame_index)
            .is_some_and(|frame| frame.kind == "table")
        {
          let (row_index, cell_index, header) = table_cell_position(document, text).unwrap_or((
            text.tag_source_line_index().unwrap_or(0),
            0,
            false,
          ));
          push_table_text_child(
            &mut blocks,
            frame_index,
            row_index,
            cell_index,
            header,
            child,
          );
          continue;
        }

        let key = if let Some(frame_index) = text.tag_source_frame_index() {
          ParagraphTagKey::Frame(frame_index)
        } else if let Some(path) = text.tag_source_path() {
          ParagraphTagKey::Source(path.to_vec())
        } else {
          ParagraphTagKey::Loose(record.item_index)
        };
        let is_note = text
          .tag_source_frame_index()
          .and_then(|index| document.frames.get(index))
          .is_some_and(|frame| frame.kind == "notes");
        push_paragraph_text_child(&mut blocks, key, is_note, text.item.text.as_ref(), child);
      }
      PaintItem::Image(image)
        if image
          .alt_text
          .as_deref()
          .is_some_and(|alt| !alt.trim().is_empty()) =>
      {
        let mcid = record.mcid.ok_or_else(|| {
          PdfError::Writer(format!(
            "tagged image paint item {} has no marked-content identifier",
            record.item_index
          ))
        })?;
        let figure = LogicalNode::figure(
          image.alt_text.as_deref().unwrap_or_default().to_string(),
          mcid,
        );
        let node = if annotations.is_empty() {
          figure
        } else {
          annotations.push(LogicalChild::Node(figure));
          LogicalNode::link(annotations)
        };
        blocks.push(PageTagBlock::Node(node));
      }
      PaintItem::Group { .. } => {
        if let Some(alt_text) = paint_item_alt_text(item) {
          let mcid = record.mcid.ok_or_else(|| {
            PdfError::Writer(format!(
              "tagged group paint item {} has no marked-content identifier",
              record.item_index
            ))
          })?;
          let figure = LogicalNode::figure(alt_text.to_string(), mcid);
          let node = if annotations.is_empty() {
            figure
          } else {
            annotations.push(LogicalChild::Node(figure));
            LogicalNode::link(annotations)
          };
          blocks.push(PageTagBlock::Node(node));
        } else if !annotations.is_empty() {
          blocks.push(PageTagBlock::Node(LogicalNode::link(annotations)));
        }
      }
      _ if !annotations.is_empty() => {
        blocks.push(PageTagBlock::Node(LogicalNode::link(annotations)));
      }
      PaintItem::Text(_)
      | PaintItem::Image(_)
      | PaintItem::LinkArea(_)
      | PaintItem::Rect(_)
      | PaintItem::Line(_)
      | PaintItem::Polyline(_) => {}
    }
  }

  let children = blocks
    .into_iter()
    .map(|block| match block {
      PageTagBlock::Paragraph(paragraph) => {
        LogicalChild::Node(paragraph_tag_node(document, source_page_index, paragraph))
      }
      PageTagBlock::Table(table) => LogicalChild::Node(table_tag_node(table)),
      PageTagBlock::Node(node) => LogicalChild::Node(node),
    })
    .collect();
  Ok(LogicalNode::standard(StructRole::Part, children))
}

fn annotation_children(
  record: &TaggedPaintRecord,
  annotation_refs: &[(Ref, i32)],
) -> Result<Vec<LogicalChild>> {
  record
    .annotation_range
    .clone()
    .map(|index| {
      let &(annotation_id, parent_tree_key) = annotation_refs.get(index).ok_or_else(|| {
        PdfError::Writer(format!(
          "tagged link annotation {index} exceeds the page annotation count {}",
          annotation_refs.len()
        ))
      })?;
      Ok(LogicalChild::ObjectRef {
        annotation_id,
        parent_tree_key,
      })
    })
    .collect()
}

fn tagged_leaf(mcid: Option<i32>, mut annotations: Vec<LogicalChild>) -> Option<LogicalChild> {
  let content = LogicalChild::MarkedContent(mcid?);
  if annotations.is_empty() {
    Some(content)
  } else {
    annotations.push(content);
    Some(LogicalChild::Node(LogicalNode::link(annotations)))
  }
}

fn push_paragraph_text_child(
  blocks: &mut Vec<PageTagBlock>,
  key: ParagraphTagKey,
  is_note: bool,
  text: &str,
  child: LogicalChild,
) {
  if let Some(PageTagBlock::Paragraph(paragraph)) = blocks
    .iter_mut()
    .find(|block| matches!(block, PageTagBlock::Paragraph(paragraph) if paragraph.key == key))
  {
    paragraph.text.push_str(text);
    paragraph.children.push(child);
    return;
  }
  blocks.push(PageTagBlock::Paragraph(ParagraphTagBuilder {
    key,
    is_note,
    text: text.to_string(),
    children: vec![child],
  }));
}

fn push_table_text_child(
  blocks: &mut Vec<PageTagBlock>,
  frame_index: usize,
  row_index: usize,
  cell_index: usize,
  header: bool,
  child: LogicalChild,
) {
  let table_index = blocks
    .iter()
    .position(
      |block| matches!(block, PageTagBlock::Table(table) if table.frame_index == frame_index),
    )
    .unwrap_or_else(|| {
      blocks.push(PageTagBlock::Table(TableTagBuilder {
        frame_index,
        rows: Vec::new(),
      }));
      blocks.len() - 1
    });
  let PageTagBlock::Table(table) = &mut blocks[table_index] else {
    unreachable!();
  };
  let row_index_in_table = table
    .rows
    .iter()
    .position(|row| row.row_index == row_index)
    .unwrap_or_else(|| {
      table.rows.push(TableRowTagBuilder {
        row_index,
        header,
        cells: Vec::new(),
      });
      table.rows.len() - 1
    });
  let row = &mut table.rows[row_index_in_table];
  row.header |= header;
  if let Some(cell) = row
    .cells
    .iter_mut()
    .find(|cell| cell.cell_index == cell_index)
  {
    cell.header |= header;
    cell.children.push(child);
  } else {
    row.cells.push(TableCellTagBuilder {
      cell_index,
      header,
      children: vec![child],
    });
  }
}

fn table_cell_position(
  document: &common::LayoutDocument<'static>,
  text: &PaintText<'_>,
) -> Option<(usize, usize, bool)> {
  let frame = document.frames.get(text.tag_source_frame_index()?)?;
  let line = frame.lines.get(text.tag_source_line_index()?)?;
  frame
    .fragments
    .iter()
    .filter(|fragment| fragment.kind == common::FrameFragmentKind::TableCell)
    .filter(|fragment| {
      fragment.item_range.start < line.item_range.end
        && line.item_range.start < fragment.item_range.end
    })
    .min_by_key(|fragment| fragment.item_range.end - fragment.item_range.start)
    .map(|fragment| {
      (
        fragment.row_index,
        fragment.cell_index.unwrap_or(0),
        fragment.split == common::FragmentSplitKind::RepeatedHeader,
      )
    })
}

fn paragraph_tag_node(
  document: &common::LayoutDocument<'static>,
  page_index: usize,
  paragraph: ParagraphTagBuilder,
) -> LogicalNode {
  if paragraph.is_note {
    return LogicalNode::standard(StructRole::Note, paragraph.children);
  }
  let normalized = normalize_tag_text(&paragraph.text);
  if let Some(outline) = document.outline_entries.iter().find(|entry| {
    entry.page_index == page_index && normalize_tag_text(entry.text.as_ref()) == normalized
  }) {
    heading_tag_node(
      u16::from(outline.level).saturating_add(1),
      paragraph.children,
    )
  } else {
    LogicalNode::standard(StructRole::P, paragraph.children)
  }
}

fn heading_tag_node(level: u16, children: Vec<LogicalChild>) -> LogicalNode {
  let role = match level {
    1 => TagRole::Standard(StructRole::H1),
    2 => TagRole::Standard(StructRole::H2),
    3 => TagRole::Standard(StructRole::H3),
    4 => TagRole::Standard(StructRole::H4),
    5 => TagRole::Standard(StructRole::H5),
    6 => TagRole::Standard(StructRole::H6),
    _ => TagRole::Custom {
      name: format!("H{level}").into_bytes(),
      fallback: StructRole::P,
    },
  };
  LogicalNode {
    role,
    alt_text: None,
    children,
  }
}

fn normalize_tag_text(text: &str) -> String {
  text.split_whitespace().collect()
}

fn table_tag_node(table: TableTagBuilder) -> LogicalNode {
  let mut head = Vec::new();
  let mut body = Vec::new();
  for row in table.rows {
    let cells = row
      .cells
      .into_iter()
      .map(|cell| {
        let paragraph = LogicalNode::standard(StructRole::P, cell.children);
        LogicalChild::Node(LogicalNode::standard(
          if cell.header {
            StructRole::TH
          } else {
            StructRole::TD
          },
          vec![LogicalChild::Node(paragraph)],
        ))
      })
      .collect();
    let row_node = LogicalChild::Node(LogicalNode::standard(StructRole::TR, cells));
    if row.header {
      head.push(row_node);
    } else {
      body.push(row_node);
    }
  }
  let mut children = Vec::new();
  if !head.is_empty() {
    children.push(LogicalChild::Node(LogicalNode::standard(
      StructRole::THead,
      head,
    )));
  }
  if !body.is_empty() {
    children.push(LogicalChild::Node(LogicalNode::standard(
      StructRole::TBody,
      body,
    )));
  }
  LogicalNode::standard(StructRole::Table, children)
}
