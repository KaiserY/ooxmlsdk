use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
  let docx = WordprocessingDocument::new_from_file("examples/read_docx/samples/demo.docx")?;

  println!("{}", docx.main_document_part.root_element.to_xml()?);

  println!("{}", docx.main_document_part.root_element.validate()?);

  let file = File::open("examples/read_docx/samples/demo.docx")?;

  let reader = BufReader::new(file);

  let docx = WordprocessingDocument::new(reader)?;

  println!("{}", docx.main_document_part.root_element.to_xml()?);

  println!("{}", docx.main_document_part.root_element.validate()?);

  Ok(())
}
