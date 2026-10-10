use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cell, Section, SectionChoice, Shape, ShapeChoice, StyleSheet, StyleSheetChoice,
};
use ooxmlsdk_visio::{
  StyleSlot, TextRun, read_shape_text, shape_cell, styled_shape_cell, write_shape_text,
};

#[test]
fn text_marks_round_trip_in_order() {
  let mut shape = Shape {
    id: 1,
    shape_choice: vec![ShapeChoice::Cell(Box::new(Cell {
      n: "PinX".to_string(),
      v: Some("1".to_string()),
      ..Default::default()
    }))],
    ..Default::default()
  };
  let runs = vec![
    TextRun::Character(0),
    TextRun::Paragraph(0),
    TextRun::Tabs(1),
    TextRun::Field {
      index: 2,
      text: Some("hello".to_string()),
    },
  ];
  write_shape_text(&mut shape, &runs);
  assert_eq!(read_shape_text(&shape), runs);
  assert!(
    shape
      .shape_choice
      .iter()
      .any(|choice| matches!(choice, ShapeChoice::Cell(_)))
  );
}

#[test]
fn style_sheet_supplies_missing_cell_value() {
  let styles = [StyleSheet {
    id: 2,
    style_sheet_choice: vec![StyleSheetChoice::Section(Section {
      n: "Line".to_string(),
      section_choice: vec![SectionChoice::Cell(Box::new(Cell {
        n: "LineColor".to_string(),
        v: Some("2".to_string()),
        f: Some("THEMEVAL(\"AccentColor\")".to_string()),
        ..Default::default()
      }))],
      ..Default::default()
    })],
    ..Default::default()
  }];
  let mut shape = Shape {
    id: 1,
    line_style: Some(2),
    ..Default::default()
  };
  let from_style =
    styled_shape_cell(&shape, &styles, StyleSlot::Line, "Line", "LineColor").expect("style cell");
  assert_eq!(from_style.value, Some("2"));
  assert_eq!(from_style.formula, Some("THEMEVAL(\"AccentColor\")"));

  shape.shape_choice.push(ShapeChoice::Section(Section {
    n: "Line".to_string(),
    section_choice: vec![SectionChoice::Cell(Box::new(Cell {
      n: "LineColor".to_string(),
      v: Some("1".to_string()),
      ..Default::default()
    }))],
    ..Default::default()
  }));
  assert_eq!(
    styled_shape_cell(&shape, &styles, StyleSlot::Line, "Line", "LineColor")
      .unwrap()
      .value,
    Some("1")
  );
  assert_eq!(
    shape_cell(&shape, "Line", "LineColor").unwrap().value,
    Some("1")
  );
}
