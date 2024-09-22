use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

fn main() {
  let docx = WordprocessingDocument::new("examples/read_docx/samples/demo.docx").unwrap();

  println!("{}", docx.main_document_part.root_element.children.len());
}
