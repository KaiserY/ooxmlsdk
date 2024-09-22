use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

fn main() {
  let docx = WordprocessingDocument::new("examples/read_docx/samples/demo.docx").unwrap();

  docx.save("/tmp/demo.docx").unwrap();
}
