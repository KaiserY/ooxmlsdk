use ooxmlsdk::parts::{
  font_part::FontPart, image_part::ImagePart, wordprocessing_document::WordprocessingDocument,
};

fn main() {
  let mut docx = WordprocessingDocument::new("examples/read_docx/samples/demo.docx").unwrap();

  if let Some(theme_part) = docx.main_document_part.theme_part.as_mut() {
    theme_part.image_parts = vec![];

    theme_part.image_parts.push(ImagePart {
      inner_path: "word/media/image1.gif".to_string(),
      path: "examples/write_docx/samples/image1.gif".to_string(),
    });

    theme_part.image_parts.push(ImagePart {
      inner_path: "word/media/image2.png".to_string(),
      path: "examples/write_docx/samples/image2.png".to_string(),
    });

    theme_part.image_parts.push(ImagePart {
      inner_path: "word/media/image3.png".to_string(),
      path: "examples/write_docx/samples/image3.png".to_string(),
    });

    theme_part.image_parts.push(ImagePart {
      inner_path: "word/media/image4.png".to_string(),
      path: "examples/write_docx/samples/image4.png".to_string(),
    });
  }

  if let Some(font_table_part) = docx.main_document_part.font_table_part.as_mut() {
    font_table_part.font_parts = vec![];

    font_table_part.font_parts.push(FontPart {
      inner_path: "word/fonts/font1.odttf".to_string(),
      path: "examples/write_docx/samples/font1.odttf".to_string(),
    });

    font_table_part.font_parts.push(FontPart {
      inner_path: "word/fonts/font2.odttf".to_string(),
      path: "examples/write_docx/samples/font2.odttf".to_string(),
    });

    font_table_part.font_parts.push(FontPart {
      inner_path: "word/fonts/font3.odttf".to_string(),
      path: "examples/write_docx/samples/font3.odttf".to_string(),
    });

    font_table_part.font_parts.push(FontPart {
      inner_path: "word/fonts/font4.odttf".to_string(),
      path: "examples/write_docx/samples/font4.odttf".to_string(),
    });

    font_table_part.font_parts.push(FontPart {
      inner_path: "word/fonts/font5.odttf".to_string(),
      path: "examples/write_docx/samples/font5.odttf".to_string(),
    });

    font_table_part.font_parts.push(FontPart {
      inner_path: "word/fonts/font6.odttf".to_string(),
      path: "examples/write_docx/samples/font6.odttf".to_string(),
    });
  }

  docx.save("/tmp/demo.docx").unwrap();
}
