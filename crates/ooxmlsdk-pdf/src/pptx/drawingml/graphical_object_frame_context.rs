use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::shape::{GraphicDataKind, Shape};
use super::table::TableProperties;

const PML_OLE_URI: &str = "http://schemas.openxmlformats.org/presentationml/2006/ole";
const PML_OLE_PURL_URI: &str = "http://purl.oclc.org/ooxml/presentationml/ole";
const DML_DIAGRAM_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/diagram";
const DML_DIAGRAM_PURL_URI: &str = "http://purl.oclc.org/ooxml/drawingml/diagram";
const DML_CHART_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";
const DML_CHART_PURL_URI: &str = "http://purl.oclc.org/ooxml/drawingml/chart";
const DML_CHART_EX_URI: &str = "http://schemas.microsoft.com/office/drawing/2014/chartex";
const DML_TABLE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/table";
const DML_TABLE_PURL_URI: &str = "http://purl.oclc.org/ooxml/drawingml/table";

#[derive(Debug, Default)]
pub(crate) struct GraphicalObjectFrameContext;

impl GraphicalObjectFrameContext {
  pub(crate) fn dispatch_graphic_data(&self, graphic_data: &a::GraphicData, shape: &mut Shape) {
    // Source: LibreOffice oox/source/drawingml/graphicshapecontext.cxx
    // GraphicalObjectFrameContext dispatches graphicData by URI, while the
    // PPT parent handles ext drawings at p:graphicFrame end.
    let uri = graphic_data.uri.as_str();
    let kind = match uri {
      PML_OLE_URI | PML_OLE_PURL_URI => GraphicDataKind::Ole,
      DML_DIAGRAM_URI | DML_DIAGRAM_PURL_URI => GraphicDataKind::Diagram,
      DML_CHART_URI | DML_CHART_PURL_URI => GraphicDataKind::Chart,
      DML_CHART_EX_URI => GraphicDataKind::ChartEx,
      DML_TABLE_URI | DML_TABLE_PURL_URI => GraphicDataKind::Table,
      _ => GraphicDataKind::Unsupported,
    };
    shape.set_graphic_data(uri.to_string(), kind);
    match kind {
      GraphicDataKind::Ole => shape.set_ole_object_type(),
      GraphicDataKind::Diagram => shape.set_diagram_type(),
      GraphicDataKind::Chart => shape.set_chart_type(),
      GraphicDataKind::ChartEx => shape.set_chart_ex_type(),
      GraphicDataKind::Table => {
        shape.set_table_type();
        shape.table_properties =
          TableProperties::from_graphic_data_xml_children(&graphic_data.xml_children);
      }
      GraphicDataKind::Unsupported => {}
    }
  }
}
