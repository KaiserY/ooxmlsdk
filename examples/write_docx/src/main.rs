use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

fn main() {
  let mut docx = WordprocessingDocument::new("examples/read_docx/samples/demo.docx").unwrap();

  if let Some(numbering_definitions_part) =
    docx.main_document_part.numbering_definitions_part.as_mut()
  {
    numbering_definitions_part.image_parts[0].path =
      "examples/write_docx/samples/image1.gif".to_string()
  }

  docx.main_document_part.image_parts[0].path =
    "examples/write_docx/samples/image4.png".to_string();
  docx.main_document_part.image_parts[1].path =
    "examples/write_docx/samples/image3.png".to_string();
  docx.main_document_part.image_parts[2].path =
    "examples/write_docx/samples/image2.png".to_string();

  if let Some(font_table_part) = docx.main_document_part.font_table_part.as_mut() {
    font_table_part.font_parts[0].path = "examples/write_docx/samples/font3.odttf".to_string();
    font_table_part.font_parts[1].path = "examples/write_docx/samples/font2.odttf".to_string();
    font_table_part.font_parts[2].path = "examples/write_docx/samples/font1.odttf".to_string();
    font_table_part.font_parts[3].path = "examples/write_docx/samples/font6.odttf".to_string();
    font_table_part.font_parts[4].path = "examples/write_docx/samples/font5.odttf".to_string();
    font_table_part.font_parts[5].path = "examples/write_docx/samples/font4.odttf".to_string();
  }

  docx.save("/tmp/demo.docx").unwrap();
}
