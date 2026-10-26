use std::io::Cursor;

use ooxmlsdk::parts::vba_project_part::VbaProjectPart;
use ooxmlsdk::parts::visio_document::VisioDocument;
use ooxmlsdk::parts::visio_pages_part::VisioPagesPart;
use ooxmlsdk::sdk::{SdkPart, VisioDocumentType};

const VBA_BYTES: &[u8] = b"not a real vba project";
const PAGES_RELATIONSHIP: &str = "http://schemas.microsoft.com/visio/2010/relationships/pages";
const VBA_RELATIONSHIP: &str = "http://schemas.microsoft.com/office/2006/relationships/vbaProject";

fn round_trip(kind: VisioDocumentType) {
  let with_vba = matches!(
    kind,
    VisioDocumentType::MacroEnabledDrawing
      | VisioDocumentType::MacroEnabledStencil
      | VisioDocumentType::MacroEnabledTemplate
  );
  let mut document = VisioDocument::create(kind);
  document.add_visio_document_part().expect("add main part");
  document
    .change_document_type(kind)
    .expect("set main content type");
  if with_vba {
    let main = document
      .visio_document_part()
      .expect("main part on a new package");
    let vba = main
      .add_new_part_auto_id::<_, VbaProjectPart>(&mut document)
      .expect("add vba project");
    vba
      .set_data(&mut document, VBA_BYTES.to_vec())
      .expect("store vba bytes");
  }

  assert_eq!(document.document_type(), kind);
  assert_eq!(document.document_type().content_type(), kind.content_type());

  let bytes = document.to_package_bytes().expect("save package");
  let opened = VisioDocument::new(Cursor::new(bytes)).expect("open package");
  assert_eq!(opened.document_type(), kind);
  assert_eq!(opened.document_type().content_type(), kind.content_type());

  let main = opened.visio_document_part().expect("main part after open");
  assert!(
    main
      .child_part_by_relationship_type::<_, VisioPagesPart>(&opened, PAGES_RELATIONSHIP)
      .is_none(),
    "{kind:?} opens with no pages part"
  );

  let vba = main.child_part_by_relationship_type::<_, VbaProjectPart>(&opened, VBA_RELATIONSHIP);
  if with_vba {
    let vba = vba.expect("vba part");
    let mut stored = Vec::new();
    vba
      .write_data_to(&opened, &mut stored)
      .expect("read vba bytes");
    assert_eq!(stored, VBA_BYTES);
  } else {
    assert!(vba.is_none());
  }
}

#[test]
fn six_content_types_round_trip_without_pages() {
  for kind in [
    VisioDocumentType::Drawing,
    VisioDocumentType::MacroEnabledDrawing,
    VisioDocumentType::Stencil,
    VisioDocumentType::MacroEnabledStencil,
    VisioDocumentType::Template,
    VisioDocumentType::MacroEnabledTemplate,
  ] {
    round_trip(kind);
  }
}
