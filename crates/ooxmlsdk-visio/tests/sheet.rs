use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cell, Master, Masters, Page, PageContents, PageSheet, PageSheetChoice, Pages, RowUnion,
  RowUnionChoice, Section, SectionChoice, Shape, ShapeChoice, Shapes,
};
use ooxmlsdk_visio::{master_by_id, page_by_id, page_sheet_cell, shape_by_id, shape_cell};

fn cell(name: &str, value: &str, formula: Option<&str>, unit: Option<&str>) -> Cell {
  Cell {
    n: name.to_string(),
    v: Some(value.to_string()),
    f: formula.map(str::to_string),
    u: unit.map(str::to_string),
    ..Default::default()
  }
}

fn sample_shape() -> Shape {
  Shape {
    id: 7,
    shape_choice: vec![ShapeChoice::Section(Section {
      n: "Geometry".to_string(),
      section_choice: vec![
        SectionChoice::Cell(Box::new(cell("NoFill", "0", None, None))),
        SectionChoice::RowUnion(Box::new(RowUnion {
          t: Some("LineTo".to_string()),
          row_union_choice: vec![RowUnionChoice::Cell(Box::new(cell(
            "X",
            "1",
            Some("Width*0.5"),
            Some("IN"),
          )))],
          ..Default::default()
        })),
      ],
      ..Default::default()
    })],
    shapes: Some(Shapes {
      shape: vec![Shape {
        id: 8,
        ..Default::default()
      }],
    }),
    ..Default::default()
  }
}

#[test]
fn ids_and_stored_cells_are_readable() {
  let pages = Pages {
    page: vec![
      Page {
        id: 1,
        ..Default::default()
      },
      Page {
        id: 3,
        name: Some("Later".to_string()),
        ..Default::default()
      },
    ],
    ..Default::default()
  };
  assert_eq!(
    page_by_id(&pages, 3).unwrap().name.as_deref(),
    Some("Later")
  );
  assert!(page_by_id(&pages, 9).is_none());

  let masters = Masters {
    master: vec![Master {
      id: 4,
      name: Some("Box".to_string()),
      ..Default::default()
    }],
    ..Default::default()
  };
  assert_eq!(
    master_by_id(&masters, 4).unwrap().name.as_deref(),
    Some("Box")
  );
  assert!(master_by_id(&masters, 1).is_none());

  let contents = PageContents {
    shapes: Some(Shapes {
      shape: vec![sample_shape()],
    }),
    ..Default::default()
  };
  let nested = shape_by_id(contents.shapes.as_ref().unwrap(), 8).unwrap();
  assert_eq!(nested.id, 8);
  let shape = shape_by_id(contents.shapes.as_ref().unwrap(), 7).unwrap();
  let line = shape_cell(shape, "Geometry", "X").unwrap();
  assert_eq!(line.value, Some("1"));
  assert_eq!(line.formula, Some("Width*0.5"));
  assert_eq!(line.unit, Some("IN"));
  assert_eq!(line.error, None);
  assert_eq!(
    shape_cell(shape, "Geometry", "NoFill").unwrap().value,
    Some("0")
  );
  assert!(shape_cell(shape, "Geometry", "Missing").is_none());

  let sheet = PageSheet {
    page_sheet_choice: vec![PageSheetChoice::Section(Section {
      n: "PageProps".to_string(),
      section_choice: vec![SectionChoice::Cell(Box::new(cell(
        "PageWidth",
        "8.5",
        None,
        Some("IN"),
      )))],
      ..Default::default()
    })],
    ..Default::default()
  };
  let width = page_sheet_cell(&sheet, "PageProps", "PageWidth").unwrap();
  assert_eq!(width.value, Some("8.5"));
  assert_eq!(width.unit, Some("IN"));
  assert_eq!(width.formula, None);
}
