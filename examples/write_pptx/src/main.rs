use ooxmlsdk::parts::presentation_document::PresentationDocument;

fn main() {
  let pptx = PresentationDocument::new_from_file("examples/read_pptx/samples/demo.pptx").unwrap();

  pptx.save_to_file("/tmp/demo.pptx").unwrap();
}
