use std::collections::{HashMap, HashSet};

use kurbo::Affine;
use ooxmlsdk::schemas::{
  schemas_microsoft_com_office_drawing_2008_diagram as dsp,
  schemas_openxmlformats_org_drawingml_2006_diagram as dgm,
  schemas_openxmlformats_org_drawingml_2006_main as a,
};
use ooxmlsdk::simple_type::Coordinate32Value;

use crate::common::{
  DrawingPath, drawingml_custom_geometry, drawingml_geometry, drawingml_preset_geometry,
};
use crate::model::RgbColor;
use crate::render::math::text_math_text;

// LibreOffice DiagramLayoutAtom::layoutShape() synthesizes this DrawingML
// hanging indent when stBulletLvl turns a SmartArt tx paragraph into a bullet.
const SMARTART_TX_BULLET_INDENT_EMU: i32 = 285_750;
// Microsoft's SmartArt layout documentation defines the default secondary
// font size used by bulleted lines as 78 percent of the primary font size.
const SMARTART_DEFAULT_SECONDARY_FONT_SCALE: f32 = 0.78;
// ECMA-376 preset shape adjustments use 100000 as the full guide scale,
// while diagram layout shape adjustments are stored as normalized doubles.
const DRAWINGML_ADJUST_FULL_SCALE: f64 = 100_000.0;

#[derive(Clone, Debug)]
pub struct DiagramShape {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
  pub text_body: DiagramTextBody,
  pub preset_geometry: Option<Box<a::PresetGeometry>>,
  pub shape_properties: Option<Box<dgm::ShapeProperties>>,
  pub style: Option<Box<dgm::Style>>,
  pub line_fill: Option<RgbColor>,
  pub text_fill: Option<RgbColor>,
  pub shape_rotation_deg: f32,
  pub text_rotation_deg: f32,
  pub draw_geometry: bool,
  pub is_connector: bool,
  pub connector_angle_deg: f32,
  pub is_blip_placeholder: bool,
  pub fill: RgbColor,
  pub text_order: usize,
  pub font_size_pt: Option<f32>,
  pub minimum_font_size_pt: Option<f32>,
  pub font_sync_group: Option<String>,
}

impl DiagramShape {
  pub(crate) fn drawing_paths(&self) -> Option<Vec<DrawingPath>> {
    let mut paths = match self
      .shape_properties
      .as_deref()
      .and_then(|properties| properties.shape_properties_choice1.as_ref())
    {
      Some(dgm::ShapePropertiesChoice::PresetGeometry(preset)) => {
        drawingml_preset_geometry::paths(Some(preset), self.x, self.y, self.width, self.height)?
      }
      Some(dgm::ShapePropertiesChoice::CustomGeometry(geometry)) => {
        drawingml_custom_geometry::paths(geometry, self.x, self.y, self.width, self.height)?
      }
      None => drawingml_preset_geometry::paths(
        Some(self.preset_geometry.as_deref()?),
        self.x,
        self.y,
        self.width,
        self.height,
      )?,
    };
    let transform = Affine::rotate_about(
      f64::from(self.shape_rotation_deg.to_radians()),
      (
        f64::from(self.x + self.width / 2.0),
        f64::from(self.y + self.height / 2.0),
      ),
    );
    for path in &mut paths {
      path.commands =
        drawingml_geometry::transform_commands(std::mem::take(&mut path.commands), transform);
    }
    Some(paths)
  }
}

pub(crate) fn drawing_shape_paths(
  properties: &dsp::ShapeProperties,
  bounds: DiagramBounds,
) -> Option<Vec<DrawingPath>> {
  let mut paths = match properties.shape_properties_choice1.as_ref()? {
    dsp::ShapePropertiesChoice::PresetGeometry(preset) => drawingml_preset_geometry::paths(
      Some(preset),
      bounds.x,
      bounds.y,
      bounds.width,
      bounds.height,
    )?,
    dsp::ShapePropertiesChoice::CustomGeometry(geometry) => {
      drawingml_custom_geometry::paths(geometry, bounds.x, bounds.y, bounds.width, bounds.height)?
    }
  };
  let rotation = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .unwrap_or_default() as f64
    / 60_000.0;
  let transform = Affine::rotate_about(
    rotation.to_radians(),
    (
      f64::from(bounds.x + bounds.width / 2.0),
      f64::from(bounds.y + bounds.height / 2.0),
    ),
  );
  for path in &mut paths {
    path.commands =
      drawingml_geometry::transform_commands(std::mem::take(&mut path.commands), transform);
  }
  Some(paths)
}

#[derive(Clone, Debug, Default)]
pub struct DiagramTextBody {
  pub body_properties: Option<Box<a::BodyProperties>>,
  pub list_style: Option<Box<a::ListStyle>>,
  pub auto_fit: bool,
  pub paragraphs: Vec<DiagramTextParagraph>,
  custom_text: bool,
}

impl DiagramTextBody {
  pub fn is_empty(&self) -> bool {
    self.paragraphs.iter().all(DiagramTextParagraph::is_empty)
  }

  fn source_order(&self) -> Option<usize> {
    self
      .paragraphs
      .iter()
      .filter_map(|paragraph| paragraph.source_order)
      .min()
  }

  fn append_point(&mut self, point: &dgm::Point, depth: i32) {
    let Some(text_body) = point.text_body.as_deref() else {
      return;
    };
    if text_body
      .paragraph
      .first()
      .is_none_or(|paragraph| paragraph.paragraph_choice.is_empty())
    {
      return;
    }
    if self.body_properties.is_none() {
      self.body_properties = Some(text_body.body_properties.clone());
      self.list_style = text_body.list_style.clone();
      self.custom_text = point
        .property_set
        .as_deref()
        .and_then(|properties| properties.text_changed)
        .is_some_and(|value| value.as_bool());
    }
    let first_paragraph_properties = text_body
      .paragraph
      .first()
      .and_then(|paragraph| paragraph.paragraph_properties.clone());
    for paragraph in &text_body.paragraph {
      let mut diagram_paragraph =
        DiagramTextParagraph::from_dml_runs(paragraph, first_paragraph_properties.clone());
      if depth != -1 {
        diagram_paragraph.apply_binding_depth(depth);
      }
      self.paragraphs.push(diagram_paragraph);
    }
  }

  fn apply_font_sizes(&mut self, primary_font_size_pt: f32, secondary_font_size_pt: f32) {
    let primary_font_size = (primary_font_size_pt * 100.0).round() as i32;
    let secondary_font_size = (secondary_font_size_pt * 100.0).round() as i32;
    for paragraph in &mut self.paragraphs {
      let uses_secondary_font_size = paragraph
        .paragraph_properties
        .as_deref()
        .and_then(|properties| properties.paragraph_properties_choice4.as_ref())
        .is_some_and(|bullet| !matches!(bullet, a::ParagraphPropertiesChoice4::NoBullet));
      let font_size = if uses_secondary_font_size {
        secondary_font_size
      } else {
        primary_font_size
      };
      for run in &mut paragraph.runs {
        run.apply_layout_font_size(font_size);
      }
    }
  }

  fn enable_auto_fit_if_default_text(&mut self, has_direct_font_size: bool) {
    if !self.custom_text && !has_direct_font_size {
      self.auto_fit = true;
    }
  }

  fn has_direct_font_size(&self) -> bool {
    self
      .paragraphs
      .iter()
      .flat_map(|paragraph| &paragraph.runs)
      .any(|run| {
        run
          .run_properties
          .as_ref()
          .and_then(|properties| properties.font_size)
          .is_some()
      })
  }

  fn apply_text_margins(
    &mut self,
    shape_width_pt: f32,
    shape_height_pt: f32,
    primary_font_size_pt: Option<f32>,
    constraints: &[DiagramConstraint],
  ) {
    for constraint in constraints {
      if !constraint.for_name.is_empty()
        || !matches!(
          constraint.target,
          dgm::ConstraintValues::LeftMargin
            | dgm::ConstraintValues::RightMargin
            | dgm::ConstraintValues::TopMargin
            | dgm::ConstraintValues::BottomMargin
        )
      {
        continue;
      }
      let referenced_value = match constraint.reference {
        dgm::ConstraintValues::Width => Some(shape_width_pt),
        dgm::ConstraintValues::Height => Some(shape_height_pt),
        dgm::ConstraintValues::PrimaryFontSize | dgm::ConstraintValues::SecondaryFontSize => {
          primary_font_size_pt
        }
        dgm::ConstraintValues::None if constraint.value != 0.0 => {
          Some(constraint_value_points(constraint))
        }
        _ => None,
      };
      let Some(margin_pt) = referenced_value.map(|value| value * constraint.factor) else {
        continue;
      };
      let margin = Coordinate32Value::Emu(points_to_emu(margin_pt));
      let mut body_properties = self.body_properties.clone().unwrap_or_default();
      match constraint.target {
        dgm::ConstraintValues::LeftMargin => body_properties.left_inset = Some(margin),
        dgm::ConstraintValues::RightMargin => body_properties.right_inset = Some(margin),
        dgm::ConstraintValues::TopMargin => body_properties.top_inset = Some(margin),
        dgm::ConstraintValues::BottomMargin => body_properties.bottom_inset = Some(margin),
        _ => {}
      }
      self.body_properties = Some(body_properties);
    }
  }

  fn set_vertical_anchor(&mut self, anchor: a::TextAnchoringTypeValues) {
    let mut body_properties = self.body_properties.clone().unwrap_or_default();
    body_properties.anchor = Some(anchor);
    self.body_properties = Some(body_properties);
  }

  fn apply_text_algorithm_paragraph_rules(
    &mut self,
    start_bullets_at_level: i32,
    alignment: Option<a::TextAlignmentTypeValues>,
  ) {
    let Some(base_level) = self
      .paragraphs
      .iter()
      .filter_map(|paragraph| paragraph.level)
      .min()
    else {
      return;
    };
    let start_bullets_at_level = (start_bullets_at_level - 1).max(0) as u8;
    let mut is_bullet_list = false;
    for paragraph in &mut self.paragraphs {
      let normalized_level = paragraph
        .level
        .unwrap_or(base_level)
        .saturating_sub(base_level);
      paragraph.level = Some(normalized_level);
      let mut properties = paragraph.paragraph_properties.clone().unwrap_or_default();
      properties.level = Some(i32::from(normalized_level));
      if normalized_level >= start_bullets_at_level {
        if properties.left_margin.is_none() {
          properties.left_margin = Some(
            SMARTART_TX_BULLET_INDENT_EMU
              * i32::from(normalized_level - start_bullets_at_level + 1),
          );
          paragraph.synthesized_bullet_left_margin = true;
        }
        if properties.indent.is_none() {
          properties.indent = Some(-SMARTART_TX_BULLET_INDENT_EMU);
          paragraph.synthesized_bullet_indent = true;
        }
        properties.paragraph_properties_choice4 = Some(
          a::ParagraphPropertiesChoice4::CharacterBullet(a::CharacterBullet {
            char: "\u{2022}".to_string(),
          }),
        );
        is_bullet_list = true;
      } else if properties.paragraph_properties_choice4.is_none() {
        properties.paragraph_properties_choice4 = Some(a::ParagraphPropertiesChoice4::NoBullet);
      }
      paragraph.paragraph_properties = Some(properties);
    }

    let alignment = alignment.or((!is_bullet_list).then_some(a::TextAlignmentTypeValues::Center));
    if let Some(alignment) = alignment {
      for paragraph in &mut self.paragraphs {
        let mut properties = paragraph.paragraph_properties.clone().unwrap_or_default();
        properties.alignment = Some(alignment);
        paragraph.paragraph_properties = Some(properties);
      }
    }
  }
}

#[derive(Clone, Debug, Default)]
pub struct DiagramTextParagraph {
  pub source_order: Option<usize>,
  pub level: Option<u8>,
  pub paragraph_properties: Option<Box<a::ParagraphProperties>>,
  pub end_paragraph_run_properties: Option<Box<a::EndParagraphRunProperties>>,
  pub runs: Vec<DiagramTextRun>,
  pub synthesized_bullet_left_margin: bool,
  pub synthesized_bullet_indent: bool,
}

impl DiagramTextParagraph {
  fn from_dml_runs(
    source: &a::Paragraph,
    paragraph_properties: Option<Box<a::ParagraphProperties>>,
  ) -> Self {
    let level = paragraph_properties
      .as_ref()
      .and_then(|properties| properties.level)
      .map(|level| level as u8);
    Self {
      source_order: None,
      level,
      paragraph_properties,
      end_paragraph_run_properties: source.end_paragraph_run_properties.clone(),
      runs: source
        .paragraph_choice
        .iter()
        .filter_map(DiagramTextRun::from_dml)
        .collect(),
      synthesized_bullet_left_margin: false,
      synthesized_bullet_indent: false,
    }
  }

  fn is_empty(&self) -> bool {
    self.runs.iter().all(|run| run.text.trim().is_empty())
  }

  fn apply_binding_depth(&mut self, depth: i32) {
    let clamped_depth = depth.clamp(0, 8);
    if self
      .paragraph_properties
      .as_ref()
      .and_then(|properties| properties.level)
      .is_none()
    {
      self.level = Some(clamped_depth as u8);
      let mut properties = self.paragraph_properties.clone().unwrap_or_default();
      properties.level = Some(clamped_depth);
      self.paragraph_properties = Some(properties);
    }
  }
}

#[derive(Clone, Debug)]
pub struct DiagramTextRun {
  pub text: String,
  pub kind: DiagramTextRunKind,
  pub field_type: Option<String>,
  pub run_properties: Option<Box<a::RunProperties>>,
  pub field_paragraph_properties: Option<Box<a::ParagraphProperties>>,
}

impl DiagramTextRun {
  fn from_dml(choice: &a::ParagraphChoice) -> Option<Self> {
    match choice {
      a::ParagraphChoice::Run(run) => Some(Self {
        text: run.text.clone(),
        kind: DiagramTextRunKind::Run,
        field_type: None,
        run_properties: run.run_properties.clone(),
        field_paragraph_properties: None,
      }),
      a::ParagraphChoice::Break(line_break) => Some(Self {
        text: "\n".to_string(),
        kind: DiagramTextRunKind::Break,
        field_type: None,
        run_properties: line_break.run_properties.clone(),
        field_paragraph_properties: None,
      }),
      a::ParagraphChoice::Field(field) => field.text.as_ref().map(|text| Self {
        text: text.clone(),
        kind: DiagramTextRunKind::Field,
        field_type: field.r#type.clone(),
        run_properties: field.run_properties.clone(),
        field_paragraph_properties: field.paragraph_properties.clone(),
      }),
      a::ParagraphChoice::TextMath(math) => Some(Self {
        text: text_math_text(math),
        kind: DiagramTextRunKind::Math,
        field_type: None,
        run_properties: None,
        field_paragraph_properties: None,
      }),
      a::ParagraphChoice::AlternateContent(_) => None,
    }
  }

  fn apply_layout_font_size(&mut self, font_size: i32) {
    if matches!(
      self.kind,
      DiagramTextRunKind::Run | DiagramTextRunKind::Field | DiagramTextRunKind::Math
    ) {
      let mut properties = self.run_properties.clone().unwrap_or_default();
      if properties.font_size.is_none() {
        properties.font_size = Some(font_size);
      }
      self.run_properties = Some(properties);
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DiagramTextRunKind {
  #[default]
  Run,
  Break,
  Field,
  Math,
}

#[derive(Clone, Debug, Default)]
pub struct DiagramStyleColors {
  pub fill_by_label: HashMap<String, Vec<RgbColor>>,
  pub line_by_label: HashMap<String, Vec<RgbColor>>,
  pub text_fill_by_label: HashMap<String, Vec<RgbColor>>,
}

#[derive(Clone, Debug, Default)]
pub struct DiagramStyles {
  pub style_by_label: HashMap<String, Box<dgm::Style>>,
}

#[derive(Clone, Copy, Debug)]
pub struct DiagramBounds {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

pub fn layout_shapes(
  data: &dgm::DataModelRoot,
  layout: Option<&dgm::LayoutDefinition>,
  styles: Option<&DiagramStyles>,
  colors: Option<&DiagramStyleColors>,
  bounds: DiagramBounds,
  accent_fill: RgbColor,
) -> Vec<DiagramShape> {
  // SmartArtDiagram::createShapeHierarchyFromModel() creates shapes from the
  // diagram data model, then applies layout atoms. This is the shared entry
  // point for that same model-to-shapes stage.
  if let Some(layout) = layout
    && let Some(mut tree) =
      build_diagram_shape_tree(data, layout, styles, colors, accent_fill, bounds)
  {
    layout_diagram_shape_tree(&mut tree);
    sort_diagram_shape_children_by_z_order(&mut tree);
    let mut shapes = Vec::new();
    flatten_diagram_shape_tree(&tree, bounds.x, bounds.y, &mut shapes);
    return shapes;
  }

  Vec::new()
}

#[derive(Clone, Debug)]
struct DiagramShapeNode {
  internal_name: String,
  text_body: DiagramTextBody,
  fill: RgbColor,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  algorithms: Vec<LayoutAlgorithm>,
  child_order: dgm::ChildOrderValues,
  has_geometry: bool,
  hidden_geometry: bool,
  is_connector: bool,
  shape_rotation_deg: f32,
  connector_angle_deg: f32,
  is_blip_placeholder: bool,
  z_order_offset: i32,
  shape_properties: Option<Box<dgm::ShapeProperties>>,
  preset_geometry: Option<Box<a::PresetGeometry>>,
  style: Option<Box<dgm::Style>>,
  line_fill: Option<RgbColor>,
  text_fill: Option<RgbColor>,
  text_rotation_deg: f32,
  aspect_ratio: f32,
  data_node_type: Option<dgm::ElementValues>,
  font_size_pt: Option<f32>,
  minimum_font_size_pt: Option<f32>,
  font_sync_group: Option<String>,
  text_order: usize,
  constraints: Vec<DiagramConstraint>,
  direct_constraints: Vec<DiagramConstraint>,
  rules: Vec<DiagramRule>,
  children: Vec<DiagramShapeNode>,
}

#[derive(Clone, Copy, Debug)]
struct PresentationDataBinding<'a> {
  point: &'a dgm::Point,
  depth: i32,
  source_order: usize,
}

#[derive(Clone, Debug)]
struct DiagramConstraint {
  for_name: String,
  ref_for_name: String,
  factor: f32,
  value: f32,
  target: dgm::ConstraintValues,
  reference: dgm::ConstraintValues,
  relationship: Option<dgm::ConstraintRelationshipValues>,
  operator: Option<dgm::BoolOperatorValues>,
  point_type: Option<dgm::ElementValues>,
}

#[derive(Clone, Debug)]
struct DiagramRule {
  for_name: String,
  target: dgm::ConstraintValues,
  point_type: Option<dgm::ElementValues>,
  value: f32,
}

#[derive(Clone, Copy, Debug)]
struct LayoutAlgorithm {
  kind: dgm::AlgorithmValues,
  linear_direction: LinearDirection,
  secondary_linear_direction: LinearDirection,
  grow_direction: GrowDirection,
  continue_direction: ContinueDirection,
  start_angle: f32,
  span_angle: f32,
  center_shape_mapping_first_node: bool,
  rotation_path_along_path: bool,
  aspect_ratio: Option<f32>,
  auto_text_rotation: Option<dgm::AutoTextRotationValues>,
  text_anchor_vertical: dgm::TextAnchorVerticalValues,
  start_bullets_at_level: i32,
  parent_text_left_to_right_alignment: Option<dgm::TextAlignmentValues>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LinearDirection {
  #[default]
  Left,
  Right,
  Top,
  Bottom,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum GrowDirection {
  #[default]
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ContinueDirection {
  #[default]
  SameDirection,
  ReverseDirection,
}

pub fn presentation_point_list_orders(data: &dgm::DataModelRoot) -> HashMap<String, usize> {
  let points = diagram_points(data);
  let data_orders: HashMap<&str, usize> = points
    .iter()
    .enumerate()
    .filter(|(_, point)| {
      point
        .r#type
        .is_none_or(|kind| matches!(kind, dgm::PointValues::Node | dgm::PointValues::Assistant))
    })
    .map(|(order, point)| (point.model_id.as_str(), order))
    .collect();
  let Some(connections) = data.connection_list.as_ref() else {
    return HashMap::new();
  };
  connections
    .connection
    .iter()
    .filter(|connection| connection.r#type == Some(dgm::ConnectionValues::PresentationOf))
    .filter_map(|connection| {
      data_orders
        .get(connection.source_id.as_str())
        .map(|order| (connection.destination_id.clone(), *order))
    })
    .collect()
}

fn diagram_points(data: &dgm::DataModelRoot) -> Vec<&dgm::Point> {
  data
    .point_list
    .xml_children
    .iter()
    .filter_map(|child| match child {
      dgm::PointListChoice::Point(point) => Some(point.as_ref()),
      dgm::PointListChoice::AlternateContent(_) => None,
    })
    .collect()
}

fn build_diagram_shape_tree(
  data: &dgm::DataModelRoot,
  layout: &dgm::LayoutDefinition,
  styles: Option<&DiagramStyles>,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
  bounds: DiagramBounds,
) -> Option<DiagramShapeNode> {
  let connections = data.connection_list.as_ref()?;
  let points = diagram_points(data);
  let metrics = layout_node_metrics(Some(layout));
  let point_by_id: HashMap<&str, &dgm::Point> = points
    .iter()
    .copied()
    .map(|point| (point.model_id.as_str(), point))
    .collect();
  let mut data_by_presentation: HashMap<&str, Vec<(u32, PresentationDataBinding<'_>)>> =
    HashMap::new();
  let point_orders: HashMap<&str, usize> = points
    .iter()
    .enumerate()
    .map(|(index, point)| (point.model_id.as_str(), index))
    .collect();
  let mut points_by_presentation_name: HashMap<&str, Vec<&dgm::Point>> = HashMap::new();
  for point in &points {
    if let Some(name) = presentation_name(point) {
      points_by_presentation_name
        .entry(name)
        .or_default()
        .push(point);
    }
  }
  for connection in &connections.connection {
    if connection.r#type == Some(dgm::ConnectionValues::PresentationOf)
      && let (Some(data_point), Some(presentation_point)) = (
        point_by_id.get(connection.source_id.as_str()),
        point_by_id.get(connection.destination_id.as_str()),
      )
    {
      data_by_presentation
        .entry(presentation_point.model_id.as_str())
        .or_default()
        .push((
          connection.destination_position,
          PresentationDataBinding {
            point: data_point,
            depth: presentation_source_depth(data, data_point.model_id.as_str()),
            source_order: point_orders
              .get(data_point.model_id.as_str())
              .copied()
              .unwrap_or_default(),
          },
        ));
    }
  }
  let data_by_presentation: HashMap<&str, Vec<PresentationDataBinding<'_>>> = data_by_presentation
    .into_iter()
    .map(|(presentation_id, mut data_points)| {
      data_points.sort_by_key(|(position, _)| *position);
      (
        presentation_id,
        data_points
          .into_iter()
          .map(|(_, data_point)| data_point)
          .collect(),
      )
    })
    .collect();
  let root_presentation = points.iter().find(|point| {
    point.r#type == Some(dgm::PointValues::Presentation)
      && associated_data_point(point, &point_by_id)
        .is_some_and(|point| point.r#type == Some(dgm::PointValues::Document))
  })?;
  let root_point = associated_data_point(root_presentation, &point_by_id)?;
  let mut for_each_by_name = HashMap::new();
  collect_for_each_refs_from_layout_node(&layout.layout_node, &mut for_each_by_name);
  let mut visitor = DiagramShapeCreationVisitor {
    point_by_id: &point_by_id,
    point_orders: &point_orders,
    points_by_presentation_name: &points_by_presentation_name,
    data_by_presentation: &data_by_presentation,
    for_each_by_name: &for_each_by_name,
    connections,
    metrics: &metrics,
    styles,
    colors,
    fallback_fill,
    current_point: root_point,
    current_index: 0,
    current_step: 1,
    current_count: 1,
    tree: Some(DiagramShapeNode {
      internal_name: String::new(),
      text_body: DiagramTextBody::default(),
      fill: fallback_fill,
      x: 0.0,
      y: 0.0,
      width: bounds.width,
      height: bounds.height,
      algorithms: Vec::new(),
      child_order: dgm::ChildOrderValues::Bottom,
      has_geometry: false,
      hidden_geometry: false,
      is_connector: false,
      shape_rotation_deg: 0.0,
      connector_angle_deg: 0.0,
      is_blip_placeholder: false,
      z_order_offset: 0,
      shape_properties: None,
      preset_geometry: None,
      style: None,
      line_fill: None,
      text_fill: None,
      text_rotation_deg: 0.0,
      aspect_ratio: 0.0,
      data_node_type: None,
      font_size_pt: None,
      minimum_font_size_pt: None,
      font_sync_group: None,
      text_order: usize::MAX,
      constraints: Vec::new(),
      direct_constraints: Vec::new(),
      rules: Vec::new(),
      children: Vec::new(),
    }),
    parent_path: Vec::new(),
    tree_root_mapped: false,
  };
  visitor.visit_layout_node(&layout.layout_node);
  visitor.tree
}

fn collect_for_each_refs_from_layout_node<'a>(
  node: &'a dgm::LayoutNode,
  refs: &mut HashMap<&'a str, &'a dgm::ForEach>,
) {
  for choice in &node.layout_node_choice {
    match choice {
      dgm::LayoutNodeChoice::ForEach(for_each) => collect_for_each_refs(for_each, refs),
      dgm::LayoutNodeChoice::LayoutNode(node) => collect_for_each_refs_from_layout_node(node, refs),
      dgm::LayoutNodeChoice::Choose(choose) => collect_for_each_refs_from_choose(choose, refs),
      _ => {}
    }
  }
}

fn collect_for_each_refs<'a>(
  for_each: &'a dgm::ForEach,
  refs: &mut HashMap<&'a str, &'a dgm::ForEach>,
) {
  if let Some(name) = for_each.name.as_deref() {
    refs.insert(name, for_each);
  }
  for choice in &for_each.for_each_choice {
    match choice {
      dgm::ForEachChoice::ForEach(child) => collect_for_each_refs(child, refs),
      dgm::ForEachChoice::LayoutNode(node) => collect_for_each_refs_from_layout_node(node, refs),
      dgm::ForEachChoice::Choose(choose) => collect_for_each_refs_from_choose(choose, refs),
      _ => {}
    }
  }
}

fn collect_for_each_refs_from_choose<'a>(
  choose: &'a dgm::Choose,
  refs: &mut HashMap<&'a str, &'a dgm::ForEach>,
) {
  for branch in &choose.diagram_choose_if {
    for choice in &branch.diagram_choose_if_choice {
      match choice {
        dgm::DiagramChooseIfChoice::ForEach(for_each) => collect_for_each_refs(for_each, refs),
        dgm::DiagramChooseIfChoice::LayoutNode(node) => {
          collect_for_each_refs_from_layout_node(node, refs)
        }
        dgm::DiagramChooseIfChoice::Choose(choose) => {
          collect_for_each_refs_from_choose(choose, refs)
        }
        _ => {}
      }
    }
  }
  if let Some(branch) = choose.diagram_choose_else.as_ref() {
    for choice in &branch.diagram_choose_else_choice {
      match choice {
        dgm::DiagramChooseElseChoice::ForEach(for_each) => collect_for_each_refs(for_each, refs),
        dgm::DiagramChooseElseChoice::LayoutNode(node) => {
          collect_for_each_refs_from_layout_node(node, refs)
        }
        dgm::DiagramChooseElseChoice::Choose(choose) => {
          collect_for_each_refs_from_choose(choose, refs)
        }
        _ => {}
      }
    }
  }
}

/// Selects zero-based entries using DrawingML's one-based iterator syntax.
///
/// ECMA-376 Part 1 §21.4.2.14 and `AG_IteratorAttributes` define `st=1`,
/// `cnt=0`, and `step=1` as the defaults. A zero count means that the
/// iterator is not count-limited. Negative starts and steps are used by
/// Office SmartArt layouts to walk a set from its last entry.
fn iterator_indices(available: usize, start: i32, count: usize, step: i32) -> Vec<usize> {
  if available == 0 || step == 0 {
    return Vec::new();
  }
  let available = available.min(i32::MAX as usize) as i32;
  let mut index = match start.cmp(&0) {
    std::cmp::Ordering::Greater => start - 1,
    std::cmp::Ordering::Equal => 0,
    std::cmp::Ordering::Less => available.saturating_add(start),
  };
  let limit = if count == 0 { usize::MAX } else { count };
  let mut indices = Vec::new();
  while index >= 0 && index < available && indices.len() < limit {
    indices.push(index as usize);
    let Some(next) = index.checked_add(step) else {
      break;
    };
    index = next;
  }
  indices
}

fn point_matches_element_type(point: &dgm::Point, point_type: dgm::ElementValues) -> bool {
  let kind = point.r#type.unwrap_or(dgm::PointValues::Node);
  match point_type {
    dgm::ElementValues::All => true,
    dgm::ElementValues::Document => kind == dgm::PointValues::Document,
    dgm::ElementValues::Node | dgm::ElementValues::Normal => kind == dgm::PointValues::Node,
    dgm::ElementValues::NonNormal => kind == dgm::PointValues::Assistant,
    dgm::ElementValues::Assistant => kind == dgm::PointValues::Assistant,
    dgm::ElementValues::NonAssistant => kind != dgm::PointValues::Assistant,
    dgm::ElementValues::ParentTransition => kind == dgm::PointValues::ParentTransition,
    dgm::ElementValues::Presentation => kind == dgm::PointValues::Presentation,
    dgm::ElementValues::SiblingTransition => kind == dgm::PointValues::SiblingTransition,
  }
}

struct DiagramShapeCreationVisitor<'a> {
  point_by_id: &'a HashMap<&'a str, &'a dgm::Point>,
  point_orders: &'a HashMap<&'a str, usize>,
  points_by_presentation_name: &'a HashMap<&'a str, Vec<&'a dgm::Point>>,
  data_by_presentation: &'a HashMap<&'a str, Vec<PresentationDataBinding<'a>>>,
  for_each_by_name: &'a HashMap<&'a str, &'a dgm::ForEach>,
  connections: &'a dgm::ConnectionList,
  metrics: &'a LayoutNodeMetrics,
  styles: Option<&'a DiagramStyles>,
  colors: Option<&'a DiagramStyleColors>,
  fallback_fill: RgbColor,
  current_point: &'a dgm::Point,
  current_index: usize,
  current_step: usize,
  current_count: usize,
  tree: Option<DiagramShapeNode>,
  parent_path: Vec<usize>,
  tree_root_mapped: bool,
}

impl<'a> DiagramShapeCreationVisitor<'a> {
  fn visit_layout_node(&mut self, node: &'a dgm::LayoutNode) {
    let Some(name) = node.name.as_deref() else {
      self.visit_layout_node_children(node);
      return;
    };
    let Some(points) = self.points_by_presentation_name.get(name) else {
      return;
    };
    let Some(new_point) = points.get(self.current_index).copied() else {
      return;
    };
    if !self.has_connection(self.current_point, new_point) {
      return;
    }

    let previous_point = self.current_point;
    let previous_path = self.parent_path.clone();
    self.current_point = new_point;
    if let Some(path) = self.append_shape_for_layout_node(node, name, new_point) {
      self.parent_path = path;
    }
    self.visit_layout_node_children(node);
    self.parent_path = previous_path;
    self.current_point = previous_point;
  }

  fn visit_layout_node_children(&mut self, node: &'a dgm::LayoutNode) {
    for choice in &node.layout_node_choice {
      match choice {
        dgm::LayoutNodeChoice::ForEach(for_each) => self.visit_for_each(for_each),
        dgm::LayoutNodeChoice::LayoutNode(child) => self.visit_layout_node(child),
        dgm::LayoutNodeChoice::Choose(choose) => self.visit_choose(choose),
        _ => {}
      }
    }
  }

  fn visit_for_each(&mut self, for_each: &'a dgm::ForEach) {
    if let Some(reference) = for_each.reference.as_deref() {
      if let Some(reference_atom) = self.for_each_by_name.get(reference) {
        self.visit_for_each(reference_atom);
      }
      return;
    }
    let hide_last_transition = for_each
      .hide_last_trans
      .as_ref()
      .and_then(|values| values.first())
      .copied()
      .map(bool::from)
      .unwrap_or(true);
    if hide_last_transition
      && for_each.axis.as_ref().is_some_and(|axes| {
        axes
          .first()
          .is_some_and(|axis| *axis == dgm::AxisValues::FollowSibling)
      })
      && self.current_index.saturating_add(self.current_step) >= self.current_count
    {
      return;
    }
    let point_type = for_each
      .point_type
      .as_ref()
      .and_then(|types| types.first())
      .copied()
      .unwrap_or_default();
    let mut children = 1usize;
    if matches!(
      point_type,
      dgm::ElementValues::Node | dgm::ElementValues::NonAssistant
    ) {
      children = self.shallow_presentation_name_count(&for_each.for_each_choice);
    }
    let requested_count = for_each
      .count
      .as_ref()
      .and_then(|counts| counts.first())
      .map(|count| *count as usize)
      .unwrap_or_default();
    let start = for_each
      .start
      .as_ref()
      .and_then(|starts| starts.first())
      .copied()
      .unwrap_or(1);
    let step = for_each
      .step
      .as_ref()
      .and_then(|steps| steps.first())
      .copied()
      .unwrap_or(1);
    let indices = iterator_indices(children, start, requested_count, step);
    let old_index = self.current_index;
    let old_step = self.current_step;
    let old_count = self.current_count;
    self.current_step = step.unsigned_abs() as usize;
    self.current_count = children;
    for index in indices {
      self.current_index = index;
      for choice in &for_each.for_each_choice {
        self.visit_for_each_choice(choice);
      }
    }
    self.current_index = old_index;
    self.current_step = old_step;
    self.current_count = old_count;
  }

  fn visit_for_each_choice(&mut self, choice: &'a dgm::ForEachChoice) {
    match choice {
      dgm::ForEachChoice::ForEach(for_each) => self.visit_for_each(for_each),
      dgm::ForEachChoice::LayoutNode(node) => self.visit_layout_node(node),
      dgm::ForEachChoice::Choose(choose) => self.visit_choose(choose),
      _ => {}
    }
  }

  fn visit_choose(&mut self, choose: &'a dgm::Choose) {
    for branch in &choose.diagram_choose_if {
      if self.choose_if_decision(branch) {
        for choice in &branch.diagram_choose_if_choice {
          self.visit_choose_if_choice(choice);
        }
        return;
      }
    }
    if let Some(branch) = choose.diagram_choose_else.as_ref() {
      for choice in &branch.diagram_choose_else_choice {
        self.visit_choose_else_choice(choice);
      }
    }
  }

  fn choose_if_decision(&self, branch: &dgm::DiagramChooseIf) -> bool {
    match branch.function {
      dgm::FunctionValues::Variable => match branch.argument.as_deref() {
        Some("dir") => {
          let direction = self
            .presentation_direction(self.current_point)
            .unwrap_or(dgm::DirectionValues::Normal);
          self.compare_condition(
            branch.operator,
            direction_condition_value(direction),
            parse_direction_condition_value(branch.val.as_str()),
          )
        }
        Some("hierBranch") => {
          let hierarchy_branch = self.presentation_hierarchy_branch(self.current_point);
          self.compare_condition(
            branch.operator,
            hierarchy_branch_condition_value(hierarchy_branch),
            parse_hierarchy_branch_condition_value(branch.val.as_str()),
          )
        }
        _ => true,
      },
      dgm::FunctionValues::Count => self.compare_condition(
        branch.operator,
        self.node_count_for_condition(branch),
        branch.val.parse::<i32>().unwrap_or_default(),
      ),
      dgm::FunctionValues::MaxDepth => self.compare_condition(
        branch.operator,
        self.max_depth_for_condition(branch),
        branch.val.parse::<i32>().unwrap_or_default(),
      ),
      dgm::FunctionValues::Depth => self.compare_condition(
        branch.operator,
        self.depth_for_condition(self.current_point),
        branch.val.parse::<i32>().unwrap_or_default(),
      ),
      dgm::FunctionValues::Position => self.compare_condition(
        branch.operator,
        self.iteration_position(),
        branch.val.parse::<i32>().unwrap_or_default(),
      ),
      dgm::FunctionValues::ReversePosition => self.compare_condition(
        branch.operator,
        self.iteration_reverse_position(),
        branch.val.parse::<i32>().unwrap_or_default(),
      ),
      dgm::FunctionValues::PositionEven => self.compare_condition(
        branch.operator,
        i32::from(self.iteration_position() % 2 == 0),
        branch.val.parse::<i32>().unwrap_or_default(),
      ),
      dgm::FunctionValues::PositionOdd => self.compare_condition(
        branch.operator,
        i32::from(self.iteration_position() % 2 != 0),
        branch.val.parse::<i32>().unwrap_or_default(),
      ),
    }
  }

  fn iteration_position(&self) -> i32 {
    self.current_index.saturating_add(1) as i32
  }

  fn iteration_reverse_position(&self) -> i32 {
    self.current_count.saturating_sub(self.current_index) as i32
  }

  fn compare_condition(
    &self,
    operator: dgm::FunctionOperatorValues,
    left: i32,
    right: i32,
  ) -> bool {
    match operator {
      dgm::FunctionOperatorValues::Equal => left == right,
      dgm::FunctionOperatorValues::NotEqualTo => left != right,
      dgm::FunctionOperatorValues::GreaterThan => left > right,
      dgm::FunctionOperatorValues::LessThan => left < right,
      dgm::FunctionOperatorValues::GreaterThanOrEqualTo => left >= right,
      dgm::FunctionOperatorValues::LessThanOrEqualTo => left <= right,
    }
  }

  fn node_count_for_condition(&self, branch: &dgm::DiagramChooseIf) -> i32 {
    self.condition_axis_points(branch).len() as i32
  }

  fn condition_axis_points(&self, branch: &dgm::DiagramChooseIf) -> Vec<&'a dgm::Point> {
    let Some(start_point) = self.condition_data_point(self.current_point) else {
      return Vec::new();
    };
    let Some(axes) = branch.axis.as_deref() else {
      return Vec::new();
    };
    let mut points = vec![start_point];
    for (level, axis) in axes.iter().copied().enumerate() {
      let point_type = branch
        .point_type
        .as_deref()
        .and_then(|types| types.get(level))
        .copied()
        .unwrap_or_default();
      points = self.points_on_axis(&points, axis, point_type);
      let start = branch
        .start
        .as_deref()
        .and_then(|starts| starts.get(level))
        .copied()
        .unwrap_or(1);
      let count = branch
        .count
        .as_deref()
        .and_then(|counts| counts.get(level))
        .copied()
        .unwrap_or_default() as usize;
      let step = branch
        .step
        .as_deref()
        .and_then(|steps| steps.get(level))
        .copied()
        .unwrap_or(1);
      points = iterator_indices(points.len(), start, count, step)
        .into_iter()
        .filter_map(|index| points.get(index).copied())
        .collect();
    }
    points
  }

  fn points_on_axis(
    &self,
    points: &[&'a dgm::Point],
    axis: dgm::AxisValues,
    point_type: dgm::ElementValues,
  ) -> Vec<&'a dgm::Point> {
    let mut selected = Vec::new();
    for point in points {
      match axis {
        dgm::AxisValues::_Self => self.push_matching_point(&mut selected, point, point_type),
        dgm::AxisValues::Child => {
          self.push_children_on_axis(&mut selected, point, point_type, false);
        }
        dgm::AxisValues::Descendant => {
          self.push_descendants_on_axis(&mut selected, point, point_type, &mut HashSet::new());
        }
        dgm::AxisValues::DescendantOrSelf => {
          self.push_matching_point(&mut selected, point, point_type);
          self.push_descendants_on_axis(&mut selected, point, point_type, &mut HashSet::new());
        }
        dgm::AxisValues::Parent => {
          if let Some(parent) = self.parent_data_point(point) {
            self.push_matching_point(&mut selected, parent, point_type);
          }
        }
        dgm::AxisValues::Ancestor => {
          self.push_ancestors_on_axis(&mut selected, point, point_type, false);
        }
        dgm::AxisValues::AncestorOrSelf => {
          self.push_matching_point(&mut selected, point, point_type);
          self.push_ancestors_on_axis(&mut selected, point, point_type, false);
        }
        dgm::AxisValues::FollowSibling => {
          self.push_siblings_on_axis(&mut selected, point, point_type, true, false);
        }
        dgm::AxisValues::PrecedingSibling => {
          self.push_siblings_on_axis(&mut selected, point, point_type, false, false);
        }
        dgm::AxisValues::Follow => {
          self.push_siblings_on_axis(&mut selected, point, point_type, true, true);
        }
        dgm::AxisValues::Preceding => {
          self.push_siblings_on_axis(&mut selected, point, point_type, false, true);
        }
        dgm::AxisValues::Root => {
          let mut root = *point;
          let mut visited = HashSet::new();
          while visited.insert(root.model_id.to_string()) {
            let Some(parent) = self.parent_data_point(root) else {
              break;
            };
            root = parent;
          }
          self.push_matching_point(&mut selected, root, point_type);
        }
        dgm::AxisValues::None => {}
      }
    }
    selected
  }

  fn push_children_on_axis(
    &self,
    selected: &mut Vec<&'a dgm::Point>,
    point: &dgm::Point,
    point_type: dgm::ElementValues,
    include_descendants: bool,
  ) {
    let mut child_connections: Vec<&dgm::Connection> = self
      .connections
      .connection
      .iter()
      .filter(|connection| {
        connection
          .r#type
          .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
          && connection.source_id == point.model_id
      })
      .collect();
    child_connections.sort_by_key(|connection| connection.source_position);
    for connection in child_connections {
      if matches!(
        point_type,
        dgm::ElementValues::All | dgm::ElementValues::ParentTransition
      ) && let Some(transition) = connection
        .parent_transition_id
        .as_deref()
        .and_then(|id| self.point_by_id.get(id))
        .copied()
      {
        self.push_unique_point(selected, transition);
      }
      if let Some(child) = self
        .point_by_id
        .get(connection.destination_id.as_str())
        .copied()
      {
        self.push_matching_point(selected, child, point_type);
        if include_descendants {
          self.push_descendants_on_axis(selected, child, point_type, &mut HashSet::new());
        }
      }
      if matches!(
        point_type,
        dgm::ElementValues::All | dgm::ElementValues::SiblingTransition
      ) && let Some(transition) = connection
        .sibling_transition_id
        .as_deref()
        .and_then(|id| self.point_by_id.get(id))
        .copied()
      {
        self.push_unique_point(selected, transition);
      }
    }
  }

  fn push_descendants_on_axis(
    &self,
    selected: &mut Vec<&'a dgm::Point>,
    point: &dgm::Point,
    point_type: dgm::ElementValues,
    visited: &mut HashSet<String>,
  ) {
    if !visited.insert(point.model_id.to_string()) {
      return;
    }
    let mut child_connections: Vec<&dgm::Connection> = self
      .connections
      .connection
      .iter()
      .filter(|connection| {
        connection
          .r#type
          .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
          && connection.source_id == point.model_id
      })
      .collect();
    child_connections.sort_by_key(|connection| connection.source_position);
    for connection in child_connections {
      if matches!(
        point_type,
        dgm::ElementValues::All | dgm::ElementValues::ParentTransition
      ) && let Some(transition) = connection
        .parent_transition_id
        .as_deref()
        .and_then(|id| self.point_by_id.get(id))
        .copied()
      {
        self.push_unique_point(selected, transition);
      }
      if let Some(child) = self
        .point_by_id
        .get(connection.destination_id.as_str())
        .copied()
      {
        self.push_matching_point(selected, child, point_type);
        self.push_descendants_on_axis(selected, child, point_type, visited);
      }
      if matches!(
        point_type,
        dgm::ElementValues::All | dgm::ElementValues::SiblingTransition
      ) && let Some(transition) = connection
        .sibling_transition_id
        .as_deref()
        .and_then(|id| self.point_by_id.get(id))
        .copied()
      {
        self.push_unique_point(selected, transition);
      }
    }
    visited.remove(point.model_id.as_str());
  }

  fn push_ancestors_on_axis(
    &self,
    selected: &mut Vec<&'a dgm::Point>,
    point: &dgm::Point,
    point_type: dgm::ElementValues,
    include_self: bool,
  ) {
    let mut current = point;
    let mut visited = HashSet::new();
    if include_self {
      self.push_matching_point(selected, current, point_type);
    }
    while visited.insert(current.model_id.to_string()) {
      let Some(parent) = self.parent_data_point(current) else {
        break;
      };
      self.push_matching_point(selected, parent, point_type);
      current = parent;
    }
  }

  fn push_siblings_on_axis(
    &self,
    selected: &mut Vec<&'a dgm::Point>,
    point: &dgm::Point,
    point_type: dgm::ElementValues,
    following: bool,
    include_descendants: bool,
  ) {
    let Some(current_connection) = self.parent_connection_for_point(point) else {
      return;
    };
    if following
      && matches!(
        point_type,
        dgm::ElementValues::All | dgm::ElementValues::SiblingTransition
      )
      && let Some(transition) = current_connection
        .sibling_transition_id
        .as_deref()
        .and_then(|id| self.point_by_id.get(id))
        .copied()
    {
      self.push_unique_point(selected, transition);
      if point_type == dgm::ElementValues::SiblingTransition {
        return;
      }
    }
    if !following
      && matches!(
        point_type,
        dgm::ElementValues::All | dgm::ElementValues::ParentTransition
      )
      && let Some(transition) = current_connection
        .parent_transition_id
        .as_deref()
        .and_then(|id| self.point_by_id.get(id))
        .copied()
    {
      self.push_unique_point(selected, transition);
      if point_type == dgm::ElementValues::ParentTransition {
        return;
      }
    }
    let mut sibling_connections: Vec<&dgm::Connection> = self
      .connections
      .connection
      .iter()
      .filter(|connection| {
        connection
          .r#type
          .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
          && connection.source_id == current_connection.source_id
          && if following {
            connection.source_position > current_connection.source_position
          } else {
            connection.source_position < current_connection.source_position
          }
      })
      .collect();
    sibling_connections.sort_by_key(|connection| connection.source_position);
    if !following {
      sibling_connections.reverse();
    }
    for connection in sibling_connections {
      let Some(sibling) = self
        .point_by_id
        .get(connection.destination_id.as_str())
        .copied()
      else {
        continue;
      };
      self.push_matching_point(selected, sibling, point_type);
      if include_descendants {
        self.push_descendants_on_axis(selected, sibling, point_type, &mut HashSet::new());
      }
    }
  }

  fn parent_connection_for_point(&self, point: &dgm::Point) -> Option<&'a dgm::Connection> {
    if matches!(
      point.r#type,
      Some(dgm::PointValues::ParentTransition | dgm::PointValues::SiblingTransition)
    ) {
      let connection_id = point.connection_id.as_deref()?;
      return self
        .connections
        .connection
        .iter()
        .find(|connection| connection.model_id == connection_id);
    }
    self.connections.connection.iter().find(|connection| {
      connection
        .r#type
        .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
        && connection.destination_id == point.model_id
    })
  }

  fn parent_data_point(&self, point: &dgm::Point) -> Option<&'a dgm::Point> {
    let connection = self.parent_connection_for_point(point)?;
    self.point_by_id.get(connection.source_id.as_str()).copied()
  }

  fn push_matching_point(
    &self,
    selected: &mut Vec<&'a dgm::Point>,
    point: &'a dgm::Point,
    point_type: dgm::ElementValues,
  ) {
    if point_matches_element_type(point, point_type) {
      self.push_unique_point(selected, point);
    }
  }

  fn push_unique_point(&self, selected: &mut Vec<&'a dgm::Point>, point: &'a dgm::Point) {
    if !selected
      .iter()
      .any(|existing| existing.model_id == point.model_id)
    {
      selected.push(point);
    }
  }

  fn depth_for_condition(&self, point: &dgm::Point) -> i32 {
    let Some(node_id) = self.condition_data_node_id(point) else {
      return 0;
    };
    self.depth_from_node(node_id, &mut HashSet::new())
  }

  fn depth_from_node(&self, node_id: &str, visited: &mut HashSet<String>) -> i32 {
    if !visited.insert(node_id.to_string()) {
      return 0;
    }
    let depth = self
      .connections
      .connection
      .iter()
      .find(|connection| {
        connection
          .r#type
          .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
          && connection.destination_id == node_id
      })
      .map(|connection| self.depth_from_node(connection.source_id.as_str(), visited) + 1)
      .unwrap_or_default();
    visited.remove(node_id);
    depth
  }

  fn max_depth_for_condition(&self, branch: &dgm::DiagramChooseIf) -> i32 {
    let Some(mut node_id) = self.condition_data_node_id(self.current_point) else {
      return 0;
    };
    if let Some(first_axis) = branch.axis.as_ref().and_then(|axes| axes.first()) {
      match first_axis {
        dgm::AxisValues::Parent => {
          if let Some(parent_id) =
            self.navigate_connection(dgm::ConnectionValues::ParentOf, node_id, false)
          {
            node_id = parent_id;
          }
        }
        dgm::AxisValues::Root => {
          while let Some(parent_id) =
            self.navigate_connection(dgm::ConnectionValues::ParentOf, node_id, false)
          {
            node_id = parent_id;
          }
        }
        _ => {}
      }
    }
    self.max_depth_from_node(node_id, &mut HashSet::new())
  }

  fn condition_data_node_id(&self, point: &dgm::Point) -> Option<&'a str> {
    let data_point = self.condition_data_point(point)?;
    if matches!(
      data_point.r#type,
      Some(dgm::PointValues::ParentTransition | dgm::PointValues::SiblingTransition)
    ) && let Some(connection_id) = data_point.connection_id.as_deref()
      && let Some(connection) = self
        .connections
        .connection
        .iter()
        .find(|connection| connection.model_id == connection_id)
    {
      return Some(connection.destination_id.as_str());
    }
    Some(data_point.model_id.as_str())
  }

  fn condition_data_point(&self, point: &dgm::Point) -> Option<&'a dgm::Point> {
    let point_id = presentation_association_id(point).unwrap_or(point.model_id.as_str());
    self.point_by_id.get(point_id).copied()
  }

  fn max_depth_from_node(&self, node_id: &str, visited: &mut HashSet<String>) -> i32 {
    if !visited.insert(node_id.to_string()) {
      return 0;
    }
    let max_depth = self
      .connections
      .connection
      .iter()
      .filter(|connection| {
        connection
          .r#type
          .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
          && connection.source_id == node_id
      })
      .map(|connection| self.max_depth_from_node(connection.destination_id.as_str(), visited) + 1)
      .max()
      .unwrap_or_default();
    visited.remove(node_id);
    max_depth
  }

  fn presentation_direction(&self, point: &dgm::Point) -> Option<dgm::DirectionValues> {
    point
      .property_set
      .as_deref()
      .and_then(|properties| properties.presentation_layout_variables.as_deref())
      .and_then(|variables| variables.direction.as_ref())
      .and_then(|direction| direction.val)
  }

  fn presentation_hierarchy_branch(&self, point: &dgm::Point) -> dgm::HierarchyBranchStyleValues {
    if let Some(hierarchy_branch) = point
      .property_set
      .as_deref()
      .and_then(|properties| properties.presentation_layout_variables.as_deref())
      .and_then(|variables| variables.hierarchy_branch.as_ref())
      .and_then(|hierarchy_branch| hierarchy_branch.val)
    {
      return hierarchy_branch;
    }
    if let Some(parent_id) = self.navigate_connection(
      dgm::ConnectionValues::PresentationParentOf,
      point.model_id.as_str(),
      false,
    ) && let Some(parent) = self.point_by_id.get(parent_id).copied()
      && let Some(hierarchy_branch) = parent
        .property_set
        .as_deref()
        .and_then(|properties| properties.presentation_layout_variables.as_deref())
        .and_then(|variables| variables.hierarchy_branch.as_ref())
        .and_then(|hierarchy_branch| hierarchy_branch.val)
    {
      return hierarchy_branch;
    }
    dgm::HierarchyBranchStyleValues::Standard
  }

  fn navigate_connection(
    &self,
    kind: dgm::ConnectionValues,
    from: &str,
    source_to_destination: bool,
  ) -> Option<&'a str> {
    self.connections.connection.iter().find_map(|connection| {
      let matches_type = if kind == dgm::ConnectionValues::ParentOf {
        connection
          .r#type
          .is_none_or(|connection_kind| connection_kind == kind)
      } else {
        connection.r#type == Some(kind)
      };
      if !matches_type {
        return None;
      }
      if source_to_destination {
        (connection.source_id == from).then_some(connection.destination_id.as_str())
      } else {
        (connection.destination_id == from).then_some(connection.source_id.as_str())
      }
    })
  }

  fn visit_choose_if_choice(&mut self, choice: &'a dgm::DiagramChooseIfChoice) {
    match choice {
      dgm::DiagramChooseIfChoice::ForEach(for_each) => self.visit_for_each(for_each),
      dgm::DiagramChooseIfChoice::LayoutNode(node) => self.visit_layout_node(node),
      dgm::DiagramChooseIfChoice::Choose(choose) => self.visit_choose(choose),
      _ => {}
    }
  }

  fn visit_choose_else_choice(&mut self, choice: &'a dgm::DiagramChooseElseChoice) {
    match choice {
      dgm::DiagramChooseElseChoice::ForEach(for_each) => self.visit_for_each(for_each),
      dgm::DiagramChooseElseChoice::LayoutNode(node) => self.visit_layout_node(node),
      dgm::DiagramChooseElseChoice::Choose(choose) => self.visit_choose(choose),
      _ => {}
    }
  }

  fn shallow_presentation_name_count(&self, choices: &'a [dgm::ForEachChoice]) -> usize {
    let mut count = 1usize;
    for choice in choices {
      match choice {
        dgm::ForEachChoice::ForEach(for_each) => {
          count = count.max(self.shallow_presentation_name_count(&for_each.for_each_choice));
        }
        dgm::ForEachChoice::LayoutNode(node) => {
          if let Some(name) = node.name.as_deref()
            && let Some(points) = self.points_by_presentation_name.get(name)
          {
            count = count.max(points.len());
          }
        }
        dgm::ForEachChoice::Choose(choose) => {
          count = count.max(self.shallow_choose_count(choose));
        }
        _ => {}
      }
    }
    count
  }

  fn shallow_choose_count(&self, choose: &'a dgm::Choose) -> usize {
    let mut count = 1usize;
    for branch in &choose.diagram_choose_if {
      if self.choose_if_decision(branch) {
        for choice in &branch.diagram_choose_if_choice {
          count = count.max(self.shallow_choose_if_choice_count(choice));
        }
        return count;
      }
    }
    if let Some(branch) = choose.diagram_choose_else.as_ref() {
      for choice in &branch.diagram_choose_else_choice {
        count = count.max(self.shallow_choose_else_choice_count(choice));
      }
    }
    count
  }

  fn shallow_choose_if_choice_count(&self, choice: &'a dgm::DiagramChooseIfChoice) -> usize {
    match choice {
      dgm::DiagramChooseIfChoice::ForEach(for_each) => {
        self.shallow_presentation_name_count(&for_each.for_each_choice)
      }
      dgm::DiagramChooseIfChoice::LayoutNode(node) => node
        .name
        .as_deref()
        .and_then(|name| self.points_by_presentation_name.get(name))
        .map(Vec::len)
        .unwrap_or(1),
      dgm::DiagramChooseIfChoice::Choose(choose) => self.shallow_choose_count(choose),
      _ => 1,
    }
  }

  fn shallow_choose_else_choice_count(&self, choice: &'a dgm::DiagramChooseElseChoice) -> usize {
    match choice {
      dgm::DiagramChooseElseChoice::ForEach(for_each) => {
        self.shallow_presentation_name_count(&for_each.for_each_choice)
      }
      dgm::DiagramChooseElseChoice::LayoutNode(node) => node
        .name
        .as_deref()
        .and_then(|name| self.points_by_presentation_name.get(name))
        .map(Vec::len)
        .unwrap_or(1),
      dgm::DiagramChooseElseChoice::Choose(choose) => self.shallow_choose_count(choose),
      _ => 1,
    }
  }

  fn append_shape_for_layout_node(
    &mut self,
    layout_node: &'a dgm::LayoutNode,
    name: &str,
    presentation_point: &'a dgm::Point,
  ) -> Option<Vec<usize>> {
    let shape_atom = self.active_shape_atom(layout_node);
    let data_points = self
      .data_by_presentation
      .get(presentation_point.model_id.as_str());
    let mut text_body = DiagramTextBody::default();
    let mut shape_properties =
      presentation_point
        .shape_properties
        .as_ref()
        .and_then(|properties| {
          diagram_shape_properties_has_blip_fill(properties).then(|| properties.clone())
        });
    let style_label = presentation_point
      .property_set
      .as_deref()
      .and_then(|property_set| property_set.presentation_style_label.as_deref())
      .or(layout_node.style_label.as_deref());
    let style = style_label
      .and_then(|label| {
        self
          .styles
          .and_then(|styles| styles.style_by_label.get(label))
      })
      .cloned();
    let line_fill = style_label
      .and_then(|label| {
        self
          .colors
          .and_then(|colors| colors.line_by_label.get(label))
      })
      .and_then(|fills| color_by_index(fills, self.current_index));
    let text_fill = style_label
      .and_then(|label| {
        self
          .colors
          .and_then(|colors| colors.text_fill_by_label.get(label))
      })
      .and_then(|fills| color_by_index(fills, self.current_index));
    let data_node_type = data_points
      .and_then(|points| points.first())
      .map(|binding| binding.point.r#type.unwrap_or_default())
      .and_then(point_type_to_element_type);
    let mut text_order = usize::MAX;
    if let Some(data_points) = data_points {
      let min_depth = data_points
        .iter()
        .map(|binding| binding.depth)
        .min()
        .unwrap_or_default();
      for binding in data_points {
        let data_point = binding.point;
        let first_new_paragraph = text_body.paragraphs.len();
        text_body.append_point(data_point, binding.depth);
        for paragraph in &mut text_body.paragraphs[first_new_paragraph..] {
          paragraph.source_order = Some(binding.source_order);
        }
        text_order = text_order.min(binding.source_order);
        if binding.depth == 0 || (shape_properties.is_none() && binding.depth == min_depth) {
          shape_properties = data_point.shape_properties.clone();
        }
      }
    } else {
      let first_new_paragraph = text_body.paragraphs.len();
      text_body.append_point(presentation_point, 0);
      if let Some(source_order) = self.point_orders.get(presentation_point.model_id.as_str()) {
        for paragraph in &mut text_body.paragraphs[first_new_paragraph..] {
          paragraph.source_order = Some(*source_order);
        }
        text_order = *source_order;
      }
    }
    self.tree.as_ref()?;
    let active_algorithms = self.active_algorithms(layout_node);
    let has_geometry = shape_atom
      .and_then(|shape| shape.r#type.as_deref())
      .is_some_and(|shape_type| !shape_type.is_empty());
    let hidden_geometry = shape_atom
      .and_then(|shape| shape.hide_geometry)
      .map(bool::from)
      .unwrap_or(false);
    let text_order = text_body.source_order().unwrap_or(text_order);
    let is_connector = shape_atom
      .and_then(|shape| shape.r#type.as_deref())
      .is_some_and(|shape_type| shape_type == "conn");
    let preset_geometry = shape_atom.and_then(diagram_layout_preset_geometry);
    let child = DiagramShapeNode {
      internal_name: name.to_string(),
      text_body,
      fill: diagram_node_fill(Some(presentation_point), self.colors, self.fallback_fill),
      x: 0.0,
      y: 0.0,
      width: 0.0,
      height: 0.0,
      algorithms: active_algorithms.clone(),
      child_order: layout_node.child_order.unwrap_or_default(),
      has_geometry,
      hidden_geometry,
      is_connector,
      shape_rotation_deg: shape_atom
        .and_then(|shape| shape.rotation)
        .unwrap_or_default() as f32,
      connector_angle_deg: 0.0,
      is_blip_placeholder: shape_atom
        .and_then(|shape| shape.blip_placeholder)
        .map(bool::from)
        .unwrap_or(false),
      z_order_offset: shape_atom
        .and_then(|shape| shape.z_order_offset)
        .unwrap_or_default(),
      shape_properties,
      preset_geometry,
      style,
      line_fill,
      text_fill,
      text_rotation_deg: 0.0,
      aspect_ratio: active_algorithms
        .iter()
        .rev()
        .find_map(|algorithm| algorithm.aspect_ratio)
        .unwrap_or_default(),
      data_node_type,
      font_size_pt: self.metrics.font_sizes.get(name).copied(),
      minimum_font_size_pt: None,
      font_sync_group: None,
      text_order,
      constraints: self.active_constraints(layout_node),
      direct_constraints: direct_constraints_unfiltered(layout_node),
      rules: self.active_rules(layout_node),
      children: Vec::new(),
    };
    if self.parent_path.is_empty() && !self.tree_root_mapped {
      let tree = self.tree.as_mut().expect("tree exists");
      let width = tree.width;
      let height = tree.height;
      *tree = DiagramShapeNode {
        x: 0.0,
        y: 0.0,
        width,
        height,
        ..child
      };
      self.tree_root_mapped = true;
      return Some(Vec::new());
    }
    Some(self.push_tree_child(child))
  }

  fn push_tree_child(&mut self, child: DiagramShapeNode) -> Vec<usize> {
    let tree = self.tree.as_mut().expect("tree exists");
    let parent = diagram_shape_node_mut(tree, &self.parent_path);
    parent.children.push(child);
    let mut path = self.parent_path.clone();
    path.push(parent.children.len() - 1);
    path
  }

  fn active_shape_atom(&self, node: &'a dgm::LayoutNode) -> Option<&'a dgm::Shape> {
    for choice in &node.layout_node_choice {
      match choice {
        dgm::LayoutNodeChoice::Shape(shape) => return Some(shape),
        dgm::LayoutNodeChoice::Choose(choose) => {
          if let Some(shape) = self.active_shape_atom_in_choose(choose) {
            return Some(shape);
          }
        }
        _ => {}
      }
    }
    None
  }

  fn active_algorithms(&self, node: &'a dgm::LayoutNode) -> Vec<LayoutAlgorithm> {
    let mut active = Vec::new();
    for choice in &node.layout_node_choice {
      match choice {
        dgm::LayoutNodeChoice::Algorithm(algorithm) => active.push(layout_algorithm(algorithm)),
        dgm::LayoutNodeChoice::Choose(choose) => {
          self.collect_active_algorithms_from_choose(choose, &mut active);
        }
        _ => {}
      }
    }
    active
  }

  fn collect_active_algorithms_from_choose(
    &self,
    choose: &'a dgm::Choose,
    active: &mut Vec<LayoutAlgorithm>,
  ) {
    for branch in &choose.diagram_choose_if {
      if self.choose_if_decision(branch) {
        for choice in &branch.diagram_choose_if_choice {
          match choice {
            dgm::DiagramChooseIfChoice::Algorithm(algorithm) => {
              active.push(layout_algorithm(algorithm));
            }
            dgm::DiagramChooseIfChoice::Choose(choose) => {
              self.collect_active_algorithms_from_choose(choose, active);
            }
            _ => {}
          }
        }
        return;
      }
    }
    if let Some(branch) = choose.diagram_choose_else.as_ref() {
      for choice in &branch.diagram_choose_else_choice {
        match choice {
          dgm::DiagramChooseElseChoice::Algorithm(algorithm) => {
            active.push(layout_algorithm(algorithm));
          }
          dgm::DiagramChooseElseChoice::Choose(choose) => {
            self.collect_active_algorithms_from_choose(choose, active);
          }
          _ => {}
        }
      }
    }
  }

  fn active_shape_atom_in_choose(&self, choose: &'a dgm::Choose) -> Option<&'a dgm::Shape> {
    for branch in &choose.diagram_choose_if {
      if self.choose_if_decision(branch) {
        for choice in &branch.diagram_choose_if_choice {
          match choice {
            dgm::DiagramChooseIfChoice::Shape(shape) => return Some(shape),
            dgm::DiagramChooseIfChoice::Choose(choose) => {
              if let Some(shape) = self.active_shape_atom_in_choose(choose) {
                return Some(shape);
              }
            }
            _ => {}
          }
        }
        return None;
      }
    }
    if let Some(branch) = choose.diagram_choose_else.as_ref() {
      for choice in &branch.diagram_choose_else_choice {
        match choice {
          dgm::DiagramChooseElseChoice::Shape(shape) => return Some(shape),
          dgm::DiagramChooseElseChoice::Choose(choose) => {
            if let Some(shape) = self.active_shape_atom_in_choose(choose) {
              return Some(shape);
            }
          }
          _ => {}
        }
      }
    }
    None
  }

  fn active_constraints(&self, node: &'a dgm::LayoutNode) -> Vec<DiagramConstraint> {
    let mut constraints = direct_constraints(node);
    self.collect_active_constraints_from_choices(&node.layout_node_choice, &mut constraints);
    constraints
  }

  fn collect_active_constraints_from_choices(
    &self,
    choices: &'a [dgm::LayoutNodeChoice],
    constraints: &mut Vec<DiagramConstraint>,
  ) {
    for choice in choices {
      if let dgm::LayoutNodeChoice::Choose(choose) = choice {
        self.collect_active_constraints_from_choose(choose, constraints);
      }
    }
  }

  fn collect_active_constraints_from_choose(
    &self,
    choose: &'a dgm::Choose,
    constraints: &mut Vec<DiagramConstraint>,
  ) {
    for branch in &choose.diagram_choose_if {
      if self.choose_if_decision(branch) {
        for choice in &branch.diagram_choose_if_choice {
          match choice {
            dgm::DiagramChooseIfChoice::Constraints(list) => {
              constraints.extend(parse_constraints(list, true));
            }
            dgm::DiagramChooseIfChoice::Choose(choose) => {
              self.collect_active_constraints_from_choose(choose, constraints);
            }
            _ => {}
          }
        }
        return;
      }
    }
    if let Some(branch) = choose.diagram_choose_else.as_ref() {
      for choice in &branch.diagram_choose_else_choice {
        match choice {
          dgm::DiagramChooseElseChoice::Constraints(list) => {
            constraints.extend(parse_constraints(list, true));
          }
          dgm::DiagramChooseElseChoice::Choose(choose) => {
            self.collect_active_constraints_from_choose(choose, constraints);
          }
          _ => {}
        }
      }
    }
  }

  fn active_rules(&self, node: &'a dgm::LayoutNode) -> Vec<DiagramRule> {
    let mut rules = direct_rules(node);
    self.collect_active_rules_from_choices(&node.layout_node_choice, &mut rules);
    rules
  }

  fn collect_active_rules_from_choices(
    &self,
    choices: &'a [dgm::LayoutNodeChoice],
    rules: &mut Vec<DiagramRule>,
  ) {
    for choice in choices {
      if let dgm::LayoutNodeChoice::Choose(choose) = choice {
        self.collect_active_rules_from_choose(choose, rules);
      }
    }
  }

  fn collect_active_rules_from_choose(
    &self,
    choose: &'a dgm::Choose,
    rules: &mut Vec<DiagramRule>,
  ) {
    for branch in &choose.diagram_choose_if {
      if self.choose_if_decision(branch) {
        for choice in &branch.diagram_choose_if_choice {
          match choice {
            dgm::DiagramChooseIfChoice::RuleList(list) => rules.extend(parse_rules(list)),
            dgm::DiagramChooseIfChoice::Choose(choose) => {
              self.collect_active_rules_from_choose(choose, rules);
            }
            _ => {}
          }
        }
        return;
      }
    }
    if let Some(branch) = choose.diagram_choose_else.as_ref() {
      for choice in &branch.diagram_choose_else_choice {
        match choice {
          dgm::DiagramChooseElseChoice::RuleList(list) => rules.extend(parse_rules(list)),
          dgm::DiagramChooseElseChoice::Choose(choose) => {
            self.collect_active_rules_from_choose(choose, rules);
          }
          _ => {}
        }
      }
    }
  }

  fn has_connection(&self, source: &dgm::Point, destination: &dgm::Point) -> bool {
    self.connections.connection.iter().any(|connection| {
      connection.source_id == source.model_id && connection.destination_id == destination.model_id
    })
  }
}

fn presentation_name(point: &dgm::Point) -> Option<&str> {
  point
    .property_set
    .as_deref()
    .and_then(|properties| properties.presentation_name.as_deref())
}

fn presentation_association_id(point: &dgm::Point) -> Option<&str> {
  point
    .property_set
    .as_deref()
    .and_then(|properties| properties.presentation_element_id.as_deref())
}

fn associated_data_point<'a>(
  presentation_point: &dgm::Point,
  point_by_id: &HashMap<&'a str, &'a dgm::Point>,
) -> Option<&'a dgm::Point> {
  presentation_association_id(presentation_point)
    .and_then(|id| point_by_id.get(id))
    .copied()
}

fn presentation_source_depth(data: &dgm::DataModelRoot, source_id: &str) -> i32 {
  let depth = data
    .connection_list
    .as_ref()
    .map(|connections| presentation_source_depth_from_connections(connections, source_id))
    .unwrap_or_default();
  if depth == 0 { -1 } else { depth }
}

fn presentation_source_depth_from_connections(
  connections: &dgm::ConnectionList,
  source_id: &str,
) -> i32 {
  for connection in &connections.connection {
    let has_transitions =
      connection.parent_transition_id.is_some() && connection.sibling_transition_id.is_some();
    let is_parent_of = connection
      .r#type
      .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf);
    if has_transitions && is_parent_of && connection.destination_id == source_id {
      return presentation_source_depth_from_connections(
        connections,
        connection.source_id.as_str(),
      ) + 1;
    }
  }
  0
}

fn direction_condition_value(direction: dgm::DirectionValues) -> i32 {
  match direction {
    dgm::DirectionValues::Normal => 0,
    dgm::DirectionValues::Reversed => 1,
  }
}

fn parse_direction_condition_value(value: &str) -> i32 {
  match value {
    "rev" => direction_condition_value(dgm::DirectionValues::Reversed),
    _ => direction_condition_value(dgm::DirectionValues::Normal),
  }
}

fn hierarchy_branch_condition_value(hierarchy_branch: dgm::HierarchyBranchStyleValues) -> i32 {
  match hierarchy_branch {
    dgm::HierarchyBranchStyleValues::Left => 0,
    dgm::HierarchyBranchStyleValues::Right => 1,
    dgm::HierarchyBranchStyleValues::Hanging => 2,
    dgm::HierarchyBranchStyleValues::Standard => 3,
    dgm::HierarchyBranchStyleValues::Initial => 4,
  }
}

fn parse_hierarchy_branch_condition_value(value: &str) -> i32 {
  match value {
    "l" => hierarchy_branch_condition_value(dgm::HierarchyBranchStyleValues::Left),
    "r" => hierarchy_branch_condition_value(dgm::HierarchyBranchStyleValues::Right),
    "hang" => hierarchy_branch_condition_value(dgm::HierarchyBranchStyleValues::Hanging),
    "init" => hierarchy_branch_condition_value(dgm::HierarchyBranchStyleValues::Initial),
    _ => hierarchy_branch_condition_value(dgm::HierarchyBranchStyleValues::Standard),
  }
}

fn point_type_to_element_type(point_type: dgm::PointValues) -> Option<dgm::ElementValues> {
  match point_type {
    dgm::PointValues::Node => Some(dgm::ElementValues::Node),
    dgm::PointValues::Assistant => Some(dgm::ElementValues::Assistant),
    dgm::PointValues::Document => Some(dgm::ElementValues::Document),
    dgm::PointValues::Presentation => Some(dgm::ElementValues::Presentation),
    dgm::PointValues::ParentTransition => Some(dgm::ElementValues::ParentTransition),
    dgm::PointValues::SiblingTransition => Some(dgm::ElementValues::SiblingTransition),
  }
}

fn diagram_shape_node_mut<'a>(
  node: &'a mut DiagramShapeNode,
  path: &[usize],
) -> &'a mut DiagramShapeNode {
  let mut current = node;
  for index in path {
    current = &mut current.children[*index];
  }
  current
}

fn direct_constraints(node: &dgm::LayoutNode) -> Vec<DiagramConstraint> {
  node
    .layout_node_choice
    .iter()
    .filter_map(|choice| match choice {
      dgm::LayoutNodeChoice::Constraints(constraints) => Some(constraints),
      _ => None,
    })
    .flat_map(|constraints| parse_constraints(constraints, true))
    .collect()
}

fn direct_constraints_unfiltered(node: &dgm::LayoutNode) -> Vec<DiagramConstraint> {
  node
    .layout_node_choice
    .iter()
    .filter_map(|choice| match choice {
      dgm::LayoutNodeChoice::Constraints(constraints) => Some(constraints),
      _ => None,
    })
    .flat_map(parse_constraints_unfiltered)
    .collect()
}

fn parse_constraints(
  constraints: &dgm::Constraints,
  require_for_name: bool,
) -> Vec<DiagramConstraint> {
  constraints
    .constraint
    .iter()
    .filter_map(|constraint| parse_constraint(constraint, require_for_name))
    .collect()
}

fn parse_constraints_unfiltered(constraints: &dgm::Constraints) -> Vec<DiagramConstraint> {
  constraints
    .constraint
    .iter()
    .filter_map(parse_constraint_unfiltered)
    .collect()
}

fn parse_constraint(
  constraint: &dgm::Constraint,
  require_for_name: bool,
) -> Option<DiagramConstraint> {
  let mut require_for_name = require_for_name;
  if require_for_name {
    if matches!(
      constraint.r#type,
      dgm::ConstraintValues::Spacing
        | dgm::ConstraintValues::LeftMargin
        | dgm::ConstraintValues::RightMargin
        | dgm::ConstraintValues::TopMargin
        | dgm::ConstraintValues::BottomMargin
        | dgm::ConstraintValues::PrimaryFontSize
        | dgm::ConstraintValues::SecondaryFontSize
    ) {
      require_for_name = false;
    }
    if constraint.point_type == Some(dgm::ElementValues::SiblingTransition) {
      require_for_name = false;
    }
  }
  if require_for_name
    && constraint
      .for_name
      .as_deref()
      .unwrap_or_default()
      .is_empty()
  {
    return None;
  }
  if !matches!(
    constraint.operator.unwrap_or_default(),
    dgm::BoolOperatorValues::None | dgm::BoolOperatorValues::Equal
  ) || constraint.r#type == dgm::ConstraintValues::None
  {
    return None;
  }
  Some(DiagramConstraint {
    for_name: constraint.for_name.clone().unwrap_or_default(),
    ref_for_name: constraint.reference_for_name.clone().unwrap_or_default(),
    factor: constraint.fact.unwrap_or(1.0) as f32,
    value: constraint.val.unwrap_or_default() as f32,
    target: constraint.r#type,
    reference: constraint.reference_type.unwrap_or_default(),
    relationship: constraint.r#for,
    operator: constraint.operator,
    point_type: constraint.point_type,
  })
}

fn parse_constraint_unfiltered(constraint: &dgm::Constraint) -> Option<DiagramConstraint> {
  (constraint.r#type != dgm::ConstraintValues::None).then(|| DiagramConstraint {
    for_name: constraint.for_name.clone().unwrap_or_default(),
    ref_for_name: constraint.reference_for_name.clone().unwrap_or_default(),
    factor: constraint.fact.unwrap_or(1.0) as f32,
    value: constraint.val.unwrap_or_default() as f32,
    target: constraint.r#type,
    reference: constraint.reference_type.unwrap_or_default(),
    relationship: constraint.r#for,
    operator: constraint.operator,
    point_type: constraint.point_type,
  })
}

fn direct_rules(node: &dgm::LayoutNode) -> Vec<DiagramRule> {
  node
    .layout_node_choice
    .iter()
    .filter_map(|choice| match choice {
      dgm::LayoutNodeChoice::RuleList(rules) => Some(rules),
      _ => None,
    })
    .flat_map(parse_rules)
    .collect()
}

fn parse_rules(rules: &dgm::RuleList) -> Vec<DiagramRule> {
  rules
    .rule
    .iter()
    .filter(|rule| rule.r#type != dgm::ConstraintValues::None)
    .map(|rule| DiagramRule {
      for_name: rule.for_name.clone().unwrap_or_default(),
      target: rule.r#type,
      point_type: rule.point_type,
      value: rule.val.unwrap_or_default() as f32,
    })
    .collect()
}

#[derive(Clone, Debug, Default)]
struct LayoutNodeMetrics {
  font_sizes: HashMap<String, f32>,
}

fn layout_node_metrics(layout: Option<&dgm::LayoutDefinition>) -> LayoutNodeMetrics {
  let mut metrics = LayoutNodeMetrics::default();
  if let Some(layout) = layout {
    collect_layout_node_metrics(&layout.layout_node, &mut metrics);
  }
  metrics
}

fn collect_layout_node_metrics(node: &dgm::LayoutNode, metrics: &mut LayoutNodeMetrics) {
  collect_constraints(node, metrics);
  for choice in &node.layout_node_choice {
    match choice {
      dgm::LayoutNodeChoice::LayoutNode(child) => collect_layout_node_metrics(child, metrics),
      dgm::LayoutNodeChoice::ForEach(for_each) => collect_for_each_metrics(for_each, metrics),
      dgm::LayoutNodeChoice::Choose(choose) => collect_choose_metrics(choose, metrics),
      _ => {}
    }
  }
}

fn collect_for_each_metrics(for_each: &dgm::ForEach, metrics: &mut LayoutNodeMetrics) {
  for choice in &for_each.for_each_choice {
    match choice {
      dgm::ForEachChoice::LayoutNode(node) => collect_layout_node_metrics(node, metrics),
      dgm::ForEachChoice::ForEach(child) => collect_for_each_metrics(child, metrics),
      dgm::ForEachChoice::Choose(choose) => collect_choose_metrics(choose, metrics),
      _ => {}
    }
  }
}

fn collect_choose_metrics(choose: &dgm::Choose, metrics: &mut LayoutNodeMetrics) {
  for branch in &choose.diagram_choose_if {
    for choice in &branch.diagram_choose_if_choice {
      match choice {
        dgm::DiagramChooseIfChoice::LayoutNode(node) => collect_layout_node_metrics(node, metrics),
        dgm::DiagramChooseIfChoice::ForEach(for_each) => {
          collect_for_each_metrics(for_each, metrics)
        }
        _ => {}
      }
    }
  }
  if let Some(branch) = choose.diagram_choose_else.as_ref() {
    for choice in &branch.diagram_choose_else_choice {
      match choice {
        dgm::DiagramChooseElseChoice::LayoutNode(node) => {
          collect_layout_node_metrics(node.as_ref(), metrics)
        }
        dgm::DiagramChooseElseChoice::ForEach(for_each) => {
          collect_for_each_metrics(for_each.as_ref(), metrics);
        }
        _ => {}
      }
    }
  }
}

fn collect_constraints(node: &dgm::LayoutNode, metrics: &mut LayoutNodeMetrics) {
  for choice in &node.layout_node_choice {
    let dgm::LayoutNodeChoice::Constraints(constraints) = choice else {
      continue;
    };
    for constraint in &constraints.constraint {
      if constraint.r#type == dgm::ConstraintValues::PrimaryFontSize
        && let (Some(name), Some(value)) = (constraint.for_name.as_deref(), constraint.val)
      {
        metrics.font_sizes.insert(name.to_string(), value as f32);
      }
    }
  }
}

fn diagram_node_fill(
  presentation_point: Option<&dgm::Point>,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
) -> RgbColor {
  let Some(style_label) = presentation_point
    .and_then(|point| point.property_set.as_deref())
    .and_then(|property_set| property_set.presentation_style_label.as_deref())
  else {
    return fallback_fill;
  };
  let style_index = presentation_point
    .and_then(|point| point.property_set.as_deref())
    .and_then(|property_set| property_set.presentation_style_index)
    .unwrap_or_default()
    .max(0) as usize;
  colors
    .and_then(|colors| colors.fill_by_label.get(style_label))
    .and_then(|fills| color_by_index(fills, style_index))
    .unwrap_or(fallback_fill)
}

fn diagram_shape_properties_has_blip_fill(properties: &dgm::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(dgm::ShapePropertiesChoice2::BlipFill(_))
  )
}

fn color_by_index(colors: &[RgbColor], index: usize) -> Option<RgbColor> {
  (!colors.is_empty()).then(|| colors[index % colors.len()])
}

fn layout_algorithm(algorithm: &dgm::Algorithm) -> LayoutAlgorithm {
  let linear_direction = algorithm
    .parameter
    .iter()
    .find(|parameter| parameter.r#type == dgm::ParameterIdValues::LinearDirection)
    .and_then(|parameter| parameter.val.as_deref())
    .map(linear_direction_from_value)
    .unwrap_or_default();
  let secondary_linear_direction = algorithm
    .parameter
    .iter()
    .find(|parameter| parameter.r#type == dgm::ParameterIdValues::SecondaryLinearDirection)
    .and_then(|parameter| parameter.val.as_deref())
    .and_then(|value| (value != "none").then(|| linear_direction_from_value(value)))
    .unwrap_or_default();
  let grow_direction = algorithm
    .parameter
    .iter()
    .find(|parameter| parameter.r#type == dgm::ParameterIdValues::GrowDirection)
    .and_then(|parameter| parameter.val.as_deref())
    .map(grow_direction_from_value)
    .unwrap_or_default();
  let continue_direction = algorithm
    .parameter
    .iter()
    .find(|parameter| parameter.r#type == dgm::ParameterIdValues::ContinueDirection)
    .and_then(|parameter| parameter.val.as_deref())
    .map(continue_direction_from_value)
    .unwrap_or_default();
  LayoutAlgorithm {
    kind: algorithm.r#type,
    linear_direction,
    secondary_linear_direction,
    grow_direction,
    continue_direction,
    start_angle: algorithm_parameter_f32(algorithm, dgm::ParameterIdValues::StartAngle)
      .unwrap_or_default(),
    span_angle: algorithm_parameter_f32(algorithm, dgm::ParameterIdValues::SpanAngle)
      .unwrap_or(360.0),
    center_shape_mapping_first_node: algorithm
      .parameter
      .iter()
      .find(|parameter| parameter.r#type == dgm::ParameterIdValues::CenterShapeMapping)
      .and_then(|parameter| parameter.val.as_deref())
      == Some("fNode"),
    rotation_path_along_path: algorithm
      .parameter
      .iter()
      .find(|parameter| parameter.r#type == dgm::ParameterIdValues::RotationPath)
      .and_then(|parameter| parameter.val.as_deref())
      == Some("alongPath"),
    aspect_ratio: algorithm_parameter_f32(algorithm, dgm::ParameterIdValues::AspectRatio),
    auto_text_rotation: algorithm
      .parameter
      .iter()
      .find(|parameter| parameter.r#type == dgm::ParameterIdValues::AutoTextRotation)
      .and_then(|parameter| parameter.val.as_deref())
      .map(auto_text_rotation_from_value),
    text_anchor_vertical: algorithm
      .parameter
      .iter()
      .find(|parameter| parameter.r#type == dgm::ParameterIdValues::TextAnchorVertical)
      .and_then(|parameter| parameter.val.as_deref())
      .map(text_anchor_vertical_from_value)
      .unwrap_or(dgm::TextAnchorVerticalValues::Middle),
    start_bullets_at_level: algorithm_parameter_f32(
      algorithm,
      dgm::ParameterIdValues::StartBulletsAtLevel,
    )
    .unwrap_or(2.0)
    .round() as i32,
    parent_text_left_to_right_alignment: algorithm
      .parameter
      .iter()
      .find(|parameter| parameter.r#type == dgm::ParameterIdValues::ParentTextLeftToRightAlignment)
      .and_then(|parameter| parameter.val.as_deref())
      .map(text_alignment_from_value),
  }
}

fn text_alignment_from_value(value: &str) -> dgm::TextAlignmentValues {
  match value {
    "ctr" => dgm::TextAlignmentValues::Center,
    "r" => dgm::TextAlignmentValues::Right,
    _ => dgm::TextAlignmentValues::Left,
  }
}

fn drawingml_alignment_from_diagram(value: dgm::TextAlignmentValues) -> a::TextAlignmentTypeValues {
  match value {
    dgm::TextAlignmentValues::Center => a::TextAlignmentTypeValues::Center,
    dgm::TextAlignmentValues::Right => a::TextAlignmentTypeValues::Right,
    dgm::TextAlignmentValues::Left => a::TextAlignmentTypeValues::Left,
  }
}

fn auto_text_rotation_from_value(value: &str) -> dgm::AutoTextRotationValues {
  match value {
    "grav" => dgm::AutoTextRotationValues::Gravity,
    "none" => dgm::AutoTextRotationValues::None,
    _ => dgm::AutoTextRotationValues::Upright,
  }
}

fn text_anchor_vertical_from_value(value: &str) -> dgm::TextAnchorVerticalValues {
  match value {
    "t" => dgm::TextAnchorVerticalValues::Top,
    "b" => dgm::TextAnchorVerticalValues::Bottom,
    _ => dgm::TextAnchorVerticalValues::Middle,
  }
}

fn linear_direction_from_value(value: &str) -> LinearDirection {
  match value {
    "fromR" => LinearDirection::Right,
    "fromT" => LinearDirection::Top,
    "fromB" => LinearDirection::Bottom,
    _ => LinearDirection::Left,
  }
}

fn grow_direction_from_value(value: &str) -> GrowDirection {
  match value {
    "tR" => GrowDirection::TopRight,
    "bL" => GrowDirection::BottomLeft,
    "bR" => GrowDirection::BottomRight,
    _ => GrowDirection::TopLeft,
  }
}

fn continue_direction_from_value(value: &str) -> ContinueDirection {
  match value {
    "revDir" => ContinueDirection::ReverseDirection,
    _ => ContinueDirection::SameDirection,
  }
}

fn algorithm_parameter_f32(
  algorithm: &dgm::Algorithm,
  parameter_type: dgm::ParameterIdValues,
) -> Option<f32> {
  algorithm
    .parameter
    .iter()
    .find(|parameter| parameter.r#type == parameter_type)
    .and_then(|parameter| parameter.val.as_deref())
    .and_then(|value| value.parse::<f32>().ok())
}

fn layout_diagram_shape_tree(root: &mut DiagramShapeNode) {
  assign_diagram_font_sync_groups(root, &[]);
  layout_diagram_shape_node(root, &[], &[]);
}

#[derive(Clone)]
struct DiagramFontSyncGroup {
  key: String,
  for_name: String,
  point_type: Option<dgm::ElementValues>,
}

fn assign_diagram_font_sync_groups(
  node: &mut DiagramShapeNode,
  inherited_groups: &[DiagramFontSyncGroup],
) {
  node.font_sync_group = inherited_groups
    .iter()
    .rev()
    .find(|group| {
      (group.for_name.is_empty() || group.for_name == node.internal_name)
        && group
          .point_type
          .is_none_or(|point_type| Some(point_type) == node.data_node_type)
    })
    .map(|group| group.key.clone());

  let mut descendant_groups = inherited_groups.to_vec();
  for (index, constraint) in node.direct_constraints.iter().enumerate() {
    if constraint.target == dgm::ConstraintValues::PrimaryFontSize
      && constraint.relationship == Some(dgm::ConstraintRelationshipValues::Descendant)
      && constraint.operator == Some(dgm::BoolOperatorValues::Equal)
    {
      descendant_groups.push(DiagramFontSyncGroup {
        key: format!("{}:{index}", node.internal_name),
        for_name: constraint.for_name.clone(),
        point_type: constraint.point_type,
      });
    }
  }
  for child in &mut node.children {
    assign_diagram_font_sync_groups(child, &descendant_groups);
  }
}

fn layout_diagram_shape_node(
  node: &mut DiagramShapeNode,
  inherited_constraints: &[DiagramConstraint],
  inherited_rules: &[DiagramRule],
) {
  let mut constraints = inherited_constraints.to_vec();
  constraints.extend(node.constraints.clone());
  let mut rules = inherited_rules.to_vec();
  rules.extend(node.rules.clone());
  for algorithm in node.algorithms.clone() {
    layout_shape_children(node, algorithm, &constraints, &rules);
  }
  for child in &mut node.children {
    layout_diagram_shape_node(child, &constraints, &rules);
  }
}

fn layout_shape_children(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
  rules: &[DiagramRule],
) {
  node
    .children
    .retain(|child| algorithm.kind == dgm::AlgorithmValues::Linear || !child.is_empty_group());
  match algorithm.kind {
    dgm::AlgorithmValues::Composite => composite_layout_tree(node, algorithm, constraints),
    dgm::AlgorithmValues::Linear => linear_layout_tree(node, algorithm, constraints, rules),
    dgm::AlgorithmValues::Cycle => cycle_layout_tree(node, algorithm),
    dgm::AlgorithmValues::HierarchyRoot | dgm::AlgorithmValues::HierarchyChild => {
      hierarchy_layout_tree(node, algorithm)
    }
    dgm::AlgorithmValues::Snake => snake_layout_tree(node, algorithm, constraints),
    dgm::AlgorithmValues::Text => apply_text_algorithm(node, constraints, rules),
    dgm::AlgorithmValues::Space => {
      // ECMA-376 §21.4.7.1 assigns `sp` only spacing/no-op layout duties;
      // text layout belongs to `tx`. LibreOffice's DiagramLayoutAtom::layoutShape
      // likewise clears the `sp` shape text before the `tx` atom lays it out.
      node.text_body = DiagramTextBody::default();
    }
    dgm::AlgorithmValues::Connector => connector_layout_tree(node),
    dgm::AlgorithmValues::Pyramid => pyramid_layout_tree(node),
  }
}

fn apply_text_algorithm(
  node: &mut DiagramShapeNode,
  constraints: &[DiagramConstraint],
  rules: &[DiagramRule],
) {
  let has_direct_font_size = node.text_body.has_direct_font_size();
  let font_size = constraints
    .iter()
    .rev()
    .find(|constraint| {
      constraint.target == dgm::ConstraintValues::PrimaryFontSize
        && (constraint.for_name.is_empty() || constraint.for_name == node.internal_name)
    })
    .map(|constraint| constraint.value)
    .filter(|value| *value > 0.0);
  if let Some(font_size) = font_size {
    node.font_size_pt = Some(font_size);
  }
  // ECMA-376 Part 1 §21.4.2.24 defines a primFontSz rule value as the
  // lower limit used while the tx algorithm shrinks text to fit.
  node.minimum_font_size_pt = rules
    .iter()
    .rev()
    .find(|rule| {
      rule.target == dgm::ConstraintValues::PrimaryFontSize
        && (rule.for_name.is_empty() || rule.for_name == node.internal_name)
        && rule
          .point_type
          .is_none_or(|point_type| Some(point_type) == node.data_node_type)
    })
    .map(|rule| rule.value)
    .filter(|value| *value > 0.0);
  node.text_body.apply_text_margins(
    node.width,
    node.height,
    font_size.or(node.font_size_pt),
    constraints,
  );
  node
    .text_body
    .enable_auto_fit_if_default_text(has_direct_font_size);
  let shape_rotation = shape_rotation_degrees(node);
  node.text_rotation_deg = shape_rotation
    + text_pre_rotation_degrees(
      node
        .algorithms
        .iter()
        .rev()
        .find_map(|algorithm| algorithm.auto_text_rotation)
        .unwrap_or(dgm::AutoTextRotationValues::Upright),
      shape_rotation,
    );
  node.text_body.set_vertical_anchor(
    match node
      .algorithms
      .iter()
      .rev()
      .map(|algorithm| algorithm.text_anchor_vertical)
      .next()
      .unwrap_or(dgm::TextAnchorVerticalValues::Middle)
    {
      dgm::TextAnchorVerticalValues::Top => a::TextAnchoringTypeValues::Top,
      dgm::TextAnchorVerticalValues::Bottom => a::TextAnchoringTypeValues::Bottom,
      dgm::TextAnchorVerticalValues::Middle => a::TextAnchoringTypeValues::Center,
    },
  );
  let alignment = node
    .algorithms
    .iter()
    .rev()
    .find_map(|algorithm| algorithm.parent_text_left_to_right_alignment)
    .map(drawingml_alignment_from_diagram);
  node.text_body.apply_text_algorithm_paragraph_rules(
    node
      .algorithms
      .iter()
      .rev()
      .map(|algorithm| algorithm.start_bullets_at_level)
      .next()
      .unwrap_or(2),
    alignment,
  );
  if let Some(primary_font_size) = font_size {
    let secondary_font_size = constraints
      .iter()
      .rev()
      .find(|constraint| {
        constraint.target == dgm::ConstraintValues::SecondaryFontSize
          && (constraint.for_name.is_empty() || constraint.for_name == node.internal_name)
      })
      .map(|constraint| {
        if constraint.value > 0.0 {
          constraint.value
        } else if constraint.reference == dgm::ConstraintValues::PrimaryFontSize {
          primary_font_size * constraint.factor
        } else {
          primary_font_size * SMARTART_DEFAULT_SECONDARY_FONT_SCALE
        }
      })
      .filter(|value| *value > 0.0)
      .unwrap_or(primary_font_size * SMARTART_DEFAULT_SECONDARY_FONT_SCALE);
    node
      .text_body
      .apply_font_sizes(primary_font_size, secondary_font_size);
  }
}

fn points_to_emu(value: f32) -> i32 {
  (value * 12_700.0).round() as i32
}

fn diagram_layout_preset_geometry(shape: &dgm::Shape) -> Option<Box<a::PresetGeometry>> {
  let preset = shape.r#type.as_deref()?.parse().ok()?;
  let mut adjustments = shape
    .adjust_list
    .as_ref()
    .map(|list| list.adjust.clone())
    .unwrap_or_default();
  adjustments.sort_by_key(|adjustment| adjustment.index);
  let adjust_value_list = (!adjustments.is_empty()).then(|| a::AdjustValueList {
    shape_guide: adjustments
      .into_iter()
      .map(|adjustment| a::ShapeGuide {
        name: format!("adj{}", adjustment.index),
        formula: format!(
          "val {}",
          (adjustment.val * DRAWINGML_ADJUST_FULL_SCALE).round()
        ),
      })
      .collect(),
  });
  Some(Box::new(a::PresetGeometry {
    xmlns: Vec::new(),
    preset,
    adjust_value_list,
  }))
}

fn shape_rotation_degrees(node: &DiagramShapeNode) -> f32 {
  node.shape_rotation_deg
    + node
      .shape_properties
      .as_deref()
      .and_then(|properties| properties.transform2_d.as_deref())
      .and_then(|transform| transform.rotation)
      .map(|rotation| rotation as f32 / 60_000.0)
      .unwrap_or_default()
}

fn text_pre_rotation_degrees(
  auto_text_rotation: dgm::AutoTextRotationValues,
  shape_rotation: f32,
) -> f32 {
  let mut normalized = shape_rotation;
  while normalized < 0.0 {
    normalized += 360.0;
  }
  while normalized > 360.0 {
    normalized -= 360.0;
  }
  match auto_text_rotation {
    dgm::AutoTextRotationValues::Upright => {
      if normalized >= 315.0 {
        0.0
      } else if normalized > 225.0 {
        -270.0
      } else if normalized >= 135.0 {
        -180.0
      } else if normalized > 45.0 {
        -90.0
      } else {
        0.0
      }
    }
    dgm::AutoTextRotationValues::Gravity if normalized > 90.0 && normalized < 270.0 => -180.0,
    dgm::AutoTextRotationValues::Gravity | dgm::AutoTextRotationValues::None => 0.0,
  }
}

impl DiagramShapeNode {
  fn is_empty_group(&self) -> bool {
    self.text_body.is_empty() && self.children.is_empty() && !self.has_geometry
  }
}

fn composite_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
) {
  let mut properties: HashMap<String, HashMap<dgm::ConstraintValues, f32>> = HashMap::new();
  let parent_width = if algorithm.aspect_ratio.unwrap_or_default() == 1.0 {
    node.width.min(node.height)
  } else {
    node.width
  };
  let parent_x_offset = if parent_width < node.width {
    (node.width - parent_width) / 2.0
  } else {
    0.0
  };
  properties.insert(
    String::new(),
    HashMap::from([
      (dgm::ConstraintValues::Width, parent_width),
      (dgm::ConstraintValues::Height, node.height),
      (dgm::ConstraintValues::Left, parent_x_offset),
      (dgm::ConstraintValues::Top, 0.0),
      (dgm::ConstraintValues::Right, node.width - parent_x_offset),
      (dgm::ConstraintValues::Bottom, node.height),
    ]),
  );
  for constraint in constraints {
    apply_constraint_to_layout(constraint, &mut properties);
  }
  let mut vertical_min = f32::MAX;
  let mut vertical_max = 0.0_f32;
  for child in &mut node.children {
    for constraint in constraints
      .iter()
      .filter(|constraint| constraint.for_name == child.internal_name)
    {
      apply_constraint_to_layout(constraint, &mut properties);
    }
    for constraint in &child.direct_constraints {
      if !constraint.for_name.is_empty() || !constraint.ref_for_name.is_empty() {
        continue;
      }
      if constraint.value == 0.0 && constraint.reference == dgm::ConstraintValues::None {
        continue;
      }
      let mut constraint = constraint.clone();
      constraint.for_name = child.internal_name.clone();
      constraint.ref_for_name = child.internal_name.clone();
      apply_constraint_to_layout(&constraint, &mut properties);
    }
    let properties_for_child = properties.get(child.internal_name.as_str());
    let mut width = node.width;
    let mut height = node.height;
    let mut x = 0.0;
    let mut y = 0.0;
    if let Some(properties_for_child) = properties_for_child {
      if let Some(value) = properties_for_child.get(&dgm::ConstraintValues::Width) {
        width = (*value).min(node.width);
      }
      if let Some(value) = properties_for_child.get(&dgm::ConstraintValues::Height) {
        height = (*value).min(node.height);
      }
      if let Some(value) = properties_for_child.get(&dgm::ConstraintValues::Left) {
        x = *value;
      } else if let Some(value) = properties_for_child.get(&dgm::ConstraintValues::CenterHeight) {
        x = *value - width / 2.0;
      } else if let Some(value) = properties_for_child.get(&dgm::ConstraintValues::Right) {
        x = *value - width;
      }
      if let Some(value) = properties_for_child.get(&dgm::ConstraintValues::Top) {
        y = *value;
      } else if let Some(value) = properties_for_child.get(&dgm::ConstraintValues::CenterWidth) {
        y = *value - height / 2.0;
      } else if let Some(value) = properties_for_child.get(&dgm::ConstraintValues::Bottom) {
        y = *value - height;
      }
      if let (Some(left), Some(right)) = (
        properties_for_child.get(&dgm::ConstraintValues::Left),
        properties_for_child.get(&dgm::ConstraintValues::Right),
      ) {
        width = right - left;
      }
      if let (Some(top), Some(bottom)) = (
        properties_for_child.get(&dgm::ConstraintValues::Top),
        properties_for_child.get(&dgm::ConstraintValues::Bottom),
      ) {
        height = bottom - top;
      }
    }
    x += parent_x_offset;
    child.x = x.max(0.0);
    child.y = y.max(0.0);
    child.width = width.min((node.width - child.x).max(0.0));
    child.height = height.min((node.height - child.y).max(0.0));
    vertical_min = vertical_min.min(child.y);
    vertical_max = vertical_max.max(child.y + child.height);
  }
  if vertical_min >= 0.0 && vertical_min <= vertical_max && vertical_max <= node.height {
    let diff = node.height - (vertical_max - vertical_min);
    if diff > 0.0 {
      for child in &mut node.children {
        child.y += diff / 2.0;
      }
    }
  }
  if node.child_order == dgm::ChildOrderValues::Top {
    node.children.reverse();
  }
}

fn apply_constraint_to_layout(
  constraint: &DiagramConstraint,
  properties: &mut HashMap<String, HashMap<dgm::ConstraintValues, f32>>,
) {
  if constraint.for_name.is_empty() {
    return;
  }
  let reference = properties.get(constraint.ref_for_name.as_str());
  let value = reference
    .and_then(|properties| properties.get(&constraint.reference).copied())
    .or_else(|| {
      reference.and_then(|properties| {
        if constraint.reference == dgm::ConstraintValues::Right {
          Some(
            properties.get(&dgm::ConstraintValues::Left).copied()?
              + properties.get(&dgm::ConstraintValues::Width).copied()?,
          )
        } else {
          None
        }
      })
    })
    .map(|value| value * constraint.factor)
    .unwrap_or_else(|| constraint_value_points(constraint));
  properties
    .entry(constraint.for_name.clone())
    .or_default()
    .insert(constraint.target, value);
}

fn constraint_value_points(constraint: &DiagramConstraint) -> f32 {
  if matches!(
    constraint.reference,
    dgm::ConstraintValues::PrimaryFontSize | dgm::ConstraintValues::SecondaryFontSize
  ) {
    constraint.value
  } else {
    constraint.value * 72.0 / 25.4
  }
}

fn linear_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
  rules: &[DiagramRule],
) {
  if node.children.is_empty() || node.width == 0.0 || node.height == 0.0 {
    return;
  }
  let horizontal = matches!(
    algorithm.linear_direction,
    LinearDirection::Left | LinearDirection::Right
  );
  let reverse = matches!(
    algorithm.linear_direction,
    LinearDirection::Right | LinearDirection::Bottom
  );
  let connector_angle = match algorithm.linear_direction {
    LinearDirection::Left => 0.0,
    LinearDirection::Right => 180.0,
    LinearDirection::Top => 270.0,
    LinearDirection::Bottom => 90.0,
  };
  let mut properties: HashMap<String, HashMap<dgm::ConstraintValues, f32>> = HashMap::new();
  for constraint in constraints {
    let target_names = if !constraint.for_name.is_empty() {
      vec![constraint.for_name.as_str()]
    } else if let Some(point_type) = constraint.point_type {
      node
        .children
        .iter()
        .filter(|child| child.data_node_type == Some(point_type))
        .map(|child| child.internal_name.as_str())
        .collect()
    } else {
      Vec::new()
    };
    for target_name in target_names {
      if constraint.target == dgm::ConstraintValues::Width {
        properties
          .entry(target_name.to_string())
          .or_default()
          .insert(
            dgm::ConstraintValues::Width,
            (node.width * constraint.factor).min(node.width),
          );
      }
      if constraint.target == dgm::ConstraintValues::Height {
        properties
          .entry(target_name.to_string())
          .or_default()
          .insert(
            dgm::ConstraintValues::Height,
            (node.height * constraint.factor).min(node.height),
          );
      }
    }
  }
  let mut space_width = 0.0;
  let mut space_height = 0.0;
  for constraint in constraints {
    if matches!(constraint.for_name.as_str(), "sp" | "space" | "sibTrans") {
      // A materialized spacing/transition layout node already consumes its
      // constrained width or height in the linear child sequence. Adding the
      // same value again between every child double-counts sibling spacing.
      let has_explicit_spacing_child = node
        .children
        .iter()
        .any(|child| child.internal_name == constraint.for_name);
      if has_explicit_spacing_child {
        continue;
      }
      if constraint.target == dgm::ConstraintValues::Width {
        space_width = node.width * constraint.factor;
      }
      if constraint.target == dgm::ConstraintValues::Height {
        space_height = node.height * constraint.factor;
      }
    }
  }
  let mut shrink_names: HashSet<String> = rules
    .iter()
    .filter(|rule| !rule.for_name.is_empty())
    .map(|rule| rule.for_name.clone())
    .collect();
  if !horizontal {
    shrink_names.clear();
  }
  if shrink_names.is_empty() {
    node.children.retain(|child| !child.is_empty_group());
  }

  let mut count = node.children.len() as f32;
  if !shrink_names.is_empty() {
    let mut shrink_dependencies = HashSet::new();
    for child in &node.children {
      if shrink_names.contains(child.internal_name.as_str()) {
        continue;
      }
      if count > 1.0 {
        count -= 1.0;
        let mut dependency = false;
        let mut factor = 0.0;
        for constraint in constraints
          .iter()
          .filter(|constraint| constraint.for_name == child.internal_name)
        {
          if horizontal && constraint.target != dgm::ConstraintValues::Width {
            continue;
          }
          if !horizontal && constraint.target != dgm::ConstraintValues::Height {
            continue;
          }
          factor = constraint.factor;
          if !shrink_names.contains(constraint.ref_for_name.as_str()) {
            continue;
          }
          count += constraint.factor;
          shrink_dependencies.insert(child.internal_name.clone());
          dependency = true;
          break;
        }
        if !dependency && child.is_empty_group() {
          let scale_down_empty_spacing = if horizontal {
            properties
              .get(child.internal_name.as_str())
              .and_then(|properties| properties.get(&dgm::ConstraintValues::Width))
              .is_some_and(|width| *width > 0.0)
          } else {
            properties
              .get(child.internal_name.as_str())
              .and_then(|properties| properties.get(&dgm::ConstraintValues::Height))
              .is_some_and(|height| *height > 0.0)
          };
          if scale_down_empty_spacing {
            count += factor;
            shrink_dependencies.insert(child.internal_name.clone());
          }
        }
      }
    }
    shrink_names.extend(shrink_dependencies);
    space_width = 0.0;
    space_height = 0.0;
  }

  let count = count.max(1.0);
  let base_width = if horizontal {
    node.width / count
  } else {
    node.width
  };
  let base_height = if horizontal {
    node.height
  } else {
    node.height / count
  };
  let mut total_primary = 0.0;
  for child in &node.children {
    let child_properties = properties.get(child.internal_name.as_str());
    let width = child_properties
      .and_then(|properties| properties.get(&dgm::ConstraintValues::Width))
      .copied()
      .unwrap_or(base_width);
    let height = child_properties
      .and_then(|properties| properties.get(&dgm::ConstraintValues::Height))
      .copied()
      .unwrap_or(base_height);
    total_primary += if horizontal { width } else { height };
  }
  total_primary += if horizontal {
    (count - 1.0).max(0.0) * space_width
  } else {
    (count - 1.0).max(0.0) * space_height
  };
  let width_scale = if horizontal && total_primary > node.width {
    node.width / total_primary
  } else {
    1.0
  };
  let height_scale = if !horizontal && total_primary > node.height {
    node.height / total_primary
  } else {
    1.0
  };
  space_width *= width_scale;
  space_height *= height_scale;
  let mut cursor = if reverse {
    if horizontal { node.width } else { node.height }
  } else {
    0.0
  };
  for child in &mut node.children {
    let child_properties = properties.get(child.internal_name.as_str());
    let mut width = child_properties
      .and_then(|properties| properties.get(&dgm::ConstraintValues::Width))
      .copied()
      .unwrap_or(base_width);
    let mut height = child_properties
      .and_then(|properties| properties.get(&dgm::ConstraintValues::Height))
      .copied()
      .unwrap_or(base_height);
    if shrink_names.is_empty() || shrink_names.contains(child.internal_name.as_str()) {
      width *= width_scale;
      height *= height_scale;
    }
    if horizontal {
      let x = if reverse {
        cursor -= width;
        cursor
      } else {
        let x = cursor;
        cursor += width;
        x
      };
      child.x = x.max(0.0);
      child.y = ((node.height - height) / 2.0).max(0.0);
      child.width = width;
      child.height = height;
      if child.is_connector {
        child.connector_angle_deg = connector_angle;
      }
      if reverse {
        cursor -= space_width;
      } else {
        cursor += space_width;
      }
    } else {
      let y = if reverse {
        cursor -= height;
        cursor
      } else {
        let y = cursor;
        cursor += height;
        y
      };
      child.x = ((node.width - width) / 2.0).max(0.0);
      child.y = y.max(0.0);
      child.width = width;
      child.height = height;
      if child.is_connector {
        child.connector_angle_deg = connector_angle;
      }
      if reverse {
        cursor -= space_height;
      } else {
        cursor += space_height;
      }
    }
  }
  if node.child_order == dgm::ChildOrderValues::Top {
    node.children.reverse();
  }
}

fn cycle_layout_tree(node: &mut DiagramShapeNode, algorithm: LayoutAlgorithm) {
  if node.children.is_empty() {
    return;
  }
  let center_x = lo_i32(node.width / 2.0);
  let center_y = lo_i32(node.height / 2.0);
  let child_width = lo_i32(node.width / 4.0);
  let child_height = lo_i32(node.height / 4.0);
  let connector_width = lo_i32(node.width / 12.0);
  let connector_height = lo_i32(node.height / 12.0);
  let radius = lo_i32(((node.width - child_width) / 2.0).min((node.height - child_height) / 2.0));
  let mut start = 0usize;
  if algorithm.center_shape_mapping_first_node
    && let Some(center) = node.children.first_mut()
  {
    center.x = center_x - lo_i32(child_width / 2.0);
    center.y = center_y - lo_i32(child_height / 2.0);
    center.width = child_width;
    center.height = child_height;
    start = 1;
  }
  let count = node.children.len().saturating_sub(start);
  if count == 0 {
    return;
  }
  let connector_radius = lo_i32(
    radius
      * ((algorithm.span_angle as i32 / count as i32) as f32)
        .to_radians()
        .cos(),
  );
  let connector_angle = if algorithm.span_angle > 0.0 {
    0.0
  } else {
    180.0
  };
  for (index, child) in node.children.iter_mut().skip(start).enumerate() {
    let angle = (index as f32) * algorithm.span_angle / count as f32 + algorithm.start_angle;
    let radians = angle.to_radians();
    let (width, height, current_radius) = if child.is_connector {
      (connector_width, connector_height, connector_radius)
    } else {
      (child_width, child_height, radius)
    };
    child.x = lo_i32(center_x + current_radius * radians.sin() - lo_i32(width / 2.0));
    child.y = lo_i32(center_y + current_radius * radians.cos() - lo_i32(height / 2.0));
    child.width = width;
    child.height = height;
    if algorithm.rotation_path_along_path {
      child.shape_rotation_deg = angle;
    }
    if child.is_connector {
      child.shape_rotation_deg = connector_angle + angle;
      child.connector_angle_deg = connector_angle + angle;
    }
  }
}

fn connector_layout_tree(node: &mut DiagramShapeNode) {
  if !node.is_connector {
    return;
  }
  let mut properties = HashMap::from([(
    String::new(),
    HashMap::from([
      (dgm::ConstraintValues::Width, node.width),
      (dgm::ConstraintValues::Height, node.height),
      (dgm::ConstraintValues::Left, 0.0),
      (dgm::ConstraintValues::Top, 0.0),
      (dgm::ConstraintValues::Right, node.width),
      (dgm::ConstraintValues::Bottom, node.height),
    ]),
  )]);
  for constraint in &node.direct_constraints {
    let Some(reference_properties) = properties.get(constraint.ref_for_name.as_str()) else {
      continue;
    };
    let Some(reference) = reference_properties.get(&constraint.reference).copied() else {
      continue;
    };
    properties
      .entry(constraint.for_name.clone())
      .or_default()
      .insert(constraint.target, reference * constraint.factor);
  }
  let Some(parent_properties) = properties.get("") else {
    return;
  };
  let width = parent_properties
    .get(&dgm::ConstraintValues::Width)
    .copied()
    .unwrap_or(node.width);
  let height = parent_properties
    .get(&dgm::ConstraintValues::Height)
    .copied()
    .unwrap_or(node.height);
  node.x += (node.width - width) / 2.0;
  node.y += (node.height - height) / 2.0;
  node.width = width;
  node.height = height;
}

fn pyramid_layout_tree(node: &mut DiagramShapeNode) {
  if node.children.is_empty() || node.width == 0.0 || node.height == 0.0 {
    return;
  }
  let count = node.children.len();
  let aspect_ratio = 0.32;
  let mut child_width = node.width / count as f32;
  let child_height = node.height / count as f32;
  let mut x = aspect_ratio * child_width * (count - 1) as f32;
  let mut y = aspect_ratio * child_height;
  for child in &mut node.children {
    child.x = x;
    child.y = y;
    if count > 1 {
      x -= child_height / (count - 1) as f32;
    }
    child_width += child_height;
    child.width = child_width;
    child.height = child_height;
    y += child_height;
  }
}

fn hierarchy_layout_tree(node: &mut DiagramShapeNode, algorithm: LayoutAlgorithm) {
  if node.children.is_empty() || node.width == 0.0 || node.height == 0.0 {
    return;
  }

  let direction = if algorithm.kind == dgm::AlgorithmValues::HierarchyRoot {
    LinearDirection::Top
  } else {
    algorithm.linear_direction
  };
  let mut count = node.children.len();
  if algorithm.kind == dgm::AlgorithmValues::HierarchyChild {
    count = node
      .children
      .iter()
      .filter(|child| !child.is_connector)
      .count();
  }
  if count == 0 {
    return;
  }

  let space_width = 0.1;
  let space_height = 0.3;
  if algorithm.kind == dgm::AlgorithmValues::HierarchyRoot && count == 3 {
    let assistant_index = node
      .children
      .iter()
      .position(|child| child_contains_data_node_type(child, dgm::ElementValues::Assistant));
    if assistant_index == Some(2)
      && !child_contains_data_node_type(&node.children[1], dgm::ElementValues::Assistant)
    {
      node.children.swap(1, 2);
    }
  }

  let horizontal_shapes_count = if algorithm.secondary_linear_direction == LinearDirection::Top {
    2
  } else if matches!(direction, LinearDirection::Left | LinearDirection::Right) {
    count
  } else {
    1
  };
  let vertical_count = vertical_shapes_count(node).max(1);
  let mut child_width = lo_i32(
    node.width
      / (horizontal_shapes_count as f32 + (horizontal_shapes_count - 1) as f32 * space_width),
  );
  let child_height =
    lo_i32(node.height / (vertical_count as f32 + (vertical_count - 1) as f32 * space_height));
  let connector_width = 1.0;
  let connector_height = child_height;
  let mut x = 0.0;
  let mut y = 0.0;
  if algorithm.kind == dgm::AlgorithmValues::HierarchyChild && horizontal_shapes_count == 1 {
    let child_indent = 0.1;
    x = lo_i32(child_width * child_indent);
    child_width = lo_i32(child_width * (1.0 - 2.0 * child_indent));
  }

  let mut index = 0usize;
  let mut row_height = 0.0_f32;
  for child in &mut node.children {
    child.x = x;
    child.y = y;
    if algorithm.kind == dgm::AlgorithmValues::HierarchyChild && child.is_connector {
      child.width = connector_width;
      child.height = connector_height;
      continue;
    }

    let child_vertical_count = vertical_shapes_count(child).max(1);
    let height = lo_i32(
      child_height
        * (child_vertical_count as f32 + (child_vertical_count - 1) as f32 * space_height),
    );
    child.width = child_width;
    child.height = height;

    if matches!(direction, LinearDirection::Top | LinearDirection::Bottom) {
      y += lo_i32(height + child_height * space_height);
    } else {
      x += lo_i32(child_width + child_width * space_width);
    }
    row_height = row_height.max(height);

    if algorithm.secondary_linear_direction == LinearDirection::Top && index % 2 == 1 {
      x = 0.0;
      y += lo_i32(row_height + child_height * space_height);
      row_height = 0.0;
    }
    index += 1;
  }
}

fn vertical_shapes_count(node: &DiagramShapeNode) -> usize {
  let Some(algorithm) = node.algorithms.last().copied() else {
    return if node.is_connector { 0 } else { 1 };
  };
  if node.children.is_empty() {
    return if node.is_connector { 0 } else { 1 };
  }
  let direction = if algorithm.kind == dgm::AlgorithmValues::HierarchyRoot {
    LinearDirection::Top
  } else {
    algorithm.linear_direction
  };
  if matches!(direction, LinearDirection::Top | LinearDirection::Bottom) {
    node.children.iter().map(vertical_shapes_count).sum()
  } else if matches!(direction, LinearDirection::Left | LinearDirection::Right)
    && algorithm.secondary_linear_direction == LinearDirection::Top
  {
    node
      .children
      .iter()
      .map(vertical_shapes_count)
      .sum::<usize>()
      .div_ceil(2)
  } else {
    node
      .children
      .iter()
      .map(vertical_shapes_count)
      .max()
      .unwrap_or(1)
  }
}

fn child_contains_data_node_type(
  node: &DiagramShapeNode,
  data_node_type: dgm::ElementValues,
) -> bool {
  node.data_node_type == Some(data_node_type)
    || node
      .children
      .iter()
      .any(|child| child_contains_data_node_type(child, data_node_type))
}

fn snake_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
) {
  if node.children.is_empty() || node.width == 0.0 || node.height == 0.0 {
    return;
  }

  let child_aspect_ratio = node
    .children
    .first()
    .map(|child| child.aspect_ratio)
    .unwrap_or(0.0);
  let mut shape_width = node.width;
  let shape_height = node.height;
  if child_aspect_ratio != 0.0
    && shape_height != 0.0
    && child_aspect_ratio < shape_width / shape_height
  {
    shape_width = shape_height * child_aspect_ratio;
  }

  let mut space_from_constraint = 1.0;
  let mut properties_by_name: HashMap<String, HashMap<dgm::ConstraintValues, f32>> = HashMap::new();
  let mut properties_by_type: HashMap<dgm::ElementValues, HashMap<dgm::ConstraintValues, f32>> =
    HashMap::new();
  properties_by_name.insert(
    String::new(),
    HashMap::from([
      (dgm::ConstraintValues::Width, shape_width),
      (dgm::ConstraintValues::Height, shape_height),
    ]),
  );
  for constraint in constraints {
    if matches!(
      constraint.reference,
      dgm::ConstraintValues::Width | dgm::ConstraintValues::Height
    ) && constraint.target == dgm::ConstraintValues::Spacing
      && constraint.for_name.is_empty()
    {
      space_from_constraint = constraint.factor;
    }
    let Some(reference_properties) = properties_by_name.get(constraint.ref_for_name.as_str())
    else {
      continue;
    };
    let Some(reference) = reference_properties.get(&constraint.reference) else {
      continue;
    };
    if constraint.value != 0.0 {
      continue;
    }
    let value = lo_i32(reference * constraint.factor);
    if let Some(point_type) = constraint.point_type {
      properties_by_type
        .entry(point_type)
        .or_default()
        .insert(constraint.target, value);
    } else {
      properties_by_name
        .entry(constraint.for_name.clone())
        .or_default()
        .insert(constraint.target, value);
    }
  }

  let shape_widths: Vec<f32> = node
    .children
    .iter()
    .map(|child| {
      child
        .data_node_type
        .and_then(|data_node_type| properties_by_type.get(&data_node_type))
        .and_then(|properties| properties.get(&dgm::ConstraintValues::Width))
        .copied()
        .unwrap_or(shape_width)
    })
    .collect();

  let space_from_constraints = space_from_constraint != 1.0;
  let mut horizontal = true;
  let (increment_x, increment_y) = match algorithm.grow_direction {
    GrowDirection::TopLeft => (1.0, 1.0),
    GrowDirection::TopRight => (-1.0, 1.0),
    GrowDirection::BottomLeft => {
      horizontal = false;
      (1.0, -1.0)
    }
    GrowDirection::BottomRight => {
      horizontal = false;
      (-1.0, -1.0)
    }
  };

  let count = node.children.len();
  let space = if space_from_constraints {
    space_from_constraint
  } else {
    0.3
  };
  let grid_aspect_ratio = 0.54;
  let mut columns = 1usize;
  let mut rows = 1usize;
  let mut max_row_width = 0.0_f32;
  if child_aspect_ratio != 0.0 && count as f32 <= child_aspect_ratio {
    rows = count;
  } else {
    for candidate_rows in 1..count {
      let candidate_columns = count.div_ceil(candidate_rows);
      rows = candidate_rows;
      columns = candidate_columns;
      let row_width: f32 = shape_widths.iter().take(candidate_columns).sum();
      if row_width != 0.0 && shape_height * candidate_rows as f32 / row_width >= grid_aspect_ratio {
        max_row_width = row_width;
        break;
      }
    }
  }

  let mut child_width = lo_i32(node.width / (columns as f32 + (columns - 1) as f32 * space));
  let mut child_height = lo_i32(child_width * grid_aspect_ratio);
  if columns == 1 && rows > 1 {
    let mut num_spaces = -1.0;
    if space_from_constraints {
      num_spaces += 4.0;
    }
    child_height = lo_i32(node.height / (rows as f32 + (rows as f32 + num_spaces) * space));
    if child_aspect_ratio > 1.0 {
      child_width = node.width.min(lo_i32(child_height * child_aspect_ratio));
    }
    horizontal = false;
  }

  let mut x = if increment_x == -1.0 {
    node.width - child_width
  } else {
    0.0
  };
  let mut y = if increment_y == -1.0 {
    node.height - child_height
  } else if space_from_constraints && !horizontal {
    child_height * space * 2.0
  } else {
    0.0
  };
  let start_x = x;
  let mut column_index = 0usize;
  let mut row_height = 0.0_f32;
  let widths_from_constraints = count >= 2
    && node.children.get(1).and_then(|child| child.data_node_type)
      == Some(dgm::ElementValues::SiblingTransition);
  for (index, child) in node.children.iter_mut().enumerate() {
    child.x = x;
    child.y = y;
    let mut current_width = child_width;
    let mut current_height = child_height;
    if widths_from_constraints && max_row_width != 0.0 {
      current_width = lo_i32(node.width * shape_widths[index] / max_row_width);
    }
    if child_aspect_ratio != 0.0 {
      current_height = lo_i32(current_width / child_aspect_ratio).min(lo_i32(
        node.height / (rows as f32 + (rows - 1) as f32 * space),
      ));
    }
    row_height = row_height.max(current_height);
    child.width = current_width;
    child.height = current_height;

    let placed = index + 1;
    match algorithm.continue_direction {
      ContinueDirection::SameDirection => {
        if placed % columns == 0 || placed / columns + 1 != rows {
          x += increment_x * lo_i32(current_width + space * current_width);
        }
        column_index += 1;
        if column_index == columns {
          if (placed + 1) % columns != 0
            && placed + 1 >= 3
            && (placed + 1) / columns + 1 == rows
            && count != rows * columns
          {
            x = if widths_from_constraints {
              start_x
            } else {
              start_x + lo_i32(increment_x * (current_width + space * current_width)) / 2.0
            };
          } else {
            x = start_x;
          }
          y += increment_y * lo_i32(row_height + space * row_height);
          column_index = 0;
          row_height = 0.0;
        }
        if placed % columns != 0 && placed >= 3 && placed / columns + 1 == rows {
          x += increment_x * lo_i32(current_width + space * current_width);
        }
      }
      ContinueDirection::ReverseDirection => {
        if (placed % columns == 0 || placed / columns + 1 != rows)
          && !(placed / columns + 1).is_multiple_of(2)
        {
          x += lo_i32(current_width + space * current_width);
        } else if placed % columns != 0 && placed / columns + 1 != rows {
          x -= lo_i32(current_width + space * current_width);
        }
        column_index += 1;
        if column_index == columns {
          if (placed + 1) % columns != 0
            && placed + 1 >= 4
            && (placed + 1) / columns + 1 == rows
            && count != rows * columns
            && (placed / columns + 1).is_multiple_of(2)
          {
            x -= current_width * 3.0 / 2.0;
          } else if (placed + 1) % columns != 0
            && placed + 1 >= 4
            && (placed + 1) / columns + 1 == rows
            && count != rows * columns
            && !(placed / columns + 1).is_multiple_of(2)
          {
            x = start_x + lo_i32(increment_x * (current_width + space * current_width)) / 2.0;
          } else if !(placed / columns + 1).is_multiple_of(2) {
            x = start_x;
          }
          y += increment_y * lo_i32(child_height + space * child_height);
          column_index = 0;
        }
        if placed % columns != 0
          && placed >= 3
          && placed / columns + 1 == rows
          && (placed / columns + 1).is_multiple_of(2)
        {
          x -= increment_x * lo_i32(current_width + space * current_width);
        } else if placed % columns != 0
          && placed >= 3
          && placed / columns + 1 == rows
          && !(placed / columns + 1).is_multiple_of(2)
        {
          x += increment_x * lo_i32(current_width + space * current_width);
        }
      }
    }
  }
}

fn lo_i32(value: f32) -> f32 {
  (value as i32) as f32
}

fn sort_diagram_shape_children_by_z_order(node: &mut DiagramShapeNode) {
  let mut z_orders: Vec<i32> = (0..node.children.len()).map(|index| index as i32).collect();
  for index in 0..node.children.len() {
    let offset = node.children[index].z_order_offset;
    if offset <= 0 {
      continue;
    }
    z_orders[index] += offset;
    for next in 0..offset as usize {
      let next_index = index + next + 1;
      if next_index >= z_orders.len() {
        break;
      }
      z_orders[next_index] -= 1;
    }
    node.children[index].z_order_offset = 0;
  }
  let mut indexed_children: Vec<_> = node.children.drain(..).enumerate().collect();
  indexed_children.sort_by_key(|(index, _)| z_orders[*index]);
  node.children = indexed_children
    .into_iter()
    .map(|(_, mut child)| {
      sort_diagram_shape_children_by_z_order(&mut child);
      child
    })
    .collect();
}

fn flatten_diagram_shape_tree(
  node: &DiagramShapeNode,
  offset_x: f32,
  offset_y: f32,
  shapes: &mut Vec<DiagramShape>,
) {
  let x = offset_x + node.x;
  let y = offset_y + node.y;
  let draw_geometry = node.has_geometry && !node.hidden_geometry;
  if draw_geometry || !node.text_body.is_empty() {
    shapes.push(DiagramShape {
      x,
      y,
      width: node.width,
      height: node.height,
      text_body: node.text_body.clone(),
      preset_geometry: node.preset_geometry.clone(),
      shape_properties: node.shape_properties.clone(),
      style: node.style.clone(),
      line_fill: node.line_fill,
      text_fill: node.text_fill,
      shape_rotation_deg: shape_rotation_degrees(node),
      text_rotation_deg: node.text_rotation_deg,
      draw_geometry,
      is_connector: node.is_connector,
      connector_angle_deg: node.connector_angle_deg,
      is_blip_placeholder: node.is_blip_placeholder,
      fill: node.fill,
      text_order: node.text_order,
      font_size_pt: node.font_size_pt,
      minimum_font_size_pt: node.minimum_font_size_pt,
      font_sync_group: node.font_sync_group.clone(),
    });
  }
  for child in &node.children {
    flatten_diagram_shape_tree(child, x, y, shapes);
  }
}
