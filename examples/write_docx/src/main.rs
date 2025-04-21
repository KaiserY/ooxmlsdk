use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

fn main() {
  let docx = WordprocessingDocument::new_from_file("examples/read_docx/samples/demo.docx").unwrap();

  docx.save_to_file("/tmp/demo.docx").unwrap();
}
