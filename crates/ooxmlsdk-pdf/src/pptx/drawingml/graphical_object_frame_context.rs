use super::shape::{FrameType, Shape};

#[derive(Debug, Default)]
pub(crate) struct GraphicalObjectFrameContext;

impl GraphicalObjectFrameContext {
  pub(crate) fn dispatch_graphic_data(&self, uri: &str, shape: &mut Shape) {
    // Source: LibreOffice oox/source/drawingml/graphicshapecontext.cxx
    // GraphicalObjectFrameContext dispatches graphicData by URI, while the
    // PPT parent handles ext drawings at p:graphicFrame end.
    if uri.contains("/chart") {
      shape.set_chart_type();
    } else if uri.contains("/table") {
      shape.set_table_type();
    } else if uri.contains("/diagram") {
      shape.set_diagram_type();
    } else if uri.contains("/oleObject") {
      shape.set_ole_object_type();
    } else {
      shape.frame_type = FrameType::Generic;
    }
  }
}
