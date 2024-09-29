use ooxmlsdk::parts::presentation_document::PresentationDocument;

fn main() {
  let mut pptx = PresentationDocument::new("examples/read_pptx/samples/demo.pptx").unwrap();

  if let Some(thumbnail_part) = pptx.thumbnail_part.as_mut() {
    thumbnail_part.path = "examples/write_pptx/samples/thumbnail.jpeg".to_string();
  }

  if let Some(slide_layout_part) = pptx.presentation_part.slide_parts[1]
    .slide_layout_part
    .as_mut()
  {
    slide_layout_part.image_parts[0].path = "examples/write_pptx/samples/image2.jpeg".to_string();
  }

  pptx.presentation_part.slide_master_parts[0].image_parts[0].path =
    "examples/write_pptx/samples/image1.jpeg".to_string();

  pptx.save("/tmp/demo.pptx").unwrap();
}
