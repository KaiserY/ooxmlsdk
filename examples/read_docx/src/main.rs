use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
use std::fs::File;
use std::io::BufReader;

fn main() {
  let docx = WordprocessingDocument::new("examples/read_docx/samples/demo.docx").unwrap();

  println!(
    "{}",
    docx.main_document_part.root_element.to_xml().unwrap()
  );

  println!(
    "{}",
    docx.main_document_part.root_element.validate().unwrap()
  );

  let file = File::open("examples/read_docx/samples/demo.docx").unwrap();

  let reader = BufReader::new(file);

  let docx = WordprocessingDocument::new_from_reader(reader).unwrap();

  println!(
    "{}",
    docx.main_document_part.root_element.to_xml().unwrap()
  );

  println!(
    "{}",
    docx.main_document_part.root_element.validate().unwrap()
  );
}
