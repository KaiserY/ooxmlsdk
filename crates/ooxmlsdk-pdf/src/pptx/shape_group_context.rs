use super::drawingml::shape::ShapeService;
use super::shape::PptShape;
use super::slide::{ShapeLocation, SlidePersist};

#[derive(Debug)]
pub(crate) struct PPTShapeGroupContext {
  shape_location: ShapeLocation,
  graphic_shape: Option<PptShape>,
}

impl PPTShapeGroupContext {
  pub(crate) fn new(shape_location: ShapeLocation) -> Self {
    Self {
      shape_location,
      graphic_shape: None,
    }
  }

  pub(crate) fn on_create_context(&mut self, slide_persist: &mut SlidePersist) {
    // Source: LibreOffice oox/source/ppt/pptshapegroupcontext.cxx
    // Full parser will dispatch sp/cxnSp/grpSp/pic/graphicFrame here.
    let shape = PptShape::new(ShapeService::CustomShape, self.shape_location);
    shape.add_shape(slide_persist);
  }

  pub(crate) fn import_ext_drawings(&mut self, _slide_persist: &mut SlidePersist) {
    if let Some(shape) = &mut self.graphic_shape {
      shape.shape.keep_diagram_drawing();
    }
  }

  pub(crate) fn apply_font_ref_color(&mut self) {}
}
