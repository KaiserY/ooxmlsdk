use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cell, Master, Masters, Page, PageSheet, PageSheetChoice, Pages, RowUnion, RowUnionChoice,
  Section, SectionChoice, Shape, ShapeChoice, Shapes,
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

pub fn shape_cell<'a>(shape: &'a Shape, section: &str, cell: &str) -> Option<CellRecord<'a>> {
  let sections = shape.shape_choice.iter().filter_map(|choice| match choice {
    ShapeChoice::Section(section) => Some(section),
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

fn record(cell: &Cell) -> CellRecord<'_> {
  CellRecord {
    value: cell.v.as_deref(),
    formula: cell.f.as_deref(),
    unit: cell.u.as_deref(),
    error: cell.e.as_deref(),
  }
}

fn is_deleted(flag: Option<BooleanValue>) -> bool {
  matches!(flag, Some(BooleanValue::True | BooleanValue::One))
}
