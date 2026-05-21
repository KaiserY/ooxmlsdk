use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::slide_part::SlidePart;

use crate::error::Result;

use super::shape_group_context::PPTShapeGroupContext;
use super::slide::{ShapeLocation, SlidePersist};

#[derive(Debug)]
pub(crate) struct SlideFragmentHandler {
  slide_persist: SlidePersist,
  shape_location: ShapeLocation,
  slide_name: Option<String>,
  slide_properties: SlideProperties,
  char_vector: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct SlideProperties;

impl SlideFragmentHandler {
  pub(crate) fn new(slide_persist: SlidePersist, shape_location: ShapeLocation) -> Self {
    Self {
      slide_persist,
      shape_location,
      slide_name: None,
      slide_properties: SlideProperties,
      char_vector: String::new(),
    }
  }

  pub(crate) fn import_slide_part(
    &mut self,
    package: &mut PresentationDocument,
    slide_part: &SlidePart,
  ) -> Result<()> {
    // Source: LibreOffice oox/source/ppt/slidefragmenthandler.cxx
    // The constructor imports related vmlDrawing/legacyDrawing before XML
    // contexts create shapes; destruction converts and inserts VML drawing.
    self.slide_persist.drawing.imported = slide_part.vml_drawing_parts(package).next().is_some();
    let slide = slide_part.root_element(package)?;
    self.slide_name = slide.common_slide_data.name.clone();
    self.on_create_context();
    Ok(())
  }

  pub(crate) fn finalize_import(self) -> SlidePersist {
    self.slide_persist
  }

  pub(crate) fn on_create_context(&mut self) {
    let mut group_context = PPTShapeGroupContext::new(self.shape_location);
    group_context.on_create_context(&mut self.slide_persist);
  }

  pub(crate) fn on_characters(&mut self, text: &str) {
    self.char_vector.push_str(text);
  }

  pub(crate) fn get_char_vector(&self) -> &str {
    &self.char_vector
  }
}
