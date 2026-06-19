use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::shape::{
  DiagramRelationshipIds, GraphicDataKind, GraphicDataRecord, OleObjectRecord, Shape,
};
use super::table::TableProperties;
use crate::pptx::slide::SlidePersist;

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
  pub(crate) fn dispatch_graphic_data(
    &self,
    graphic_data: &a::GraphicData,
    slide_persist: &SlidePersist,
    shape: &mut Shape,
  ) {
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
    let record = graphic_data_record(graphic_data, kind, slide_persist);
    shape.set_graphic_data_record(record);
    match kind {
      GraphicDataKind::Ole => shape.set_ole_object_type(),
      GraphicDataKind::Diagram => shape.set_diagram_type(),
      GraphicDataKind::Chart => shape.set_chart_type(),
      GraphicDataKind::ChartEx => shape.set_chart_ex_type(),
      GraphicDataKind::Table => {
        shape.set_table_type();
        shape.table_properties = graphic_data.graphic_data_choice.iter().find_map(|choice| {
          if let a::GraphicDataChoice::Table(table) = choice {
            Some(TableProperties::from_dml_table(table))
          } else {
            None
          }
        });
      }
      GraphicDataKind::Unsupported => {}
    }
  }
}

fn graphic_data_record(
  graphic_data: &a::GraphicData,
  kind: GraphicDataKind,
  slide_persist: &SlidePersist,
) -> GraphicDataRecord {
  let mut record = GraphicDataRecord {
    uri: graphic_data.uri.clone(),
    kind,
    chart_relationship_id: None,
    chart_resource: None,
    extended_chart_resource: None,
    has_inline_chart_space: false,
    diagram_relationship_ids: None,
    diagram_data_resource: None,
    diagram_layout_resource: None,
    diagram_style_resource: None,
    diagram_color_resource: None,
    ole_object: None,
    ole_binary_resource: None,
    embedded_package_resource: None,
  };
  for choice in &graphic_data.graphic_data_choice {
    match choice {
      a::GraphicDataChoice::ChartReference(reference) => {
        record.chart_relationship_id = Some(reference.id.clone());
        record.chart_resource = slide_persist.chart_resources.get(&reference.id).cloned();
        record.extended_chart_resource = slide_persist
          .extended_chart_resources
          .get(&reference.id)
          .cloned();
      }
      a::GraphicDataChoice::ChartSpace(_) => {
        record.has_inline_chart_space = true;
      }
      a::GraphicDataChoice::RelationshipIds(relationship_ids) => {
        record.diagram_relationship_ids = Some(DiagramRelationshipIds {
          data_part: relationship_ids.data_part.clone(),
          layout_part: relationship_ids.layout_part.clone(),
          style_part: relationship_ids.style_part.clone(),
          color_part: relationship_ids.color_part.clone(),
        });
        record.diagram_data_resource = slide_persist
          .diagram_data_resources
          .get(&relationship_ids.data_part)
          .cloned();
        record.diagram_layout_resource = slide_persist
          .diagram_layout_resources
          .get(&relationship_ids.layout_part)
          .cloned();
        record.diagram_style_resource = slide_persist
          .diagram_style_resources
          .get(&relationship_ids.style_part)
          .cloned();
        record.diagram_color_resource = slide_persist
          .diagram_color_resources
          .get(&relationship_ids.color_part)
          .cloned();
      }
      a::GraphicDataChoice::POleObject(ole_object) => {
        record.ole_object = Some(OleObjectRecord {
          relationship_id: ole_object.id.clone(),
          name: ole_object.name.clone(),
          prog_id: ole_object.prog_id.clone(),
          show_as_icon: ole_object
            .show_as_icon
            .as_ref()
            .is_some_and(|value| value.as_bool()),
        });
        if let Some(relationship_id) = &ole_object.id {
          record.ole_binary_resource = slide_persist
            .embedded_object_resources
            .get(relationship_id)
            .cloned();
          record.embedded_package_resource = slide_persist
            .embedded_package_resources
            .get(relationship_id)
            .cloned();
        }
      }
      _ => {}
    }
  }
  record
}
