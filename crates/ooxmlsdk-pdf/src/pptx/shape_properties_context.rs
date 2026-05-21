use super::shape::PptShape;

#[derive(Debug)]
pub(crate) struct PPTShapePropertiesContext<'a> {
  shape: &'a mut PptShape,
}

impl<'a> PPTShapePropertiesContext<'a> {
  pub(crate) fn new(shape: &'a mut PptShape) -> Self {
    Self { shape }
  }

  pub(crate) fn on_create_context(&mut self, element_name: &str) {
    // Source: LibreOffice oox/source/ppt/pptshapepropertiescontext.cxx
    self.shape.set_has_noninherited_shape_properties();
    if element_name == "a:xfrm" {
      // LibreOffice sets PROP_IsPlaceholderDependent=false before delegating
      // to the generic DrawingML shape-properties context.
    }
  }
}
