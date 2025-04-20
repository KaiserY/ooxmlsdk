use ooxmlsdk::parts::presentation_document::PresentationDocument;

fn main() {
  let pptx = PresentationDocument::new("examples/read_pptx/samples/demo.pptx").unwrap();

  pptx.save("/tmp/demo.pptx").unwrap();
}
