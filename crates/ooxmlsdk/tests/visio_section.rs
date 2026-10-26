use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main as visio;
use ooxmlsdk::sdk::SdkType;

fn cell(name: &str, value: &str, formula: Option<&str>) -> visio::Cell {
  visio::Cell {
    n: name.to_string(),
    v: Some(value.to_string()),
    f: formula.map(str::to_string),
    ..Default::default()
  }
}

fn geometry_and_user_page() -> visio::PageContents {
  visio::PageContents {
    shapes: Some(visio::Shapes {
      shape: vec![visio::Shape {
        id: 1,
        shape_choice: vec![
          visio::ShapeChoice::Section(visio::Section {
            n: "Geometry".to_string(),
            section_choice: vec![
              visio::SectionChoice::Cell(Box::new(cell("NoFill", "0", None))),
              visio::SectionChoice::RowUnion(Box::new(visio::RowUnion {
                t: Some("LineTo".to_string()),
                ix: Some(2),
                row_union_choice: vec![
                  visio::RowUnionChoice::Cell(Box::new(cell("X", "1", Some("Width*0.5")))),
                  visio::RowUnionChoice::Cell(Box::new(cell("Y", "2", None))),
                ],
                ..Default::default()
              })),
            ],
            ..Default::default()
          }),
          visio::ShapeChoice::Section(visio::Section {
            n: "User".to_string(),
            section_choice: vec![visio::SectionChoice::RowUnion(Box::new(visio::RowUnion {
              n: Some("Foo".to_string()),
              row_union_choice: vec![
                visio::RowUnionChoice::Cell(Box::new(cell("Value", "bar", None))),
                visio::RowUnionChoice::Cell(Box::new(cell("Prompt", "ask", None))),
              ],
              ..Default::default()
            }))],
            ..Default::default()
          }),
        ],
        ..Default::default()
      }],
    }),
    ..Default::default()
  }
}

fn sheet_cells(
  page: &visio::PageContents,
) -> Vec<(String, String, String, String, Option<String>)> {
  let mut found = Vec::new();
  let Some(shapes) = &page.shapes else {
    return found;
  };
  for shape in &shapes.shape {
    for choice in &shape.shape_choice {
      let visio::ShapeChoice::Section(section) = choice else {
        continue;
      };
      for item in &section.section_choice {
        match item {
          visio::SectionChoice::Cell(cell) => {
            found.push((
              section.n.clone(),
              String::new(),
              cell.n.clone(),
              cell.v.clone().unwrap_or_default(),
              cell.f.clone(),
            ));
          }
          visio::SectionChoice::RowUnion(row) => {
            let row_key = row.t.clone().or_else(|| row.n.clone()).unwrap_or_default();
            for choice in &row.row_union_choice {
              let visio::RowUnionChoice::Cell(cell) = choice else {
                continue;
              };
              found.push((
                section.n.clone(),
                row_key.clone(),
                cell.n.clone(),
                cell.v.clone().unwrap_or_default(),
                cell.f.clone(),
              ));
            }
          }
        }
      }
    }
  }
  found
}

#[test]
fn section_rows_round_trip_geometry_line_and_user_cells() {
  let page = geometry_and_user_page();
  let xml = page.to_xml().expect("write page contents");
  let read = visio::PageContents::from_bytes(xml.as_bytes()).expect("read page contents");
  let cells = sheet_cells(&read);
  assert_eq!(
    cells,
    vec![
      (
        "Geometry".to_string(),
        String::new(),
        "NoFill".to_string(),
        "0".to_string(),
        None,
      ),
      (
        "Geometry".to_string(),
        "LineTo".to_string(),
        "X".to_string(),
        "1".to_string(),
        Some("Width*0.5".to_string()),
      ),
      (
        "Geometry".to_string(),
        "LineTo".to_string(),
        "Y".to_string(),
        "2".to_string(),
        None,
      ),
      (
        "User".to_string(),
        "Foo".to_string(),
        "Value".to_string(),
        "bar".to_string(),
        None,
      ),
      (
        "User".to_string(),
        "Foo".to_string(),
        "Prompt".to_string(),
        "ask".to_string(),
        None,
      ),
    ]
  );
  assert_eq!(read.shapes.expect("shapes").shape[0].id, 1);
}
