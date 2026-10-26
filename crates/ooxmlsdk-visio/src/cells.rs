use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cell, DocumentSheet, DocumentSheetChoice, PageSheet, PageSheetChoice, RowUnion, RowUnionChoice,
  Section, SectionChoice, Shape, ShapeChoice, StyleSheet, StyleSheetChoice, Tab,
};

use crate::sheet::{CellRecord, is_deleted, record};

/// A `Section` addressed by `N`, and by `IX` when the caller sets one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionAddress<'a> {
  pub name: &'a str,
  pub index: Option<u32>,
}

/// A `Row` addressed by `IX`, by `N`, or by both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowAddress<'a> {
  pub index: Option<u32>,
  pub name: Option<&'a str>,
}

/// Read a direct `Cell` on a shape. `shape_cell` still looks only at sections.
pub fn read_direct_cell<'a>(shape: &'a Shape, name: &str) -> Option<CellRecord<'a>> {
  shape.shape_choice.iter().find_map(|choice| match choice {
    ShapeChoice::Cell(cell) if cell.n == name => Some(record(cell)),
    _ => None,
  })
}

/// Write stored `V` on a direct cell. An existing `F` stays unless `formula`
/// is set. A new cell omits `F`. Other children stay in place.
pub fn write_direct_cell(shape: &mut Shape, name: &str, value: &str, formula: Option<&str>) {
  if let Some(index) = shape
    .shape_choice
    .iter()
    .position(|choice| matches!(choice, ShapeChoice::Cell(cell) if cell.n == name))
  {
    let ShapeChoice::Cell(cell) = &mut shape.shape_choice[index] else {
      return;
    };
    store_value(cell, value, formula);
    return;
  }
  shape
    .shape_choice
    .push(ShapeChoice::Cell(Box::new(new_cell(name, value, formula))));
}

pub fn read_page_direct_cell<'a>(sheet: &'a PageSheet, name: &str) -> Option<CellRecord<'a>> {
  sheet
    .page_sheet_choice
    .iter()
    .find_map(|choice| match choice {
      PageSheetChoice::Cell(cell) if cell.n == name => Some(record(cell)),
      _ => None,
    })
}

pub fn write_page_direct_cell(
  sheet: &mut PageSheet,
  name: &str,
  value: &str,
  formula: Option<&str>,
) {
  if let Some(index) = sheet
    .page_sheet_choice
    .iter()
    .position(|choice| matches!(choice, PageSheetChoice::Cell(cell) if cell.n == name))
  {
    let PageSheetChoice::Cell(cell) = &mut sheet.page_sheet_choice[index] else {
      return;
    };
    store_value(cell, value, formula);
    return;
  }
  sheet
    .page_sheet_choice
    .push(PageSheetChoice::Cell(Box::new(new_cell(
      name, value, formula,
    ))));
}

macro_rules! shape_cells {
  ($($fn_name:ident => $cell:literal),* $(,)?) => {
    $(
      pub fn $fn_name(shape: &mut Shape, value: &str) {
        write_direct_cell(shape, $cell, value, None);
      }
    )*
  };
}

shape_cells! {
  pin_x => "PinX",
  pin_y => "PinY",
  width => "Width",
  height => "Height",
  loc_pin_x => "LocPinX",
  loc_pin_y => "LocPinY",
  angle => "Angle",
  flip_x => "FlipX",
  flip_y => "FlipY",
  line_weight => "LineWeight",
  line_color => "LineColor",
  line_color_trans => "LineColorTrans",
  fill_foregnd => "FillForegnd",
  fill_foregnd_trans => "FillForegndTrans",
  begin_x => "BeginX",
  begin_y => "BeginY",
  end_x => "EndX",
  end_y => "EndY",
  img_offset_x => "ImgOffsetX",
  img_offset_y => "ImgOffsetY",
  img_width => "ImgWidth",
  img_height => "ImgHeight",
  clipping_path => "ClippingPath",
  layer_member => "LayerMember",
}

macro_rules! page_cells {
  ($($fn_name:ident => $cell:literal),* $(,)?) => {
    $(
      pub fn $fn_name(sheet: &mut PageSheet, value: &str) {
        write_page_direct_cell(sheet, $cell, value, None);
      }
    )*
  };
}

page_cells! {
  page_width => "PageWidth",
  page_height => "PageHeight",
  page_scale => "PageScale",
  drawing_scale => "DrawingScale",
}

/// Read a cell in a section. `row` is `None` for a cell that sits on the
/// section itself, such as Geometry `NoFill`.
pub fn read_section_cell<'a>(
  shape: &'a Shape,
  section: SectionAddress<'_>,
  row: Option<RowAddress<'_>>,
  cell: &str,
) -> Option<CellRecord<'a>> {
  let section = find_shape_section(shape, section)?;
  read_in_section(section, row, cell)
}

pub fn write_section_cell(
  shape: &mut Shape,
  section: SectionAddress<'_>,
  row: Option<RowAddress<'_>>,
  cell: &str,
  value: &str,
  formula: Option<&str>,
) {
  let index = ensure_shape_section(shape, section);
  let ShapeChoice::Section(section) = &mut shape.shape_choice[index] else {
    return;
  };
  write_in_section(section, row, cell, value, formula);
}

pub fn read_page_section_cell<'a>(
  sheet: &'a PageSheet,
  section: SectionAddress<'_>,
  row: Option<RowAddress<'_>>,
  cell: &str,
) -> Option<CellRecord<'a>> {
  let section = find_page_section(sheet, section)?;
  read_in_section(section, row, cell)
}

pub fn write_page_section_cell(
  sheet: &mut PageSheet,
  section: SectionAddress<'_>,
  row: Option<RowAddress<'_>>,
  cell: &str,
  value: &str,
  formula: Option<&str>,
) {
  let index = ensure_page_section(sheet, section);
  let PageSheetChoice::Section(section) = &mut sheet.page_sheet_choice[index] else {
    return;
  };
  write_in_section(section, row, cell, value, formula);
}

pub fn read_style_direct_cell<'a>(style: &'a StyleSheet, name: &str) -> Option<CellRecord<'a>> {
  style
    .style_sheet_choice
    .iter()
    .find_map(|choice| match choice {
      StyleSheetChoice::Cell(cell) if cell.n == name => Some(record(cell)),
      _ => None,
    })
}

pub fn write_style_direct_cell(
  style: &mut StyleSheet,
  name: &str,
  value: &str,
  formula: Option<&str>,
) {
  if let Some(index) = style
    .style_sheet_choice
    .iter()
    .position(|choice| matches!(choice, StyleSheetChoice::Cell(cell) if cell.n == name))
  {
    let StyleSheetChoice::Cell(cell) = &mut style.style_sheet_choice[index] else {
      return;
    };
    store_value(cell, value, formula);
    return;
  }
  style
    .style_sheet_choice
    .push(StyleSheetChoice::Cell(Box::new(new_cell(
      name, value, formula,
    ))));
}

pub fn read_style_section_cell<'a>(
  style: &'a StyleSheet,
  section: SectionAddress<'_>,
  row: Option<RowAddress<'_>>,
  cell: &str,
) -> Option<CellRecord<'a>> {
  let section = find_style_section(style, section)?;
  read_in_section(section, row, cell)
}

pub fn write_style_section_cell(
  style: &mut StyleSheet,
  section: SectionAddress<'_>,
  row: Option<RowAddress<'_>>,
  cell: &str,
  value: &str,
  formula: Option<&str>,
) {
  let index = ensure_style_section(style, section);
  let StyleSheetChoice::Section(section) = &mut style.style_sheet_choice[index] else {
    return;
  };
  write_in_section(section, row, cell, value, formula);
}

pub fn read_document_direct_cell<'a>(
  sheet: &'a DocumentSheet,
  name: &str,
) -> Option<CellRecord<'a>> {
  sheet
    .document_sheet_choice
    .iter()
    .find_map(|choice| match choice {
      DocumentSheetChoice::Cell(cell) if cell.n == name => Some(record(cell)),
      _ => None,
    })
}

pub fn write_document_direct_cell(
  sheet: &mut DocumentSheet,
  name: &str,
  value: &str,
  formula: Option<&str>,
) {
  if let Some(index) = sheet
    .document_sheet_choice
    .iter()
    .position(|choice| matches!(choice, DocumentSheetChoice::Cell(cell) if cell.n == name))
  {
    let DocumentSheetChoice::Cell(cell) = &mut sheet.document_sheet_choice[index] else {
      return;
    };
    store_value(cell, value, formula);
    return;
  }
  sheet
    .document_sheet_choice
    .push(DocumentSheetChoice::Cell(Box::new(new_cell(
      name, value, formula,
    ))));
}

pub fn read_document_section_cell<'a>(
  sheet: &'a DocumentSheet,
  section: SectionAddress<'_>,
  row: Option<RowAddress<'_>>,
  cell: &str,
) -> Option<CellRecord<'a>> {
  let section = find_document_section(sheet, section)?;
  read_in_section(section, row, cell)
}

pub fn write_document_section_cell(
  sheet: &mut DocumentSheet,
  section: SectionAddress<'_>,
  row: Option<RowAddress<'_>>,
  cell: &str,
  value: &str,
  formula: Option<&str>,
) {
  let index = ensure_document_section(sheet, section);
  let DocumentSheetChoice::Section(section) = &mut sheet.document_sheet_choice[index] else {
    return;
  };
  write_in_section(section, row, cell, value, formula);
}

/// One geometry row. Cell sets follow the `T` value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeometryRow {
  MoveTo {
    x: String,
    y: String,
  },
  LineTo {
    x: String,
    y: String,
  },
  RelMoveTo {
    x: String,
    y: String,
  },
  RelLineTo {
    x: String,
    y: String,
  },
  ArcTo {
    x: String,
    y: String,
    a: String,
  },
  SplineKnot {
    x: String,
    y: String,
    a: String,
  },
  PolylineTo {
    x: String,
    y: String,
    a: String,
  },
  InfiniteLine {
    x: String,
    y: String,
    a: String,
    b: String,
  },
  RelQuadBezTo {
    x: String,
    y: String,
    a: String,
    b: String,
  },
  Ellipse {
    x: String,
    y: String,
    a: String,
    b: String,
    c: String,
    d: String,
  },
  EllipticalArcTo {
    x: String,
    y: String,
    a: String,
    b: String,
    c: String,
    d: String,
  },
  RelEllipticalArcTo {
    x: String,
    y: String,
    a: String,
    b: String,
    c: String,
    d: String,
  },
  SplineStart {
    x: String,
    y: String,
    a: String,
    b: String,
    c: String,
    d: String,
  },
  RelCubBezTo {
    x: String,
    y: String,
    a: String,
    b: String,
    c: String,
    d: String,
  },
  NurbsTo {
    x: String,
    y: String,
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
  },
}

impl GeometryRow {
  fn kind(&self) -> &'static str {
    match self {
      Self::MoveTo { .. } => "MoveTo",
      Self::LineTo { .. } => "LineTo",
      Self::RelMoveTo { .. } => "RelMoveTo",
      Self::RelLineTo { .. } => "RelLineTo",
      Self::ArcTo { .. } => "ArcTo",
      Self::SplineKnot { .. } => "SplineKnot",
      Self::PolylineTo { .. } => "PolylineTo",
      Self::InfiniteLine { .. } => "InfiniteLine",
      Self::RelQuadBezTo { .. } => "RelQuadBezTo",
      Self::Ellipse { .. } => "Ellipse",
      Self::EllipticalArcTo { .. } => "EllipticalArcTo",
      Self::RelEllipticalArcTo { .. } => "RelEllipticalArcTo",
      Self::SplineStart { .. } => "SplineStart",
      Self::RelCubBezTo { .. } => "RelCubBezTo",
      Self::NurbsTo { .. } => "NURBSTo",
    }
  }

  fn cells(&self) -> Vec<(&'static str, &str)> {
    match self {
      Self::MoveTo { x, y }
      | Self::LineTo { x, y }
      | Self::RelMoveTo { x, y }
      | Self::RelLineTo { x, y } => vec![("X", x), ("Y", y)],
      Self::ArcTo { x, y, a } | Self::SplineKnot { x, y, a } | Self::PolylineTo { x, y, a } => {
        vec![("X", x), ("Y", y), ("A", a)]
      }
      Self::InfiniteLine { x, y, a, b } | Self::RelQuadBezTo { x, y, a, b } => {
        vec![("X", x), ("Y", y), ("A", a), ("B", b)]
      }
      Self::Ellipse { x, y, a, b, c, d }
      | Self::EllipticalArcTo { x, y, a, b, c, d }
      | Self::RelEllipticalArcTo { x, y, a, b, c, d }
      | Self::SplineStart { x, y, a, b, c, d }
      | Self::RelCubBezTo { x, y, a, b, c, d } => {
        vec![("X", x), ("Y", y), ("A", a), ("B", b), ("C", c), ("D", d)]
      }
      Self::NurbsTo {
        x,
        y,
        a,
        b,
        c,
        d,
        e,
      } => vec![
        ("X", x),
        ("Y", y),
        ("A", a),
        ("B", b),
        ("C", c),
        ("D", d),
        ("E", e),
      ],
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredGeometryRow {
  pub index: Option<u32>,
  pub kind: Option<String>,
  pub cells: Vec<(String, Option<String>)>,
}

/// Append a geometry row. The row's `IX` is the next index in that section.
/// `section_index` selects a second `Geometry` section when the shape has one.
pub fn append_geometry_row(
  shape: &mut Shape,
  section_index: Option<u32>,
  row: &GeometryRow,
) -> u32 {
  let index = ensure_shape_section(
    shape,
    SectionAddress {
      name: "Geometry",
      index: section_index,
    },
  );
  let ShapeChoice::Section(section) = &mut shape.shape_choice[index] else {
    return 0;
  };
  let ix = next_row_index(section);
  let mut stored = RowUnion {
    t: Some(row.kind().to_string()),
    ix: Some(ix),
    ..Default::default()
  };
  for (name, value) in row.cells() {
    stored
      .row_union_choice
      .push(RowUnionChoice::Cell(Box::new(new_cell(name, value, None))));
  }
  section
    .section_choice
    .push(SectionChoice::RowUnion(Box::new(stored)));
  ix
}

pub fn geometry_rows(shape: &Shape, section_index: Option<u32>) -> Vec<StoredGeometryRow> {
  let Some(section) = find_shape_section(
    shape,
    SectionAddress {
      name: "Geometry",
      index: section_index,
    },
  ) else {
    return Vec::new();
  };
  section
    .section_choice
    .iter()
    .filter_map(|item| {
      let SectionChoice::RowUnion(row) = item else {
        return None;
      };
      if is_deleted(row.del) {
        return None;
      }
      Some(StoredGeometryRow {
        index: row.ix,
        kind: row.t.clone(),
        cells: row
          .row_union_choice
          .iter()
          .filter_map(|choice| {
            let RowUnionChoice::Cell(cell) = choice else {
              return None;
            };
            Some((cell.n.clone(), cell.v.clone()))
          })
          .collect(),
      })
    })
    .collect()
}

/// Append a `Layer` row on the page sheet and assign the next `IX`.
pub fn append_layer(sheet: &mut PageSheet, cells: &[(&str, &str)]) -> u32 {
  let index = ensure_page_section(
    sheet,
    SectionAddress {
      name: "Layer",
      index: None,
    },
  );
  let PageSheetChoice::Section(section) = &mut sheet.page_sheet_choice[index] else {
    return 0;
  };
  let ix = next_row_index(section);
  section
    .section_choice
    .push(SectionChoice::RowUnion(Box::new(named_row(
      None,
      Some(ix),
      cells,
    ))));
  ix
}

/// Write cells on a `Hyperlink` row named `row_name`. The row is created when
/// it is missing.
pub fn write_hyperlink(shape: &mut Shape, row_name: &str, cells: &[(&str, &str)]) {
  write_named_row(shape, "Hyperlink", row_name, cells);
}

/// Write cells on a `Property` row named `row_name`. Shape data uses this
/// section name.
pub fn write_property(shape: &mut Shape, row_name: &str, cells: &[(&str, &str)]) {
  write_named_row(shape, "Property", row_name, cells);
}

/// Write cells on a `User` row named `row_name`.
pub fn write_user(shape: &mut Shape, row_name: &str, cells: &[(&str, &str)]) {
  write_named_row(shape, "User", row_name, cells);
}

/// Write cells on an `ActionTag` row named `row_name`.
pub fn write_action_tag(shape: &mut Shape, row_name: &str, cells: &[(&str, &str)]) {
  write_named_row(shape, "ActionTag", row_name, cells);
}

/// Write cells on an `Actions` row named `row_name` and assign `IX` when the
/// row is new.
pub fn write_actions(shape: &mut Shape, row_name: &str, cells: &[(&str, &str)]) -> u32 {
  write_named_indexed_row(shape, "Actions", row_name, cells)
}

/// Write cells on a `Connection` row named `row_name` and assign `IX` when the
/// row is new.
pub fn write_connection(shape: &mut Shape, row_name: &str, cells: &[(&str, &str)]) -> u32 {
  write_named_indexed_row(shape, "Connection", row_name, cells)
}

/// Write cells on a `Control` row named `row_name` and assign `IX` when the
/// row is new.
pub fn write_control(shape: &mut Shape, row_name: &str, cells: &[(&str, &str)]) -> u32 {
  write_named_indexed_row(shape, "Control", row_name, cells)
}

/// Append a `Character` row and assign the next `IX`.
pub fn append_character_row(shape: &mut Shape, cells: &[(&str, &str)]) -> u32 {
  append_indexed_row(shape, "Character", cells)
}

/// Append a `Paragraph` row and assign the next `IX`.
pub fn append_paragraph_row(shape: &mut Shape, cells: &[(&str, &str)]) -> u32 {
  append_indexed_row(shape, "Paragraph", cells)
}

/// Append a `Scratch` row and assign the next `IX`.
pub fn append_scratch_row(shape: &mut Shape, cells: &[(&str, &str)]) -> u32 {
  append_indexed_row(shape, "Scratch", cells)
}

/// Append a `Field` row and assign the next `IX`.
pub fn append_field_row(shape: &mut Shape, cells: &[(&str, &str)]) -> u32 {
  append_indexed_row(shape, "Field", cells)
}

/// Append a `Reviewer` row and assign the next `IX`.
pub fn append_reviewer_row(shape: &mut Shape, cells: &[(&str, &str)]) -> u32 {
  append_indexed_row(shape, "Reviewer", cells)
}

/// Append an `Annotation` row and assign the next `IX`.
pub fn append_annotation_row(shape: &mut Shape, cells: &[(&str, &str)]) -> u32 {
  append_indexed_row(shape, "Annotation", cells)
}

/// Append a `LineGradient` stop and assign the next `IX`.
pub fn append_line_gradient_row(shape: &mut Shape, cells: &[(&str, &str)]) -> u32 {
  append_indexed_row(shape, "LineGradient", cells)
}

/// Append a `FillGradient` stop and assign the next `IX`.
pub fn append_fill_gradient_row(shape: &mut Shape, cells: &[(&str, &str)]) -> u32 {
  append_indexed_row(shape, "FillGradient", cells)
}

/// Append a tab stop on the first `Tabs` row. The returned `IX` is the tab's.
pub fn append_tab(shape: &mut Shape, position: &str, alignment: &str) -> u32 {
  let index = ensure_shape_section(
    shape,
    SectionAddress {
      name: "Tabs",
      index: None,
    },
  );
  let ShapeChoice::Section(section) = &mut shape.shape_choice[index] else {
    return 0;
  };
  if !section
    .section_choice
    .iter()
    .any(|item| matches!(item, SectionChoice::RowUnion(row) if !is_deleted(row.del)))
  {
    let ix = next_row_index(section);
    section
      .section_choice
      .push(SectionChoice::RowUnion(Box::new(RowUnion {
        ix: Some(ix),
        ..Default::default()
      })));
  }
  let Some(row_index) = section
    .section_choice
    .iter()
    .position(|item| matches!(item, SectionChoice::RowUnion(row) if !is_deleted(row.del)))
  else {
    return 0;
  };
  let SectionChoice::RowUnion(row) = &mut section.section_choice[row_index] else {
    return 0;
  };
  let tab_ix = row
    .row_union_choice
    .iter()
    .filter_map(|choice| match choice {
      RowUnionChoice::Tab(tab) => tab.ix,
      RowUnionChoice::Cell(_) => None,
    })
    .max()
    .map(|ix| ix.saturating_add(1))
    .unwrap_or(0);
  row.row_union_choice.push(RowUnionChoice::Tab(Tab {
    ix: Some(tab_ix),
    cell: vec![
      new_cell("Position", position, None),
      new_cell("Alignment", alignment, None),
    ],
  }));
  tab_ix
}

fn append_indexed_row(shape: &mut Shape, section_name: &str, cells: &[(&str, &str)]) -> u32 {
  let index = ensure_shape_section(
    shape,
    SectionAddress {
      name: section_name,
      index: None,
    },
  );
  let ShapeChoice::Section(section) = &mut shape.shape_choice[index] else {
    return 0;
  };
  let ix = next_row_index(section);
  section
    .section_choice
    .push(SectionChoice::RowUnion(Box::new(named_row(
      None,
      Some(ix),
      cells,
    ))));
  ix
}

fn write_named_indexed_row(
  shape: &mut Shape,
  section_name: &str,
  row_name: &str,
  cells: &[(&str, &str)],
) -> u32 {
  let index = ensure_shape_section(
    shape,
    SectionAddress {
      name: section_name,
      index: None,
    },
  );
  let ShapeChoice::Section(section) = &mut shape.shape_choice[index] else {
    return 0;
  };
  let address = RowAddress {
    index: None,
    name: Some(row_name),
  };
  let row_index = match section
    .section_choice
    .iter()
    .position(|item| matches!(item, SectionChoice::RowUnion(row) if row_matches(row, address)))
  {
    Some(found) => found,
    None => {
      let ix = next_row_index(section);
      section
        .section_choice
        .push(SectionChoice::RowUnion(Box::new(RowUnion {
          n: Some(row_name.to_string()),
          ix: Some(ix),
          ..Default::default()
        })));
      section.section_choice.len() - 1
    }
  };
  let ix = match &section.section_choice[row_index] {
    SectionChoice::RowUnion(row) => row.ix.unwrap_or(0),
    SectionChoice::Cell(_) => 0,
  };
  for (name, value) in cells {
    let SectionChoice::RowUnion(row) = &mut section.section_choice[row_index] else {
      return ix;
    };
    write_row_cell(row, name, value, None);
  }
  ix
}

fn write_named_row(shape: &mut Shape, section_name: &str, row_name: &str, cells: &[(&str, &str)]) {
  let section = SectionAddress {
    name: section_name,
    index: None,
  };
  let row = Some(RowAddress {
    index: None,
    name: Some(row_name),
  });
  if cells.is_empty() {
    let index = ensure_shape_section(shape, section);
    let ShapeChoice::Section(section) = &mut shape.shape_choice[index] else {
      return;
    };
    ensure_row(
      section,
      RowAddress {
        index: None,
        name: Some(row_name),
      },
    );
    return;
  }
  for (name, value) in cells {
    write_section_cell(shape, section, row, name, value, None);
  }
}

fn read_in_section<'a>(
  section: &'a Section,
  row: Option<RowAddress<'_>>,
  cell: &str,
) -> Option<CellRecord<'a>> {
  match row_target(row) {
    None => section.section_choice.iter().find_map(|item| match item {
      SectionChoice::Cell(cell_value) if cell_value.n == cell => Some(record(cell_value)),
      _ => None,
    }),
    Some(row) => section.section_choice.iter().find_map(|item| {
      let SectionChoice::RowUnion(stored) = item else {
        return None;
      };
      if !row_matches(stored, row) {
        return None;
      }
      stored.row_union_choice.iter().find_map(|choice| {
        let RowUnionChoice::Cell(cell_value) = choice else {
          return None;
        };
        (cell_value.n == cell).then(|| record(cell_value))
      })
    }),
  }
}

fn write_in_section(
  section: &mut Section,
  row: Option<RowAddress<'_>>,
  cell: &str,
  value: &str,
  formula: Option<&str>,
) {
  match row_target(row) {
    None => write_section_level_cell(section, cell, value, formula),
    Some(row) => {
      let index = ensure_row(section, row);
      let SectionChoice::RowUnion(stored) = &mut section.section_choice[index] else {
        return;
      };
      write_row_cell(stored, cell, value, formula);
    }
  }
}

fn row_target(row: Option<RowAddress<'_>>) -> Option<RowAddress<'_>> {
  row.filter(|row| row.index.is_some() || row.name.is_some())
}

fn write_section_level_cell(section: &mut Section, cell: &str, value: &str, formula: Option<&str>) {
  if let Some(index) = section
    .section_choice
    .iter()
    .position(|item| matches!(item, SectionChoice::Cell(stored) if stored.n == cell))
  {
    let SectionChoice::Cell(stored) = &mut section.section_choice[index] else {
      return;
    };
    store_value(stored, value, formula);
    return;
  }
  section
    .section_choice
    .push(SectionChoice::Cell(Box::new(new_cell(
      cell, value, formula,
    ))));
}

fn write_row_cell(row: &mut RowUnion, cell: &str, value: &str, formula: Option<&str>) {
  if let Some(index) = row
    .row_union_choice
    .iter()
    .position(|choice| matches!(choice, RowUnionChoice::Cell(stored) if stored.n == cell))
  {
    let RowUnionChoice::Cell(stored) = &mut row.row_union_choice[index] else {
      return;
    };
    store_value(stored, value, formula);
    return;
  }
  row
    .row_union_choice
    .push(RowUnionChoice::Cell(Box::new(new_cell(
      cell, value, formula,
    ))));
}

fn ensure_row(section: &mut Section, row: RowAddress<'_>) -> usize {
  if let Some(index) = section
    .section_choice
    .iter()
    .position(|item| matches!(item, SectionChoice::RowUnion(stored) if row_matches(stored, row)))
  {
    return index;
  }
  let index = section.section_choice.len();
  section
    .section_choice
    .push(SectionChoice::RowUnion(Box::new(RowUnion {
      n: row.name.map(str::to_string),
      ix: row.index,
      ..Default::default()
    })));
  index
}

fn named_row(name: Option<&str>, index: Option<u32>, cells: &[(&str, &str)]) -> RowUnion {
  let mut row = RowUnion {
    n: name.map(str::to_string),
    ix: index,
    ..Default::default()
  };
  for (cell, value) in cells {
    row
      .row_union_choice
      .push(RowUnionChoice::Cell(Box::new(new_cell(cell, value, None))));
  }
  row
}

fn find_shape_section<'a>(shape: &'a Shape, address: SectionAddress<'_>) -> Option<&'a Section> {
  shape.shape_choice.iter().find_map(|choice| {
    let ShapeChoice::Section(section) = choice else {
      return None;
    };
    section_matches(section, address).then_some(section)
  })
}

fn find_page_section<'a>(sheet: &'a PageSheet, address: SectionAddress<'_>) -> Option<&'a Section> {
  sheet.page_sheet_choice.iter().find_map(|choice| {
    let PageSheetChoice::Section(section) = choice else {
      return None;
    };
    section_matches(section, address).then_some(section)
  })
}

fn ensure_shape_section(shape: &mut Shape, address: SectionAddress<'_>) -> usize {
  if let Some(index) = shape.shape_choice.iter().position(
    |choice| matches!(choice, ShapeChoice::Section(section) if section_matches(section, address)),
  ) {
    return index;
  }
  let index = shape.shape_choice.len();
  shape
    .shape_choice
    .push(ShapeChoice::Section(new_section(address)));
  index
}

fn find_style_section<'a>(
  style: &'a StyleSheet,
  address: SectionAddress<'_>,
) -> Option<&'a Section> {
  style.style_sheet_choice.iter().find_map(|choice| {
    let StyleSheetChoice::Section(section) = choice else {
      return None;
    };
    section_matches(section, address).then_some(section)
  })
}

fn find_document_section<'a>(
  sheet: &'a DocumentSheet,
  address: SectionAddress<'_>,
) -> Option<&'a Section> {
  sheet.document_sheet_choice.iter().find_map(|choice| {
    let DocumentSheetChoice::Section(section) = choice else {
      return None;
    };
    section_matches(section, address).then_some(section)
  })
}

fn ensure_style_section(style: &mut StyleSheet, address: SectionAddress<'_>) -> usize {
  if let Some(index) = style.style_sheet_choice.iter().position(|choice| {
    matches!(choice, StyleSheetChoice::Section(section) if section_matches(section, address))
  }) {
    return index;
  }
  let index = style.style_sheet_choice.len();
  style
    .style_sheet_choice
    .push(StyleSheetChoice::Section(new_section(address)));
  index
}

fn ensure_document_section(sheet: &mut DocumentSheet, address: SectionAddress<'_>) -> usize {
  if let Some(index) = sheet.document_sheet_choice.iter().position(|choice| {
    matches!(choice, DocumentSheetChoice::Section(section) if section_matches(section, address))
  }) {
    return index;
  }
  let index = sheet.document_sheet_choice.len();
  sheet
    .document_sheet_choice
    .push(DocumentSheetChoice::Section(new_section(address)));
  index
}

fn ensure_page_section(sheet: &mut PageSheet, address: SectionAddress<'_>) -> usize {
  if let Some(index) = sheet.page_sheet_choice.iter().position(|choice| {
    matches!(choice, PageSheetChoice::Section(section) if section_matches(section, address))
  }) {
    return index;
  }
  let index = sheet.page_sheet_choice.len();
  sheet
    .page_sheet_choice
    .push(PageSheetChoice::Section(new_section(address)));
  index
}

fn new_section(address: SectionAddress<'_>) -> Section {
  Section {
    n: address.name.to_string(),
    ix: address.index,
    ..Default::default()
  }
}

fn section_matches(section: &Section, address: SectionAddress<'_>) -> bool {
  if section.n != address.name || is_deleted(section.del) {
    return false;
  }
  address.index.is_none_or(|index| section.ix == Some(index))
}

fn row_matches(row: &RowUnion, address: RowAddress<'_>) -> bool {
  if is_deleted(row.del) {
    return false;
  }
  let index_ok = address.index.is_none_or(|index| row.ix == Some(index));
  let name_ok = address
    .name
    .is_none_or(|name| row.n.as_deref() == Some(name));
  index_ok && name_ok
}

fn next_row_index(section: &Section) -> u32 {
  section
    .section_choice
    .iter()
    .filter_map(|item| match item {
      SectionChoice::RowUnion(row) => row.ix,
      SectionChoice::Cell(_) => None,
    })
    .max()
    .map(|index| index.saturating_add(1))
    .unwrap_or(0)
}

fn new_cell(name: &str, value: &str, formula: Option<&str>) -> Cell {
  Cell {
    n: name.to_string(),
    v: Some(value.to_string()),
    f: formula.map(str::to_string),
    ..Default::default()
  }
}

fn store_value(cell: &mut Cell, value: &str, formula: Option<&str>) {
  cell.v = Some(value.to_string());
  if let Some(formula) = formula {
    cell.f = Some(formula.to_string());
  }
}
