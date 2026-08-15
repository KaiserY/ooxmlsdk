use ooxmlsdk::parts::control_properties_part::ControlPropertiesPart;
use ooxmlsdk::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart;
use std::collections::HashMap;

use ooxmlsdk::parts::image_part::ImagePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::vml_drawing_part::VmlDrawingPart;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_excel as xvml;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main as x14;
use ooxmlsdk::schemas::schemas_microsoft_com_office_word as wvml;
use ooxmlsdk::schemas::schemas_microsoft_com_vml as vml;
use ooxmlsdk::sdk::{SdkPart, SdkType};
use quick_xml::events::Event;
use quick_xml::{Writer, events::BytesStart};

use super::drawing::ImageResource;
use super::worksheet::{CellAddress, CellRange};
use crate::error::Result;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorksheetObjectResourceCatalog {
  pub(crate) vml_drawings: Vec<VmlDrawingResourceCatalog>,
  pub(crate) controls: Vec<ControlPersistenceResourceCatalog>,
  pub(crate) control_properties: Vec<ControlPropertiesResourceCatalog>,
  pub(crate) embedded_objects: Vec<BinaryResourceCatalog>,
  pub(crate) embedded_packages: Vec<BinaryResourceCatalog>,
  pub(crate) images: usize,
  pub(crate) named_sheet_views: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct VmlDrawingResourceCatalog {
  pub(crate) images: usize,
  pub(crate) image_resources: HashMap<String, ImageResource>,
  pub(crate) legacy_diagram_texts: usize,
  pub(crate) shapes: Vec<VmlShapeModel>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VmlShapeModel {
  pub(crate) is_shape_type: bool,
  pub(crate) kind: VmlShapeKind,
  pub(crate) id: Option<String>,
  pub(crate) shape_id: Option<String>,
  pub(crate) text: String,
  pub(crate) text_style: VmlTextStyle,
  pub(crate) text_horizontal_alignment: Option<String>,
  pub(crate) text_vertical_alignment: Option<String>,
  pub(crate) style: Option<String>,
  pub(crate) object_type: Option<String>,
  pub(crate) image_relationship_id: Option<String>,
  pub(crate) shape_type_reference: Option<String>,
  pub(crate) coordinate_size: Option<String>,
  pub(crate) coordinate_origin: Option<String>,
  pub(crate) path: Option<String>,
  pub(crate) fill_color: Option<String>,
  pub(crate) fill_color2: Option<String>,
  pub(crate) fill_opacity: Option<String>,
  pub(crate) fill_opacity2: Option<String>,
  pub(crate) fill_colors: Option<String>,
  pub(crate) fill_type: Option<vml::FillTypeValues>,
  pub(crate) fill_angle: Option<String>,
  pub(crate) fill_focus: Option<String>,
  pub(crate) fill_focus_position: Option<String>,
  pub(crate) fill_focus_size: Option<String>,
  pub(crate) fill_method: Option<vml::FillMethodValues>,
  pub(crate) fill_rotate_with_shape: Option<bool>,
  pub(crate) fill_image_relationship_id: Option<String>,
  pub(crate) fill_image_size: Option<String>,
  pub(crate) fill_image_origin: Option<String>,
  pub(crate) fill_image_position: Option<String>,
  pub(crate) fill_image_aspect: Option<vml::ImageAspectValues>,
  pub(crate) stroke_color: Option<String>,
  pub(crate) stroke_opacity: Option<String>,
  pub(crate) stroke_weight: Option<String>,
  pub(crate) stroke_dash_style: Option<String>,
  pub(crate) stroke_line_style: Option<vml::StrokeLineStyleValues>,
  pub(crate) stroke_join_style: Option<vml::StrokeJoinStyleValues>,
  pub(crate) stroke_end_cap: Option<vml::StrokeEndCapValues>,
  pub(crate) stroke_start_arrow: Option<vml::StrokeArrowValues>,
  pub(crate) stroke_start_arrow_width: Option<vml::StrokeArrowWidthValues>,
  pub(crate) stroke_start_arrow_length: Option<vml::StrokeArrowLengthValues>,
  pub(crate) stroke_end_arrow: Option<vml::StrokeArrowValues>,
  pub(crate) stroke_end_arrow_width: Option<vml::StrokeArrowWidthValues>,
  pub(crate) stroke_end_arrow_length: Option<vml::StrokeArrowLengthValues>,
  pub(crate) wrap_type: Option<wvml::WrapValues>,
  pub(crate) wrap_side: Option<wvml::WrapSideValues>,
  pub(crate) wrap_anchor_x: Option<wvml::HorizontalAnchorValues>,
  pub(crate) wrap_anchor_y: Option<wvml::VerticalAnchorValues>,
  pub(crate) filled: bool,
  pub(crate) stroked: bool,
  pub(crate) filled_authored: Option<bool>,
  pub(crate) stroked_authored: Option<bool>,
  pub(crate) from: Option<String>,
  pub(crate) to: Option<String>,
  pub(crate) control1: Option<String>,
  pub(crate) control2: Option<String>,
  pub(crate) points: Option<String>,
  pub(crate) start_angle: Option<String>,
  pub(crate) end_angle: Option<String>,
  pub(crate) arc_size: Option<String>,
  pub(crate) anchor: Option<VmlClientAnchor>,
  pub(crate) auto_size_picture: bool,
  pub(crate) checked: Option<i64>,
  pub(crate) disable_3d: bool,
  pub(crate) note_row: Option<u32>,
  pub(crate) note_column: Option<u32>,
  pub(crate) print_object: bool,
  pub(crate) allow_in_cell: bool,
  pub(crate) visible: bool,
  pub(crate) hidden: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct VmlTextStyle {
  pub(crate) font_family: Option<String>,
  pub(crate) font_size_twips: Option<i32>,
  pub(crate) color: Option<String>,
  pub(crate) bold: bool,
  pub(crate) italic: bool,
  pub(crate) underline: bool,
  pub(crate) strikethrough: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum VmlShapeKind {
  Shape,
  Group,
  Arc,
  Curve,
  Image,
  Line,
  Oval,
  Polyline,
  #[default]
  Rectangle,
  RoundRectangle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct VmlClientAnchor {
  pub(crate) from_col: u32,
  pub(crate) from_col_offset_px: i32,
  pub(crate) from_row: u32,
  pub(crate) from_row_offset_px: i32,
  pub(crate) to_col: u32,
  pub(crate) to_col_offset_px: i32,
  pub(crate) to_row: u32,
  pub(crate) to_row_offset_px: i32,
}

impl VmlClientAnchor {
  pub(crate) fn cell_range(self) -> CellRange {
    CellRange::new(
      CellAddress {
        col: self.from_col.saturating_add(1),
        row: self.from_row.saturating_add(1),
      },
      CellAddress {
        col: self.to_col.saturating_add(1),
        row: self.to_row.saturating_add(1),
      },
    )
  }
}

impl Default for VmlShapeModel {
  fn default() -> Self {
    Self {
      kind: VmlShapeKind::Rectangle,
      is_shape_type: false,
      id: None,
      shape_id: None,
      text: String::new(),
      text_style: VmlTextStyle::default(),
      text_horizontal_alignment: None,
      text_vertical_alignment: None,
      style: None,
      object_type: None,
      image_relationship_id: None,
      shape_type_reference: None,
      coordinate_size: None,
      coordinate_origin: None,
      path: None,
      fill_color: None,
      fill_color2: None,
      fill_opacity: None,
      fill_opacity2: None,
      fill_colors: None,
      fill_type: None,
      fill_angle: None,
      fill_focus: None,
      fill_focus_position: None,
      fill_focus_size: None,
      fill_method: None,
      fill_rotate_with_shape: None,
      fill_image_relationship_id: None,
      fill_image_size: None,
      fill_image_origin: None,
      fill_image_position: None,
      fill_image_aspect: None,
      stroke_color: None,
      stroke_opacity: None,
      stroke_weight: None,
      stroke_dash_style: None,
      stroke_line_style: None,
      stroke_join_style: None,
      stroke_end_cap: None,
      stroke_start_arrow: None,
      stroke_start_arrow_width: None,
      stroke_start_arrow_length: None,
      stroke_end_arrow: None,
      stroke_end_arrow_width: None,
      stroke_end_arrow_length: None,
      wrap_type: None,
      wrap_side: None,
      wrap_anchor_x: None,
      wrap_anchor_y: None,
      filled: true,
      stroked: true,
      filled_authored: None,
      stroked_authored: None,
      from: None,
      to: None,
      control1: None,
      control2: None,
      points: None,
      start_angle: None,
      end_angle: None,
      arc_size: None,
      anchor: None,
      auto_size_picture: false,
      checked: None,
      disable_3d: false,
      note_row: None,
      note_column: None,
      print_object: true,
      allow_in_cell: true,
      visible: false,
      hidden: false,
    }
  }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ControlPersistenceResourceCatalog {
  pub(crate) binary_data_parts: Vec<BinaryResourceCatalog>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ControlPropertiesResourceCatalog {
  pub(crate) has_object_type: bool,
  pub(crate) has_checked: bool,
  pub(crate) boolean_flags: usize,
  pub(crate) numeric_fields: usize,
  pub(crate) formula_fields: usize,
  pub(crate) alignment_fields: usize,
  pub(crate) list_items: usize,
  pub(crate) has_extension_list: bool,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct BinaryResourceCatalog;

impl WorksheetObjectResourceCatalog {
  pub(crate) fn from_worksheet_part(
    package: &SpreadsheetDocument,
    part: &WorksheetPart,
  ) -> Result<Self> {
    let vml_drawing_parts = part.vml_drawing_parts(package).collect::<Vec<_>>();
    let control_parts = part
      .embedded_control_persistence_parts(package)
      .collect::<Vec<_>>();
    let control_properties_parts = part.control_properties_parts(package).collect::<Vec<_>>();
    Ok(Self {
      vml_drawings: vml_drawing_parts
        .iter()
        .map(|part| VmlDrawingResourceCatalog::from_part(package, part))
        .collect(),
      controls: control_parts
        .iter()
        .map(|part| ControlPersistenceResourceCatalog::from_part(package, part))
        .collect(),
      control_properties: control_properties_parts
        .iter()
        .map(|part| ControlPropertiesResourceCatalog::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      embedded_objects: part
        .embedded_object_parts(package)
        .map(|part| BinaryResourceCatalog::from_part(&part))
        .collect(),
      embedded_packages: part
        .embedded_package_parts(package)
        .map(|part| BinaryResourceCatalog::from_part(&part))
        .collect(),
      images: part.image_parts(package).count(),
      named_sheet_views: part.named_sheet_views_parts(package).count(),
    })
  }

  pub(crate) fn from_chartsheet_part(
    package: &SpreadsheetDocument,
    part: &ooxmlsdk::parts::chartsheet_part::ChartsheetPart,
  ) -> Self {
    let vml_drawing_parts = part.vml_drawing_parts(package).collect::<Vec<_>>();
    Self {
      vml_drawings: vml_drawing_parts
        .iter()
        .map(|part| VmlDrawingResourceCatalog::from_part(package, part))
        .collect(),
      images: part.image_parts(package).count(),
      ..Self::default()
    }
  }
}

impl VmlDrawingResourceCatalog {
  fn from_part(package: &SpreadsheetDocument, part: &VmlDrawingPart) -> Self {
    let shapes = part
      .try_data_bytes(package)
      .ok()
      .map(|data| vml_shapes(&data))
      .unwrap_or_default();
    Self {
      images: part.image_parts(package).count(),
      image_resources: collect_vml_image_resources(package, part),
      legacy_diagram_texts: part.legacy_diagram_text_parts(package).count(),
      shapes,
    }
  }
}

pub(crate) fn vml_shapes(data: &[u8]) -> Vec<VmlShapeModel> {
  let normalized = strip_office_vml_conditional_markers(data);
  let mut reader = quick_xml::Reader::from_reader(normalized.as_slice());
  reader.config_mut().trim_text(false);

  let mut shapes = Vec::new();
  let mut writer: Option<Writer<Vec<u8>>> = None;
  let mut fragment_depth = 0usize;
  let mut fragment_local_name = Vec::new();
  let mut namespace_attrs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        if let Some(shape_writer) = writer.as_mut() {
          fragment_depth = fragment_depth.saturating_add(1);
          if shape_writer
            .write_event(Event::Start(event.into_owned()))
            .is_err()
          {
            writer = None;
            fragment_depth = 0;
            fragment_local_name.clear();
          }
        } else if is_vml_shape_element(event.name().as_ref()) {
          let mut shape_writer = Writer::new(Vec::new());
          fragment_local_name = xml_local_name(event.name().as_ref()).to_vec();
          let mut start = event.into_owned();
          push_missing_xml_namespace_attrs(&mut start, &namespace_attrs);
          if shape_writer.write_event(Event::Start(start)).is_ok() {
            writer = Some(shape_writer);
            fragment_depth = 1;
          }
        } else {
          collect_xml_namespace_attrs(&event, &mut namespace_attrs);
        }
      }
      Ok(Event::Empty(event)) => {
        if let Some(shape_writer) = writer.as_mut() {
          if shape_writer
            .write_event(Event::Empty(event.into_owned()))
            .is_err()
          {
            writer = None;
            fragment_depth = 0;
            fragment_local_name.clear();
          }
        } else if is_vml_shape_element(event.name().as_ref()) {
          let mut shape_writer = Writer::new(Vec::new());
          let local_name = xml_local_name(event.name().as_ref()).to_vec();
          let mut start = event.into_owned();
          push_missing_xml_namespace_attrs(&mut start, &namespace_attrs);
          if shape_writer.write_event(Event::Empty(start)).is_ok() {
            shapes.extend(vml_shape_models_from_bytes(
              &local_name,
              &shape_writer.into_inner(),
            ));
          }
        } else {
          collect_xml_namespace_attrs(&event, &mut namespace_attrs);
        }
      }
      Ok(Event::End(event)) => {
        if let Some(shape_writer) = writer.as_mut()
          && shape_writer
            .write_event(Event::End(event.into_owned()))
            .is_err()
        {
          writer = None;
          fragment_depth = 0;
          fragment_local_name.clear();
          continue;
        }
        if fragment_depth > 0 {
          fragment_depth -= 1;
          if fragment_depth == 0
            && let Some(shape_writer) = writer.take()
          {
            shapes.extend(vml_shape_models_from_bytes(
              &fragment_local_name,
              &shape_writer.into_inner(),
            ));
            fragment_local_name.clear();
          }
        }
      }
      Ok(Event::Eof) => break,
      Ok(event) => {
        if let Some(shape_writer) = writer.as_mut()
          && shape_writer.write_event(event.into_owned()).is_err()
        {
          writer = None;
          fragment_depth = 0;
          fragment_local_name.clear();
        }
      }
      Err(_) => break,
    }
  }
  resolve_vml_shape_type_inheritance(shapes)
}

fn strip_office_vml_conditional_markers(data: &[u8]) -> Vec<u8> {
  let mut output = Vec::with_capacity(data.len());
  let mut offset = 0;
  while offset < data.len() {
    let remaining = &data[offset..];
    let marker_end = if remaining.starts_with(b"<![if") {
      remaining
        .windows(2)
        .position(|window| window == b"]>")
        .map(|index| index + 2)
    } else if remaining.starts_with(b"<![endif]>") {
      Some(b"<![endif]>".len())
    } else {
      None
    };
    if let Some(marker_end) = marker_end {
      offset += marker_end;
    } else {
      output.push(data[offset]);
      offset += 1;
    }
  }
  output
}

fn is_vml_shape_element(name: &[u8]) -> bool {
  matches!(
    xml_local_name(name),
    b"shape"
      | b"group"
      | b"arc"
      | b"curve"
      | b"image"
      | b"line"
      | b"oval"
      | b"polyline"
      | b"rect"
      | b"roundrect"
      | b"shapetype"
  )
}

fn vml_shape_models_from_bytes(local_name: &[u8], bytes: &[u8]) -> Vec<VmlShapeModel> {
  macro_rules! typed_model {
    ($type:ty, $kind:expr) => {
      <$type>::from_bytes(bytes)
        .ok()
        .map(|shape| vml_shape_from_typed(&shape, $kind))
        .into_iter()
        .collect()
    };
  }

  match local_name {
    b"shapetype" => vml::Shapetype::from_bytes(bytes)
      .ok()
      .map(vml_shape_type_model)
      .into_iter()
      .collect(),
    b"shape" => vml::Shape::from_bytes(bytes)
      .ok()
      .map(|shape| {
        let mut model = vml_shape_from_typed(&shape, VmlShapeKind::Shape);
        model.shape_type_reference = shape.r#type;
        model.coordinate_origin = shape.coordinate_origin;
        model.path = shape.edge_path.or(model.path);
        model
      })
      .into_iter()
      .collect(),
    b"group" => vml::Group::from_bytes(bytes)
      .ok()
      .map(|group| vml_group_shape_models(&group))
      .unwrap_or_default(),
    b"arc" => vml::Arc::from_bytes(bytes)
      .ok()
      .map(|shape| {
        let mut model = vml_shape_from_typed(&shape, VmlShapeKind::Arc);
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        model.start_angle = shape.start_angle.as_ref().map(ToString::to_string);
        model.end_angle = shape.end_angle.as_ref().map(ToString::to_string);
        model
      })
      .into_iter()
      .collect(),
    b"curve" => vml::Curve::from_bytes(bytes)
      .ok()
      .map(|shape| {
        let mut model = vml_shape_from_typed(&shape, VmlShapeKind::Curve);
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        model.from = shape.from;
        model.to = shape.to;
        model.control1 = shape.control1;
        model.control2 = shape.control2;
        model
      })
      .into_iter()
      .collect(),
    b"image" => typed_model!(vml::ImageFile, VmlShapeKind::Image),
    b"line" => vml::Line::from_bytes(bytes)
      .ok()
      .map(|shape| {
        let mut model = vml_shape_from_typed(&shape, VmlShapeKind::Line);
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        model.from = shape.from;
        model.to = shape.to;
        model
      })
      .into_iter()
      .collect(),
    b"oval" => vml::Oval::from_bytes(bytes)
      .ok()
      .map(|shape| {
        let mut model = vml_shape_from_typed(&shape, VmlShapeKind::Oval);
        model.coordinate_origin = shape.coordinate_origin;
        model
      })
      .into_iter()
      .collect(),
    b"polyline" => vml::PolyLine::from_bytes(bytes)
      .ok()
      .map(|shape| {
        let mut model = vml_shape_from_typed(&shape, VmlShapeKind::Polyline);
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        model.points = shape.points;
        model
      })
      .into_iter()
      .collect(),
    b"rect" => vml::Rectangle::from_bytes(bytes)
      .ok()
      .map(|shape| {
        let mut model = vml_shape_from_typed(&shape, VmlShapeKind::Rectangle);
        model.coordinate_origin = shape.coordinate_origin;
        model
      })
      .into_iter()
      .collect(),
    b"roundrect" => vml::RoundRectangle::from_bytes(bytes)
      .ok()
      .map(|shape| {
        let mut model = vml_shape_from_typed(&shape, VmlShapeKind::RoundRectangle);
        model.arc_size = shape.arc_size;
        model
      })
      .into_iter()
      .collect(),
    _ => Vec::new(),
  }
}

fn vml_shape_type_model(shape_type: vml::Shapetype) -> VmlShapeModel {
  let path = shape_type.edge_path.clone().or_else(|| {
    shape_type
      .shapetype_choice
      .iter()
      .find_map(|choice| match choice {
        vml::ShapetypeChoice::Path(path) => path.value.clone(),
        _ => None,
      })
  });
  let mut model = VmlShapeModel {
    kind: VmlShapeKind::Shape,
    is_shape_type: true,
    id: shape_type.id,
    style: shape_type.style,
    coordinate_size: shape_type.coordinate_size,
    coordinate_origin: shape_type.coordinate_origin,
    path,
    fill_color: shape_type.fill_color,
    stroke_color: shape_type.stroke_color,
    stroke_weight: shape_type.stroke_weight,
    filled: shape_type.filled.is_none_or(|value| value.as_bool()),
    stroked: shape_type.stroked.is_none_or(|value| value.as_bool()),
    filled_authored: shape_type.filled.map(|value| value.as_bool()),
    stroked_authored: shape_type.stroked.map(|value| value.as_bool()),
    ..VmlShapeModel::default()
  };
  for choice in &shape_type.shapetype_choice {
    match choice {
      vml::ShapetypeChoice::Fill(fill) => collect_typed_vml_fill(&mut model, fill),
      vml::ShapetypeChoice::Stroke(stroke) => collect_typed_vml_stroke(&mut model, stroke),
      vml::ShapetypeChoice::TextWrap(text_wrap) => {
        model.wrap_type = text_wrap.r#type;
        model.wrap_side = text_wrap.side;
        model.wrap_anchor_x = text_wrap.anchor_x;
        model.wrap_anchor_y = text_wrap.anchor_y;
      }
      _ => {}
    }
  }
  model
}

fn resolve_vml_shape_type_inheritance(models: Vec<VmlShapeModel>) -> Vec<VmlShapeModel> {
  let shape_types = models
    .iter()
    .filter(|model| model.is_shape_type)
    .filter_map(|model| {
      Some((
        model.id.as_deref()?.trim_start_matches('#').to_string(),
        model.clone(),
      ))
    })
    .collect::<HashMap<_, _>>();
  models
    .into_iter()
    .filter(|model| !model.is_shape_type)
    .map(|mut model| {
      let Some(shape_type) = model
        .shape_type_reference
        .as_deref()
        .map(|value| value.trim_start_matches('#'))
        .and_then(|id| shape_types.get(id))
      else {
        return model;
      };
      model.style = merge_vml_style(shape_type.style.as_deref(), model.style.as_deref());
      if model.coordinate_size.is_none() {
        model
          .coordinate_size
          .clone_from(&shape_type.coordinate_size);
      }
      if model.coordinate_origin.is_none() {
        model
          .coordinate_origin
          .clone_from(&shape_type.coordinate_origin);
      }
      if model.path.is_none() {
        model.path.clone_from(&shape_type.path);
      }
      if model.fill_color.is_none() {
        model.fill_color.clone_from(&shape_type.fill_color);
      }
      if model.fill_color2.is_none() {
        model.fill_color2.clone_from(&shape_type.fill_color2);
      }
      if model.fill_opacity.is_none() {
        model.fill_opacity.clone_from(&shape_type.fill_opacity);
      }
      if model.fill_opacity2.is_none() {
        model.fill_opacity2.clone_from(&shape_type.fill_opacity2);
      }
      if model.fill_colors.is_none() {
        model.fill_colors.clone_from(&shape_type.fill_colors);
      }
      if model.fill_type.is_none() {
        model.fill_type = shape_type.fill_type;
      }
      if model.fill_angle.is_none() {
        model.fill_angle.clone_from(&shape_type.fill_angle);
      }
      if model.fill_focus.is_none() {
        model.fill_focus.clone_from(&shape_type.fill_focus);
      }
      if model.fill_focus_position.is_none() {
        model
          .fill_focus_position
          .clone_from(&shape_type.fill_focus_position);
      }
      if model.fill_focus_size.is_none() {
        model
          .fill_focus_size
          .clone_from(&shape_type.fill_focus_size);
      }
      if model.fill_method.is_none() {
        model.fill_method = shape_type.fill_method;
      }
      if model.fill_rotate_with_shape.is_none() {
        model.fill_rotate_with_shape = shape_type.fill_rotate_with_shape;
      }
      if model.fill_image_relationship_id.is_none() {
        model
          .fill_image_relationship_id
          .clone_from(&shape_type.fill_image_relationship_id);
      }
      if model.fill_image_size.is_none() {
        model
          .fill_image_size
          .clone_from(&shape_type.fill_image_size);
      }
      if model.fill_image_origin.is_none() {
        model
          .fill_image_origin
          .clone_from(&shape_type.fill_image_origin);
      }
      if model.fill_image_position.is_none() {
        model
          .fill_image_position
          .clone_from(&shape_type.fill_image_position);
      }
      if model.fill_image_aspect.is_none() {
        model.fill_image_aspect = shape_type.fill_image_aspect;
      }
      if model.stroke_color.is_none() {
        model.stroke_color.clone_from(&shape_type.stroke_color);
      }
      if model.stroke_opacity.is_none() {
        model.stroke_opacity.clone_from(&shape_type.stroke_opacity);
      }
      if model.stroke_weight.is_none() {
        model.stroke_weight.clone_from(&shape_type.stroke_weight);
      }
      if model.stroke_dash_style.is_none() {
        model
          .stroke_dash_style
          .clone_from(&shape_type.stroke_dash_style);
      }
      if model.stroke_line_style.is_none() {
        model.stroke_line_style = shape_type.stroke_line_style;
      }
      if model.stroke_join_style.is_none() {
        model.stroke_join_style = shape_type.stroke_join_style;
      }
      if model.stroke_end_cap.is_none() {
        model.stroke_end_cap = shape_type.stroke_end_cap;
      }
      if model.stroke_start_arrow.is_none() {
        model.stroke_start_arrow = shape_type.stroke_start_arrow;
      }
      if model.stroke_start_arrow_width.is_none() {
        model.stroke_start_arrow_width = shape_type.stroke_start_arrow_width;
      }
      if model.stroke_start_arrow_length.is_none() {
        model.stroke_start_arrow_length = shape_type.stroke_start_arrow_length;
      }
      if model.stroke_end_arrow.is_none() {
        model.stroke_end_arrow = shape_type.stroke_end_arrow;
      }
      if model.stroke_end_arrow_width.is_none() {
        model.stroke_end_arrow_width = shape_type.stroke_end_arrow_width;
      }
      if model.stroke_end_arrow_length.is_none() {
        model.stroke_end_arrow_length = shape_type.stroke_end_arrow_length;
      }
      if model.wrap_type.is_none() {
        model.wrap_type = shape_type.wrap_type;
      }
      if model.wrap_side.is_none() {
        model.wrap_side = shape_type.wrap_side;
      }
      if model.wrap_anchor_x.is_none() {
        model.wrap_anchor_x = shape_type.wrap_anchor_x;
      }
      if model.wrap_anchor_y.is_none() {
        model.wrap_anchor_y = shape_type.wrap_anchor_y;
      }
      if model.filled_authored.is_none() {
        model.filled = shape_type.filled;
      }
      if model.stroked_authored.is_none() {
        model.stroked = shape_type.stroked;
      }
      model
    })
    .collect()
}

fn merge_vml_style(base: Option<&str>, direct: Option<&str>) -> Option<String> {
  let mut declarations = Vec::<(String, String)>::new();
  for source in [base, direct].into_iter().flatten() {
    for declaration in source.split(';') {
      let Some((name, value)) = declaration.split_once(':') else {
        continue;
      };
      let name = name.trim().to_ascii_lowercase();
      if let Some(existing) = declarations
        .iter_mut()
        .find(|(existing_name, _)| *existing_name == name)
      {
        existing.1 = value.trim().to_string();
      } else {
        declarations.push((name, value.trim().to_string()));
      }
    }
  }
  (!declarations.is_empty()).then(|| {
    declarations
      .into_iter()
      .map(|(name, value)| format!("{name}:{value}"))
      .collect::<Vec<_>>()
      .join(";")
  })
}

trait VmlShapeElement {
  fn id(&self) -> Option<String>;
  fn shape_id(&self) -> Option<String>;
  fn style(&self) -> Option<String>;
  fn user_hidden(&self) -> bool;
  fn allow_in_cell(&self) -> bool;
  fn coordinate_size(&self) -> Option<String>;
  fn coordinate_origin(&self) -> Option<String>;
  fn edge_path(&self) -> Option<String>;
  fn fill_color(&self) -> Option<String>;
  fn stroke_color(&self) -> Option<String>;
  fn stroke_weight(&self) -> Option<String>;
  fn filled(&self) -> Option<bool>;
  fn stroked(&self) -> Option<bool>;
  fn collect_model_children(&self, model: &mut VmlShapeModel);
}

macro_rules! impl_vml_shape_element {
  ($type:ident, $children:ident, $choice:ident) => {
    impl VmlShapeElement for vml::$type {
      fn id(&self) -> Option<String> {
        self.id.clone()
      }

      fn shape_id(&self) -> Option<String> {
        self.optional_string.clone()
      }

      fn style(&self) -> Option<String> {
        self.style.clone()
      }

      fn user_hidden(&self) -> bool {
        self.user_hidden.is_some_and(|value| value.as_bool())
      }

      fn allow_in_cell(&self) -> bool {
        self.allow_in_cell.is_none_or(|value| value.as_bool())
      }

      fn coordinate_size(&self) -> Option<String> {
        self.coordinate_size.clone()
      }

      fn coordinate_origin(&self) -> Option<String> {
        None
      }

      fn edge_path(&self) -> Option<String> {
        self.$children.iter().find_map(|child| match child {
          vml::$choice::Path(path) => path.value.clone(),
          _ => None,
        })
      }

      fn fill_color(&self) -> Option<String> {
        self.fill_color.clone()
      }

      fn stroke_color(&self) -> Option<String> {
        self.stroke_color.clone()
      }

      fn stroke_weight(&self) -> Option<String> {
        self.stroke_weight.clone()
      }

      fn filled(&self) -> Option<bool> {
        self.filled.map(|value| value.as_bool())
      }

      fn stroked(&self) -> Option<bool> {
        self.stroked.map(|value| value.as_bool())
      }

      fn collect_model_children(&self, model: &mut VmlShapeModel) {
        for child in &self.$children {
          match child {
            vml::$choice::TextBox(text_box) => collect_typed_vml_textbox(model, text_box),
            vml::$choice::ImageData(image_data) => {
              collect_typed_vml_image_data(model, image_data);
            }
            vml::$choice::ClientData(client_data) => {
              collect_typed_vml_client_data(model, client_data);
            }
            vml::$choice::Fill(fill) => collect_typed_vml_fill(model, fill),
            vml::$choice::Stroke(stroke) => collect_typed_vml_stroke(model, stroke),
            vml::$choice::TextWrap(text_wrap) => {
              model.wrap_type = text_wrap.r#type;
              model.wrap_side = text_wrap.side;
              model.wrap_anchor_x = text_wrap.anchor_x;
              model.wrap_anchor_y = text_wrap.anchor_y;
            }
            _ => {}
          }
        }
      }
    }
  };
}

impl_vml_shape_element!(Shape, shape_choice, ShapeChoice);
impl_vml_shape_element!(Arc, arc_choice, ArcChoice);
impl_vml_shape_element!(Curve, curve_choice, CurveChoice);
impl_vml_shape_element!(ImageFile, image_file_choice, ImageFileChoice);
impl_vml_shape_element!(Line, line_choice, LineChoice);
impl_vml_shape_element!(Oval, oval_choice, OvalChoice);
impl_vml_shape_element!(PolyLine, poly_line_choice, PolyLineChoice);
impl_vml_shape_element!(Rectangle, rectangle_choice, RectangleChoice);
impl_vml_shape_element!(RoundRectangle, round_rectangle_choice, RoundRectangleChoice);

fn vml_shape_from_typed(shape: &impl VmlShapeElement, kind: VmlShapeKind) -> VmlShapeModel {
  let mut model = VmlShapeModel {
    kind,
    id: shape.id(),
    shape_id: shape.shape_id(),
    style: shape.style(),
    hidden: shape.user_hidden(),
    allow_in_cell: shape.allow_in_cell(),
    coordinate_size: shape.coordinate_size(),
    coordinate_origin: shape.coordinate_origin(),
    path: shape.edge_path(),
    fill_color: shape.fill_color(),
    stroke_color: shape.stroke_color(),
    stroke_weight: shape.stroke_weight(),
    filled: shape.filled().unwrap_or(true),
    stroked: shape.stroked().unwrap_or(true),
    filled_authored: shape.filled(),
    stroked_authored: shape.stroked(),
    ..VmlShapeModel::default()
  };
  if model
    .style
    .as_deref()
    .is_some_and(|style| style.contains("visibility:hidden"))
  {
    model.hidden = true;
  }
  shape.collect_model_children(&mut model);
  model.text = normalize_vml_text(&model.text);
  model
}

pub(crate) fn vml_arc_model(shape: &vml::Arc) -> VmlShapeModel {
  let mut model = vml_shape_from_typed(shape, VmlShapeKind::Arc);
  model.coordinate_origin.clone_from(&shape.coordinate_origin);
  model.start_angle = shape.start_angle.as_ref().map(ToString::to_string);
  model.end_angle = shape.end_angle.as_ref().map(ToString::to_string);
  model
}

pub(crate) fn vml_shape_model(
  shape: &vml::Shape,
  shape_type: Option<&vml::Shapetype>,
) -> VmlShapeModel {
  let mut model = vml_shape_from_typed(shape, VmlShapeKind::Shape);
  model.shape_type_reference.clone_from(&shape.r#type);
  model.path = shape.edge_path.clone().or(model.path);
  let Some(shape_type) = shape_type else {
    return model;
  };
  resolve_vml_shape_type_inheritance(vec![vml_shape_type_model(shape_type.clone()), model])
    .into_iter()
    .next()
    .unwrap_or_default()
}

pub(crate) fn vml_curve_model(shape: &vml::Curve) -> VmlShapeModel {
  let mut model = vml_shape_from_typed(shape, VmlShapeKind::Curve);
  model.coordinate_origin.clone_from(&shape.coordinate_origin);
  model.from.clone_from(&shape.from);
  model.to.clone_from(&shape.to);
  model.control1.clone_from(&shape.control1);
  model.control2.clone_from(&shape.control2);
  model
}

pub(crate) fn vml_line_model(shape: &vml::Line) -> VmlShapeModel {
  let mut model = vml_shape_from_typed(shape, VmlShapeKind::Line);
  model.coordinate_origin.clone_from(&shape.coordinate_origin);
  model.from.clone_from(&shape.from);
  model.to.clone_from(&shape.to);
  model
}

pub(crate) fn vml_image_file_model(shape: &vml::ImageFile) -> VmlShapeModel {
  vml_shape_from_typed(shape, VmlShapeKind::Image)
}

pub(crate) fn vml_oval_model(shape: &vml::Oval) -> VmlShapeModel {
  let mut model = vml_shape_from_typed(shape, VmlShapeKind::Oval);
  model.coordinate_origin.clone_from(&shape.coordinate_origin);
  model
}

pub(crate) fn vml_rectangle_model(shape: &vml::Rectangle) -> VmlShapeModel {
  vml_shape_from_typed(shape, VmlShapeKind::Rectangle)
}

pub(crate) fn vml_polyline_model(shape: &vml::PolyLine) -> VmlShapeModel {
  let mut model = vml_shape_from_typed(shape, VmlShapeKind::Polyline);
  model.coordinate_origin.clone_from(&shape.coordinate_origin);
  model.points.clone_from(&shape.points);
  model
}

pub(crate) fn vml_round_rectangle_model(shape: &vml::RoundRectangle) -> VmlShapeModel {
  let mut model = vml_shape_from_typed(shape, VmlShapeKind::RoundRectangle);
  model.arc_size.clone_from(&shape.arc_size);
  model
}

fn collect_typed_vml_fill(model: &mut VmlShapeModel, fill: &vml::Fill) {
  if let Some(on) = fill.on {
    model.filled = on.as_bool();
    model.filled_authored = Some(model.filled);
  }
  if fill.color.is_some() {
    model.fill_color.clone_from(&fill.color);
  }
  if fill.color2.is_some() {
    model.fill_color2.clone_from(&fill.color2);
  }
  if fill.opacity.is_some() {
    model.fill_opacity.clone_from(&fill.opacity);
  }
  if fill.opacity2.is_some() {
    model.fill_opacity2.clone_from(&fill.opacity2);
  }
  if fill.colors.is_some() {
    model.fill_colors.clone_from(&fill.colors);
  }
  if fill.r#type.is_some() {
    model.fill_type = fill.r#type;
  }
  if let Some(angle) = fill.angle.as_ref() {
    model.fill_angle = Some(angle.to_string());
  }
  if fill.focus.is_some() {
    model.fill_focus.clone_from(&fill.focus);
  }
  if fill.focus_position.is_some() {
    model.fill_focus_position.clone_from(&fill.focus_position);
  }
  if fill.focus_size.is_some() {
    model.fill_focus_size.clone_from(&fill.focus_size);
  }
  if fill.method.is_some() {
    model.fill_method = fill.method;
  }
  if let Some(rotate) = fill.rotate {
    model.fill_rotate_with_shape = Some(rotate.as_bool());
  }
  if fill.relationship_id.is_some() || fill.id.is_some() {
    model.fill_image_relationship_id = fill.relationship_id.clone().or_else(|| fill.id.clone());
  }
  if fill.size.is_some() {
    model.fill_image_size.clone_from(&fill.size);
  }
  if fill.origin.is_some() {
    model.fill_image_origin.clone_from(&fill.origin);
  }
  if fill.position.is_some() {
    model.fill_image_position.clone_from(&fill.position);
  }
  if fill.aspect.is_some() {
    model.fill_image_aspect = fill.aspect;
  }
}

fn collect_typed_vml_stroke(model: &mut VmlShapeModel, stroke: &vml::Stroke) {
  if let Some(on) = stroke.on {
    model.stroked = on.as_bool();
    model.stroked_authored = Some(model.stroked);
  }
  if stroke.color.is_some() {
    model.stroke_color.clone_from(&stroke.color);
  }
  if stroke.weight.is_some() {
    model.stroke_weight.clone_from(&stroke.weight);
  }
  if stroke.opacity.is_some() {
    model.stroke_opacity.clone_from(&stroke.opacity);
  }
  if stroke.dash_style.is_some() {
    model.stroke_dash_style.clone_from(&stroke.dash_style);
  }
  if stroke.line_style.is_some() {
    model.stroke_line_style = stroke.line_style;
  }
  if stroke.join_style.is_some() {
    model.stroke_join_style = stroke.join_style;
  }
  if stroke.end_cap.is_some() {
    model.stroke_end_cap = stroke.end_cap;
  }
  if stroke.start_arrow.is_some() {
    model.stroke_start_arrow = stroke.start_arrow;
  }
  if stroke.start_arrow_width.is_some() {
    model.stroke_start_arrow_width = stroke.start_arrow_width;
  }
  if stroke.start_arrow_length.is_some() {
    model.stroke_start_arrow_length = stroke.start_arrow_length;
  }
  if stroke.end_arrow.is_some() {
    model.stroke_end_arrow = stroke.end_arrow;
  }
  if stroke.end_arrow_width.is_some() {
    model.stroke_end_arrow_width = stroke.end_arrow_width;
  }
  if stroke.end_arrow_length.is_some() {
    model.stroke_end_arrow_length = stroke.end_arrow_length;
  }
}

fn vml_group_shape_models(group: &vml::Group) -> Vec<VmlShapeModel> {
  vml_group_shape_models_with_style(group, group.style.as_deref(), true)
}

fn vml_group_shape_models_with_style(
  group: &vml::Group,
  group_style: Option<&str>,
  root_anchor: bool,
) -> Vec<VmlShapeModel> {
  let mut shapes = Vec::new();
  let mut group_model = VmlShapeModel {
    kind: VmlShapeKind::Group,
    style: group_style.map(ToOwned::to_owned),
    hidden: group.user_hidden.is_some_and(|value| value.as_bool()),
    ..VmlShapeModel::default()
  };

  for child in &group.group_choice {
    match child {
      vml::GroupChoice::Group(child_group) => {
        let child_style = crate::docx::vml_group_child_style(
          group,
          group_style,
          child_group.style.as_deref(),
          root_anchor,
          true,
        );
        shapes.extend(vml_group_shape_models_with_style(
          child_group,
          child_style.as_deref().or(child_group.style.as_deref()),
          false,
        ));
      }
      vml::GroupChoice::Shape(shape) => {
        let mut model = vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::Shape,
          root_anchor,
        );
        model.shape_type_reference.clone_from(&shape.r#type);
        model.path = shape.edge_path.clone().or(model.path);
        shapes.push(model);
      }
      vml::GroupChoice::Arc(shape) => {
        let mut model = vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::Arc,
          root_anchor,
        );
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        model.start_angle = shape.start_angle.as_ref().map(ToString::to_string);
        model.end_angle = shape.end_angle.as_ref().map(ToString::to_string);
        shapes.push(model);
      }
      vml::GroupChoice::Curve(shape) => {
        let mut model = vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::Curve,
          root_anchor,
        );
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        model.from.clone_from(&shape.from);
        model.to.clone_from(&shape.to);
        model.control1.clone_from(&shape.control1);
        model.control2.clone_from(&shape.control2);
        shapes.push(model);
      }
      vml::GroupChoice::ImageFile(shape) => {
        shapes.push(vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::Image,
          root_anchor,
        ));
      }
      vml::GroupChoice::Line(shape) => {
        let mut model = vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::Line,
          root_anchor,
        );
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        model.from.clone_from(&shape.from);
        model.to.clone_from(&shape.to);
        shapes.push(model);
      }
      vml::GroupChoice::Oval(shape) => {
        let mut model = vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::Oval,
          root_anchor,
        );
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        shapes.push(model);
      }
      vml::GroupChoice::PolyLine(shape) => {
        let mut model = vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::Polyline,
          root_anchor,
        );
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        model.points.clone_from(&shape.points);
        shapes.push(model);
      }
      vml::GroupChoice::Rectangle(shape) => {
        let mut model = vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::Rectangle,
          root_anchor,
        );
        model.coordinate_origin.clone_from(&shape.coordinate_origin);
        shapes.push(model);
      }
      vml::GroupChoice::RoundRectangle(shape) => {
        let mut model = vml_group_child_model(
          group,
          group_style,
          shape.as_ref(),
          VmlShapeKind::RoundRectangle,
          root_anchor,
        );
        model.arc_size.clone_from(&shape.arc_size);
        shapes.push(model);
      }
      vml::GroupChoice::Shapetype(shape_type) => {
        shapes.push(vml_shape_type_model(shape_type.as_ref().clone()));
      }
      vml::GroupChoice::ClientData(client_data) => {
        collect_typed_vml_client_data(&mut group_model, client_data);
      }
      _ => {}
    }
  }

  if group_model.object_type.is_some() || group_model.anchor.is_some() {
    shapes.insert(0, group_model);
  }
  shapes
}

fn vml_group_child_model(
  group: &vml::Group,
  group_style: Option<&str>,
  shape: &impl VmlShapeElement,
  kind: VmlShapeKind,
  root_anchor: bool,
) -> VmlShapeModel {
  let mut model = vml_shape_from_typed(shape, kind);
  if let Some(style) = crate::docx::vml_group_child_style(
    group,
    group_style,
    model.style.as_deref(),
    root_anchor,
    false,
  ) {
    model.style = Some(style);
  }
  model
}

fn collect_typed_vml_textbox(model: &mut VmlShapeModel, text_box: &vml::TextBox) {
  let Some(vml::TextBoxChoice::XmlAny(xml)) = text_box.text_box_choice.as_ref() else {
    return;
  };
  model.text.push_str(&xml_text_content(xml));
  collect_first_vml_text_style(&mut model.text_style, xml);
}

fn collect_first_vml_text_style(style: &mut VmlTextStyle, xml: &[u8]) {
  let mut reader = quick_xml::Reader::from_reader(xml);
  reader.config_mut().trim_text(false);
  let mut found_font = false;
  let mut in_first_font = false;
  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        let local_name = event.local_name();
        match local_name.as_ref() {
          b"font" if !found_font => {
            found_font = true;
            in_first_font = true;
            for attr in event.attributes().flatten() {
              let value = String::from_utf8_lossy(attr.value.as_ref()).into_owned();
              match xml_local_name(attr.key.as_ref()) {
                b"face" => style.font_family = Some(value),
                b"size" => style.font_size_twips = value.parse().ok(),
                b"color" => style.color = Some(value),
                _ => {}
              }
            }
          }
          b"b" if !found_font || in_first_font => style.bold = true,
          b"i" if !found_font || in_first_font => style.italic = true,
          b"u" if !found_font || in_first_font => style.underline = true,
          b"s" if !found_font || in_first_font => style.strikethrough = true,
          _ => {}
        }
      }
      Ok(Event::Empty(event)) if !found_font && event.local_name().as_ref() == b"font" => {
        found_font = true;
        for attr in event.attributes().flatten() {
          let value = String::from_utf8_lossy(attr.value.as_ref()).into_owned();
          match xml_local_name(attr.key.as_ref()) {
            b"face" => style.font_family = Some(value),
            b"size" => style.font_size_twips = value.parse().ok(),
            b"color" => style.color = Some(value),
            _ => {}
          }
        }
      }
      Ok(Event::End(event)) if event.local_name().as_ref() == b"font" && in_first_font => {
        in_first_font = false;
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }
}

fn xml_text_content(xml: &[u8]) -> String {
  let mut reader = quick_xml::Reader::from_reader(xml);
  reader.config_mut().trim_text(false);
  let mut text = String::new();
  loop {
    match reader.read_event() {
      Ok(Event::Text(value)) => {
        if let Ok(value) = value.xml10_content() {
          text.push_str(&value);
        }
      }
      Ok(Event::CData(value)) => {
        if let Ok(value) = value.xml10_content() {
          text.push_str(&value);
        }
      }
      Ok(Event::GeneralRef(value)) => {
        if let Ok(value) = value.decode() {
          let reference = format!("&{value};");
          if let Ok(value) = quick_xml::escape::unescape(&reference) {
            text.push_str(&value);
          }
        }
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }
  text
}

fn collect_typed_vml_image_data(model: &mut VmlShapeModel, image_data: &vml::ImageData) {
  model.image_relationship_id = image_data
    .relationship_id
    .clone()
    .or_else(|| image_data.rel_id.clone())
    .or_else(|| image_data.picture.clone())
    .or_else(|| image_data.rel_href.clone())
    .or_else(|| image_data.id.clone());
}

fn collect_typed_vml_client_data(model: &mut VmlShapeModel, client_data: &xvml::ClientData) {
  model.object_type = Some(vml_object_type_name(client_data.object_type).to_string());
  for child in &client_data.client_data_choice {
    match child {
      xvml::ClientDataChoice::Anchor(value) => model.anchor = parse_vml_client_anchor(value),
      xvml::ClientDataChoice::AutoSizePicture(value) => {
        model.auto_size_picture = typed_vml_bool(*value, true);
      }
      xvml::ClientDataChoice::PrintObject(value) => {
        model.print_object = typed_vml_bool(*value, true);
      }
      xvml::ClientDataChoice::Visible(value) => {
        model.visible = typed_vml_bool(*value, true);
      }
      xvml::ClientDataChoice::Checked(value) => {
        // ECMA-376 Part 4 §19.4.2.11: 0 is unchecked, 1 checked, and 2
        // mixed. Preserve the tri-state value instead of reducing it to bool.
        model.checked = Some(*value);
      }
      xvml::ClientDataChoice::Disable3D(value) => {
        // A blank NoThreeD element is the normal true spelling emitted by
        // Excel. Preserve it as control semantics instead of falling back to
        // the inherited VML shapetype appearance.
        model.disable_3d = typed_vml_bool(*value, true);
      }
      xvml::ClientDataChoice::HorizontalTextAlignment(value) => {
        model.text_horizontal_alignment = Some(value.to_string());
      }
      xvml::ClientDataChoice::VerticalTextAlignment(value) => {
        model.text_vertical_alignment = Some(value.to_string());
      }
      xvml::ClientDataChoice::CommentRowTarget(value) => {
        model.note_row = u32::try_from(*value).ok();
      }
      xvml::ClientDataChoice::CommentColumnTarget(value) => {
        model.note_column = u32::try_from(*value).ok();
      }
      _ => {}
    }
  }
}

fn collect_vml_image_resources(
  package: &SpreadsheetDocument,
  part: &VmlDrawingPart,
) -> HashMap<String, ImageResource> {
  part
    .related_parts_of_type::<_, ImagePart>(package)
    .filter_map(|related_part| {
      Some((
        related_part.relationship_id().to_string(),
        ImageResource {
          data: related_part.part().try_data_bytes(package).ok()?,
          content_type: related_part
            .part()
            .content_type(package)
            .map(str::to_string),
        },
      ))
    })
    .collect()
}

fn parse_vml_client_anchor(value: &str) -> Option<VmlClientAnchor> {
  let values = value
    .split(',')
    .map(|part| part.trim().parse::<i32>().ok())
    .collect::<Option<Vec<_>>>()?;
  if values.len() != 8 {
    return None;
  }
  Some(VmlClientAnchor {
    from_col: u32::try_from(values[0]).ok()?,
    from_col_offset_px: values[1],
    from_row: u32::try_from(values[2]).ok()?,
    from_row_offset_px: values[3],
    to_col: u32::try_from(values[4]).ok()?,
    to_col_offset_px: values[5],
    to_row: u32::try_from(values[6]).ok()?,
    to_row_offset_px: values[7],
  })
}

fn typed_vml_bool(value: xvml::BooleanEntryWithBlankValues, default: bool) -> bool {
  match value {
    xvml::BooleanEntryWithBlankValues::True | xvml::BooleanEntryWithBlankValues::T => true,
    xvml::BooleanEntryWithBlankValues::False | xvml::BooleanEntryWithBlankValues::F => false,
    xvml::BooleanEntryWithBlankValues::Empty => default,
  }
}

fn normalize_vml_text(text: &str) -> String {
  text
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
    .trim()
    .to_string()
}

fn collect_xml_namespace_attrs(
  event: &BytesStart<'_>,
  namespace_attrs: &mut Vec<(Vec<u8>, Vec<u8>)>,
) {
  for attr in event.attributes().flatten() {
    let key = attr.key.as_ref();
    if is_xml_namespace_attr(key)
      && !namespace_attrs
        .iter()
        .any(|(existing, _)| existing.as_slice() == key)
    {
      namespace_attrs.push((key.to_vec(), attr.value.as_ref().to_vec()));
    }
  }
}

fn push_missing_xml_namespace_attrs(
  event: &mut BytesStart<'static>,
  namespace_attrs: &[(Vec<u8>, Vec<u8>)],
) {
  for (key, value) in namespace_attrs {
    if !bytes_start_has_attr(event, key) {
      event.push_attribute((key.as_slice(), value.as_slice()));
    }
  }
}

fn bytes_start_has_attr(event: &BytesStart<'_>, key: &[u8]) -> bool {
  event
    .attributes()
    .flatten()
    .any(|attr| attr.key.as_ref() == key)
}

fn is_xml_namespace_attr(key: &[u8]) -> bool {
  key == b"xmlns" || key.starts_with(b"xmlns:")
}

fn xml_local_name(name: &[u8]) -> &[u8] {
  name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
}

fn vml_object_type_name(value: xvml::ObjectValues) -> &'static str {
  match value {
    xvml::ObjectValues::Button => "Button",
    xvml::ObjectValues::Checkbox => "Checkbox",
    xvml::ObjectValues::Dialog => "Dialog",
    xvml::ObjectValues::Drop => "Drop",
    xvml::ObjectValues::Edit => "Edit",
    xvml::ObjectValues::GroupBox => "GBox",
    xvml::ObjectValues::Label => "Label",
    xvml::ObjectValues::AuditingLine => "LineA",
    xvml::ObjectValues::List => "List",
    xvml::ObjectValues::Movie => "Movie",
    xvml::ObjectValues::Note => "Note",
    xvml::ObjectValues::Picture => "Pict",
    xvml::ObjectValues::Radio => "Radio",
    xvml::ObjectValues::AuditingRectangle => "RectA",
    xvml::ObjectValues::Scroll => "Scroll",
    xvml::ObjectValues::Spin => "Spin",
    xvml::ObjectValues::Shape => "Shape",
    xvml::ObjectValues::Group => "Group",
    xvml::ObjectValues::Rectangle => "Rect",
  }
}

impl ControlPersistenceResourceCatalog {
  fn from_part(package: &SpreadsheetDocument, part: &EmbeddedControlPersistencePart) -> Self {
    Self {
      binary_data_parts: part
        .embedded_control_persistence_binary_data_parts(package)
        .map(|part| BinaryResourceCatalog::from_part(&part))
        .collect(),
    }
  }
}

impl ControlPropertiesResourceCatalog {
  fn from_part(package: &SpreadsheetDocument, part: &ControlPropertiesPart) -> Result<Self> {
    let properties = part.root_element(package)?;
    Ok(Self::from_properties(properties))
  }

  fn from_properties(properties: &x14::FormControlProperties) -> Self {
    Self {
      has_object_type: properties.object_type.is_some(),
      has_checked: properties.checked.is_some(),
      boolean_flags: bool_attr_count([
        properties.colored,
        properties.first_button,
        properties.horizontal,
        properties.just_last_x,
        properties.lock_text,
        properties.no_three_d,
        properties.no_three_d2,
        properties.multiple_lines,
        properties.vertical_bar,
        properties.password_edit,
      ]),
      numeric_fields: usize::from(properties.drop_lines.is_some())
        + usize::from(properties.scroll_bar_width.is_some())
        + usize::from(properties.incremental.is_some())
        + usize::from(properties.max.is_some())
        + usize::from(properties.min.is_some())
        + usize::from(properties.page.is_some())
        + usize::from(properties.selected.is_some())
        + usize::from(properties.val.is_some())
        + usize::from(properties.minimum_width.is_some()),
      formula_fields: usize::from(properties.fmla_group.is_some())
        + usize::from(properties.fmla_link.is_some())
        + usize::from(properties.fmla_range.is_some())
        + usize::from(properties.fmla_textbox.is_some()),
      alignment_fields: usize::from(properties.drop_style.is_some())
        + usize::from(properties.selection_type.is_some())
        + usize::from(properties.text_horizontal_align.is_some())
        + usize::from(properties.text_vertical_align.is_some())
        + usize::from(properties.edit_val.is_some()),
      list_items: properties
        .list_items
        .as_ref()
        .map_or(0, |items| items.list_item.len()),
      has_extension_list: properties.extension_list.is_some(),
      text_len: properties
        .fmla_group
        .as_ref()
        .map_or(0, |value| value.len())
        + properties.fmla_link.as_ref().map_or(0, |value| value.len())
        + properties
          .fmla_range
          .as_ref()
          .map_or(0, |value| value.len())
        + properties
          .fmla_textbox
          .as_ref()
          .map_or(0, |value| value.len())
        + properties
          .multiple_selection
          .as_ref()
          .map_or(0, |value| value.len()),
    }
  }
}

impl BinaryResourceCatalog {
  fn from_part(_part: &impl SdkPart) -> Self {
    Self
  }
}

fn bool_attr_count<const N: usize>(
  values: [Option<ooxmlsdk::simple_type::BooleanValue>; N],
) -> usize {
  values
    .into_iter()
    .filter(|value| value.is_some_and(|value| value.as_bool()))
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vml_drawing_dispatches_known_shape_types_to_static_models() {
    vml::RoundRectangle::from_bytes(
      br#"<v:roundrect xmlns:v="urn:schemas-microsoft-com:vml" xmlns:x="urn:schemas-microsoft-com:office:excel"><v:textbox><div>Round</div></v:textbox><x:ClientData ObjectType="Button"/></v:roundrect>"#,
    )
    .expect("typed round rectangle");
    vml::Line::from_bytes(
      br#"<v:line xmlns:v="urn:schemas-microsoft-com:vml" xmlns:x="urn:schemas-microsoft-com:office:excel" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" from="0,0" to="12,12"><v:imagedata r:id="rId7"/><x:ClientData ObjectType="Pict"><x:Visible/></x:ClientData></v:line>"#,
    )
    .expect("typed line");

    let xml = br#"<xml xmlns:v="urn:schemas-microsoft-com:vml"
        xmlns:o="urn:schemas-microsoft-com:office:office"
        xmlns:x="urn:schemas-microsoft-com:office:excel"
        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
      <v:roundrect style="position:absolute;width:72pt;height:24pt">
        <v:textbox><div><font>Round &amp; ready</font></div></v:textbox>
        <x:ClientData ObjectType="Button">
          <x:Anchor>1, 2, 3, 4, 5, 6, 7, 8</x:Anchor>
          <x:PrintObject>False</x:PrintObject>
        </x:ClientData>
      </v:roundrect>
      <v:line style="position:absolute;width:12pt;height:12pt" from="0,0" to="12,12">
        <v:imagedata r:id="rId7"/>
        <x:ClientData ObjectType="Pict"><x:AutoPict/><x:Visible/></x:ClientData>
      </v:line>
      <v:group>
        <v:shape id="ToggleButton2" o:spid="_x0000_s1028"
          style="position:absolute;width:36pt;height:18pt">
          <v:textbox><div><b>Nested</b></div></v:textbox>
        </v:shape>
      </v:group>
    </xml>"#;

    let shapes = vml_shapes(xml);
    assert_eq!(shapes.len(), 3);

    assert_eq!(shapes[0].text, "Round & ready");
    assert_eq!(shapes[0].object_type.as_deref(), Some("Button"));
    assert_eq!(
      shapes[0].anchor,
      Some(VmlClientAnchor {
        from_col: 1,
        from_col_offset_px: 2,
        from_row: 3,
        from_row_offset_px: 4,
        to_col: 5,
        to_col_offset_px: 6,
        to_row: 7,
        to_row_offset_px: 8,
      })
    );
    assert!(!shapes[0].print_object);

    assert_eq!(shapes[1].image_relationship_id.as_deref(), Some("rId7"));
    assert_eq!(shapes[1].object_type.as_deref(), Some("Pict"));
    assert!(shapes[1].auto_size_picture);
    assert!(shapes[1].visible);
    assert_eq!(shapes[2].text, "Nested");
    assert_eq!(shapes[2].id.as_deref(), Some("ToggleButton2"));
    assert_eq!(shapes[2].shape_id.as_deref(), Some("_x0000_s1028"));
  }

  #[test]
  fn checkbox_preserves_first_vml_font_alignment_tristate_and_flat_style() {
    let shapes = vml_shapes(
      br##"<xml xmlns:v="urn:schemas-microsoft-com:vml"
          xmlns:x="urn:schemas-microsoft-com:office:excel">
        <v:shape>
          <v:textbox><div><font face="Segoe UI" size="160" color="#123456"><b>Caption</b></font></div></v:textbox>
          <x:ClientData ObjectType="Checkbox">
            <x:Anchor>1, 2, 3, 4, 5, 6, 7, 8</x:Anchor>
            <x:TextHAlign>Right</x:TextHAlign>
            <x:TextVAlign>Center</x:TextVAlign>
            <x:Checked>2</x:Checked>
            <x:NoThreeD/>
          </x:ClientData>
        </v:shape>
      </xml>"##,
    );

    assert_eq!(shapes.len(), 1);
    let shape = &shapes[0];
    assert_eq!(shape.text, "Caption");
    assert_eq!(shape.text_style.font_family.as_deref(), Some("Segoe UI"));
    assert_eq!(shape.text_style.font_size_twips, Some(160));
    assert_eq!(shape.text_style.color.as_deref(), Some("#123456"));
    assert!(shape.text_style.bold);
    assert_eq!(shape.text_horizontal_alignment.as_deref(), Some("Right"));
    assert_eq!(shape.text_vertical_alignment.as_deref(), Some("Center"));
    assert_eq!(shape.checked, Some(2));
    assert!(shape.disable_3d);
  }

  #[test]
  fn powerpoint_vml_conditionals_preserve_control_preview_images() {
    let shapes = vml_shapes(
      br#"<xml xmlns:v="urn:schemas-microsoft-com:vml"
          xmlns:o="urn:schemas-microsoft-com:office:office">
        <v:shape id="OptionButton1" o:spid="_x0000_s1027"
          style="position:absolute;left:114pt;top:98pt;width:148pt;height:57pt">
          <![if gte mso 9]>
          <v:imagedata o:relid="rId1"/>
          <![endif]>
        </v:shape>
      </xml>"#,
    );

    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0].id.as_deref(), Some("OptionButton1"));
    assert_eq!(shapes[0].shape_id.as_deref(), Some("_x0000_s1027"));
    assert_eq!(shapes[0].image_relationship_id.as_deref(), Some("rId1"));
  }

  #[test]
  fn xml_text_content_is_limited_to_vml_textbox_wildcard_payload() {
    assert_eq!(
      normalize_vml_text(&xml_text_content(
        br#"<div>one<![CDATA[ two ]]><span>three &amp; four</span></div>"#
      )),
      "one two three & four"
    );
  }

  #[test]
  fn vml_shape_type_paint_is_inherited_then_overridden_by_child_elements() {
    let xml = br##"<xml xmlns:v="urn:schemas-microsoft-com:vml">
      <v:shapetype id="base" coordsize="1000,500" path="m0,0l1000,500e"
        fillcolor="#112233" strokecolor="#445566" stroked="f"/>
      <v:shape type="#base" style="margin-left:1pt;margin-top:2pt;width:10pt;height:5pt">
        <v:fill on="t" color="#AABBCC"/>
        <v:stroke on="t" color="#DDEEFF" weight="2pt"/>
      </v:shape>
    </xml>"##;

    let shapes = vml_shapes(xml);
    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0].coordinate_size.as_deref(), Some("1000,500"));
    assert_eq!(shapes[0].path.as_deref(), Some("m0,0l1000,500e"));
    assert_eq!(shapes[0].fill_color.as_deref(), Some("#AABBCC"));
    assert_eq!(shapes[0].stroke_color.as_deref(), Some("#DDEEFF"));
    assert_eq!(shapes[0].stroke_weight.as_deref(), Some("2pt"));
    assert!(shapes[0].filled);
    assert!(shapes[0].stroked);
  }

  #[test]
  fn vml_group_coordinate_space_is_lowered_into_child_style() {
    let xml = br#"<xml xmlns:v="urn:schemas-microsoft-com:vml">
      <v:group style="width:100pt;height:50pt" coordsize="1000,500">
        <v:curve style="left:100;top:50;width:200;height:100"
          from="0,0" control1="0,100" control2="200,0" to="200,100"/>
      </v:group>
    </xml>"#;

    let shapes = vml_shapes(xml);
    assert_eq!(shapes.len(), 1);
    let style = shapes[0].style.as_deref().unwrap();
    assert!(style.contains("left:10pt"));
    assert!(style.contains("top:5pt"));
    assert!(style.contains("width:20pt"));
    assert!(style.contains("height:10pt"));
    assert_eq!(shapes[0].control1.as_deref(), Some("0,100"));
    assert_eq!(shapes[0].control2.as_deref(), Some("200,0"));
  }

  #[test]
  fn vml_group_local_shape_type_is_available_to_nested_shape() {
    let xml = br##"<xml xmlns:v="urn:schemas-microsoft-com:vml">
      <v:group style="width:100pt;height:50pt" coordsize="1000,500">
        <v:shapetype id="local" coordsize="200,100" path="m0,0l200,100e"
          fillcolor="#123456"/>
        <v:shape type="#local" style="left:0;top:0;width:200;height:100"/>
      </v:group>
    </xml>"##;

    let shapes = vml_shapes(xml);
    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0].path.as_deref(), Some("m0,0l200,100e"));
    assert_eq!(shapes[0].coordinate_size.as_deref(), Some("200,100"));
    assert_eq!(shapes[0].fill_color.as_deref(), Some("#123456"));
  }
}
