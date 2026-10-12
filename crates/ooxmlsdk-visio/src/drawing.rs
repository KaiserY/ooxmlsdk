use ooxmlsdk::common::SdkError;
use ooxmlsdk::parts::visio_document::VisioDocument as VisioPackage;
use ooxmlsdk::parts::visio_page_part::VisioPagePart;
use ooxmlsdk::parts::visio_pages_part::VisioPagesPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_visio_2012_main::{
  Cell, Page, PageChoice, PageContents, Pages, Rel, RowUnion, RowUnionChoice, Section,
  SectionChoice, Shape, ShapeChoice, Shapes, VisioDocument,
};
use ooxmlsdk::sdk::{SdkType, VisioDocumentType};

use crate::ids::Ids;

/// One drawing page and one shape. The shape stores `PinX` and a Geometry
/// `LineTo`. Ids come from `ids`.
pub fn minimal_drawing(ids: &mut Ids) -> Result<VisioPackage, SdkError> {
  let page_id = ids.page();
  let shape_id = ids.shape();
  let mut package = VisioPackage::create(VisioDocumentType::Drawing);
  package.add_visio_document_part()?;
  let document_part = package.visio_document_part()?;
  document_part.set_data(
    &mut package,
    VisioDocument::default().to_xml()?.into_bytes(),
  )?;

  let pages_part = document_part.add_new_part_auto_id::<_, VisioPagesPart>(&mut package)?;
  let pages = Pages {
    page: vec![Page {
      id: page_id,
      name: Some("Page-1".to_string()),
      page_choice: vec![PageChoice::Rel(Rel::default())],
      ..Default::default()
    }],
    ..Default::default()
  };
  pages_part.set_data(&mut package, pages.to_xml()?.into_bytes())?;

  let page_part = pages_part.add_new_part_auto_id::<_, VisioPagePart>(&mut package)?;
  let contents = PageContents {
    shapes: Some(Shapes {
      shape: vec![Shape {
        id: shape_id,
        shape_choice: vec![
          ShapeChoice::Cell(Box::new(Cell {
            n: "PinX".to_string(),
            v: Some("1".to_string()),
            f: Some("Width*0.5".to_string()),
            ..Default::default()
          })),
          ShapeChoice::Section(Section {
            n: "Geometry".to_string(),
            section_choice: vec![SectionChoice::RowUnion(Box::new(RowUnion {
              t: Some("LineTo".to_string()),
              ix: Some(1),
              row_union_choice: vec![
                RowUnionChoice::Cell(Box::new(Cell {
                  n: "X".to_string(),
                  v: Some("1".to_string()),
                  ..Default::default()
                })),
                RowUnionChoice::Cell(Box::new(Cell {
                  n: "Y".to_string(),
                  v: Some("2".to_string()),
                  ..Default::default()
                })),
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
  };
  page_part.set_data(&mut package, contents.to_xml()?.into_bytes())?;
  Ok(package)
}
