use std::io::Cursor;

use ooxmlsdk::parts::visio_page_part::VisioPagePart;
use ooxmlsdk::parts::visio_pages_part::VisioPagesPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  PageContents, Pages, ShapeChoice,
};
use ooxmlsdk::sdk::{SdkPart, SdkType};
use ooxmlsdk_visio::{Ids, minimal_drawing, shape_by_id, shape_cell};

const PAGES: &str = "http://schemas.microsoft.com/visio/2010/relationships/pages";
const PAGE: &str = "http://schemas.microsoft.com/visio/2010/relationships/page";

#[test]
fn ids_increase_inside_each_sequence() {
  let mut ids = Ids::new();
  assert_eq!((ids.page(), ids.page(), ids.page()), (1, 2, 3));
  assert_eq!((ids.master(), ids.master()), (1, 2));
  assert_eq!((ids.shape(), ids.shape()), (1, 2));
}

#[test]
fn minimal_drawing_round_trip_keeps_pin_and_line() {
  let mut ids = Ids::new();
  let package = minimal_drawing(&mut ids).expect("build drawing");
  let bytes = package.to_package_bytes().expect("save drawing");
  let opened =
    ooxmlsdk::parts::visio_document::VisioDocument::new(Cursor::new(bytes)).expect("open drawing");
  let document = opened.visio_document_part().expect("document part");
  let pages_part = document
    .child_part_by_relationship_type::<_, VisioPagesPart>(&opened, PAGES)
    .expect("pages part");
  let mut pages_bytes = Vec::new();
  pages_part
    .write_data_to(&opened, &mut pages_bytes)
    .expect("read pages");
  let pages = Pages::from_bytes(&pages_bytes).expect("parse pages");
  assert_eq!(pages.page.len(), 1);
  assert_eq!(pages.page[0].id, 1);

  let page_part = pages_part
    .child_part_by_relationship_type::<_, VisioPagePart>(&opened, PAGE)
    .expect("page part");
  let mut page_bytes = Vec::new();
  page_part
    .write_data_to(&opened, &mut page_bytes)
    .expect("read page");
  let contents = PageContents::from_bytes(&page_bytes).expect("parse page");
  let shapes = contents.shapes.as_ref().expect("shapes");
  let shape = shape_by_id(shapes, 1).expect("shape");
  let pin = shape
    .shape_choice
    .iter()
    .find_map(|choice| match choice {
      ShapeChoice::Cell(cell) if cell.n == "PinX" => Some(cell),
      _ => None,
    })
    .expect("PinX");
  assert_eq!(pin.v.as_deref(), Some("1"));
  assert_eq!(pin.f.as_deref(), Some("Width*0.5"));
  let line_x = shape_cell(shape, "Geometry", "X").expect("line X");
  let line_y = shape_cell(shape, "Geometry", "Y").expect("line Y");
  assert_eq!(line_x.value, Some("1"));
  assert_eq!(line_y.value, Some("2"));
}
