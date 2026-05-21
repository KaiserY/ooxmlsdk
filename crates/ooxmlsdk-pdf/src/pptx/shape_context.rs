use super::shape::PptShape;
use super::slide::SlidePersist;

#[derive(Debug)]
pub(crate) struct PPTShapeContext<'a> {
  shape: &'a mut PptShape,
}

impl<'a> PPTShapeContext<'a> {
  pub(crate) fn new(shape: &'a mut PptShape) -> Self {
    Self { shape }
  }

  pub(crate) fn on_create_context(&mut self, slide_persist: &SlidePersist) {
    // Source: LibreOffice oox/source/ppt/pptshapecontext.cxx
    // Placeholder lookup applies layout/master references before shape
    // properties and text body are finalized.
    if self.shape.is_placeholder_candidate()
      && let Some(reference) = self.shape.find_placeholder(slide_persist).cloned()
    {
      self.shape.shape.apply_shape_reference(&reference);
    }
  }
}
