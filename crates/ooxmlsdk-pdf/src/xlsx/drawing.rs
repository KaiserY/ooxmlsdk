use ooxmlsdk::parts::chart_part::ChartPart;
use ooxmlsdk::parts::drawings_part::DrawingsPart;
use ooxmlsdk::parts::extended_chart_part::ExtendedChartPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing as xdr;

use crate::error::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct DrawingResourceCatalog {
  pub(crate) anchors: Vec<DrawingAnchorModel>,
  pub(crate) charts: Vec<ChartResourceCatalog>,
  pub(crate) extended_charts: Vec<ChartResourceCatalog>,
  pub(crate) diagram_data: usize,
  pub(crate) diagram_layouts: usize,
  pub(crate) diagram_styles: usize,
  pub(crate) diagram_colors: usize,
  pub(crate) diagram_drawings: usize,
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
  pub(crate) has_chart_drawing: bool,
  pub(crate) has_embedded_package: bool,
  pub(crate) images: usize,
  pub(crate) has_theme_override: bool,
  pub(crate) styles: usize,
  pub(crate) color_styles: usize,
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
    Ok(Self {
      anchors,
      charts: chart_parts
        .iter()
        .map(|chart| ChartResourceCatalog::from_chart_part(package, &chart))
        .collect(),
      extended_charts: extended_chart_parts
        .iter()
        .map(|chart| ChartResourceCatalog::from_extended_chart_part(package, &chart))
        .collect(),
      diagram_data: part.diagram_data_parts(package).count(),
      diagram_layouts: part.diagram_layout_definition_parts(package).count(),
      diagram_styles: part.diagram_style_parts(package).count(),
      diagram_colors: part.diagram_colors_parts(package).count(),
      diagram_drawings: part.diagram_persist_layout_parts(package).count(),
      images: part.image_parts(package).count(),
      custom_xml_parts: part.custom_xml_parts(package).count(),
      web_extensions: part.web_extension_parts(package).count(),
    })
  }

  pub(crate) fn chart_count(&self) -> usize {
    self.charts.len() + self.extended_charts.len()
  }

  pub(crate) fn diagram_resource_count(&self) -> usize {
    self.diagram_data
      + self.diagram_layouts
      + self.diagram_styles
      + self.diagram_colors
      + self.diagram_drawings
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

impl ChartResourceCatalog {
  pub(crate) fn from_chart_part(package: &mut SpreadsheetDocument, part: &ChartPart) -> Self {
    Self {
      has_chart_drawing: part.chart_drawing_part(package).is_some(),
      has_embedded_package: part.embedded_package_part(package).is_some(),
      images: part.image_parts(package).count(),
      has_theme_override: part.theme_override_part(package).is_some(),
      styles: part.chart_style_parts(package).count(),
      color_styles: part.chart_color_style_parts(package).count(),
    }
  }

  pub(crate) fn from_extended_chart_part(
    package: &mut SpreadsheetDocument,
    part: &ExtendedChartPart,
  ) -> Self {
    Self {
      has_chart_drawing: part.chart_drawing_part(package).is_some(),
      has_embedded_package: part.embedded_package_part(package).is_some(),
      images: part.image_parts(package).count(),
      has_theme_override: part.theme_override_part(package).is_some(),
      styles: part.chart_style_parts(package).count(),
      color_styles: part.chart_color_style_parts(package).count(),
    }
  }

  pub(crate) fn child_resource_count(&self) -> usize {
    usize::from(self.has_chart_drawing)
      + usize::from(self.has_embedded_package)
      + self.images
      + usize::from(self.has_theme_override)
      + self.styles
      + self.color_styles
  }
}
