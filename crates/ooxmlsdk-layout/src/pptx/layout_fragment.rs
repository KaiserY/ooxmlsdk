use super::slide::SlidePersist;
use super::slide_fragment::SlideFragmentHandler;

#[derive(Debug)]
pub(crate) struct LayoutFragmentHandler {
  slide_handler: SlideFragmentHandler,
}

impl LayoutFragmentHandler {
  pub(crate) fn new(slide_handler: SlideFragmentHandler) -> Self {
    Self { slide_handler }
  }

  pub(crate) fn on_create_context(&mut self, show_master_shapes: bool) {
    // showMasterSp=false hides inherited master shapes.
    if !show_master_shapes {
      // The actual SlidePersist is owned by the slide handler. This hook is
      // kept explicit so the full layout parser lands in the upstream slot.
    }
  }

  pub(crate) fn into_slide_persist(self) -> SlidePersist {
    self.slide_handler.finalize_import()
  }
}
