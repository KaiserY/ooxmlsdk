use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cell, RowUnion, RowUnionChoice, Section, SectionChoice, Shape, ShapeChoice, Shapes,
};
use ooxmlsdk::simple_type::BooleanValue;
use ooxmlsdk_visio::{MasterShapes, effective_shape_cell};

fn cell(name: &str, value: &str, formula: Option<&str>) -> Cell {
  Cell {
    n: name.to_string(),
    v: Some(value.to_string()),
    f: formula.map(str::to_string),
    ..Default::default()
  }
}

fn master_shape() -> Shape {
  Shape {
    id: 1,
    shape_choice: vec![
      ShapeChoice::Section(Section {
        n: "Geometry".to_string(),
        section_choice: vec![SectionChoice::RowUnion(Box::new(RowUnion {
          t: Some("LineTo".to_string()),
          ix: Some(1),
          row_union_choice: vec![RowUnionChoice::Cell(Box::new(cell(
            "X",
            "5",
            Some("Width*0.5"),
          )))],
          ..Default::default()
        }))],
        ..Default::default()
      }),
      ShapeChoice::Section(Section {
        n: "User".to_string(),
        section_choice: vec![SectionChoice::RowUnion(Box::new(RowUnion {
          n: Some("Foo".to_string()),
          row_union_choice: vec![RowUnionChoice::Cell(Box::new(cell(
            "Value", "master", None,
          )))],
          ..Default::default()
        }))],
        ..Default::default()
      }),
    ],
    ..Default::default()
  }
}

fn instance(local: Vec<ShapeChoice>) -> Shape {
  Shape {
    id: 10,
    master: Some(4),
    master_shape: Some(1),
    shape_choice: local,
    ..Default::default()
  }
}

#[test]
fn local_value_wins_and_deleted_rows_do_not_inherit() {
  let master = master_shape();
  let master_shapes = Shapes {
    shape: vec![master],
  };
  let masters = [MasterShapes {
    id: 4,
    shapes: &master_shapes,
  }];
  let bare = instance(Vec::new());
  let inherited = effective_shape_cell(&bare, "Geometry", "X", &masters).unwrap();
  assert_eq!(inherited.value, Some("5"));
  assert_eq!(inherited.formula, Some("Width*0.5"));

  let local = instance(vec![ShapeChoice::Section(Section {
    n: "Geometry".to_string(),
    section_choice: vec![SectionChoice::RowUnion(Box::new(RowUnion {
      t: Some("LineTo".to_string()),
      ix: Some(1),
      row_union_choice: vec![RowUnionChoice::Cell(Box::new(cell("X", "1", None)))],
      ..Default::default()
    }))],
    ..Default::default()
  })]);
  assert_eq!(
    effective_shape_cell(&local, "Geometry", "X", &masters)
      .unwrap()
      .value,
    Some("1")
  );

  let deleted = instance(vec![ShapeChoice::Section(Section {
    n: "User".to_string(),
    section_choice: vec![SectionChoice::RowUnion(Box::new(RowUnion {
      n: Some("Foo".to_string()),
      del: Some(BooleanValue::One),
      ..Default::default()
    }))],
    ..Default::default()
  })]);
  assert!(effective_shape_cell(&deleted, "User", "Value", &masters).is_none());
  assert_eq!(
    effective_shape_cell(&deleted, "Geometry", "X", &masters)
      .unwrap()
      .value,
    Some("5")
  );
}
