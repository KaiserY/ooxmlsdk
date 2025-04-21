use ooxmlsdk::parts::presentation_document::PresentationDocument;

fn main() {
  let pptx = PresentationDocument::new_from_file("examples/read_pptx/samples/demo.pptx").unwrap();

  println!("{}", pptx.presentation_part.root_element.to_xml().unwrap());
}
