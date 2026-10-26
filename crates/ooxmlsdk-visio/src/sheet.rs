use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cell, Master, Masters, Page, PageSheet, PageSheetChoice, Pages, RowUnion, RowUnionChoice,
  Section, SectionChoice, Shape, ShapeChoice, Shapes, StyleSheet, StyleSheetChoice,
};
use ooxmlsdk::simple_type::BooleanValue;

/// Stored ShapeSheet fields. `formula` is the `F` attribute, not a computed result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellRecord<'a> {
  pub value: Option<&'a str>,
  pub formula: Option<&'a str>,
  pub unit: Option<&'a str>,
  pub error: Option<&'a str>,
}

pub fn page_by_id(pages: &Pages, id: u32) -> Option<&Page> {
  pages.page.iter().find(|page| page.id == id)
}

pub fn master_by_id(masters: &Masters, id: u32) -> Option<&Master> {
  masters.master.iter().find(|master| master.id == id)
}

pub fn shape_by_id(shapes: &Shapes, id: u32) -> Option<&Shape> {
  for shape in &shapes.shape {
    if shape.id == id {
      return Some(shape);
    }
    if let Some(nested) = &shape.shapes
      && let Some(found) = shape_by_id(nested, id)
    {
      return Some(found);
    }
  }
  None
}

pub fn shape_by_id_mut(shapes: &mut Shapes, id: u32) -> Option<&mut Shape> {
  shapes.shape.iter_mut().find_map(|shape| {
    if shape.id == id {
      Some(shape)
    } else {
      shape
        .shapes
        .as_mut()
        .and_then(|nested| shape_by_id_mut(nested, id))
    }
  })
}

/// Append a shape and take its id from `ids`.
pub fn append_shape(shapes: &mut Shapes, ids: &mut crate::ids::Ids) -> u32 {
  let id = ids.shape();
  shapes.shape.push(Shape {
    id,
    ..Default::default()
  });
  id
}

/// Append a child shape and take its id from `ids`. The caller sets `Type`
/// when the parent is a group.
pub fn append_child_shape(parent: &mut Shape, ids: &mut crate::ids::Ids) -> u32 {
  let shapes = parent.shapes.get_or_insert_with(Shapes::default);
  append_shape(shapes, ids)
}

/// Store the master and master-shape ids on an instance. Local cells still
/// override the master when they are present.
pub fn set_master_instance(shape: &mut Shape, master_id: u32, master_shape_id: u32) {
  shape.master = Some(master_id);
  shape.master_shape = Some(master_shape_id);
}

pub fn shape_cell<'a>(shape: &'a Shape, section: &str, cell: &str) -> Option<CellRecord<'a>> {
  let sections = shape.shape_choice.iter().filter_map(|choice| match choice {
    ShapeChoice::Section(section) => Some(section),
    _ => None,
  });
  cell_in_sections(sections, section, cell)
}

/// Shapes that belong to one master, keyed by the master's id.
pub struct MasterShapes<'a> {
  pub id: u32,
  pub shapes: &'a Shapes,
}

/// Local cell when the instance has one. Otherwise the master shape's stored
/// cell, addressed by `Shape.Master` and `Shape.MasterShape`. A deleted section
/// or row does not inherit. Formulas are not evaluated.
pub fn effective_shape_cell<'a>(
  shape: &'a Shape,
  section: &str,
  cell: &str,
  masters: &[MasterShapes<'a>],
) -> Option<CellRecord<'a>> {
  match local_cell(shape, section, cell) {
    LocalCell::Found(record) => return Some(record),
    LocalCell::Blocked => return None,
    LocalCell::Absent => {}
  }
  let master_id = shape.master?;
  let master_shape_id = shape.master_shape?;
  let master_shapes = masters.iter().find(|master| master.id == master_id)?.shapes;
  let master_shape = shape_by_id(master_shapes, master_shape_id)?;
  let (record, row) = master_cell(master_shape, section, cell)?;
  if local_row_deleted(shape, section, row) {
    return None;
  }
  Some(record)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StyleSlot {
  Line,
  Fill,
  Text,
}

/// Local section cell when the shape has one. Otherwise the stored cell on the
/// style sheet named by `LineStyle`, `FillStyle`, or `TextStyle`. Theme
/// formulas stay as text.
pub fn styled_shape_cell<'a>(
  shape: &'a Shape,
  styles: &'a [StyleSheet],
  slot: StyleSlot,
  section: &str,
  cell: &str,
) -> Option<CellRecord<'a>> {
  if let Some(local) = shape_cell(shape, section, cell) {
    return Some(local);
  }
  let style_id = match slot {
    StyleSlot::Line => shape.line_style,
    StyleSlot::Fill => shape.fill_style,
    StyleSlot::Text => shape.text_style,
  }?;
  let style = styles.iter().find(|style| style.id == style_id)?;
  let sections = style
    .style_sheet_choice
    .iter()
    .filter_map(|choice| match choice {
      StyleSheetChoice::Section(section) => Some(section),
      _ => None,
    });
  cell_in_sections(sections, section, cell)
}

pub fn page_sheet_cell<'a>(
  sheet: &'a PageSheet,
  section: &str,
  cell: &str,
) -> Option<CellRecord<'a>> {
  let sections = sheet
    .page_sheet_choice
    .iter()
    .filter_map(|choice| match choice {
      PageSheetChoice::Section(section) => Some(section),
      _ => None,
    });
  cell_in_sections(sections, section, cell)
}

fn cell_in_sections<'a>(
  sections: impl Iterator<Item = &'a Section>,
  section_name: &str,
  cell_name: &str,
) -> Option<CellRecord<'a>> {
  for section in sections {
    if section.n != section_name || is_deleted(section.del) {
      continue;
    }
    for item in &section.section_choice {
      match item {
        SectionChoice::Cell(cell) if cell.n == cell_name => return Some(record(cell)),
        SectionChoice::RowUnion(row) if !is_deleted(row.del) => {
          if let Some(found) = cell_in_row(row, cell_name) {
            return Some(found);
          }
        }
        _ => {}
      }
    }
  }
  None
}

fn cell_in_row<'a>(row: &'a RowUnion, cell_name: &str) -> Option<CellRecord<'a>> {
  for choice in &row.row_union_choice {
    if let RowUnionChoice::Cell(cell) = choice
      && cell.n == cell_name
    {
      return Some(record(cell));
    }
  }
  None
}

pub(crate) fn record(cell: &Cell) -> CellRecord<'_> {
  CellRecord {
    value: cell.v.as_deref(),
    formula: cell.f.as_deref(),
    unit: cell.u.as_deref(),
    error: cell.e.as_deref(),
  }
}

pub(crate) fn is_deleted(flag: Option<BooleanValue>) -> bool {
  flag.is_some_and(BooleanValue::as_bool)
}

enum LocalCell<'a> {
  Found(CellRecord<'a>),
  Blocked,
  Absent,
}

#[derive(Clone, Copy)]
struct RowKey<'a> {
  name: Option<&'a str>,
  kind: Option<&'a str>,
  index: Option<u32>,
}

fn local_cell<'a>(shape: &'a Shape, section_name: &str, cell_name: &str) -> LocalCell<'a> {
  for choice in &shape.shape_choice {
    let ShapeChoice::Section(section) = choice else {
      continue;
    };
    if section.n != section_name {
      continue;
    }
    if is_deleted(section.del) {
      return LocalCell::Blocked;
    }
    for item in &section.section_choice {
      match item {
        SectionChoice::Cell(cell) if cell.n == cell_name => return LocalCell::Found(record(cell)),
        SectionChoice::RowUnion(row) if !is_deleted(row.del) => {
          if let Some(found) = cell_in_row(row, cell_name) {
            return LocalCell::Found(found);
          }
        }
        _ => {}
      }
    }
  }
  LocalCell::Absent
}

fn master_cell<'a>(
  shape: &'a Shape,
  section_name: &str,
  cell_name: &str,
) -> Option<(CellRecord<'a>, Option<RowKey<'a>>)> {
  for choice in &shape.shape_choice {
    let ShapeChoice::Section(section) = choice else {
      continue;
    };
    if section.n != section_name || is_deleted(section.del) {
      continue;
    }
    for item in &section.section_choice {
      match item {
        SectionChoice::Cell(cell) if cell.n == cell_name => {
          return Some((record(cell), None));
        }
        SectionChoice::RowUnion(row) if !is_deleted(row.del) => {
          if let Some(found) = cell_in_row(row, cell_name) {
            return Some((
              found,
              Some(RowKey {
                name: row.n.as_deref(),
                kind: row.t.as_deref(),
                index: row.ix,
              }),
            ));
          }
        }
        _ => {}
      }
    }
  }
  None
}

fn local_row_deleted(shape: &Shape, section_name: &str, row: Option<RowKey<'_>>) -> bool {
  let Some(row) = row else {
    return false;
  };
  shape.shape_choice.iter().any(|choice| {
    let ShapeChoice::Section(section) = choice else {
      return false;
    };
    if section.n != section_name || is_deleted(section.del) {
      return false;
    }
    section.section_choice.iter().any(|item| {
      let SectionChoice::RowUnion(local) = item else {
        return false;
      };
      is_deleted(local.del) && same_row(local, row)
    })
  })
}

fn same_row(local: &RowUnion, master: RowKey<'_>) -> bool {
  if master.name.is_some() {
    return local.n.as_deref() == master.name;
  }
  local.t.as_deref() == master.kind && local.ix == master.index
}
