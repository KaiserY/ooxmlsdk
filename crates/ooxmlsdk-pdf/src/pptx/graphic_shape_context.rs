use super::slide::SlidePersist;

#[derive(Debug, Default)]
pub(crate) struct PPTGraphicShapeContext;

impl PPTGraphicShapeContext {
  pub(crate) fn on_create_context(&mut self, _slide_persist: &mut SlidePersist) {
    // Source: LibreOffice oox/source/ppt/pptgraphicshapecontext.cxx
  }
}
