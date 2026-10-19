use std::io::Cursor;

use ooxmlsdk::parts::image_part::ImagePart;
use ooxmlsdk::parts::visio_page_part::VisioPagePart;
use ooxmlsdk::parts::visio_pages_part::VisioPagesPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  PageContents, PageSheet, Pages, Shape, ShapeChoice,
};
use ooxmlsdk::sdk::{SdkPart, SdkType};
use ooxmlsdk::simple_type::BooleanValue;
use ooxmlsdk_visio::{
  AuthorRecord, CommentRecord, ConnectLink, GeometryRow, Ids, ImageBytes, RowAddress,
  SectionAddress, TextRun, add_author, add_comment, add_connect, add_page, append_child_shape,
  append_geometry_row, append_layer, attach_image, begin_x, end_y, geometry_rows,
  image_relationship_id, layer_member, line_color, list_authors, list_comments, minimal_drawing,
  page_by_id, pin_x, read_direct_cell, read_page_direct_cell, read_page_section_cell,
  read_section_cell, read_shape_text, resolve_connects, shape_by_id, shape_by_id_mut,
  write_direct_cell, write_hyperlink, write_property, write_section_cell, write_shape_text,
};

const PAGES: &str = "http://schemas.microsoft.com/visio/2010/relationships/pages";
const PAGE: &str = "http://schemas.microsoft.com/visio/2010/relationships/page";
const IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";

fn round_trip(contents: &PageContents) -> PageContents {
  let xml = contents.to_xml().expect("write page");
  PageContents::from_bytes(xml.as_bytes()).expect("read page")
}

fn shape() -> Shape {
  Shape {
    id: 1,
    shape_choice: vec![ShapeChoice::Cell(Box::new(
      ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::Cell {
        n: "PinX".to_string(),
        v: Some("1".to_string()),
        f: Some("Width*0.5".to_string()),
        ..Default::default()
      },
    ))],
    ..Default::default()
  }
}

#[test]
fn direct_cell_keeps_formula_and_siblings() {
  let mut shape = shape();
  pin_x(&mut shape, "2");
  write_direct_cell(&mut shape, "Width", "4", Some("Guard(4)"));
  line_color(&mut shape, "#000000");
  let page = round_trip(&PageContents {
    shapes: Some(
      ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::Shapes {
        shape: vec![shape],
      },
    ),
    ..Default::default()
  });
  let shape = shape_by_id(page.shapes.as_ref().unwrap(), 1).unwrap();
  let pin = read_direct_cell(shape, "PinX").unwrap();
  assert_eq!(pin.value, Some("2"));
  assert_eq!(pin.formula, Some("Width*0.5"));
  let width = read_direct_cell(shape, "Width").unwrap();
  assert_eq!(width.value, Some("4"));
  assert_eq!(width.formula, Some("Guard(4)"));
  assert_eq!(
    read_direct_cell(shape, "LineColor").unwrap().value,
    Some("#000000")
  );
  assert!(matches!(
    shape.shape_choice.first(),
    Some(ShapeChoice::Cell(cell)) if cell.n == "PinX"
  ));
}

#[test]
fn section_geometry_text_and_rows_round_trip() {
  let mut shape = shape();
  write_section_cell(
    &mut shape,
    SectionAddress {
      name: "Geometry",
      index: None,
    },
    None,
    "NoFill",
    "1",
    None,
  );
  let ix = append_geometry_row(
    &mut shape,
    None,
    &GeometryRow::LineTo {
      x: "1".to_string(),
      y: "2".to_string(),
    },
  );
  assert_eq!(ix, 0);
  let second = append_geometry_row(
    &mut shape,
    Some(1),
    &GeometryRow::NurbsTo {
      x: "0".to_string(),
      y: "1".to_string(),
      a: "2".to_string(),
      b: "3".to_string(),
      c: "4".to_string(),
      d: "5".to_string(),
      e: "6".to_string(),
    },
  );
  assert_eq!(second, 0);
  write_section_cell(
    &mut shape,
    SectionAddress {
      name: "Character",
      index: None,
    },
    Some(RowAddress {
      index: Some(0),
      name: None,
    }),
    "Font",
    "Calibri",
    None,
  );
  write_section_cell(
    &mut shape,
    SectionAddress {
      name: "Paragraph",
      index: None,
    },
    Some(RowAddress {
      index: Some(0),
      name: None,
    }),
    "IndFirst",
    "0.1",
    Some("0"),
  );
  write_section_cell(
    &mut shape,
    SectionAddress {
      name: "Paragraph",
      index: None,
    },
    Some(RowAddress {
      index: Some(0),
      name: None,
    }),
    "IndFirst",
    "0.2",
    None,
  );
  write_hyperlink(
    &mut shape,
    "Row_1",
    &[
      ("Address", "https://example.test"),
      ("Description", "Example"),
    ],
  );
  write_property(&mut shape, "Cost", &[("Value", "12"), ("Label", "Cost")]);
  write_shape_text(
    &mut shape,
    &[
      TextRun::Character(0),
      TextRun::Paragraph(0),
      TextRun::Chars("Hello".to_string()),
    ],
  );
  let mut ids = Ids::new();
  let _parent = ids.shape();
  let child = append_child_shape(&mut shape, &mut ids);
  shape.r#type = Some("Group".to_string());

  let page = round_trip(&PageContents {
    shapes: Some(
      ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::Shapes {
        shape: vec![shape],
      },
    ),
    ..Default::default()
  });
  let shape = shape_by_id(page.shapes.as_ref().unwrap(), 1).unwrap();
  assert_eq!(
    read_section_cell(
      shape,
      SectionAddress {
        name: "Geometry",
        index: None
      },
      None,
      "NoFill"
    )
    .unwrap()
    .value,
    Some("1")
  );
  let rows = geometry_rows(shape, None);
  assert_eq!(rows.len(), 1);
  assert_eq!(rows[0].kind.as_deref(), Some("LineTo"));
  assert_eq!(
    rows[0].cells,
    vec![
      ("X".to_string(), Some("1".to_string())),
      ("Y".to_string(), Some("2".to_string())),
    ]
  );
  let nurbs = geometry_rows(shape, Some(1));
  assert_eq!(nurbs[0].kind.as_deref(), Some("NURBSTo"));
  assert_eq!(nurbs[0].cells.last().unwrap().0, "E");
  assert_eq!(nurbs[0].cells.last().unwrap().1.as_deref(), Some("6"));
  let font = read_section_cell(
    shape,
    SectionAddress {
      name: "Character",
      index: None,
    },
    Some(RowAddress {
      index: Some(0),
      name: None,
    }),
    "Font",
  )
  .unwrap();
  assert_eq!(font.value, Some("Calibri"));
  let indent = read_section_cell(
    shape,
    SectionAddress {
      name: "Paragraph",
      index: None,
    },
    Some(RowAddress {
      index: Some(0),
      name: None,
    }),
    "IndFirst",
  )
  .unwrap();
  assert_eq!(indent.value, Some("0.2"));
  assert_eq!(indent.formula, Some("0"));
  assert_eq!(
    read_section_cell(
      shape,
      SectionAddress {
        name: "Hyperlink",
        index: None
      },
      Some(RowAddress {
        index: None,
        name: Some("Row_1")
      }),
      "Address"
    )
    .unwrap()
    .value,
    Some("https://example.test")
  );
  assert_eq!(
    read_section_cell(
      shape,
      SectionAddress {
        name: "Property",
        index: None
      },
      Some(RowAddress {
        index: None,
        name: Some("Cost")
      }),
      "Value"
    )
    .unwrap()
    .value,
    Some("12")
  );
  assert_eq!(
    read_shape_text(shape),
    vec![
      TextRun::Character(0),
      TextRun::Paragraph(0),
      TextRun::Chars("Hello".to_string()),
    ]
  );
  assert_eq!(shape.r#type.as_deref(), Some("Group"));
  assert_eq!(
    shape_by_id(shape.shapes.as_ref().unwrap(), child)
      .unwrap()
      .id,
    child
  );
}

#[test]
fn connect_and_layer_round_trip() {
  let mut page = PageContents {
    shapes: Some(
      ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::Shapes {
        shape: vec![
          Shape {
            id: 1,
            ..Default::default()
          },
          Shape {
            id: 2,
            ..Default::default()
          },
        ],
      },
    ),
    ..Default::default()
  };
  let shapes = page.shapes.as_mut().unwrap();
  begin_x(shape_by_id_mut(shapes, 2).unwrap(), "0");
  end_y(shape_by_id_mut(shapes, 2).unwrap(), "1");
  layer_member(shape_by_id_mut(shapes, 1).unwrap(), "0");
  add_connect(
    &mut page,
    ConnectLink {
      from_sheet: 2,
      to_sheet: 1,
      from_cell: Some("EndX"),
      from_part: Some(9),
      to_cell: Some("PinX"),
      to_part: None,
    },
  )
  .unwrap();
  assert!(
    add_connect(
      &mut page,
      ConnectLink {
        from_sheet: 9,
        to_sheet: 1,
        from_cell: None,
        from_part: None,
        to_cell: None,
        to_part: None,
      },
    )
    .is_err()
  );
  let page = round_trip(&page);
  let resolved = resolve_connects(&page).unwrap();
  assert_eq!(resolved.len(), 1);
  assert_eq!(resolved[0].from.id, 2);
  assert_eq!(resolved[0].to.id, 1);
  assert_eq!(resolved[0].from_cell, Some("EndX"));
  let from = shape_by_id(page.shapes.as_ref().unwrap(), 2).unwrap();
  assert_eq!(read_direct_cell(from, "BeginX").unwrap().value, Some("0"));
  assert_eq!(read_direct_cell(from, "EndY").unwrap().value, Some("1"));

  let mut sheet = PageSheet::default();
  let ix = append_layer(
    &mut sheet,
    &[("Name", "Desk"), ("Visible", "1"), ("ColorTrans", "0")],
  );
  let sheet_xml = sheet.to_xml().unwrap();
  let sheet = PageSheet::from_bytes(sheet_xml.as_bytes()).unwrap();
  let name = read_page_section_cell(
    &sheet,
    SectionAddress {
      name: "Layer",
      index: None,
    },
    Some(RowAddress {
      index: Some(ix),
      name: None,
    }),
    "Name",
  )
  .unwrap();
  assert_eq!(name.value, Some("Desk"));
  assert!(read_page_direct_cell(&sheet, "PageWidth").is_none());
}

#[test]
fn page_image_and_comments_round_trip() {
  let mut ids = Ids::new();
  let mut package = minimal_drawing(&mut ids).unwrap();
  let page_id = add_page(&mut package, &mut ids, Some("Page-2"), "8.5", "11").unwrap();
  assert_eq!(page_id, 2);

  let document = package.visio_document_part().unwrap();
  let pages_part = document
    .child_part_by_relationship_type::<_, VisioPagesPart>(&package, PAGES)
    .unwrap();
  let page_parts: Vec<_> = pages_part
    .child_related_parts_by_relationship_type::<_, VisioPagePart>(&package, PAGE)
    .map(|related| related.part().clone())
    .collect();
  let page_part = page_parts
    .into_iter()
    .find(|part| {
      let mut bytes = Vec::new();
      part.write_data_to(&package, &mut bytes).unwrap();
      PageContents::from_bytes(&bytes)
        .ok()
        .and_then(|contents| contents.shapes)
        .is_some()
    })
    .unwrap();
  let mut bytes = Vec::new();
  page_part.write_data_to(&package, &mut bytes).unwrap();
  let mut contents = PageContents::from_bytes(&bytes).unwrap();
  let shape = shape_by_id_mut(contents.shapes.as_mut().unwrap(), 1).unwrap();
  let rel_id = attach_image(
    shape,
    &page_part,
    &mut package,
    ImageBytes {
      content_type: "image/png",
      bytes: b"png-bytes",
      foreign_type: "Bitmap",
      object_type: None,
      show_as_icon: Some(false),
      object_width: None,
      object_height: None,
      mapping_mode: None,
      extent_x: Some(1.5),
      extent_y: Some(2.5),
      compression_type: None,
      compression_level: None,
    },
  )
  .unwrap();
  ooxmlsdk_visio::img_offset_x(shape, "0");
  ooxmlsdk_visio::img_width(shape, "1.5");
  page_part
    .set_data(&mut package, contents.to_xml().unwrap().into_bytes())
    .unwrap();

  add_author(
    &mut package,
    AuthorRecord {
      id: 1,
      name: Some("Ada".to_string()),
      initials: Some("A".to_string()),
      sip: None,
      smtp: Some("ada@example.test".to_string()),
      resolution_id: None,
    },
  )
  .unwrap();
  add_comment(
    &mut package,
    CommentRecord {
      author_id: 1,
      page_id: 1,
      comment_id: 3,
      date: "2026-10-12T00:00:00".to_string(),
      text: Some("Check the box".to_string()),
      shape_id: Some(1),
      edit_date: None,
      done: Some(BooleanValue::Zero),
      auto_comment_type: None,
    },
  )
  .unwrap();

  let saved = package.to_package_bytes().unwrap();
  let opened = ooxmlsdk::parts::visio_document::VisioDocument::new(Cursor::new(saved)).unwrap();
  let document = opened.visio_document_part().unwrap();
  let pages_part = document
    .child_part_by_relationship_type::<_, VisioPagesPart>(&opened, PAGES)
    .unwrap();
  let mut pages_bytes = Vec::new();
  pages_part.write_data_to(&opened, &mut pages_bytes).unwrap();
  let pages = Pages::from_bytes(&pages_bytes).unwrap();
  let page = page_by_id(&pages, 2).unwrap();
  let width = page
    .page_choice
    .iter()
    .find_map(|choice| match choice {
      ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::PageChoice::PageSheet(
        sheet,
      ) => read_page_direct_cell(sheet, "PageWidth"),
      _ => None,
    })
    .unwrap();
  assert_eq!(width.value, Some("8.5"));
  assert_eq!(
    read_page_direct_cell(
      page
        .page_choice
        .iter()
        .find_map(|choice| {
          match choice {
        ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::PageChoice::PageSheet(
          sheet,
        ) => Some(sheet.as_ref()),
        _ => None,
      }
        })
        .unwrap(),
      "PageHeight"
    )
    .unwrap()
    .value,
    Some("11")
  );

  let page_parts: Vec<_> = pages_part
    .child_related_parts_by_relationship_type::<_, VisioPagePart>(&opened, PAGE)
    .map(|related| related.part().clone())
    .collect();
  let page_part = page_parts
    .into_iter()
    .find(|part| {
      let mut page_bytes = Vec::new();
      part.write_data_to(&opened, &mut page_bytes).unwrap();
      PageContents::from_bytes(&page_bytes)
        .ok()
        .and_then(|contents| contents.shapes)
        .is_some()
    })
    .unwrap();
  let mut page_bytes = Vec::new();
  page_part.write_data_to(&opened, &mut page_bytes).unwrap();
  let contents = PageContents::from_bytes(&page_bytes).unwrap();
  let shape = shape_by_id(contents.shapes.as_ref().unwrap(), 1).unwrap();
  assert_eq!(image_relationship_id(shape), Some(rel_id.as_str()));
  assert_eq!(
    read_direct_cell(shape, "ImgOffsetX").unwrap().value,
    Some("0")
  );
  let images: Vec<_> = page_part
    .child_related_parts_by_relationship_type::<_, ImagePart>(&opened, IMAGE)
    .map(|related| {
      (
        related.relationship_id().to_string(),
        related.part().clone(),
      )
    })
    .collect();
  let (_, image) = images.into_iter().find(|(id, _)| id == &rel_id).unwrap();
  let mut image_bytes = Vec::new();
  image.write_data_to(&opened, &mut image_bytes).unwrap();
  assert_eq!(image_bytes, b"png-bytes");

  assert_eq!(
    list_authors(&opened).unwrap()[0].name.as_deref(),
    Some("Ada")
  );
  let comments = list_comments(&opened).unwrap();
  assert_eq!(comments[0].text.as_deref(), Some("Check the box"));
  assert_eq!(comments[0].done, Some(BooleanValue::Zero));
}
