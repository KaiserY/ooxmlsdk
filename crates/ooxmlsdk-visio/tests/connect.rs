use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Connect, Connects, PageContents, Shape, Shapes,
};
use ooxmlsdk_visio::{ConnectError, resolve_connects};

fn shape(id: u32) -> Shape {
  Shape {
    id,
    ..Default::default()
  }
}

fn page_with(shapes: Vec<Shape>, connects: Vec<Connect>) -> PageContents {
  PageContents {
    shapes: Some(Shapes { shape: shapes }),
    connects: Some(Connects { connect: connects }),
    ..Default::default()
  }
}

#[test]
fn connect_resolves_shapes_on_the_same_page() {
  let page = page_with(
    vec![shape(1), shape(2)],
    vec![Connect {
      from_sheet: 1,
      to_sheet: 2,
      from_cell: Some("EndX".to_string()),
      to_cell: Some("PinX".to_string()),
      ..Default::default()
    }],
  );
  let resolved = resolve_connects(&page).expect("both shapes exist");
  assert_eq!(resolved.len(), 1);
  assert_eq!(resolved[0].from.id, 1);
  assert_eq!(resolved[0].to.id, 2);
  assert_eq!(resolved[0].from_cell, Some("EndX"));
  assert_eq!(resolved[0].to_cell, Some("PinX"));

  let missing = page_with(
    vec![shape(1)],
    vec![Connect {
      from_sheet: 1,
      to_sheet: 99,
      ..Default::default()
    }],
  );
  assert_eq!(resolve_connects(&missing), Err(ConnectError::MissingTo(99)));
}
