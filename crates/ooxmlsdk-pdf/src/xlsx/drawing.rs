use ooxmlsdk::parts::chart_part::ChartPart;
use ooxmlsdk::parts::diagram_colors_part::DiagramColorsPart;
use ooxmlsdk::parts::diagram_data_part::DiagramDataPart;
use ooxmlsdk::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart;
use ooxmlsdk::parts::diagram_persist_layout_part::DiagramPersistLayoutPart;
use ooxmlsdk::parts::diagram_style_part::DiagramStylePart;
use ooxmlsdk::parts::drawings_part::DrawingsPart;
use ooxmlsdk::parts::extended_chart_part::ExtendedChartPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2008_diagram as dsp;
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2014_chartex as cx;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing as xdr;
use ooxmlsdk::sdk::SdkPart;

use crate::error::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct DrawingResourceCatalog {
  pub(crate) anchors: Vec<DrawingAnchorModel>,
  pub(crate) charts: Vec<ChartResourceCatalog>,
  pub(crate) extended_charts: Vec<ChartResourceCatalog>,
  pub(crate) diagrams: DiagramResourceCatalog,
  pub(crate) images: usize,
  pub(crate) custom_xml_parts: usize,
  pub(crate) web_extensions: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DrawingAnchorModel {
  pub(crate) kind: DrawingAnchorKind,
  pub(crate) object: DrawingObjectModel,
  pub(crate) from: Option<DrawingMarkerModel>,
  pub(crate) to: Option<DrawingMarkerModel>,
  pub(crate) position: Option<(i64, i64)>,
  pub(crate) extent: Option<(i64, i64)>,
  pub(crate) edit_as: Option<xdr::EditAsValues>,
  pub(crate) lock_with_sheet: bool,
  pub(crate) print_with_sheet: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DrawingAnchorKind {
  TwoCell,
  OneCell,
  Absolute,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DrawingMarkerModel {
  pub(crate) column: i32,
  pub(crate) column_offset_emu: i64,
  pub(crate) row: i32,
  pub(crate) row_offset_emu: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DrawingObjectModel {
  pub(crate) kind: DrawingObjectKind,
  pub(crate) id: Option<u32>,
  pub(crate) name: Option<String>,
  pub(crate) description: Option<String>,
  pub(crate) hidden: bool,
  pub(crate) macro_name: Option<String>,
  pub(crate) text_len: usize,
  pub(crate) relationship_id: Option<String>,
  pub(crate) graphic_uri: Option<String>,
  pub(crate) child_objects: usize,
  pub(crate) has_style: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DrawingObjectKind {
  Shape,
  GroupShape,
  GraphicFrame,
  ConnectionShape,
  Picture,
  ContentPart,
  Unknown,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ChartResourceCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) extended: bool,
  pub(crate) version_len: usize,
  pub(crate) feature_list_len: usize,
  pub(crate) has_fallback_image: bool,
  pub(crate) date1904: bool,
  pub(crate) rounded_corners: bool,
  pub(crate) has_style: bool,
  pub(crate) has_pivot_source: bool,
  pub(crate) protection_flags: usize,
  pub(crate) has_title: bool,
  pub(crate) has_3d_view: bool,
  pub(crate) chart_type_groups: usize,
  pub(crate) axes: usize,
  pub(crate) has_legend: bool,
  pub(crate) chart_flags: usize,
  pub(crate) has_root_shape_properties: bool,
  pub(crate) has_text_properties: bool,
  pub(crate) external_data_relationship_id: Option<String>,
  pub(crate) external_data_auto_update: bool,
  pub(crate) has_print_settings: bool,
  pub(crate) has_user_shapes_reference: bool,
  pub(crate) extension_markers: usize,
  pub(crate) chartex_data_sets: usize,
  pub(crate) chartex_series: usize,
  pub(crate) has_chart_drawing: bool,
  pub(crate) has_embedded_package: bool,
  pub(crate) images: usize,
  pub(crate) has_theme_override: bool,
  pub(crate) styles: usize,
  pub(crate) color_styles: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DiagramResourceCatalog {
  pub(crate) data_parts: Vec<DiagramDataCatalog>,
  pub(crate) layout_parts: Vec<DiagramLayoutCatalog>,
  pub(crate) style_parts: Vec<DiagramStyleCatalog>,
  pub(crate) color_parts: Vec<DiagramColorCatalog>,
  pub(crate) drawing_parts: Vec<DiagramDrawingCatalog>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DiagramDataCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) points: usize,
  pub(crate) unknown_points: usize,
  pub(crate) connections: usize,
  pub(crate) text_points: usize,
  pub(crate) property_sets: usize,
  pub(crate) shape_properties: usize,
  pub(crate) background: bool,
  pub(crate) whole: bool,
  pub(crate) extension_markers: usize,
  pub(crate) images: usize,
  pub(crate) slides: usize,
  pub(crate) worksheets: usize,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DiagramLayoutCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) titles: usize,
  pub(crate) descriptions: usize,
  pub(crate) has_category_list: bool,
  pub(crate) has_sample_data: bool,
  pub(crate) has_style_data: bool,
  pub(crate) has_color_data: bool,
  pub(crate) layout_nodes: usize,
  pub(crate) algorithms: usize,
  pub(crate) shapes: usize,
  pub(crate) presentation_of: usize,
  pub(crate) constraints: usize,
  pub(crate) rules: usize,
  pub(crate) variables: usize,
  pub(crate) for_each: usize,
  pub(crate) choose: usize,
  pub(crate) extension_markers: usize,
  pub(crate) images: usize,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DiagramStyleCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) titles: usize,
  pub(crate) descriptions: usize,
  pub(crate) has_categories: bool,
  pub(crate) has_scene3d: bool,
  pub(crate) labels: usize,
  pub(crate) extension_markers: usize,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DiagramColorCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) titles: usize,
  pub(crate) descriptions: usize,
  pub(crate) has_categories: bool,
  pub(crate) labels: usize,
  pub(crate) extension_markers: usize,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DiagramDrawingCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) shapes: usize,
  pub(crate) groups: usize,
  pub(crate) text_shapes: usize,
  pub(crate) styled_shapes: usize,
  pub(crate) transformed_shapes: usize,
  pub(crate) extension_markers: usize,
  pub(crate) images: usize,
  pub(crate) text_len: usize,
}

impl DrawingResourceCatalog {
  pub(crate) fn from_part(package: &mut SpreadsheetDocument, part: &DrawingsPart) -> Result<Self> {
    let anchors = {
      let root = part.root_element(package)?;
      root
        .worksheet_drawing_choice
        .iter()
        .map(DrawingAnchorModel::from_choice)
        .collect()
    };
    let chart_parts = part.chart_parts(package).collect::<Vec<_>>();
    let extended_chart_parts = part.extended_chart_parts(package).collect::<Vec<_>>();
    let diagrams = DiagramResourceCatalog::from_part(package, part)?;
    Ok(Self {
      anchors,
      charts: chart_parts
        .iter()
        .map(|chart| ChartResourceCatalog::from_chart_part(package, &chart))
        .collect::<Result<Vec<_>>>()?,
      extended_charts: extended_chart_parts
        .iter()
        .map(|chart| ChartResourceCatalog::from_extended_chart_part(package, &chart))
        .collect::<Result<Vec<_>>>()?,
      diagrams,
      images: part.image_parts(package).count(),
      custom_xml_parts: part.custom_xml_parts(package).count(),
      web_extensions: part.web_extension_parts(package).count(),
    })
  }

  pub(crate) fn chart_count(&self) -> usize {
    self.charts.len() + self.extended_charts.len()
  }

  pub(crate) fn diagram_resource_count(&self) -> usize {
    self.diagrams.resource_count()
  }

  pub(crate) fn chart_child_resource_count(&self) -> usize {
    self
      .charts
      .iter()
      .chain(self.extended_charts.iter())
      .map(ChartResourceCatalog::child_resource_count)
      .sum()
  }
}

impl DrawingAnchorModel {
  fn from_choice(choice: &xdr::WorksheetDrawingChoice) -> Self {
    match choice {
      xdr::WorksheetDrawingChoice::TwoCellAnchor(anchor) => Self {
        kind: DrawingAnchorKind::TwoCell,
        object: anchor
          .two_cell_anchor_choice
          .as_ref()
          .map(DrawingObjectModel::from_two_cell_choice)
          .unwrap_or_else(DrawingObjectModel::unknown),
        from: Some(DrawingMarkerModel::from_from_marker(&anchor.from_marker)),
        to: Some(DrawingMarkerModel::from_to_marker(&anchor.to_marker)),
        position: None,
        extent: None,
        edit_as: anchor.edit_as,
        lock_with_sheet: anchor
          .client_data
          .lock_with_sheet
          .is_some_and(|value| value.as_bool()),
        print_with_sheet: anchor
          .client_data
          .print_with_sheet
          .is_some_and(|value| value.as_bool()),
      },
      xdr::WorksheetDrawingChoice::OneCellAnchor(anchor) => Self {
        kind: DrawingAnchorKind::OneCell,
        object: anchor
          .one_cell_anchor_choice
          .as_ref()
          .map(DrawingObjectModel::from_one_cell_choice)
          .unwrap_or_else(DrawingObjectModel::unknown),
        from: Some(DrawingMarkerModel::from_from_marker(&anchor.from_marker)),
        to: None,
        position: None,
        extent: Some((anchor.extent.cx, anchor.extent.cy)),
        edit_as: None,
        lock_with_sheet: anchor
          .client_data
          .lock_with_sheet
          .is_some_and(|value| value.as_bool()),
        print_with_sheet: anchor
          .client_data
          .print_with_sheet
          .is_some_and(|value| value.as_bool()),
      },
      xdr::WorksheetDrawingChoice::AbsoluteAnchor(anchor) => Self {
        kind: DrawingAnchorKind::Absolute,
        object: anchor
          .absolute_anchor_choice
          .as_ref()
          .map(DrawingObjectModel::from_absolute_choice)
          .unwrap_or_else(DrawingObjectModel::unknown),
        from: None,
        to: None,
        position: Some((anchor.position.x, anchor.position.y)),
        extent: Some((anchor.extent.cx, anchor.extent.cy)),
        edit_as: None,
        lock_with_sheet: anchor
          .client_data
          .lock_with_sheet
          .is_some_and(|value| value.as_bool()),
        print_with_sheet: anchor
          .client_data
          .print_with_sheet
          .is_some_and(|value| value.as_bool()),
      },
      xdr::WorksheetDrawingChoice::XmlAny(value) => Self {
        kind: DrawingAnchorKind::Absolute,
        object: DrawingObjectModel {
          text_len: value.len(),
          ..DrawingObjectModel::unknown()
        },
        from: None,
        to: None,
        position: None,
        extent: None,
        edit_as: None,
        lock_with_sheet: false,
        print_with_sheet: false,
      },
    }
  }
}

impl DrawingMarkerModel {
  fn from_from_marker(marker: &xdr::FromMarker) -> Self {
    Self {
      column: marker.column_id,
      column_offset_emu: marker.column_offset.to_emu(),
      row: marker.row_id,
      row_offset_emu: marker.row_offset.to_emu(),
    }
  }

  fn from_to_marker(marker: &xdr::ToMarker) -> Self {
    Self {
      column: marker.column_id,
      column_offset_emu: marker.column_offset.to_emu(),
      row: marker.row_id,
      row_offset_emu: marker.row_offset.to_emu(),
    }
  }
}

impl DrawingObjectModel {
  fn from_two_cell_choice(choice: &xdr::TwoCellAnchorChoice) -> Self {
    match choice {
      xdr::TwoCellAnchorChoice::Shape(shape) => Self::from_shape(shape),
      xdr::TwoCellAnchorChoice::GroupShape(group) => Self::from_group_shape(group),
      xdr::TwoCellAnchorChoice::GraphicFrame(frame) => Self::from_graphic_frame(frame),
      xdr::TwoCellAnchorChoice::ConnectionShape(shape) => Self::from_connection_shape(shape),
      xdr::TwoCellAnchorChoice::Picture(picture) => Self::from_picture(picture),
      xdr::TwoCellAnchorChoice::ContentPart(part) => Self::from_content_part(&part.relationship_id),
      xdr::TwoCellAnchorChoice::XmlAny(value) => Self {
        text_len: value.len(),
        ..Self::unknown()
      },
    }
  }

  fn from_one_cell_choice(choice: &xdr::OneCellAnchorChoice) -> Self {
    match choice {
      xdr::OneCellAnchorChoice::Shape(shape) => Self::from_shape(shape),
      xdr::OneCellAnchorChoice::GroupShape(group) => Self::from_group_shape(group),
      xdr::OneCellAnchorChoice::GraphicFrame(frame) => Self::from_graphic_frame(frame),
      xdr::OneCellAnchorChoice::ConnectionShape(shape) => Self::from_connection_shape(shape),
      xdr::OneCellAnchorChoice::Picture(picture) => Self::from_picture(picture),
      xdr::OneCellAnchorChoice::ContentPart(part) => Self::from_content_part(&part.relationship_id),
    }
  }

  fn from_absolute_choice(choice: &xdr::AbsoluteAnchorChoice) -> Self {
    match choice {
      xdr::AbsoluteAnchorChoice::Shape(shape) => Self::from_shape(shape),
      xdr::AbsoluteAnchorChoice::GroupShape(group) => Self::from_group_shape(group),
      xdr::AbsoluteAnchorChoice::GraphicFrame(frame) => Self::from_graphic_frame(frame),
      xdr::AbsoluteAnchorChoice::ConnectionShape(shape) => Self::from_connection_shape(shape),
      xdr::AbsoluteAnchorChoice::Picture(picture) => Self::from_picture(picture),
      xdr::AbsoluteAnchorChoice::ContentPart(part) => {
        Self::from_content_part(&part.relationship_id)
      }
    }
  }

  fn from_shape(shape: &xdr::Shape) -> Self {
    let properties = &shape
      .non_visual_shape_properties
      .non_visual_drawing_properties;
    Self {
      kind: DrawingObjectKind::Shape,
      id: Some(properties.id),
      name: Some(properties.name.clone()),
      description: properties.description.clone(),
      hidden: properties.hidden.is_some_and(|value| value.as_bool()),
      macro_name: shape.r#macro.clone(),
      text_len: shape.text_link.as_ref().map_or(0, |value| value.len())
        + usize::from(shape.text_body.is_some()),
      relationship_id: None,
      graphic_uri: None,
      child_objects: 0,
      has_style: shape.shape_style.is_some(),
    }
  }

  fn from_group_shape(group: &xdr::GroupShape) -> Self {
    let properties = &group
      .non_visual_group_shape_properties
      .non_visual_drawing_properties;
    Self {
      kind: DrawingObjectKind::GroupShape,
      id: Some(properties.id),
      name: Some(properties.name.clone()),
      description: properties.description.clone(),
      hidden: properties.hidden.is_some_and(|value| value.as_bool()),
      macro_name: None,
      text_len: 0,
      relationship_id: None,
      graphic_uri: None,
      child_objects: group.group_shape_choice.len(),
      has_style: false,
    }
  }

  fn from_graphic_frame(frame: &xdr::GraphicFrame) -> Self {
    let properties = &frame
      .non_visual_graphic_frame_properties
      .non_visual_drawing_properties;
    Self {
      kind: DrawingObjectKind::GraphicFrame,
      id: Some(properties.id),
      name: Some(properties.name.clone()),
      description: properties.description.clone(),
      hidden: properties.hidden.is_some_and(|value| value.as_bool()),
      macro_name: frame.r#macro.clone(),
      text_len: 0,
      relationship_id: None,
      graphic_uri: Some(frame.graphic.graphic_data.uri.clone()),
      child_objects: frame.graphic.graphic_data.graphic_data_choice.len(),
      has_style: false,
    }
  }

  fn from_connection_shape(shape: &xdr::ConnectionShape) -> Self {
    let properties = &shape
      .non_visual_connection_shape_properties
      .non_visual_drawing_properties;
    Self {
      kind: DrawingObjectKind::ConnectionShape,
      id: Some(properties.id),
      name: Some(properties.name.clone()),
      description: properties.description.clone(),
      hidden: properties.hidden.is_some_and(|value| value.as_bool()),
      macro_name: shape.r#macro.clone(),
      text_len: 0,
      relationship_id: None,
      graphic_uri: None,
      child_objects: 0,
      has_style: shape.shape_style.is_some(),
    }
  }

  fn from_picture(picture: &xdr::Picture) -> Self {
    let properties = &picture
      .non_visual_picture_properties
      .non_visual_drawing_properties;
    Self {
      kind: DrawingObjectKind::Picture,
      id: Some(properties.id),
      name: Some(properties.name.clone()),
      description: properties.description.clone(),
      hidden: properties.hidden.is_some_and(|value| value.as_bool()),
      macro_name: picture.r#macro.clone(),
      text_len: 0,
      relationship_id: picture
        .blip_fill
        .blip
        .as_ref()
        .and_then(|blip| blip.embed.clone().or_else(|| blip.link.clone())),
      graphic_uri: None,
      child_objects: 0,
      has_style: picture.shape_style.is_some(),
    }
  }

  fn from_content_part(relationship_id: &str) -> Self {
    Self {
      kind: DrawingObjectKind::ContentPart,
      relationship_id: Some(relationship_id.to_string()),
      ..Self::unknown()
    }
  }

  fn unknown() -> Self {
    Self {
      kind: DrawingObjectKind::Unknown,
      id: None,
      name: None,
      description: None,
      hidden: false,
      macro_name: None,
      text_len: 0,
      relationship_id: None,
      graphic_uri: None,
      child_objects: 0,
      has_style: false,
    }
  }
}

impl DiagramResourceCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &DrawingsPart) -> Result<Self> {
    let data_parts = part.diagram_data_parts(package).collect::<Vec<_>>();
    let layout_parts = part
      .diagram_layout_definition_parts(package)
      .collect::<Vec<_>>();
    let style_parts = part.diagram_style_parts(package).collect::<Vec<_>>();
    let color_parts = part.diagram_colors_parts(package).collect::<Vec<_>>();
    let drawing_parts = part
      .diagram_persist_layout_parts(package)
      .collect::<Vec<_>>();
    Ok(Self {
      data_parts: data_parts
        .iter()
        .map(|part| DiagramDataCatalog::from_part(package, &part))
        .collect::<Result<Vec<_>>>()?,
      layout_parts: layout_parts
        .iter()
        .map(|part| DiagramLayoutCatalog::from_part(package, &part))
        .collect::<Result<Vec<_>>>()?,
      style_parts: style_parts
        .iter()
        .map(|part| DiagramStyleCatalog::from_part(package, &part))
        .collect::<Result<Vec<_>>>()?,
      color_parts: color_parts
        .iter()
        .map(|part| DiagramColorCatalog::from_part(package, &part))
        .collect::<Result<Vec<_>>>()?,
      drawing_parts: drawing_parts
        .iter()
        .map(|part| DiagramDrawingCatalog::from_part(package, &part))
        .collect::<Result<Vec<_>>>()?,
    })
  }

  pub(crate) fn resource_count(&self) -> usize {
    self.data_parts.len()
      + self.layout_parts.len()
      + self.style_parts.len()
      + self.color_parts.len()
      + self.drawing_parts.len()
  }
}

impl DiagramDataCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &DiagramDataPart) -> Result<Self> {
    let model = {
      let data_model = part.root_element(package)?;
      Self::from_data_model(part.relationship_id().map(ToString::to_string), data_model)
    };
    Ok(Self {
      images: part.image_parts(package).count(),
      slides: part.slide_parts(package).count(),
      worksheets: part.worksheet_parts(package).count(),
      ..model
    })
  }

  fn from_data_model(relationship_id: Option<String>, data_model: &dgm::DataModelRoot) -> Self {
    let mut catalog = Self {
      relationship_id,
      background: data_model.background.is_some(),
      whole: data_model.whole.is_some(),
      extension_markers: data_model
        .data_model_extension_list
        .as_ref()
        .map_or(0, |list| list.data_model_extension.len()),
      ..Self::default()
    };
    for child in &data_model.point_list.xml_children {
      match child {
        dgm::PointListChoice::Point(point) => {
          catalog.points += 1;
          catalog.text_len += point.model_id.len();
          if let Some(connection_id) = point.connection_id.as_ref() {
            catalog.text_len += connection_id.len();
          }
          catalog.text_points += usize::from(point.text_body.is_some());
          catalog.property_sets += usize::from(point.property_set.is_some());
          catalog.shape_properties += usize::from(point.shape_properties.is_some());
          catalog.extension_markers += point
            .pt_extension_list
            .as_ref()
            .map_or(0, |list| list.pt_extension.len());
        }
        dgm::PointListChoice::XmlAny(value) => {
          catalog.unknown_points += 1;
          catalog.text_len += value.len();
        }
      }
    }
    if let Some(connections) = data_model.connection_list.as_ref() {
      catalog.connections = connections.connection.len();
      for connection in &connections.connection {
        catalog.text_len += connection.model_id.len()
          + connection.source_id.len()
          + connection.destination_id.len()
          + connection
            .parent_transition_id
            .as_ref()
            .map_or(0, |value| value.len())
          + connection
            .sibling_transition_id
            .as_ref()
            .map_or(0, |value| value.len())
          + connection
            .presentation_id
            .as_ref()
            .map_or(0, |value| value.len());
        catalog.extension_markers += usize::from(connection.extension_list.is_some());
      }
    }
    catalog
  }
}

impl DiagramLayoutCatalog {
  fn from_part(
    package: &mut SpreadsheetDocument,
    part: &DiagramLayoutDefinitionPart,
  ) -> Result<Self> {
    let model = {
      let layout = part.root_element(package)?;
      Self::from_layout(part.relationship_id().map(ToString::to_string), layout)
    };
    Ok(Self {
      images: part.image_parts(package).count(),
      ..model
    })
  }

  fn from_layout(relationship_id: Option<String>, layout: &dgm::LayoutDefinition) -> Self {
    let mut stats = DiagramLayoutCatalog {
      relationship_id,
      titles: layout.title.len(),
      descriptions: layout.description.len(),
      has_category_list: layout.category_list.is_some(),
      has_sample_data: layout.sample_data.is_some(),
      has_style_data: layout.style_data.is_some(),
      has_color_data: layout.color_data.is_some(),
      extension_markers: usize::from(layout.diagram_definition_extension_list.is_some()),
      text_len: layout.unique_id.as_ref().map_or(0, |value| value.len())
        + layout.min_version.as_ref().map_or(0, |value| value.len())
        + layout.default_style.as_ref().map_or(0, |value| value.len())
        + layout
          .title
          .iter()
          .map(|title| title.val.len())
          .sum::<usize>()
        + layout
          .description
          .iter()
          .map(|description| description.val.len())
          .sum::<usize>(),
      ..Self::default()
    };
    collect_layout_node(&layout.layout_node, &mut stats);
    stats
  }
}

impl DiagramStyleCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &DiagramStylePart) -> Result<Self> {
    let style = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      titles: style.style_definition_title.len(),
      descriptions: style.style_label_description.len(),
      has_categories: style.style_display_categories.is_some(),
      has_scene3d: style.scene3_d.is_some(),
      labels: style.style_label.len(),
      extension_markers: usize::from(style.extension_list.is_some()),
      text_len: style.unique_id.as_ref().map_or(0, |value| value.len())
        + style.min_version.as_ref().map_or(0, |value| value.len())
        + style
          .style_definition_title
          .iter()
          .map(|title| title.val.len())
          .sum::<usize>()
        + style
          .style_label_description
          .iter()
          .map(|description| description.val.len())
          .sum::<usize>()
        + style
          .style_label
          .iter()
          .map(|label| label.name.len())
          .sum::<usize>(),
    })
  }
}

impl DiagramColorCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &DiagramColorsPart) -> Result<Self> {
    let colors = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      titles: colors.color_definition_title.len(),
      descriptions: colors.color_transform_description.len(),
      has_categories: colors.color_transform_categories.is_some(),
      labels: colors.color_transform_style_label.len(),
      extension_markers: usize::from(colors.extension_list.is_some()),
      text_len: colors.unique_id.as_ref().map_or(0, |value| value.len())
        + colors.min_version.as_ref().map_or(0, |value| value.len())
        + colors
          .color_definition_title
          .iter()
          .map(|title| title.val.len())
          .sum::<usize>()
        + colors
          .color_transform_description
          .iter()
          .map(|description| description.val.len())
          .sum::<usize>()
        + colors
          .color_transform_style_label
          .iter()
          .map(|label| label.name.len())
          .sum::<usize>(),
    })
  }
}

impl DiagramDrawingCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &DiagramPersistLayoutPart) -> Result<Self> {
    let model = {
      let drawing = part.root_element(package)?;
      Self::from_drawing(part.relationship_id().map(ToString::to_string), drawing)
    };
    Ok(Self {
      images: part.image_parts(package).count(),
      ..model
    })
  }

  fn from_drawing(relationship_id: Option<String>, drawing: &dsp::Drawing) -> Self {
    let mut catalog = Self {
      relationship_id,
      extension_markers: usize::from(drawing.shape_tree.office_art_extension_list.is_some()),
      ..Self::default()
    };
    for child in &drawing.shape_tree.shape_tree_choice {
      collect_diagram_shape_tree_choice(child, &mut catalog);
    }
    catalog
  }
}

impl ChartResourceCatalog {
  pub(crate) fn from_chart_part(
    package: &mut SpreadsheetDocument,
    part: &ChartPart,
  ) -> Result<Self> {
    let model = {
      let chart_space = part.root_element(package)?;
      Self::from_chart_space(part.relationship_id().map(ToString::to_string), chart_space)
    };
    Ok(Self {
      has_chart_drawing: part.chart_drawing_part(package).is_some(),
      has_embedded_package: part.embedded_package_part(package).is_some(),
      images: part.image_parts(package).count(),
      has_theme_override: part.theme_override_part(package).is_some(),
      styles: part.chart_style_parts(package).count(),
      color_styles: part.chart_color_style_parts(package).count(),
      ..model
    })
  }

  pub(crate) fn from_extended_chart_part(
    package: &mut SpreadsheetDocument,
    part: &ExtendedChartPart,
  ) -> Result<Self> {
    let model = {
      let chart_space = part.root_element(package)?;
      Self::from_extended_chart_space(part.relationship_id().map(ToString::to_string), chart_space)
    };
    Ok(Self {
      has_chart_drawing: part.chart_drawing_part(package).is_some(),
      has_embedded_package: part.embedded_package_part(package).is_some(),
      images: part.image_parts(package).count(),
      has_theme_override: part.theme_override_part(package).is_some(),
      styles: part.chart_style_parts(package).count(),
      color_styles: part.chart_color_style_parts(package).count(),
      ..model
    })
  }

  pub(crate) fn child_resource_count(&self) -> usize {
    usize::from(self.has_chart_drawing)
      + usize::from(self.has_embedded_package)
      + self.images
      + usize::from(self.has_theme_override)
      + self.styles
      + self.color_styles
  }

  fn from_chart_space(relationship_id: Option<String>, chart_space: &c::ChartSpace) -> Self {
    let chart = &chart_space.chart;
    let plot_area = &chart.plot_area;
    Self {
      relationship_id,
      extended: false,
      version_len: chart_space.version.as_ref().map_or(0, |value| value.len()),
      feature_list_len: chart_space
        .feature_list
        .as_ref()
        .map_or(0, |value| value.len()),
      has_fallback_image: chart_space.fallback_img.is_some(),
      date1904: chart_space
        .date1904
        .as_ref()
        .is_some_and(|value| value.val.is_some_and(|value| value.as_bool())),
      rounded_corners: chart_space
        .rounded_corners
        .as_ref()
        .is_some_and(|value| value.val.is_some_and(|value| value.as_bool())),
      has_style: chart_space.chart_space_choice.is_some(),
      has_pivot_source: chart_space.pivot_source.is_some(),
      protection_flags: chart_space
        .protection
        .as_deref()
        .map_or(0, protection_flag_count),
      has_title: chart.title.is_some(),
      has_3d_view: chart.view3_d.is_some(),
      chart_type_groups: plot_area.plot_area_choice1.len(),
      axes: plot_area.plot_area_choice2.len(),
      has_legend: chart.legend.is_some(),
      chart_flags: usize::from(
        chart
          .auto_title_deleted
          .as_ref()
          .is_some_and(|value| value.val.is_some_and(|value| value.as_bool())),
      ) + usize::from(chart.pivot_formats.is_some())
        + usize::from(chart.floor.is_some())
        + usize::from(chart.side_wall.is_some())
        + usize::from(chart.back_wall.is_some())
        + usize::from(
          chart
            .plot_visible_only
            .as_ref()
            .is_some_and(|value| value.val.is_some_and(|value| value.as_bool())),
        )
        + usize::from(chart.display_blanks_as.is_some())
        + usize::from(
          chart
            .show_data_labels_over_maximum
            .as_ref()
            .is_some_and(|value| value.val.is_some_and(|value| value.as_bool())),
        )
        + usize::from(plot_area.layout.is_some())
        + usize::from(plot_area.data_table.is_some())
        + usize::from(plot_area.shape_properties.is_some()),
      has_root_shape_properties: chart_space.shape_properties.is_some(),
      has_text_properties: chart_space.text_properties.is_some(),
      external_data_relationship_id: chart_space
        .external_data
        .as_ref()
        .map(|external_data| external_data.id.clone()),
      external_data_auto_update: chart_space
        .external_data
        .as_ref()
        .is_some_and(|external_data| {
          external_data
            .auto_update
            .as_ref()
            .is_some_and(|value| value.val.is_some_and(|value| value.as_bool()))
        }),
      has_print_settings: chart_space.print_settings.is_some(),
      has_user_shapes_reference: chart_space.user_shapes_reference.is_some(),
      extension_markers: usize::from(chart_space.chart_space_extension_list.is_some())
        + usize::from(chart.chart_extension_list.is_some())
        + usize::from(plot_area.extension_list.is_some()),
      chartex_data_sets: 0,
      chartex_series: 0,
      ..Self::default()
    }
  }

  fn from_extended_chart_space(
    relationship_id: Option<String>,
    chart_space: &cx::ChartSpace,
  ) -> Self {
    let chart = &chart_space.chart;
    let plot_area = &chart.plot_area;
    let chart_data = chart_space.chart_data.as_ref();
    Self {
      relationship_id,
      extended: true,
      version_len: chart_space.version.as_ref().map_or(0, |value| value.len()),
      feature_list_len: chart_space
        .feature_list
        .as_ref()
        .map_or(0, |value| value.len()),
      has_fallback_image: chart_space.fallback_img.is_some(),
      has_title: chart.chart_title.is_some(),
      chart_type_groups: 1,
      axes: plot_area.axis.len(),
      has_legend: chart.legend.is_some(),
      chart_flags: usize::from(chart_space.shape_properties.is_some())
        + usize::from(chart_space.tx_pr_text_body.is_some())
        + usize::from(chart_space.color_mapping_type.is_some())
        + usize::from(chart_space.format_overrides.is_some())
        + usize::from(plot_area.shape_properties.is_some())
        + usize::from(plot_area.plot_area_region.plot_surface.is_some()),
      has_root_shape_properties: chart_space.shape_properties.is_some(),
      has_text_properties: chart_space.tx_pr_text_body.is_some(),
      external_data_relationship_id: chart_data
        .and_then(|data| data.external_data.as_ref())
        .map(|external_data| external_data.r_id.clone()),
      external_data_auto_update: chart_data
        .and_then(|data| data.external_data.as_ref())
        .is_some_and(|external_data| {
          external_data
            .cx_auto_update
            .is_some_and(|value| value.as_bool())
        }),
      has_print_settings: chart_space.print_settings.is_some(),
      extension_markers: usize::from(chart_space.extension_list.is_some())
        + usize::from(chart.extension_list.is_some())
        + usize::from(plot_area.extension_list.is_some())
        + usize::from(plot_area.plot_area_region.extension_list.is_some())
        + chart_data.map_or(0, |data| usize::from(data.extension_list.is_some())),
      chartex_data_sets: chart_data.map_or(0, |data| data.data.len()),
      chartex_series: plot_area.plot_area_region.series.len(),
      ..Self::default()
    }
  }
}

fn protection_flag_count(protection: &c::Protection) -> usize {
  usize::from(protection.chart_object.is_some())
    + usize::from(protection.data.is_some())
    + usize::from(protection.formatting.is_some())
    + usize::from(protection.selection.is_some())
    + usize::from(protection.user_interface.is_some())
}

fn collect_layout_node(node: &dgm::LayoutNode, stats: &mut DiagramLayoutCatalog) {
  stats.layout_nodes += 1;
  stats.text_len += node.name.as_ref().map_or(0, |value| value.len())
    + node.style_label.as_ref().map_or(0, |value| value.len())
    + node.move_with.as_ref().map_or(0, |value| value.len());
  for choice in &node.layout_node_choice {
    match choice {
      dgm::LayoutNodeChoice::Algorithm(_) => stats.algorithms += 1,
      dgm::LayoutNodeChoice::Shape(_) => stats.shapes += 1,
      dgm::LayoutNodeChoice::PresentationOf(_) => stats.presentation_of += 1,
      dgm::LayoutNodeChoice::Constraints(_) => stats.constraints += 1,
      dgm::LayoutNodeChoice::RuleList(_) => stats.rules += 1,
      dgm::LayoutNodeChoice::VariableList(_) => stats.variables += 1,
      dgm::LayoutNodeChoice::ForEach(for_each) => collect_for_each(for_each, stats),
      dgm::LayoutNodeChoice::LayoutNode(child) => collect_layout_node(child, stats),
      dgm::LayoutNodeChoice::Choose(choose) => collect_choose(choose, stats),
      dgm::LayoutNodeChoice::ExtensionList(_) => stats.extension_markers += 1,
    }
  }
}

fn collect_for_each(for_each: &dgm::ForEach, stats: &mut DiagramLayoutCatalog) {
  stats.for_each += 1;
  stats.text_len += for_each.name.as_ref().map_or(0, |value| value.len())
    + for_each.reference.as_ref().map_or(0, |value| value.len());
  for choice in &for_each.for_each_choice {
    match choice {
      dgm::ForEachChoice::Algorithm(_) => stats.algorithms += 1,
      dgm::ForEachChoice::Shape(_) => stats.shapes += 1,
      dgm::ForEachChoice::PresentationOf(_) => stats.presentation_of += 1,
      dgm::ForEachChoice::Constraints(_) => stats.constraints += 1,
      dgm::ForEachChoice::RuleList(_) => stats.rules += 1,
      dgm::ForEachChoice::ForEach(child) => collect_for_each(child, stats),
      dgm::ForEachChoice::LayoutNode(child) => collect_layout_node(child, stats),
      dgm::ForEachChoice::Choose(choose) => collect_choose(choose, stats),
      dgm::ForEachChoice::ExtensionList(_) => stats.extension_markers += 1,
    }
  }
}

fn collect_choose(choose: &dgm::Choose, stats: &mut DiagramLayoutCatalog) {
  stats.choose += 1;
  stats.text_len += choose.name.as_ref().map_or(0, |value| value.len());
  for choice_if in &choose.diagram_choose_if {
    stats.text_len += choice_if.name.as_ref().map_or(0, |value| value.len())
      + choice_if.argument.as_ref().map_or(0, |value| value.len())
      + choice_if.val.len();
    for choice in &choice_if.diagram_choose_if_choice {
      match choice {
        dgm::DiagramChooseIfChoice::Algorithm(_) => stats.algorithms += 1,
        dgm::DiagramChooseIfChoice::Shape(_) => stats.shapes += 1,
        dgm::DiagramChooseIfChoice::PresentationOf(_) => stats.presentation_of += 1,
        dgm::DiagramChooseIfChoice::Constraints(_) => stats.constraints += 1,
        dgm::DiagramChooseIfChoice::RuleList(_) => stats.rules += 1,
        dgm::DiagramChooseIfChoice::ForEach(for_each) => collect_for_each(for_each, stats),
        dgm::DiagramChooseIfChoice::LayoutNode(node) => collect_layout_node(node, stats),
        dgm::DiagramChooseIfChoice::Choose(choose) => collect_choose(choose, stats),
        dgm::DiagramChooseIfChoice::ExtensionList(_) => stats.extension_markers += 1,
      }
    }
  }
  if let Some(choice_else) = choose.diagram_choose_else.as_ref() {
    stats.text_len += choice_else.name.as_ref().map_or(0, |value| value.len());
    for choice in &choice_else.diagram_choose_else_choice {
      match choice {
        dgm::DiagramChooseElseChoice::Algorithm(_) => stats.algorithms += 1,
        dgm::DiagramChooseElseChoice::Shape(_) => stats.shapes += 1,
        dgm::DiagramChooseElseChoice::PresentationOf(_) => stats.presentation_of += 1,
        dgm::DiagramChooseElseChoice::Constraints(_) => stats.constraints += 1,
        dgm::DiagramChooseElseChoice::RuleList(_) => stats.rules += 1,
        dgm::DiagramChooseElseChoice::ForEach(for_each) => collect_for_each(for_each, stats),
        dgm::DiagramChooseElseChoice::LayoutNode(node) => collect_layout_node(node, stats),
        dgm::DiagramChooseElseChoice::Choose(choose) => collect_choose(choose, stats),
        dgm::DiagramChooseElseChoice::ExtensionList(_) => stats.extension_markers += 1,
      }
    }
  }
}

fn collect_diagram_shape_tree_choice(
  choice: &dsp::ShapeTreeChoice,
  catalog: &mut DiagramDrawingCatalog,
) {
  match choice {
    dsp::ShapeTreeChoice::Shape(shape) => collect_diagram_shape(shape, catalog),
    dsp::ShapeTreeChoice::GroupShape(group) => collect_diagram_group_shape(group, catalog),
  }
}

fn collect_diagram_group_shape_choice(
  choice: &dsp::GroupShapeChoice,
  catalog: &mut DiagramDrawingCatalog,
) {
  match choice {
    dsp::GroupShapeChoice::Shape(shape) => collect_diagram_shape(shape, catalog),
    dsp::GroupShapeChoice::GroupShape(group) => collect_diagram_group_shape(group, catalog),
  }
}

fn collect_diagram_shape(shape: &dsp::Shape, catalog: &mut DiagramDrawingCatalog) {
  catalog.shapes += 1;
  catalog.text_len += shape.model_id.len()
    + shape
      .shape_non_visual_properties
      .non_visual_drawing_properties
      .name
      .len()
    + shape
      .shape_non_visual_properties
      .non_visual_drawing_properties
      .description
      .as_ref()
      .map_or(0, |value| value.len());
  catalog.text_shapes += usize::from(shape.text_body.is_some());
  catalog.styled_shapes += usize::from(shape.shape_style.is_some());
  catalog.transformed_shapes += usize::from(shape.transform2_d.is_some())
    + usize::from(shape.shape_properties.transform2_d.is_some());
  catalog.extension_markers += usize::from(shape.office_art_extension_list.is_some())
    + usize::from(
      shape
        .shape_non_visual_properties
        .non_visual_drawing_shape_properties
        .extension_list
        .is_some(),
    )
    + usize::from(
      shape
        .shape_properties
        .shape_properties_extension_list
        .is_some(),
    );
}

fn collect_diagram_group_shape(group: &dsp::GroupShape, catalog: &mut DiagramDrawingCatalog) {
  catalog.groups += 1;
  catalog.text_len += group
    .group_shape_non_visual_properties
    .non_visual_drawing_properties
    .name
    .len()
    + group
      .group_shape_non_visual_properties
      .non_visual_drawing_properties
      .description
      .as_ref()
      .map_or(0, |value| value.len());
  catalog.transformed_shapes += usize::from(
    group
      .group_shape_properties
      .transform_group
      .as_ref()
      .is_some(),
  );
  catalog.extension_markers += usize::from(group.office_art_extension_list.is_some())
    + usize::from(group.group_shape_properties.extension_list.is_some())
    + usize::from(
      group
        .group_shape_non_visual_properties
        .non_visual_group_drawing_shape_properties
        .non_visual_group_drawing_shape_props_extension_list
        .is_some(),
    );
  for choice in &group.group_shape_choice {
    collect_diagram_group_shape_choice(choice, catalog);
  }
}
