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
use crate::model::common_point;
use crate::render::math::text_math_text;

// LibreOffice DiagramLayoutAtom::layoutShape() synthesizes this DrawingML
// hanging indent when stBulletLvl turns a SmartArt tx paragraph into a bullet.
const SMARTART_TX_BULLET_INDENT_EMU: i32 = 285_750;
// Microsoft's SmartArt layout documentation defines the default secondary
// font size used by bulleted lines as 78 percent of the primary font size.
const SMARTART_DEFAULT_SECONDARY_FONT_SCALE: f32 = 0.78;
// DrawingML text layout uses a 120 percent line box when the paragraph does
// not provide an explicit line-spacing value.
const SMARTART_DEFAULT_LINE_HEIGHT_SCALE: f32 = 1.2;
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
  pub connector_route: DiagramConnectorRoute,
  pub connector_dimension: dgm::ConnectorDimensionValues,
  pub connector_bend_at_end: bool,
  pub connector_begin_arrow: bool,
  pub connector_end_arrow: bool,
  pub connector_begin_points: Option<String>,
  pub connector_end_points: Option<String>,
  pub connector_beginning_padding: f32,
  pub connector_end_padding: f32,
  pub connector_bending_distance: f32,
  pub connector_start_override: Option<(f32, f32)>,
  pub connector_end_override: Option<(f32, f32)>,
  pub is_blip_placeholder: bool,
  pub fill: RgbColor,
  pub text_order: usize,
  pub font_size_pt: Option<f32>,
  pub minimum_font_size_pt: Option<f32>,
  pub font_sync_group: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiagramConnectorRoute {
  #[default]
  Straight,
  Bend,
  Curve,
  LongCurve,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ConnectorPointSet {
  #[default]
  Auto,
  TopCenter,
  BottomCenter,
  Center,
  MiddleLeft,
  MiddleRight,
  MiddleLeftOrRight,
  BottomLeft,
  BottomRight,
  TopLeft,
  TopRight,
  Radial,
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

  pub(crate) fn connector_commands(&self) -> Vec<crate::common::PathCommand> {
    let center_x = self.x + self.width / 2.0;
    let center_y = self.y + self.height / 2.0;
    let length = self.width.max(self.height).max(1.0);
    let radians = self.connector_angle_deg.to_radians();
    let delta = (radians.cos() * length / 2.0, radians.sin() * length / 2.0);
    let automatic_start = (center_x - delta.0, center_y - delta.1);
    let automatic_end = (center_x + delta.0, center_y + delta.1);
    let start = self.connector_start_override.unwrap_or_else(|| {
      diagram_connector_point(
        self.connector_begin_points.as_deref(),
        self,
        automatic_start,
        delta.0 >= 0.0,
      )
    });
    let end = self.connector_end_override.unwrap_or_else(|| {
      diagram_connector_point(
        self.connector_end_points.as_deref(),
        self,
        automatic_end,
        delta.0 < 0.0,
      )
    });
    let (start, end) = padded_connector_segment(
      start,
      end,
      self.connector_beginning_padding,
      self.connector_end_padding,
    );
    let mut commands = vec![crate::common::PathCommand::MoveTo(common_point(
      start.0, start.1,
    ))];
    match self.connector_route {
      DiagramConnectorRoute::Straight => {
        commands.push(crate::common::PathCommand::LineTo(common_point(
          end.0, end.1,
        )));
      }
      DiagramConnectorRoute::Bend => {
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;
        let authored_distance = self.connector_bending_distance.min(dx.abs().max(dy.abs()));
        let (bend1, bend2) = if authored_distance > f32::EPSILON && dy.abs() >= dx.abs() {
          // ECMA-376 defines bendDist from the connector's beginning,
          // independent of whether the routing algorithm chooses the
          // beginning or ending side as its preferred bend point.
          let bend_y = start.1 + dy.signum() * authored_distance;
          ((start.0, bend_y), (end.0, bend_y))
        } else if authored_distance > f32::EPSILON {
          let bend_x = start.0 + dx.signum() * authored_distance;
          ((bend_x, start.1), (bend_x, end.1))
        } else if self.connector_bend_at_end {
          ((start.0, end.1), (start.0, end.1))
        } else {
          ((end.0, start.1), (end.0, start.1))
        };
        if (bend1.0 - start.0).abs() > f32::EPSILON || (bend1.1 - start.1).abs() > f32::EPSILON {
          commands.push(crate::common::PathCommand::LineTo(common_point(
            bend1.0, bend1.1,
          )));
        }
        if (bend2.0 - bend1.0).abs() > f32::EPSILON || (bend2.1 - bend1.1).abs() > f32::EPSILON {
          commands.push(crate::common::PathCommand::LineTo(common_point(
            bend2.0, bend2.1,
          )));
        }
        commands.push(crate::common::PathCommand::LineTo(common_point(
          end.0, end.1,
        )));
      }
      DiagramConnectorRoute::Curve => {
        let control1 = if self.connector_bend_at_end {
          (start.0, (start.1 + end.1) / 2.0)
        } else {
          ((start.0 + end.0) / 2.0, start.1)
        };
        let control2 = if self.connector_bend_at_end {
          (end.0, (start.1 + end.1) / 2.0)
        } else {
          ((start.0 + end.0) / 2.0, end.1)
        };
        commands.push(crate::common::PathCommand::CubicTo {
          control1: common_point(control1.0, control1.1),
          control2: common_point(control2.0, control2.1),
          end: common_point(end.0, end.1),
        });
      }
      DiagramConnectorRoute::LongCurve => {
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;
        let distance = dx.hypot(dy).max(1.0);
        let authored_sweep = self.connector_bending_distance.abs();
        let sweep = authored_sweep.max(distance / 2.0);
        let side = if self.connector_bend_at_end {
          1.0
        } else {
          -1.0
        };
        let normal = (-dy / distance * sweep * side, dx / distance * sweep * side);
        commands.push(crate::common::PathCommand::CubicTo {
          control1: common_point(start.0 + dx / 3.0 + normal.0, start.1 + dy / 3.0 + normal.1),
          control2: common_point(
            start.0 + dx * 2.0 / 3.0 + normal.0,
            start.1 + dy * 2.0 / 3.0 + normal.1,
          ),
          end: common_point(end.0, end.1),
        });
      }
    }
    commands
  }

  pub(crate) fn apply_connector_ends(&self, stroke: &mut crate::common::Stroke<'static>) {
    let arrow = crate::common::StrokeEnd {
      kind: crate::common::StrokeEndKind::Triangle,
      width: crate::common::StrokeEndSize::Medium,
      length: crate::common::StrokeEndSize::Medium,
    };
    if self.connector_begin_arrow {
      stroke.head_end = Some(arrow);
    }
    if self.connector_end_arrow {
      stroke.tail_end = Some(arrow);
    }
  }
}

fn padded_connector_segment(
  start: (f32, f32),
  end: (f32, f32),
  beginning_padding: f32,
  end_padding: f32,
) -> ((f32, f32), (f32, f32)) {
  let dx = end.0 - start.0;
  let dy = end.1 - start.1;
  let length = dx.hypot(dy);
  if length <= f32::EPSILON {
    return (start, end);
  }
  let beginning_padding = beginning_padding.max(0.0).min(length);
  let end_padding = end_padding.max(0.0).min(length - beginning_padding);
  let unit = (dx / length, dy / length);
  (
    (
      start.0 + unit.0 * beginning_padding,
      start.1 + unit.1 * beginning_padding,
    ),
    (end.0 - unit.0 * end_padding, end.1 - unit.1 * end_padding),
  )
}

fn diagram_connector_point(
  authored: Option<&str>,
  shape: &DiagramShape,
  automatic: (f32, f32),
  prefer_left: bool,
) -> (f32, f32) {
  match authored {
    Some("tCtr") => (shape.x + shape.width / 2.0, shape.y),
    Some("bCtr") => (shape.x + shape.width / 2.0, shape.y + shape.height),
    Some("ctr") => (shape.x + shape.width / 2.0, shape.y + shape.height / 2.0),
    Some("midL") => (shape.x, shape.y + shape.height / 2.0),
    Some("midR") => (shape.x + shape.width, shape.y + shape.height / 2.0),
    Some("midL midR") if prefer_left => (shape.x, shape.y + shape.height / 2.0),
    Some("midL midR") => (shape.x + shape.width, shape.y + shape.height / 2.0),
    Some("bL") => (shape.x, shape.y + shape.height),
    Some("bR") => (shape.x + shape.width, shape.y + shape.height),
    Some("tL") => (shape.x, shape.y),
    Some("tR") => (shape.x + shape.width, shape.y),
    Some("radial") | None => automatic,
    Some(_) => automatic,
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

  fn direct_primary_font_size_pt(&self) -> Option<f32> {
    self
      .paragraphs
      .iter()
      .flat_map(|paragraph| &paragraph.runs)
      .filter_map(|run| {
        run
          .run_properties
          .as_ref()
          .and_then(|properties| properties.font_size)
      })
      .max()
      .map(|font_size| font_size as f32 / 100.0)
  }

  fn apply_text_margins(
    &mut self,
    shape_width_pt: f32,
    shape_height_pt: f32,
    primary_font_size_pt: Option<f32>,
    data_node_type: Option<dgm::ElementValues>,
    constraints: &[DiagramConstraint],
  ) {
    for constraint in constraints {
      if !constraint.for_name.is_empty()
        || constraint
          .point_type
          .is_some_and(|point_type| !diagram_element_type_matches(data_node_type, point_type))
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
        dgm::ConstraintValues::None if constraint.has_value => {
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

  fn set_horizontal_anchor_center(&mut self, centered: bool) {
    let mut body_properties = self.body_properties.clone().unwrap_or_default();
    body_properties.anchor_center = Some(centered.into());
    self.body_properties = Some(body_properties);
  }

  fn has_child_text(&self) -> bool {
    let Some(base_level) = self
      .paragraphs
      .iter()
      .filter_map(|paragraph| paragraph.level)
      .min()
    else {
      return false;
    };
    self
      .paragraphs
      .iter()
      .filter_map(|paragraph| paragraph.level)
      .any(|level| level > base_level)
  }

  fn is_right_to_left(&self) -> bool {
    self.paragraphs.iter().any(|paragraph| {
      paragraph
        .paragraph_properties
        .as_deref()
        .and_then(|properties| properties.right_to_left)
        .is_some_and(|value| value.as_bool())
    })
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

  fn apply_algorithm_spacing(&mut self, algorithm: &LayoutAlgorithm) {
    let Some(base_level) = self
      .paragraphs
      .iter()
      .filter_map(|paragraph| paragraph.level)
      .min()
    else {
      return;
    };
    for paragraph in &mut self.paragraphs {
      let child = paragraph.level.unwrap_or(base_level) > base_level;
      let line_spacing = if child {
        algorithm.line_spacing_children
      } else {
        algorithm.line_spacing_parent
      };
      let space_after = if child {
        algorithm.line_spacing_after_children
      } else {
        algorithm.line_spacing_after_parent
      };
      if line_spacing.is_none() && space_after.is_none() {
        continue;
      }
      let mut properties = paragraph.paragraph_properties.clone().unwrap_or_default();
      if let Some(value) = line_spacing {
        properties.line_spacing = Some(Box::new(a::LineSpacing {
          line_spacing_choice: Some(a::LineSpacingChoice::SpacingPercent(a::SpacingPercent {
            val: ooxmlsdk::units::DrawingmlPercentageValue::Decimal(
              (value * 1_000.0).round().clamp(0.0, 13_200_000.0) as i32,
            ),
          })),
        }));
      }
      if let Some(value) = space_after {
        properties.space_after = Some(Box::new(a::SpaceAfter {
          space_after_choice: Some(a::SpaceAfterChoice::SpacingPercent(a::SpacingPercent {
            val: ooxmlsdk::units::DrawingmlPercentageValue::Decimal(
              (value * 1_000.0).round().clamp(0.0, 13_200_000.0) as i32,
            ),
          })),
        }));
      }
      paragraph.paragraph_properties = Some(properties);
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
  connector_route: DiagramConnectorRoute,
  connector_dimension: dgm::ConnectorDimensionValues,
  connector_bend_at_end: bool,
  connector_begin_arrow: bool,
  connector_end_arrow: bool,
  connector_begin_points: Option<String>,
  connector_end_points: Option<String>,
  connector_beginning_padding: f32,
  connector_end_padding: f32,
  connector_bending_distance: f32,
  connector_source_node: Option<String>,
  connector_destination_node: Option<String>,
  connector_route_shortest_distance: bool,
  connector_start_override: Option<(f32, f32)>,
  connector_end_override: Option<(f32, f32)>,
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
  placeholder_line_count: usize,
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
  has_value: bool,
  target: dgm::ConstraintValues,
  reference: dgm::ConstraintValues,
  relationship: Option<dgm::ConstraintRelationshipValues>,
  reference_relationship: Option<dgm::ConstraintRelationshipValues>,
  operator: Option<dgm::BoolOperatorValues>,
  point_type: Option<dgm::ElementValues>,
  reference_point_type: Option<dgm::ElementValues>,
}

#[derive(Clone, Debug)]
struct DiagramRule {
  for_name: String,
  target: dgm::ConstraintValues,
  point_type: Option<dgm::ElementValues>,
  value: f32,
}

#[derive(Clone, Debug)]
struct LayoutAlgorithm {
  kind: dgm::AlgorithmValues,
  linear_direction: LinearDirection,
  secondary_linear_direction: LinearDirection,
  child_direction: Option<dgm::ChildDirectionValues>,
  child_alignment: Option<ChildAlignment>,
  secondary_child_alignment: Option<ChildAlignment>,
  horizontal_alignment: Option<AxisAlignment>,
  vertical_alignment: Option<AxisAlignment>,
  node_horizontal_alignment: Option<AxisAlignment>,
  node_vertical_alignment: Option<AxisAlignment>,
  hierarchy_horizontal_alignment: Option<AxisAlignment>,
  hierarchy_vertical_alignment: Option<AxisAlignment>,
  grow_direction: GrowDirection,
  continue_direction: ContinueDirection,
  flow_direction: dgm::FlowDirectionValues,
  breakpoint: dgm::BreakpointValues,
  breakpoint_fixed_value: usize,
  offset: dgm::OffsetValues,
  start_angle: f32,
  span_angle: f32,
  start_element: dgm::StartingElementValues,
  center_shape_mapping_first_node: bool,
  rotation_path_along_path: bool,
  aspect_ratio: Option<f32>,
  auto_text_rotation: Option<dgm::AutoTextRotationValues>,
  text_anchor_horizontal_center: Option<bool>,
  text_anchor_vertical: Option<dgm::TextAnchorVerticalValues>,
  text_anchor_horizontal_with_children_center: Option<bool>,
  text_anchor_vertical_with_children: Option<dgm::TextAnchorVerticalValues>,
  start_bullets_at_level: i32,
  parent_text_left_to_right_alignment: Option<dgm::TextAlignmentValues>,
  parent_text_right_to_left_alignment: Option<dgm::TextAlignmentValues>,
  shape_text_left_to_right_alignment_with_children: Option<dgm::TextAlignmentValues>,
  shape_text_right_to_left_alignment_with_children: Option<dgm::TextAlignmentValues>,
  text_alignment: Option<dgm::TextAlignmentValues>,
  text_direction: dgm::TextDirectionValues,
  text_block_direction: dgm::TextBlockDirectionValues,
  fallback_dimension: dgm::FallbackDimensionValues,
  pyramid_accent_position: dgm::PyramidAccentPositionValues,
  pyramid_accent_text_margin: dgm::PyramidAccentTextMarginValues,
  pyramid_level_node: Option<String>,
  pyramid_accent_background_node: Option<String>,
  pyramid_accent_text_node: Option<String>,
  line_spacing_parent: Option<f32>,
  line_spacing_after_parent: Option<f32>,
  line_spacing_children: Option<f32>,
  line_spacing_after_children: Option<f32>,
  connector_route: DiagramConnectorRoute,
  connector_dimension: dgm::ConnectorDimensionValues,
  connector_bend_at_end: bool,
  connector_begin_arrow: bool,
  connector_end_arrow: bool,
  connector_begin_points: ConnectorPointSet,
  connector_end_points: ConnectorPointSet,
  connector_source_node: Option<String>,
  connector_destination_node: Option<String>,
  connector_route_shortest_distance: bool,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChildAlignment {
  Top,
  Bottom,
  Left,
  Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AxisAlignment {
  Start,
  Center,
  End,
  None,
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

pub fn presentation_point_text_fills(
  data: &dgm::DataModelRoot,
  colors: Option<&DiagramStyleColors>,
) -> HashMap<String, RgbColor> {
  let Some(colors) = colors else {
    return HashMap::new();
  };
  diagram_points(data)
    .into_iter()
    .filter_map(|point| {
      let properties = point.property_set.as_deref()?;
      let label = properties.presentation_style_label.as_deref()?;
      let fills = colors.text_fill_by_label.get(label)?;
      let index = properties
        .presentation_style_index
        .unwrap_or_default()
        .max(0) as usize;
      color_by_index(fills, index).map(|fill| (point.model_id.clone(), fill))
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

fn diagram_sample_preferred_child_count(layout: &dgm::LayoutDefinition) -> usize {
  let Some(model) = layout
    .sample_data
    .as_deref()
    .and_then(|sample| sample.data_model.as_deref())
  else {
    return 0;
  };
  let points: HashMap<&str, dgm::PointValues> = model
    .point_list
    .xml_children
    .iter()
    .filter_map(|child| match child {
      dgm::PointListChoice::Point(point) => {
        Some((point.model_id.as_str(), point.r#type.unwrap_or_default()))
      }
      dgm::PointListChoice::AlternateContent(_) => None,
    })
    .collect();
  let Some(connections) = model.connection_list.as_ref() else {
    return 0;
  };
  let mut child_counts = HashMap::<&str, usize>::new();
  for connection in &connections.connection {
    if connection.r#type.unwrap_or_default() != dgm::ConnectionValues::ParentOf
      || points.get(connection.source_id.as_str()) == Some(&dgm::PointValues::Document)
      || points
        .get(connection.destination_id.as_str())
        .is_none_or(|point_type| *point_type != dgm::PointValues::Node)
    {
      continue;
    }
    *child_counts
      .entry(connection.source_id.as_str())
      .or_default() += 1;
  }
  child_counts.into_values().max().unwrap_or_default()
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
  let placeholder_line_count = diagram_sample_preferred_child_count(layout);
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
    placeholder_line_count,
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
      connector_route: DiagramConnectorRoute::Straight,
      connector_dimension: dgm::ConnectorDimensionValues::OneDimension,
      connector_bend_at_end: true,
      connector_begin_arrow: false,
      connector_end_arrow: false,
      connector_begin_points: None,
      connector_end_points: None,
      connector_beginning_padding: 0.0,
      connector_end_padding: 0.0,
      connector_bending_distance: 0.0,
      connector_source_node: None,
      connector_destination_node: None,
      connector_route_shortest_distance: false,
      connector_start_override: None,
      connector_end_override: None,
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
      placeholder_line_count,
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
  diagram_element_type_matches(point_type_to_element_type(kind), point_type)
}

fn diagram_element_type_matches(
  actual: Option<dgm::ElementValues>,
  requested: dgm::ElementValues,
) -> bool {
  let actual = actual.unwrap_or(dgm::ElementValues::Node);
  match requested {
    dgm::ElementValues::All => true,
    dgm::ElementValues::Node | dgm::ElementValues::Normal => actual == dgm::ElementValues::Node,
    dgm::ElementValues::NonNormal => actual == dgm::ElementValues::Assistant,
    dgm::ElementValues::NonAssistant => actual != dgm::ElementValues::Assistant,
    dgm::ElementValues::Document
    | dgm::ElementValues::Assistant
    | dgm::ElementValues::ParentTransition
    | dgm::ElementValues::Presentation
    | dgm::ElementValues::SiblingTransition => actual == requested,
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
  placeholder_line_count: usize,
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
    let indexed_point = points.get(self.current_index).copied();
    let new_point = indexed_point
      .filter(|point| self.has_connection(self.current_point, point))
      .or_else(|| {
        let current_association = presentation_association_id(self.current_point)?;
        let mut connected = points.iter().copied().filter(|point| {
          self.has_connection(self.current_point, point)
            && presentation_association_id(point) == Some(current_association)
        });
        let point = connected.next()?;
        connected.next().is_none().then_some(point)
      });
    let Some(new_point) = new_point else {
      return;
    };
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
    let requested_count = for_each
      .count
      .as_ref()
      .and_then(|counts| counts.first())
      .map(|count| *count as usize)
      .unwrap_or_default();
    let step = for_each
      .step
      .as_ref()
      .and_then(|steps| steps.first())
      .copied()
      .unwrap_or(1);
    let start = for_each
      .start
      .as_ref()
      .and_then(|starts| starts.first())
      .copied()
      .unwrap_or(1);
    let assistant_indices = (point_type == dgm::ElementValues::Assistant)
      .then(|| self.connected_assistant_template_indices());
    let mut children = 1usize;
    if matches!(
      point_type,
      dgm::ElementValues::Node | dgm::ElementValues::NonAssistant
    ) {
      children = self.shallow_presentation_name_count(&for_each.for_each_choice);
    }
    // LibreOffice's `LayoutAtomVisitorBase::visit(ForEachAtom&)` applies
    // `st` while selecting data through `presOf`, but intentionally starts
    // shape-template traversal at index zero. Some built-in layouts (notably
    // `hList3`) use `st="2"` with a single presentation point named
    // `pillarX`; applying that start to the presentation-name vector drops
    // the second data-bound shape entirely.
    let template_count = if requested_count == 0 {
      children
    } else {
      children.min(requested_count)
    };
    let iteration_count = assistant_indices
      .as_ref()
      .and_then(|indices| indices.last().copied())
      .map(|index| index.saturating_add(1))
      .unwrap_or(template_count);
    let indices = if let Some(assistant_indices) = assistant_indices {
      iterator_indices(assistant_indices.len(), start, requested_count, step)
        .into_iter()
        .filter_map(|index| assistant_indices.get(index).copied())
        .collect()
    } else {
      iterator_indices(template_count, 1, 0, step)
    };
    let old_index = self.current_index;
    let old_step = self.current_step;
    let old_count = self.current_count;
    self.current_step = step.unsigned_abs() as usize;
    self.current_count = iteration_count;
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

  fn connected_assistant_template_indices(&self) -> Vec<usize> {
    let mut indices = Vec::new();
    for points in self.points_by_presentation_name.values() {
      for (index, point) in points.iter().copied().enumerate() {
        if self.has_connection(self.current_point, point)
          && associated_data_point(point, self.point_by_id)
            .is_some_and(|data_point| data_point.r#type == Some(dgm::PointValues::Assistant))
        {
          indices.push(index);
        }
      }
    }
    indices.sort_unstable();
    indices.dedup();
    indices
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
        Some("orgChart") => self.compare_condition(
          branch.operator,
          i32::from(
            self
              .presentation_layout_variables(self.current_point)
              .and_then(|variables| variables.organization_chart.as_ref())
              .and_then(|value| value.val.as_ref())
              .is_some_and(|value| value.as_bool()),
          ),
          parse_boolean_condition_value(branch.val.as_str()),
        ),
        Some("chMax") => self.compare_condition(
          branch.operator,
          self
            .presentation_layout_variables(self.current_point)
            .and_then(|variables| variables.max_number_of_children.as_ref())
            .and_then(|value| value.val)
            .unwrap_or(-1),
          branch.val.parse::<i32>().unwrap_or(-1),
        ),
        Some("chPref") => self.compare_condition(
          branch.operator,
          self
            .presentation_layout_variables(self.current_point)
            .and_then(|variables| variables.preferred_number_of_children.as_ref())
            .and_then(|value| value.val)
            .unwrap_or(-1),
          branch.val.parse::<i32>().unwrap_or(-1),
        ),
        Some("bulletEnabled") => self.compare_condition(
          branch.operator,
          i32::from(
            self
              .presentation_layout_variables(self.current_point)
              .and_then(|variables| variables.bullet_enabled.as_ref())
              .and_then(|value| value.val.as_ref())
              .is_some_and(|value| value.as_bool()),
          ),
          parse_boolean_condition_value(branch.val.as_str()),
        ),
        Some("animOne") => self.compare_condition(
          branch.operator,
          animate_one_condition_value(
            self
              .presentation_layout_variables(self.current_point)
              .and_then(|variables| variables.animate_one_by_one.as_ref())
              .and_then(|value| value.val)
              .unwrap_or_default(),
          ),
          parse_animate_one_condition_value(branch.val.as_str()),
        ),
        Some("animLvl") => self.compare_condition(
          branch.operator,
          animation_level_condition_value(
            self
              .presentation_layout_variables(self.current_point)
              .and_then(|variables| variables.animation_level.as_ref())
              .and_then(|value| value.val)
              .unwrap_or_default(),
          ),
          parse_animation_level_condition_value(branch.val.as_str()),
        ),
        Some("resizeHandles") => self.compare_condition(
          branch.operator,
          resize_handles_condition_value(
            self
              .presentation_layout_variables(self.current_point)
              .and_then(|variables| variables.resize_handles.as_ref())
              .and_then(|value| value.val)
              .unwrap_or_default(),
          ),
          parse_resize_handles_condition_value(branch.val.as_str()),
        ),
        _ => false,
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
    point: &'a dgm::Point,
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

  fn presentation_layout_variables(
    &self,
    point: &'a dgm::Point,
  ) -> Option<&'a dgm::PresentationLayoutVariables> {
    let direct = point
      .property_set
      .as_deref()
      .and_then(|properties| properties.presentation_layout_variables.as_deref());
    if direct.is_some() {
      return direct;
    }
    self
      .navigate_connection(
        dgm::ConnectionValues::PresentationParentOf,
        point.model_id.as_str(),
        false,
      )
      .and_then(|parent_id| self.point_by_id.get(parent_id).copied())
      .and_then(|parent| parent.property_set.as_deref())
      .and_then(|properties| properties.presentation_layout_variables.as_deref())
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
      connector_route: DiagramConnectorRoute::Straight,
      connector_dimension: dgm::ConnectorDimensionValues::OneDimension,
      connector_bend_at_end: true,
      connector_begin_arrow: false,
      connector_end_arrow: false,
      connector_begin_points: None,
      connector_end_points: None,
      connector_beginning_padding: 0.0,
      connector_end_padding: 0.0,
      connector_bending_distance: 0.0,
      connector_source_node: None,
      connector_destination_node: None,
      connector_route_shortest_distance: false,
      connector_start_override: None,
      connector_end_override: None,
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
      direct_constraints: self.active_constraints_unfiltered(layout_node),
      rules: self.active_rules(layout_node),
      placeholder_line_count: self.placeholder_line_count,
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

  fn active_constraints_unfiltered(&self, node: &'a dgm::LayoutNode) -> Vec<DiagramConstraint> {
    let mut constraints = direct_constraints_unfiltered(node);
    self.collect_active_constraints_unfiltered_from_choices(
      &node.layout_node_choice,
      &mut constraints,
    );
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

  fn collect_active_constraints_unfiltered_from_choices(
    &self,
    choices: &'a [dgm::LayoutNodeChoice],
    constraints: &mut Vec<DiagramConstraint>,
  ) {
    for choice in choices {
      if let dgm::LayoutNodeChoice::Choose(choose) = choice {
        self.collect_active_constraints_unfiltered_from_choose(choose, constraints);
      }
    }
  }

  fn collect_active_constraints_unfiltered_from_choose(
    &self,
    choose: &'a dgm::Choose,
    constraints: &mut Vec<DiagramConstraint>,
  ) {
    for branch in &choose.diagram_choose_if {
      if self.choose_if_decision(branch) {
        for choice in &branch.diagram_choose_if_choice {
          match choice {
            dgm::DiagramChooseIfChoice::Constraints(list) => {
              constraints.extend(parse_constraints_unfiltered(list));
            }
            dgm::DiagramChooseIfChoice::Choose(choose) => {
              self.collect_active_constraints_unfiltered_from_choose(choose, constraints);
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
            constraints.extend(parse_constraints_unfiltered(list));
          }
          dgm::DiagramChooseElseChoice::Choose(choose) => {
            self.collect_active_constraints_unfiltered_from_choose(choose, constraints);
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

fn parse_boolean_condition_value(value: &str) -> i32 {
  i32::from(matches!(value, "1" | "true" | "on"))
}

fn animate_one_condition_value(value: dgm::AnimateOneByOneValues) -> i32 {
  match value {
    dgm::AnimateOneByOneValues::None => 0,
    dgm::AnimateOneByOneValues::One => 1,
    dgm::AnimateOneByOneValues::Branch => 2,
  }
}

fn parse_animate_one_condition_value(value: &str) -> i32 {
  match value {
    "one" => animate_one_condition_value(dgm::AnimateOneByOneValues::One),
    "branch" => animate_one_condition_value(dgm::AnimateOneByOneValues::Branch),
    _ => animate_one_condition_value(dgm::AnimateOneByOneValues::None),
  }
}

fn animation_level_condition_value(value: dgm::AnimationLevelStringValues) -> i32 {
  match value {
    dgm::AnimationLevelStringValues::None => 0,
    dgm::AnimationLevelStringValues::Level => 1,
    dgm::AnimationLevelStringValues::Center => 2,
  }
}

fn parse_animation_level_condition_value(value: &str) -> i32 {
  match value {
    "lvl" => animation_level_condition_value(dgm::AnimationLevelStringValues::Level),
    "ctr" => animation_level_condition_value(dgm::AnimationLevelStringValues::Center),
    _ => animation_level_condition_value(dgm::AnimationLevelStringValues::None),
  }
}

fn resize_handles_condition_value(value: dgm::ResizeHandlesStringValues) -> i32 {
  match value {
    dgm::ResizeHandlesStringValues::Exact => 0,
    dgm::ResizeHandlesStringValues::Relative => 1,
  }
}

fn parse_resize_handles_condition_value(value: &str) -> i32 {
  match value {
    "rel" => resize_handles_condition_value(dgm::ResizeHandlesStringValues::Relative),
    _ => resize_handles_condition_value(dgm::ResizeHandlesStringValues::Exact),
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
  // ECMA-376 §21.4.2.8 permits a constraint target to be selected by
  // `forName`, by `ptType`, or by both.  Requiring a name here discarded the
  // common cycle/linear form `for="ch" ptType="node"` before any layout
  // algorithm could see it.
  if require_for_name
    && constraint.point_type.is_none()
    && constraint
      .for_name
      .as_deref()
      .unwrap_or_default()
      .is_empty()
  {
    return None;
  }
  if constraint.r#type == dgm::ConstraintValues::None {
    return None;
  }
  Some(DiagramConstraint {
    for_name: constraint.for_name.clone().unwrap_or_default(),
    ref_for_name: constraint.reference_for_name.clone().unwrap_or_default(),
    factor: constraint.fact.unwrap_or(1.0) as f32,
    value: constraint.val.unwrap_or_default() as f32,
    has_value: constraint.val.is_some(),
    target: constraint.r#type,
    reference: constraint.reference_type.unwrap_or_default(),
    relationship: constraint.r#for,
    reference_relationship: constraint.reference_for,
    operator: constraint.operator,
    point_type: constraint.point_type,
    reference_point_type: constraint.reference_point_type,
  })
}

fn parse_constraint_unfiltered(constraint: &dgm::Constraint) -> Option<DiagramConstraint> {
  (constraint.r#type != dgm::ConstraintValues::None).then(|| DiagramConstraint {
    for_name: constraint.for_name.clone().unwrap_or_default(),
    ref_for_name: constraint.reference_for_name.clone().unwrap_or_default(),
    factor: constraint.fact.unwrap_or(1.0) as f32,
    value: constraint.val.unwrap_or_default() as f32,
    has_value: constraint.val.is_some(),
    target: constraint.r#type,
    reference: constraint.reference_type.unwrap_or_default(),
    relationship: constraint.r#for,
    reference_relationship: constraint.reference_for,
    operator: constraint.operator,
    point_type: constraint.point_type,
    reference_point_type: constraint.reference_point_type,
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
  let (hierarchy_horizontal_alignment, hierarchy_vertical_alignment) =
    algorithm_parameter_value(algorithm, dgm::ParameterIdValues::HierarchyAlignment)
      .map(hierarchy_axis_alignments)
      .unwrap_or((None, None));
  LayoutAlgorithm {
    kind: algorithm.r#type,
    linear_direction,
    secondary_linear_direction,
    child_direction: algorithm_parameter_value(algorithm, dgm::ParameterIdValues::ChildDirection)
      .map(|value| {
        if value == "vert" {
          dgm::ChildDirectionValues::Vertical
        } else {
          dgm::ChildDirectionValues::Horizontal
        }
      }),
    child_alignment: algorithm_parameter_value(algorithm, dgm::ParameterIdValues::ChildAlignment)
      .and_then(child_alignment_from_value),
    secondary_child_alignment: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::SecondaryChildAlignment,
    )
    .and_then(child_alignment_from_value),
    horizontal_alignment: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::HorizontalAlignment,
    )
    .map(axis_alignment_from_value),
    vertical_alignment: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::VerticalAlignment,
    )
    .map(axis_alignment_from_value),
    node_horizontal_alignment: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::NodeHorizontalAlignment,
    )
    .map(axis_alignment_from_value),
    node_vertical_alignment: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::NodeVerticalAlignment,
    )
    .map(axis_alignment_from_value),
    hierarchy_horizontal_alignment,
    hierarchy_vertical_alignment,
    grow_direction,
    continue_direction,
    flow_direction: match algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::FlowDirection,
    ) {
      Some("col") => dgm::FlowDirectionValues::Column,
      _ => dgm::FlowDirectionValues::Row,
    },
    breakpoint: match algorithm_parameter_value(algorithm, dgm::ParameterIdValues::Breakpoint) {
      Some("bal") => dgm::BreakpointValues::Balanced,
      Some("fixed") => dgm::BreakpointValues::Fixed,
      _ => dgm::BreakpointValues::EndCanvas,
    },
    breakpoint_fixed_value: algorithm_parameter_f32(
      algorithm,
      dgm::ParameterIdValues::BreakpointFixedValue,
    )
    .unwrap_or(1.0)
    .round()
    .max(1.0) as usize,
    offset: match algorithm_parameter_value(algorithm, dgm::ParameterIdValues::Offset) {
      Some("off") => dgm::OffsetValues::Offset,
      _ => dgm::OffsetValues::Center,
    },
    start_angle: algorithm_parameter_f32(algorithm, dgm::ParameterIdValues::StartAngle)
      .unwrap_or_default(),
    span_angle: algorithm_parameter_f32(algorithm, dgm::ParameterIdValues::SpanAngle)
      .unwrap_or(360.0),
    start_element: if algorithm_parameter_value(algorithm, dgm::ParameterIdValues::StartElement)
      == Some("trans")
    {
      dgm::StartingElementValues::Transition
    } else {
      dgm::StartingElementValues::Node
    },
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
    text_anchor_horizontal_center: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::TextAnchorHorizontal,
    )
    .map(text_anchor_horizontal_center_from_value),
    text_anchor_vertical: algorithm
      .parameter
      .iter()
      .find(|parameter| parameter.r#type == dgm::ParameterIdValues::TextAnchorVertical)
      .and_then(|parameter| parameter.val.as_deref())
      .map(text_anchor_vertical_from_value),
    text_anchor_horizontal_with_children_center: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::TextAnchorHorizontalWithChildren,
    )
    .map(text_anchor_horizontal_center_from_value),
    text_anchor_vertical_with_children: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::TextAnchorVerticalWithChildren,
    )
    .map(text_anchor_vertical_from_value),
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
    parent_text_right_to_left_alignment: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::ParentTextRightToLeftAlignment,
    )
    .map(text_alignment_from_value),
    shape_text_left_to_right_alignment_with_children: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::ShapeTextLeftToRightAlignment,
    )
    .map(text_alignment_from_value),
    shape_text_right_to_left_alignment_with_children: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::ShapeTextRightToLeftAlignment,
    )
    .map(text_alignment_from_value),
    text_alignment: algorithm_parameter_value(algorithm, dgm::ParameterIdValues::TextAlignment)
      .map(text_alignment_from_value),
    text_direction: if algorithm_parameter_value(algorithm, dgm::ParameterIdValues::TextDirection)
      == Some("fromB")
    {
      dgm::TextDirectionValues::FromBottom
    } else {
      dgm::TextDirectionValues::FromTop
    },
    text_block_direction: if algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::TextBlockDirection,
    ) == Some("vert")
    {
      dgm::TextBlockDirectionValues::Vertical
    } else {
      dgm::TextBlockDirectionValues::Horizontal
    },
    fallback_dimension: if algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::FallbackScale,
    ) == Some("2D")
    {
      dgm::FallbackDimensionValues::TwoDimension
    } else {
      dgm::FallbackDimensionValues::OneDimension
    },
    pyramid_accent_position: if algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::PyramidAccentPosition,
    ) == Some("aft")
    {
      dgm::PyramidAccentPositionValues::After
    } else {
      dgm::PyramidAccentPositionValues::Before
    },
    pyramid_accent_text_margin: if algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::PyramidAccentTextMargin,
    ) == Some("stack")
    {
      dgm::PyramidAccentTextMarginValues::Stack
    } else {
      dgm::PyramidAccentTextMarginValues::Step
    },
    pyramid_level_node: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::PyramidLevelNode,
    )
    .map(ToString::to_string),
    pyramid_accent_background_node: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::PyramidAccentBackgroundNode,
    )
    .map(ToString::to_string),
    pyramid_accent_text_node: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::PyramidAccentTextNode,
    )
    .map(ToString::to_string),
    line_spacing_parent: algorithm_parameter_f32(
      algorithm,
      dgm::ParameterIdValues::LineSpacingParent,
    ),
    line_spacing_after_parent: algorithm_parameter_f32(
      algorithm,
      dgm::ParameterIdValues::LineSpacingAfterParentParagraph,
    ),
    line_spacing_children: algorithm_parameter_f32(
      algorithm,
      dgm::ParameterIdValues::LineSpacingChildren,
    ),
    line_spacing_after_children: algorithm_parameter_f32(
      algorithm,
      dgm::ParameterIdValues::LineSpacingAfterChildrenParagraph,
    ),
    connector_route: match algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::ConnectionRoute,
    ) {
      Some("bend") => DiagramConnectorRoute::Bend,
      Some("curve") => DiagramConnectorRoute::Curve,
      Some("longCurve") => DiagramConnectorRoute::LongCurve,
      _ => DiagramConnectorRoute::Straight,
    },
    connector_dimension: match algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::ConnectorDimension,
    ) {
      Some("2D") => dgm::ConnectorDimensionValues::TwoDimension,
      Some("cust") => dgm::ConnectorDimensionValues::Custom,
      _ => dgm::ConnectorDimensionValues::OneDimension,
    },
    connector_bend_at_end: algorithm_parameter_value(algorithm, dgm::ParameterIdValues::BendPoint)
      .is_none_or(|value| value == "end"),
    connector_begin_arrow: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::BeginningArrowheadStyle,
    ) == Some("arr"),
    connector_end_arrow: algorithm_parameter_value(algorithm, dgm::ParameterIdValues::EndStyle)
      == Some("arr"),
    connector_begin_points: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::BeginningPoints,
    )
    .map(connector_point_set_from_value)
    .unwrap_or_default(),
    connector_end_points: algorithm_parameter_value(algorithm, dgm::ParameterIdValues::EndPoints)
      .map(connector_point_set_from_value)
      .unwrap_or_default(),
    connector_source_node: algorithm_parameter_value(algorithm, dgm::ParameterIdValues::SourceNode)
      .map(ToString::to_string),
    connector_destination_node: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::DestinationNode,
    )
    .map(ToString::to_string),
    connector_route_shortest_distance: algorithm_parameter_value(
      algorithm,
      dgm::ParameterIdValues::RouteShortestDistance,
    )
    .is_some_and(|value| matches!(value, "true" | "1" | "t")),
  }
}

fn algorithm_parameter_value(
  algorithm: &dgm::Algorithm,
  kind: dgm::ParameterIdValues,
) -> Option<&str> {
  algorithm
    .parameter
    .iter()
    .find(|parameter| parameter.r#type == kind)
    .and_then(|parameter| parameter.val.as_deref())
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

fn text_anchor_horizontal_center_from_value(value: &str) -> bool {
  matches!(value, "ctr" | "mid")
}

fn child_alignment_from_value(value: &str) -> Option<ChildAlignment> {
  Some(match value {
    "t" => ChildAlignment::Top,
    "b" => ChildAlignment::Bottom,
    "l" => ChildAlignment::Left,
    "r" => ChildAlignment::Right,
    "none" => return None,
    _ => return None,
  })
}

fn axis_alignment_from_value(value: &str) -> AxisAlignment {
  match value {
    "l" | "t" => AxisAlignment::Start,
    "ctr" | "mid" => AxisAlignment::Center,
    "r" | "b" => AxisAlignment::End,
    _ => AxisAlignment::None,
  }
}

fn hierarchy_axis_alignments(value: &str) -> (Option<AxisAlignment>, Option<AxisAlignment>) {
  let horizontal = match value {
    "tL" | "bL" => Some(AxisAlignment::Start),
    "tR" | "bR" => Some(AxisAlignment::End),
    "tCtrCh" | "tCtrDes" | "bCtrCh" | "bCtrDes" => Some(AxisAlignment::Center),
    _ => None,
  };
  let vertical = match value {
    "lT" | "rT" => Some(AxisAlignment::Start),
    "lB" | "rB" => Some(AxisAlignment::End),
    "lCtrCh" | "lCtrDes" | "rCtrCh" | "rCtrDes" => Some(AxisAlignment::Center),
    _ => None,
  };
  (horizontal, vertical)
}

fn connector_point_set_from_value(value: &str) -> ConnectorPointSet {
  match value {
    "tCtr" => ConnectorPointSet::TopCenter,
    "bCtr" => ConnectorPointSet::BottomCenter,
    "ctr" => ConnectorPointSet::Center,
    "midL" => ConnectorPointSet::MiddleLeft,
    "midR" => ConnectorPointSet::MiddleRight,
    "midL midR" | "midR midL" => ConnectorPointSet::MiddleLeftOrRight,
    "bL" => ConnectorPointSet::BottomLeft,
    "bR" => ConnectorPointSet::BottomRight,
    "tL" => ConnectorPointSet::TopLeft,
    "tR" => ConnectorPointSet::TopRight,
    "radial" => ConnectorPointSet::Radial,
    _ => ConnectorPointSet::Auto,
  }
}

fn connector_point_set_name(value: ConnectorPointSet) -> Option<String> {
  Some(
    match value {
      ConnectorPointSet::Auto => return None,
      ConnectorPointSet::TopCenter => "tCtr",
      ConnectorPointSet::BottomCenter => "bCtr",
      ConnectorPointSet::Center => "ctr",
      ConnectorPointSet::MiddleLeft => "midL",
      ConnectorPointSet::MiddleRight => "midR",
      ConnectorPointSet::MiddleLeftOrRight => "midL midR",
      ConnectorPointSet::BottomLeft => "bL",
      ConnectorPointSet::BottomRight => "bR",
      ConnectorPointSet::TopLeft => "tL",
      ConnectorPointSet::TopRight => "tR",
      ConnectorPointSet::Radial => "radial",
    }
    .to_string(),
  )
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
  layout_diagram_shape_node(root, &[], &[], None);
  apply_diagram_pyramid_adjustments(root);
  resolve_diagram_connector_targets(root);
}

fn apply_diagram_pyramid_adjustments(node: &mut DiagramShapeNode) {
  for child in &mut node.children {
    apply_diagram_pyramid_adjustments(child);
  }
  let Some(algorithm) = node
    .algorithms
    .iter()
    .rev()
    .find(|algorithm| algorithm.kind == dgm::AlgorithmValues::Pyramid)
    .cloned()
  else {
    return;
  };
  let Some(level_name) = algorithm.pyramid_level_node.as_deref() else {
    return;
  };
  let accent_background_name = algorithm.pyramid_accent_background_node.as_deref();
  let accent_text_name = algorithm.pyramid_accent_text_node.as_deref();
  let has_accent = node.children.iter().any(|level| {
    level.children.iter().any(|child| {
      accent_background_name == Some(child.internal_name.as_str())
        || accent_text_name == Some(child.internal_name.as_str())
    })
  });
  let accent_ratio = node
    .constraints
    .iter()
    .chain(&node.direct_constraints)
    .rev()
    .find(|constraint| constraint.target == dgm::ConstraintValues::PyramidAccentRatio)
    .map(|constraint| constraint.value)
    .filter(|value| *value > 0.0)
    .unwrap_or(if has_accent { 0.32 } else { 0.0 })
    .clamp(0.0, 0.8);
  let pyramid_width = node.width * (1.0 - accent_ratio);
  for level in &mut node.children {
    let physical_top = level.y.clamp(0.0, node.height);
    let physical_bottom = (level.y + level.height).clamp(0.0, node.height);
    let level_width =
      (pyramid_width * physical_bottom / node.height.max(f32::EPSILON)).max(level.height * 0.25);
    let level_top_width = pyramid_width * physical_top / node.height.max(f32::EPSILON);
    // DrawingML's trapezoid guide defines x2 as `ss * adj / 100000`,
    // where ss is the shorter side.
    let trapezoid_adjustment =
      (level_width - level_top_width) / (2.0 * level_width.min(level.height).max(f32::EPSILON));
    let pyramid_origin =
      if algorithm.pyramid_accent_position == dgm::PyramidAccentPositionValues::Before {
        node.width - pyramid_width
      } else {
        0.0
      };
    let level_x = pyramid_origin + (pyramid_width - level_width) / 2.0;
    let step_accent =
      algorithm.pyramid_accent_text_margin == dgm::PyramidAccentTextMarginValues::Step;
    for child in &mut level.children {
      let is_level = child.internal_name == level_name
        || child.internal_name.starts_with(&format!("{level_name}Tx"));
      let is_accent = accent_background_name == Some(child.internal_name.as_str())
        || accent_text_name == Some(child.internal_name.as_str());
      if is_level {
        child.x = level_x;
        child.y = 0.0;
        child.width = level_width;
        child.height = level.height;
        set_pyramid_trapezoid_adjustment(child, trapezoid_adjustment);
      } else if is_accent {
        let (accent_x, accent_width) = match algorithm.pyramid_accent_position {
          dgm::PyramidAccentPositionValues::Before => {
            let right = if step_accent { level_x } else { pyramid_origin };
            (0.0, right.max(0.0))
          }
          dgm::PyramidAccentPositionValues::After => {
            let left = if step_accent {
              level_x + level_width
            } else {
              pyramid_origin + pyramid_width
            };
            (left, (node.width - left).max(0.0))
          }
        };
        child.x = accent_x;
        child.y = 0.0;
        child.width = accent_width;
        child.height = level.height;
      }
    }
  }
}

fn set_pyramid_trapezoid_adjustment(node: &mut DiagramShapeNode, adjustment: f32) {
  let adjustment = (adjustment * DRAWINGML_ADJUST_FULL_SCALE as f32).round();
  let set_adjustment = |preset: &mut a::PresetGeometry| {
    if preset.preset != a::ShapeTypeValues::Trapezoid {
      return;
    }
    preset.adjust_value_list = Some(a::AdjustValueList {
      shape_guide: vec![a::ShapeGuide {
        name: "adj".into(),
        formula: format!("val {adjustment}"),
      }],
    });
  };
  if let Some(preset) = node.preset_geometry.as_deref_mut() {
    set_adjustment(preset);
  }
  if let Some(dgm::ShapePropertiesChoice::PresetGeometry(preset)) = node
    .shape_properties
    .as_deref_mut()
    .and_then(|properties| properties.shape_properties_choice1.as_mut())
  {
    set_adjustment(preset);
  }
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
          .is_none_or(|point_type| diagram_element_type_matches(node.data_node_type, point_type))
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

fn apply_direct_node_size_constraints(node: &mut DiagramShapeNode) {
  let original_width = node.width;
  let original_height = node.height;
  let mut width = original_width;
  let mut height = original_height;

  for constraint in &node.direct_constraints {
    if !matches!(
      constraint.target,
      dgm::ConstraintValues::Width | dgm::ConstraintValues::Height
    ) || !constraint
      .relationship
      .is_none_or(|relationship| relationship == dgm::ConstraintRelationshipValues::_Self)
      || (!constraint.for_name.is_empty() && constraint.for_name != node.internal_name)
      || (!constraint.ref_for_name.is_empty() && constraint.ref_for_name != node.internal_name)
      || constraint
        .point_type
        .is_some_and(|point_type| !diagram_element_type_matches(node.data_node_type, point_type))
    {
      continue;
    }

    let reference = match constraint.reference {
      dgm::ConstraintValues::Width => width,
      dgm::ConstraintValues::Height => height,
      dgm::ConstraintValues::None if constraint.has_value => constraint_value_points(constraint),
      _ => continue,
    };
    let desired = if constraint.reference == dgm::ConstraintValues::None {
      reference
    } else {
      reference * constraint.factor
    };
    if !desired.is_finite() || desired < 0.0 {
      continue;
    }

    let current = match constraint.target {
      dgm::ConstraintValues::Width => width,
      dgm::ConstraintValues::Height => height,
      _ => unreachable!("filtered to diagram size constraints"),
    };
    let constrained = match constraint.operator.unwrap_or_default() {
      dgm::BoolOperatorValues::None | dgm::BoolOperatorValues::Equal => desired,
      dgm::BoolOperatorValues::GreaterThanOrEqualTo => current.max(desired),
      dgm::BoolOperatorValues::LessThanOrEqualTo => current.min(desired),
    };
    match constraint.target {
      dgm::ConstraintValues::Width => width = constrained,
      dgm::ConstraintValues::Height => height = constrained,
      _ => unreachable!("filtered to diagram size constraints"),
    }
  }

  // The parent algorithm has already assigned this node's slot. Applying a
  // self width/height constraint changes the extent around that slot's center,
  // matching DrawingML's shape-centered layout model.
  node.x += (original_width - width) / 2.0;
  node.y += (original_height - height) / 2.0;
  node.width = width;
  node.height = height;
}

fn layout_diagram_shape_node(
  node: &mut DiagramShapeNode,
  inherited_constraints: &[DiagramConstraint],
  inherited_rules: &[DiagramRule],
  inherited_vertical_alignment: Option<dgm::TextAnchorVerticalValues>,
) {
  apply_direct_node_size_constraints(node);
  let mut constraints = inherited_constraints.to_vec();
  constraints.extend(node.constraints.clone());
  let mut rules = inherited_rules.to_vec();
  rules.extend(node.rules.clone());
  let vertical_alignment = node
    .algorithms
    .iter()
    .rev()
    .find_map(|algorithm| algorithm.text_anchor_vertical)
    .or(inherited_vertical_alignment);
  for algorithm in node.algorithms.clone() {
    layout_shape_children(node, algorithm, &constraints, &rules, vertical_alignment);
  }
  for child in &mut node.children {
    layout_diagram_shape_node(child, &constraints, &rules, vertical_alignment);
  }
}

fn layout_shape_children(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
  rules: &[DiagramRule],
  inherited_vertical_alignment: Option<dgm::TextAnchorVerticalValues>,
) {
  node
    .children
    .retain(|child| algorithm.kind == dgm::AlgorithmValues::Linear || !child.is_empty_group());
  match algorithm.kind {
    dgm::AlgorithmValues::Composite => composite_layout_tree(node, algorithm.clone(), constraints),
    dgm::AlgorithmValues::Linear => linear_layout_tree(node, algorithm.clone(), constraints, rules),
    dgm::AlgorithmValues::Cycle => cycle_layout_tree(node, algorithm.clone(), constraints),
    dgm::AlgorithmValues::HierarchyRoot | dgm::AlgorithmValues::HierarchyChild => {
      hierarchy_layout_tree(node, algorithm.clone(), constraints)
    }
    dgm::AlgorithmValues::Snake => snake_layout_tree(node, algorithm.clone(), constraints),
    dgm::AlgorithmValues::Text => apply_text_algorithm(
      node,
      algorithm.clone(),
      constraints,
      rules,
      inherited_vertical_alignment,
    ),
    dgm::AlgorithmValues::Space => {
      // ECMA-376 §21.4.7.1 assigns `sp` only spacing/no-op layout duties;
      // text layout belongs to `tx`. LibreOffice's DiagramLayoutAtom::layoutShape
      // likewise clears the `sp` shape text before the `tx` atom lays it out.
      node.text_body = DiagramTextBody::default();
    }
    dgm::AlgorithmValues::Connector => connector_layout_tree(node, algorithm.clone(), constraints),
    dgm::AlgorithmValues::Pyramid => pyramid_layout_tree(node, algorithm.clone()),
  }
  align_children_in_parent(
    node,
    algorithm.horizontal_alignment,
    algorithm.vertical_alignment,
  );
}

fn apply_text_algorithm(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
  rules: &[DiagramRule],
  inherited_vertical_alignment: Option<dgm::TextAnchorVerticalValues>,
) {
  if algorithm.text_direction == dgm::TextDirectionValues::FromBottom {
    node.text_body.paragraphs.reverse();
  }
  if algorithm.text_block_direction == dgm::TextBlockDirectionValues::Vertical {
    node
      .text_body
      .body_properties
      .get_or_insert_with(|| Box::new(a::BodyProperties::default()))
      .vertical = Some(a::TextVerticalValues::Vertical);
  }
  let has_child_text = node.text_body.has_child_text();
  let right_to_left = node.text_body.is_right_to_left();
  let has_direct_font_size = node.text_body.has_direct_font_size();
  let font_size = constraints
    .iter()
    .rev()
    .find(|constraint| {
      constraint.target == dgm::ConstraintValues::PrimaryFontSize
        && (constraint.for_name.is_empty() || constraint.for_name == node.internal_name)
        && constraint
          .point_type
          .is_none_or(|point_type| diagram_element_type_matches(node.data_node_type, point_type))
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
          .is_none_or(|point_type| diagram_element_type_matches(node.data_node_type, point_type))
    })
    .map(|rule| rule.value)
    .filter(|value| *value > 0.0);
  node.text_body.apply_text_margins(
    node.width,
    node.height,
    has_direct_font_size
      .then(|| node.text_body.direct_primary_font_size_pt())
      .flatten()
      .or(font_size)
      .or(node.font_size_pt),
    node.data_node_type,
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
      .find_map(|algorithm| {
        has_child_text
          .then_some(algorithm.text_anchor_vertical_with_children)
          .flatten()
          .or(algorithm.text_anchor_vertical)
      })
      .or(inherited_vertical_alignment)
      .unwrap_or(dgm::TextAnchorVerticalValues::Middle)
    {
      dgm::TextAnchorVerticalValues::Top => a::TextAnchoringTypeValues::Top,
      dgm::TextAnchorVerticalValues::Bottom => a::TextAnchoringTypeValues::Bottom,
      dgm::TextAnchorVerticalValues::Middle => a::TextAnchoringTypeValues::Center,
    },
  );
  if let Some(centered) = node.algorithms.iter().rev().find_map(|algorithm| {
    has_child_text
      .then_some(algorithm.text_anchor_horizontal_with_children_center)
      .flatten()
      .or(algorithm.text_anchor_horizontal_center)
  }) {
    node.text_body.set_horizontal_anchor_center(centered);
  }
  node.text_body.apply_algorithm_spacing(&algorithm);
  let alignment = node
    .algorithms
    .iter()
    .rev()
    .find_map(|algorithm| {
      let directional = match (has_child_text, right_to_left) {
        (true, true) => algorithm.shape_text_right_to_left_alignment_with_children,
        (true, false) => algorithm.shape_text_left_to_right_alignment_with_children,
        (false, true) => algorithm.parent_text_right_to_left_alignment,
        (false, false) => algorithm.parent_text_left_to_right_alignment,
      };
      directional.or(algorithm.text_alignment)
    })
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
          && constraint
            .point_type
            .is_none_or(|point_type| diagram_element_type_matches(node.data_node_type, point_type))
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

fn collect_matching_diagram_descendant_names(
  node: &DiagramShapeNode,
  point_type: Option<dgm::ElementValues>,
  names: &mut Vec<String>,
) {
  for child in &node.children {
    if point_type
      .is_none_or(|point_type| diagram_element_type_matches(child.data_node_type, point_type))
    {
      names.push(child.internal_name.clone());
    }
    collect_matching_diagram_descendant_names(child, point_type, names);
  }
}

fn diagram_descendant_by_name<'a>(
  node: &'a DiagramShapeNode,
  name: &str,
) -> Option<&'a DiagramShapeNode> {
  node.children.iter().find_map(|child| {
    (child.internal_name == name)
      .then_some(child)
      .or_else(|| diagram_descendant_by_name(child, name))
  })
}

fn constraint_reference_name(
  node: &DiagramShapeNode,
  constraint: &DiagramConstraint,
  target_name: Option<&str>,
) -> Option<String> {
  if !constraint.ref_for_name.is_empty() {
    return Some(constraint.ref_for_name.clone());
  }
  let reference_matches = |candidate: &DiagramShapeNode| {
    constraint
      .reference_point_type
      .is_none_or(|point_type| diagram_element_type_matches(candidate.data_node_type, point_type))
  };
  match constraint
    .reference_relationship
    .unwrap_or(dgm::ConstraintRelationshipValues::_Self)
  {
    dgm::ConstraintRelationshipValues::_Self => constraint
      .reference_point_type
      .is_none_or(|point_type| diagram_element_type_matches(node.data_node_type, point_type))
      .then(String::new),
    dgm::ConstraintRelationshipValues::Child => target_name
      .and_then(|target_name| {
        node
          .children
          .iter()
          .find(|child| child.internal_name == target_name && reference_matches(child))
      })
      .or_else(|| node.children.iter().find(|child| reference_matches(child)))
      .map(|child| child.internal_name.clone()),
    dgm::ConstraintRelationshipValues::Descendant => target_name
      .and_then(|target_name| diagram_descendant_by_name(node, target_name))
      .filter(|child| reference_matches(child))
      .or_else(|| {
        let mut stack: Vec<_> = node.children.iter().collect();
        let mut found = None;
        while let Some(candidate) = stack.pop() {
          if reference_matches(candidate) {
            found = Some(candidate);
            break;
          }
          stack.extend(candidate.children.iter());
        }
        found
      })
      .map(|child| child.internal_name.clone()),
  }
}

fn expand_constraints_for_children(
  node: &DiagramShapeNode,
  constraints: &[DiagramConstraint],
) -> Vec<DiagramConstraint> {
  let mut expanded = Vec::new();
  for constraint in constraints {
    let relationship = constraint
      .relationship
      .unwrap_or(dgm::ConstraintRelationshipValues::_Self);
    let mut target_names = if !constraint.for_name.is_empty() {
      vec![constraint.for_name.clone()]
    } else {
      match relationship {
        dgm::ConstraintRelationshipValues::_Self => Vec::new(),
        dgm::ConstraintRelationshipValues::Child => node
          .children
          .iter()
          .filter(|child| {
            constraint.point_type.is_none_or(|point_type| {
              diagram_element_type_matches(child.data_node_type, point_type)
            })
          })
          .map(|child| child.internal_name.clone())
          .collect(),
        dgm::ConstraintRelationshipValues::Descendant => {
          let mut names = Vec::new();
          collect_matching_diagram_descendant_names(node, constraint.point_type, &mut names);
          names
        }
      }
    };
    target_names.sort();
    target_names.dedup();
    if target_names.is_empty() {
      // A child/descendant selector that has no match in this layout scope is
      // inapplicable.  Keeping its empty `forName` would retarget it to the
      // current canvas: a `sibTrans` width of 0.1, for example, would then
      // shrink the parent on every solver pass.
      if relationship != dgm::ConstraintRelationshipValues::_Self {
        continue;
      }
      let mut constraint = constraint.clone();
      if let Some(reference_name) = constraint_reference_name(node, &constraint, None) {
        constraint.ref_for_name = reference_name;
      }
      expanded.push(constraint);
      continue;
    }
    for target_name in target_names {
      let mut constraint = constraint.clone();
      constraint.for_name = target_name.clone();
      if let Some(reference_name) =
        constraint_reference_name(node, &constraint, Some(target_name.as_str()))
      {
        constraint.ref_for_name = reference_name;
      }
      expanded.push(constraint);
    }
  }
  expanded
}

fn direct_constraint_applies_to_node(
  constraint: &DiagramConstraint,
  node: &DiagramShapeNode,
) -> bool {
  constraint
    .relationship
    .is_none_or(|relationship| relationship == dgm::ConstraintRelationshipValues::_Self)
    && (constraint.for_name.is_empty() || constraint.for_name == node.internal_name)
    && constraint
      .point_type
      .is_none_or(|point_type| diagram_element_type_matches(node.data_node_type, point_type))
}

fn has_unbounded_height_rule(rules: &[DiagramRule], for_name: &str) -> bool {
  rules.iter().any(|rule| {
    rule.target == dgm::ConstraintValues::Height
      && rule.value.is_infinite()
      && (rule.for_name.is_empty() || rule.for_name == for_name)
  })
}

fn diagram_placeholder_text_height(
  node: &DiagramShapeNode,
  properties: &HashMap<dgm::ConstraintValues, f32>,
) -> Option<f32> {
  if node.placeholder_line_count == 0 {
    return None;
  }
  let primary_font_size =
    layout_property_value(properties, dgm::ConstraintValues::PrimaryFontSize).or(node.font_size_pt);
  let secondary_font_size =
    layout_property_value(properties, dgm::ConstraintValues::SecondaryFontSize)
      .or(primary_font_size.map(|value| value * SMARTART_DEFAULT_SECONDARY_FONT_SCALE));
  let line_font_size = secondary_font_size.or(primary_font_size)?;
  let mut vertical_margins = 0.0;
  for constraint in &node.direct_constraints {
    if !matches!(
      constraint.target,
      dgm::ConstraintValues::TopMargin | dgm::ConstraintValues::BottomMargin
    ) {
      continue;
    }
    let value = match constraint.reference {
      dgm::ConstraintValues::PrimaryFontSize => primary_font_size,
      dgm::ConstraintValues::SecondaryFontSize => secondary_font_size,
      dgm::ConstraintValues::Width => Some(node.width),
      dgm::ConstraintValues::Height => Some(node.height),
      dgm::ConstraintValues::None if constraint.has_value => {
        Some(constraint_value_points(constraint))
      }
      _ => None,
    };
    if let Some(value) = value {
      vertical_margins += if constraint.reference == dgm::ConstraintValues::None {
        value
      } else {
        value * constraint.factor
      };
    }
  }
  Some(
    line_font_size * SMARTART_DEFAULT_LINE_HEIGHT_SCALE * node.placeholder_line_count as f32
      + vertical_margins,
  )
}

fn composite_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
) {
  let mut constraints = expand_constraints_for_children(node, constraints);
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
      (dgm::ConstraintValues::Left, 0.0),
      (dgm::ConstraintValues::Top, 0.0),
      (dgm::ConstraintValues::Right, parent_width),
      (dgm::ConstraintValues::Bottom, node.height),
      (dgm::ConstraintValues::CenterHeight, parent_width / 2.0),
      (dgm::ConstraintValues::CenterWidth, node.height / 2.0),
      (
        dgm::ConstraintValues::Diameter,
        parent_width.min(node.height),
      ),
    ]),
  );
  for child in &node.children {
    let width = if child.width > 0.0 {
      child.width
    } else {
      parent_width
    };
    let height = if child.height > 0.0 {
      child.height
    } else {
      node.height
    };
    properties.insert(
      child.internal_name.clone(),
      HashMap::from([
        (dgm::ConstraintValues::Width, width),
        (dgm::ConstraintValues::Height, height),
      ]),
    );
    for direct in &child.direct_constraints {
      if !direct_constraint_applies_to_node(direct, child) {
        continue;
      }
      let mut direct = direct.clone();
      if direct.for_name.is_empty() {
        direct.for_name.clone_from(&child.internal_name);
      }
      if direct.ref_for_name.is_empty() && direct.reference != dgm::ConstraintValues::None {
        direct.ref_for_name.clone_from(&child.internal_name);
      }
      constraints.push(direct);
    }
  }
  solve_layout_constraints(&constraints, &mut properties);
  let mut vertical_min = f32::MAX;
  let mut vertical_max = 0.0_f32;
  let child_count = node.children.len();
  for child in &mut node.children {
    // Composite constraints are order-sensitive. Re-apply the constraints for
    // the current child after earlier children have accepted their local
    // limits, so a stacked child's `t=previous.h` observes that final height.
    // Microsoft documents this ordering requirement for composite layouts,
    // and LibreOffice applies the parent constraints once per child for the
    // same reason.
    for constraint in constraints
      .iter()
      .filter(|constraint| constraint.for_name == child.internal_name)
    {
      apply_constraint_to_layout(constraint, &mut properties);
    }
    // Constraints declared on the child layout node refine the slot selected
    // by this composite algorithm.  In particular, an `lte` size constraint
    // together with an unbounded growth rule selects the boundary value; it
    // is not a request to keep an earlier, smaller inherited size.  Apply
    // these local constraints last, as CompositeAlg does in LibreOffice.
    for direct in &child.direct_constraints {
      if !direct_constraint_applies_to_node(direct, child) || !direct.ref_for_name.is_empty() {
        continue;
      }
      if direct.reference == dgm::ConstraintValues::None && !direct.has_value {
        continue;
      }
      let mut direct = direct.clone();
      direct.for_name.clone_from(&child.internal_name);
      if direct.reference != dgm::ConstraintValues::None {
        direct.ref_for_name.clone_from(&child.internal_name);
      }
      // A local boundary supersedes the inherited approximation.  The
      // constraint itself remains authoritative; rules decide which
      // constraints may move when the containing algorithm later scales.
      direct.operator = Some(dgm::BoolOperatorValues::Equal);
      apply_constraint_to_layout(&direct, &mut properties);
    }
    let properties_for_child = properties.get(child.internal_name.as_str());
    let mut width = parent_width;
    let mut height = node.height;
    let mut x = 0.0;
    let mut y = 0.0;
    if let Some(properties_for_child) = properties_for_child {
      let left = layout_property_value(properties_for_child, dgm::ConstraintValues::Left);
      let right = layout_property_value(properties_for_child, dgm::ConstraintValues::Right);
      let top = layout_property_value(properties_for_child, dgm::ConstraintValues::Top);
      let bottom = layout_property_value(properties_for_child, dgm::ConstraintValues::Bottom);
      if let Some(value) = layout_property_value(properties_for_child, dgm::ConstraintValues::Width)
      {
        width = value;
      }
      if let Some(value) =
        layout_property_value(properties_for_child, dgm::ConstraintValues::Height)
      {
        height = value;
      }
      if let (Some(left), Some(right)) = (left, right) {
        width = right - left;
      }
      if let (Some(top), Some(bottom)) = (top, bottom) {
        height = bottom - top;
      }
      x = left
        .or_else(|| {
          layout_property_value(properties_for_child, dgm::ConstraintValues::CenterHeight)
            .map(|center| center - width / 2.0)
        })
        .or_else(|| right.map(|right| right - width))
        .unwrap_or_default();
      y = top
        .or_else(|| {
          layout_property_value(properties_for_child, dgm::ConstraintValues::CenterWidth)
            .map(|center| center - height / 2.0)
        })
        .or_else(|| bottom.map(|bottom| bottom - height))
        .unwrap_or_default();
    }
    x += parent_x_offset;
    child.x = if x.is_finite() { x } else { 0.0 };
    child.y = if y.is_finite() { y } else { 0.0 };
    child.width = if width.is_finite() {
      width.max(0.0).min((node.width - child.x.max(0.0)).max(0.0))
    } else {
      parent_width
    };
    child.height = if height.is_finite() {
      height
        .max(0.0)
        .min((node.height - child.y.max(0.0)).max(0.0))
    } else {
      node.height
    };
    let has_stacked_top = constraints.iter().any(|constraint| {
      constraint.for_name == child.internal_name
        && constraint.target == dgm::ConstraintValues::Top
        && constraint.reference == dgm::ConstraintValues::Height
        && !constraint.ref_for_name.is_empty()
        && constraint.ref_for_name != child.internal_name
    });
    let has_explicit_bottom = constraints.iter().any(|constraint| {
      constraint.for_name == child.internal_name
        && constraint.target == dgm::ConstraintValues::Bottom
    });
    if child_count > 1
      && node
        .rules
        .iter()
        .any(|rule| rule.target == dgm::ConstraintValues::Height && rule.value.is_infinite())
      && has_unbounded_height_rule(&child.rules, child.internal_name.as_str())
      && child.text_body.is_empty()
      && has_stacked_top
      && !has_explicit_bottom
      && let Some(preferred_height) = properties_for_child
        .and_then(|properties| diagram_placeholder_text_height(child, properties))
    {
      // An empty descendant-text panel still reserves the line capacity shown
      // by the layout definition's sample data. The rule may grow the initial
      // constraint to that preferred size, but never beyond the remaining
      // composite canvas.
      child.height = preferred_height
        .max(child.height)
        .min((node.height - child.y.max(0.0)).max(0.0));
    }
    vertical_min = vertical_min.min(child.y);
    vertical_max = vertical_max.max(child.y + child.height);
  }
  // Composite coordinates describe the relative arrangement of the child
  // extent.  Office and LibreOffice center that complete extent whenever it
  // fits, including layouts whose children use explicit `t`/`b` constraints
  // internally (hList1 is the common counterexample to treating those as a
  // canvas-level anchor).
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
) -> bool {
  let reference = properties.get(constraint.ref_for_name.as_str());
  let value = if constraint.reference == dgm::ConstraintValues::None {
    // A constraint without either `refType` or `val` does not prescribe the
    // literal value zero.  DrawingML layout definitions use this form with
    // `op="equ"` to keep a relationship set uniform after its members have
    // been measured (for example, equal-height SmartArt siblings).  Until we
    // have a measured group value, retain the existing property.
    if !constraint.has_value {
      return false;
    }
    constraint_value_points(constraint)
  } else {
    let value = reference
      .and_then(|properties| layout_property_value(properties, constraint.reference))
      .map(|value| value * constraint.factor);
    match value {
      Some(value) => value,
      None if constraint.has_value => constraint_value_points(constraint),
      None => {
        // Layout definitions routinely reference a sibling declared later in
        // the constraint list. Leave it unresolved for the next solver pass;
        // substituting the default literal zero collapses the target shape.
        return false;
      }
    }
  };
  if !value.is_finite() {
    return false;
  }
  let properties = properties.entry(constraint.for_name.clone()).or_default();
  let value = match (
    constraint.operator.unwrap_or_default(),
    properties.get(&constraint.target).copied(),
  ) {
    (dgm::BoolOperatorValues::GreaterThanOrEqualTo, Some(current)) => current.max(value),
    (dgm::BoolOperatorValues::LessThanOrEqualTo, Some(current)) => current.min(value),
    _ => value,
  };
  let changed = properties
    .get(&constraint.target)
    .is_none_or(|current| (*current - value).abs() > f32::EPSILON);
  properties.insert(constraint.target, value);
  changed
}

fn solve_layout_constraints(
  constraints: &[DiagramConstraint],
  properties: &mut HashMap<String, HashMap<dgm::ConstraintValues, f32>>,
) {
  // A complete pass can unlock at least one forward reference. The extra
  // pass propagates offset-derived geometry (for example r + rOff -> l).
  for _ in 0..=constraints.len().min(256) {
    let mut changed = false;
    for constraint in constraints {
      changed |= apply_constraint_to_layout(constraint, properties);
    }
    if !changed {
      break;
    }
  }
}

fn layout_property_value(
  properties: &HashMap<dgm::ConstraintValues, f32>,
  property: dgm::ConstraintValues,
) -> Option<f32> {
  let offset_for = |property| match property {
    dgm::ConstraintValues::Left => Some(dgm::ConstraintValues::LeftOffset),
    dgm::ConstraintValues::Right => Some(dgm::ConstraintValues::RightOffset),
    dgm::ConstraintValues::Top => Some(dgm::ConstraintValues::TopOffset),
    dgm::ConstraintValues::Bottom => Some(dgm::ConstraintValues::BottomOffset),
    dgm::ConstraintValues::CenterHeight => Some(dgm::ConstraintValues::CenterXOffset),
    dgm::ConstraintValues::CenterWidth => Some(dgm::ConstraintValues::CenterYOffset),
    dgm::ConstraintValues::Width => Some(dgm::ConstraintValues::WidthOffset),
    dgm::ConstraintValues::Height => Some(dgm::ConstraintValues::HeightOffset),
    _ => None,
  };
  let get = |property| {
    properties.get(&property).copied().map(|value| {
      value
        + offset_for(property)
          .and_then(|offset| properties.get(&offset))
          .copied()
          .unwrap_or_default()
    })
  };
  if let Some(value) = get(property) {
    return Some(value);
  }
  match property {
    dgm::ConstraintValues::Left => get(dgm::ConstraintValues::Right)
      .zip(get(dgm::ConstraintValues::Width))
      .map(|(right, width)| right - width)
      .or_else(|| {
        get(dgm::ConstraintValues::CenterHeight)
          .zip(get(dgm::ConstraintValues::Width))
          .map(|(center, width)| center - width / 2.0)
      }),
    dgm::ConstraintValues::Right => get(dgm::ConstraintValues::Left)
      .zip(get(dgm::ConstraintValues::Width))
      .map(|(left, width)| left + width)
      .or_else(|| {
        get(dgm::ConstraintValues::CenterHeight)
          .zip(get(dgm::ConstraintValues::Width))
          .map(|(center, width)| center + width / 2.0)
      }),
    // The generated enum names follow the schema documentation's historical
    // labels: CenterHeight is `ctrX`, and CenterWidth is `ctrY`.
    dgm::ConstraintValues::CenterHeight => get(dgm::ConstraintValues::Left)
      .zip(get(dgm::ConstraintValues::Width))
      .map(|(left, width)| left + width / 2.0)
      .or_else(|| {
        get(dgm::ConstraintValues::Right)
          .zip(get(dgm::ConstraintValues::Width))
          .map(|(right, width)| right - width / 2.0)
      }),
    dgm::ConstraintValues::Top => get(dgm::ConstraintValues::Bottom)
      .zip(get(dgm::ConstraintValues::Height))
      .map(|(bottom, height)| bottom - height)
      .or_else(|| {
        get(dgm::ConstraintValues::CenterWidth)
          .zip(get(dgm::ConstraintValues::Height))
          .map(|(center, height)| center - height / 2.0)
      }),
    dgm::ConstraintValues::Bottom => get(dgm::ConstraintValues::Top)
      .zip(get(dgm::ConstraintValues::Height))
      .map(|(top, height)| top + height)
      .or_else(|| {
        get(dgm::ConstraintValues::CenterWidth)
          .zip(get(dgm::ConstraintValues::Height))
          .map(|(center, height)| center + height / 2.0)
      }),
    dgm::ConstraintValues::CenterWidth => get(dgm::ConstraintValues::Top)
      .zip(get(dgm::ConstraintValues::Height))
      .map(|(top, height)| top + height / 2.0)
      .or_else(|| {
        get(dgm::ConstraintValues::Bottom)
          .zip(get(dgm::ConstraintValues::Height))
          .map(|(bottom, height)| bottom - height / 2.0)
      }),
    dgm::ConstraintValues::Width => get(dgm::ConstraintValues::Left)
      .zip(get(dgm::ConstraintValues::Right))
      .map(|(left, right)| right - left),
    dgm::ConstraintValues::Height => get(dgm::ConstraintValues::Top)
      .zip(get(dgm::ConstraintValues::Bottom))
      .map(|(top, bottom)| bottom - top),
    dgm::ConstraintValues::Diameter => get(dgm::ConstraintValues::Width)
      .zip(get(dgm::ConstraintValues::Height))
      .map(|(width, height)| width.min(height)),
    _ => None,
  }
}

fn constraint_value_points(constraint: &DiagramConstraint) -> f32 {
  if matches!(
    constraint.target,
    dgm::ConstraintValues::PrimaryFontSize
      | dgm::ConstraintValues::SecondaryFontSize
      | dgm::ConstraintValues::PyramidAccentRatio
      | dgm::ConstraintValues::AlignmentOffset
      | dgm::ConstraintValues::UserDefinedA
      | dgm::ConstraintValues::UserDefinedB
      | dgm::ConstraintValues::UserDefinedC
      | dgm::ConstraintValues::UserDefinedD
      | dgm::ConstraintValues::UserDefinedE
      | dgm::ConstraintValues::UserDefinedF
      | dgm::ConstraintValues::UserDefinedG
      | dgm::ConstraintValues::UserDefinedH
      | dgm::ConstraintValues::UserDefinedI
      | dgm::ConstraintValues::UserDefinedJ
      | dgm::ConstraintValues::UserDefinedK
      | dgm::ConstraintValues::UserDefinedL
      | dgm::ConstraintValues::UserDefinedM
      | dgm::ConstraintValues::UserDefinedN
      | dgm::ConstraintValues::UserDefinedO
      | dgm::ConstraintValues::UserDefinedP
      | dgm::ConstraintValues::UserDefinedQ
      | dgm::ConstraintValues::UserDefinedR
      | dgm::ConstraintValues::UserDefinedS
      | dgm::ConstraintValues::UserDefinedT
      | dgm::ConstraintValues::UserDefinedU
      | dgm::ConstraintValues::UserDefinedV
      | dgm::ConstraintValues::UserDefinedW
      | dgm::ConstraintValues::UserDefinedX
      | dgm::ConstraintValues::UserDefinedY
      | dgm::ConstraintValues::UserDefinedZ
  ) || matches!(
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
  let horizontal = algorithm.child_direction.map_or_else(
    || {
      matches!(
        algorithm.linear_direction,
        LinearDirection::Left | LinearDirection::Right
      )
    },
    |direction| direction == dgm::ChildDirectionValues::Horizontal,
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
  let mut properties: HashMap<String, HashMap<dgm::ConstraintValues, f32>> = HashMap::from([(
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
  let mut constraints = expand_constraints_for_children(node, constraints);
  for child in &node.children {
    properties.entry(child.internal_name.clone()).or_default();
    for direct in &child.direct_constraints {
      if !direct_constraint_applies_to_node(direct, child) {
        continue;
      }
      let mut direct = direct.clone();
      if direct.for_name.is_empty() {
        direct.for_name.clone_from(&child.internal_name);
      }
      if direct.ref_for_name.is_empty() && direct.reference != dgm::ConstraintValues::None {
        direct.ref_for_name.clone_from(&child.internal_name);
      }
      constraints.push(direct);
    }
  }
  solve_layout_constraints(&constraints, &mut properties);
  // Linear layout constraints are normalized against the current algorithm
  // canvas before overflow scaling.  Keeping this explicit slot pass is
  // important for repeated layout names: every `compNode` instance receives
  // the same pre-scale width instead of inheriting a partially solved value
  // from another instance with that name.  This mirrors LinearLayout's first
  // approximation in LibreOffice and leaves the general solver available for
  // offsets and cross-name references.
  for constraint in &constraints {
    if !matches!(
      constraint.target,
      dgm::ConstraintValues::Width | dgm::ConstraintValues::Height
    ) || !node
      .children
      .iter()
      .any(|child| child.internal_name == constraint.for_name)
    {
      continue;
    }
    let canvas_extent = if constraint.target == dgm::ConstraintValues::Width {
      node.width
    } else {
      node.height
    };
    let value = (canvas_extent * constraint.factor)
      .max(0.0)
      .min(canvas_extent);
    properties
      .entry(constraint.for_name.clone())
      .or_default()
      .insert(constraint.target, value);
  }
  let mut space_width = 0.0;
  let mut space_height = 0.0;
  for constraint in &constraints {
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
        space_width = properties
          .get(constraint.for_name.as_str())
          .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Width))
          .unwrap_or(node.width * constraint.factor);
      }
      if constraint.target == dgm::ConstraintValues::Height {
        space_height = properties
          .get(constraint.for_name.as_str())
          .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Height))
          .unwrap_or(node.height * constraint.factor);
      }
    }
    if constraint.for_name.is_empty()
      && matches!(
        constraint.target,
        dgm::ConstraintValues::Spacing | dgm::ConstraintValues::SiblingSpacing
      )
    {
      let value = properties
        .get("")
        .and_then(|properties| layout_property_value(properties, constraint.target))
        .unwrap_or_else(|| {
          if horizontal {
            node.width * constraint.factor
          } else {
            node.height * constraint.factor
          }
        });
      if horizontal {
        space_width = value;
      } else {
        space_height = value;
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
      .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Width))
      .unwrap_or(base_width);
    let height = child_properties
      .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Height))
      .unwrap_or(base_height);
    total_primary += if horizontal { width } else { height };
  }
  total_primary += if horizontal {
    (count - 1.0).max(0.0) * space_width
  } else {
    (count - 1.0).max(0.0) * space_height
  };
  let mut width_scale = if horizontal && total_primary > node.width {
    node.width / total_primary
  } else {
    1.0
  };
  let mut height_scale = if !horizontal && total_primary > node.height {
    node.height / total_primary
  } else {
    1.0
  };
  if algorithm.fallback_dimension == dgm::FallbackDimensionValues::TwoDimension {
    if horizontal {
      height_scale = width_scale;
    } else {
      width_scale = height_scale;
    }
  }
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
      .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Width))
      .unwrap_or(base_width);
    let mut height = child_properties
      .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Height))
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
      let default_y = child_cross_axis_offset(
        node.height,
        height,
        algorithm
          .child_alignment
          .or(algorithm.secondary_child_alignment),
        algorithm.node_vertical_alignment,
        true,
      );
      child.y = child_properties
        .map(|properties| {
          constrained_axis_position(
            properties,
            dgm::ConstraintValues::Top,
            dgm::ConstraintValues::Bottom,
            dgm::ConstraintValues::CenterWidth,
            dgm::ConstraintValues::TopOffset,
            height,
            default_y,
          )
        })
        .unwrap_or(default_y);
      child.width = width.max(0.0);
      child.height = height.max(0.0);
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
      let default_x = child_cross_axis_offset(
        node.width,
        width,
        algorithm
          .child_alignment
          .or(algorithm.secondary_child_alignment),
        algorithm.node_horizontal_alignment,
        false,
      );
      child.x = child_properties
        .map(|properties| {
          constrained_axis_position(
            properties,
            dgm::ConstraintValues::Left,
            dgm::ConstraintValues::Right,
            dgm::ConstraintValues::CenterHeight,
            dgm::ConstraintValues::LeftOffset,
            width,
            default_x,
          )
        })
        .unwrap_or(default_x);
      child.y = y.max(0.0);
      child.width = width.max(0.0);
      child.height = height.max(0.0);
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

fn constrained_axis_position(
  properties: &HashMap<dgm::ConstraintValues, f32>,
  start: dgm::ConstraintValues,
  end: dgm::ConstraintValues,
  center: dgm::ConstraintValues,
  offset: dgm::ConstraintValues,
  extent: f32,
  default: f32,
) -> f32 {
  layout_property_value(properties, start)
    .or_else(|| layout_property_value(properties, center).map(|center| center - extent / 2.0))
    .or_else(|| layout_property_value(properties, end).map(|end| end - extent))
    .unwrap_or_else(|| default + properties.get(&offset).copied().unwrap_or_default())
}

fn child_cross_axis_offset(
  available: f32,
  extent: f32,
  child_alignment: Option<ChildAlignment>,
  node_alignment: Option<AxisAlignment>,
  vertical_axis: bool,
) -> f32 {
  let alignment = child_alignment
    .and_then(|alignment| match (vertical_axis, alignment) {
      (true, ChildAlignment::Top) | (false, ChildAlignment::Left) => Some(AxisAlignment::Start),
      (true, ChildAlignment::Bottom) | (false, ChildAlignment::Right) => Some(AxisAlignment::End),
      _ => None,
    })
    .or(node_alignment)
    .unwrap_or(AxisAlignment::Center);
  axis_alignment_offset(available, extent, alignment)
}

fn axis_alignment_offset(available: f32, extent: f32, alignment: AxisAlignment) -> f32 {
  let slack = (available - extent).max(0.0);
  match alignment {
    AxisAlignment::Start | AxisAlignment::None => 0.0,
    AxisAlignment::Center => slack / 2.0,
    AxisAlignment::End => slack,
  }
}

fn align_children_in_parent(
  node: &mut DiagramShapeNode,
  horizontal: Option<AxisAlignment>,
  vertical: Option<AxisAlignment>,
) {
  align_children_in_parent_with_offset(node, horizontal, vertical, None);
}

fn align_children_in_parent_with_offset(
  node: &mut DiagramShapeNode,
  horizontal: Option<AxisAlignment>,
  vertical: Option<AxisAlignment>,
  alignment_offset: Option<f32>,
) {
  if node.children.is_empty() {
    return;
  }
  let left = node
    .children
    .iter()
    .map(|child| child.x)
    .fold(f32::INFINITY, f32::min);
  let top = node
    .children
    .iter()
    .map(|child| child.y)
    .fold(f32::INFINITY, f32::min);
  let right = node
    .children
    .iter()
    .map(|child| child.x + child.width)
    .fold(f32::NEG_INFINITY, f32::max);
  let bottom = node
    .children
    .iter()
    .map(|child| child.y + child.height)
    .fold(f32::NEG_INFINITY, f32::max);
  let dx = horizontal
    .filter(|alignment| *alignment != AxisAlignment::None)
    .map(|alignment| {
      let slack = (node.width - (right - left)).max(0.0);
      let aligned = match (alignment, alignment_offset) {
        (AxisAlignment::Start, Some(offset)) => slack * (1.0 - offset),
        (AxisAlignment::End, Some(offset)) => slack * offset,
        _ => axis_alignment_offset(node.width, right - left, alignment),
      };
      aligned - left
    })
    .unwrap_or(0.0);
  let dy = vertical
    .filter(|alignment| *alignment != AxisAlignment::None)
    .map(|alignment| {
      let slack = (node.height - (bottom - top)).max(0.0);
      let aligned = match (alignment, alignment_offset) {
        (AxisAlignment::Start, Some(offset)) => slack * (1.0 - offset),
        (AxisAlignment::End, Some(offset)) => slack * offset,
        _ => axis_alignment_offset(node.height, bottom - top, alignment),
      };
      aligned - top
    })
    .unwrap_or(0.0);
  if dx == 0.0 && dy == 0.0 {
    return;
  }
  for child in &mut node.children {
    child.x += dx;
    child.y += dy;
  }
}

fn cycle_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
) {
  if node.children.is_empty() {
    return;
  }
  if algorithm.start_element == dgm::StartingElementValues::Transition
    && let Some(index) = node.children.iter().position(|child| {
      matches!(
        child.data_node_type,
        Some(dgm::ElementValues::ParentTransition | dgm::ElementValues::SiblingTransition)
      )
    })
  {
    node.children.rotate_left(index);
  }
  let mut cycle_constraints = expand_constraints_for_children(node, constraints);
  let mut properties = HashMap::from([(
    String::new(),
    HashMap::from([
      (dgm::ConstraintValues::Width, node.width),
      (dgm::ConstraintValues::Height, node.height),
      (dgm::ConstraintValues::Left, 0.0),
      (dgm::ConstraintValues::Top, 0.0),
      (dgm::ConstraintValues::Right, node.width),
      (dgm::ConstraintValues::Bottom, node.height),
      (dgm::ConstraintValues::CenterHeight, node.width / 2.0),
      (dgm::ConstraintValues::CenterWidth, node.height / 2.0),
      (dgm::ConstraintValues::Diameter, node.width.min(node.height)),
    ]),
  )]);
  for child in &node.children {
    properties.insert(
      child.internal_name.clone(),
      HashMap::from([
        (
          dgm::ConstraintValues::Width,
          if child.width > 0.0 {
            child.width
          } else {
            node.width
          },
        ),
        (
          dgm::ConstraintValues::Height,
          if child.height > 0.0 {
            child.height
          } else {
            node.height
          },
        ),
      ]),
    );
    for direct in &child.direct_constraints {
      let mut direct = direct.clone();
      if direct.for_name.is_empty() {
        direct.for_name.clone_from(&child.internal_name);
      }
      if direct.ref_for_name.is_empty() && direct.reference != dgm::ConstraintValues::None {
        direct.ref_for_name.clone_from(&child.internal_name);
      }
      cycle_constraints.push(direct);
    }
  }
  solve_layout_constraints(&cycle_constraints, &mut properties);

  let parent_properties = properties.get("");
  let diameter = parent_properties
    .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Diameter))
    .unwrap_or_else(|| node.width.min(node.height))
    .max(0.0);
  let center_x = parent_properties
    .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::CenterHeight))
    .unwrap_or(node.width / 2.0);
  let center_y = parent_properties
    .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::CenterWidth))
    .unwrap_or(node.height / 2.0);
  let default_child_width = node.width / 4.0;
  let default_child_height = node.height / 4.0;
  let default_connector_width = node.width / 12.0;
  let default_connector_height = node.height / 12.0;
  let raw_child_size = |child: &DiagramShapeNode| {
    let child_properties = properties.get(child.internal_name.as_str());
    let width = child_properties
      .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Width))
      .unwrap_or(if child.is_connector {
        default_connector_width
      } else {
        default_child_width
      });
    let height = child_properties
      .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Height))
      .unwrap_or(if child.is_connector {
        default_connector_height
      } else {
        default_child_height
      });
    (width.max(0.0), height.max(0.0))
  };
  let max_raw_width = node
    .children
    .iter()
    .filter(|child| !child.is_connector)
    .map(|child| raw_child_size(child).0)
    .fold(0.0_f32, f32::max);
  let max_raw_height = node
    .children
    .iter()
    .filter(|child| !child.is_connector)
    .map(|child| raw_child_size(child).1)
    .fold(0.0_f32, f32::max);
  let normalized_fraction = if algorithm.center_shape_mapping_first_node {
    0.5
  } else {
    0.25
  };
  let width_scale =
    if max_raw_width > node.width * normalized_fraction && max_raw_width > f32::EPSILON {
      node.width * normalized_fraction / max_raw_width
    } else {
      1.0
    };
  let height_scale =
    if max_raw_height > node.height * normalized_fraction && max_raw_height > f32::EPSILON {
      node.height * normalized_fraction / max_raw_height
    } else {
      1.0
    };
  let uniform_scale = width_scale.min(height_scale);
  let scaled_child_size = |child: &DiagramShapeNode| {
    let (width, height) = raw_child_size(child);
    (
      lo_i32(width * uniform_scale),
      lo_i32(height * uniform_scale),
    )
  };
  let sibling_spacing = parent_properties
    .and_then(|properties| {
      layout_property_value(properties, dgm::ConstraintValues::SiblingSpacing)
        .or_else(|| layout_property_value(properties, dgm::ConstraintValues::Spacing))
    })
    .unwrap_or_default()
    * uniform_scale;
  let max_orbit_width = node
    .children
    .iter()
    .skip(usize::from(algorithm.center_shape_mapping_first_node))
    .filter(|child| !child.is_connector)
    .map(|child| scaled_child_size(child).0)
    .fold(0.0_f32, f32::max)
    .max(default_child_width * uniform_scale);
  let max_orbit_height = node
    .children
    .iter()
    .skip(usize::from(algorithm.center_shape_mapping_first_node))
    .filter(|child| !child.is_connector)
    .map(|child| scaled_child_size(child).1)
    .fold(0.0_f32, f32::max)
    .max(default_child_height * uniform_scale);
  let radius = lo_i32(
    ((diameter - max_orbit_width - sibling_spacing) / 2.0)
      .min((diameter - max_orbit_height - sibling_spacing) / 2.0)
      .max(0.0),
  );
  let mut start = 0usize;
  if algorithm.center_shape_mapping_first_node
    && let Some(center) = node.children.first_mut()
  {
    let (width, height) = scaled_child_size(center);
    center.x = lo_i32(center_x - width / 2.0);
    center.y = lo_i32(center_y - height / 2.0);
    center.width = width;
    center.height = height;
    start = 1;
  }
  let count = node.children.len().saturating_sub(start);
  if count == 0 {
    return;
  }
  let connector_radius = lo_i32(
    radius
      * (algorithm.span_angle / count as f32 / 2.0)
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
    let (width, height) = scaled_child_size(child);
    let connection_distance = properties
      .get(child.internal_name.as_str())
      .and_then(|properties| {
        layout_property_value(properties, dgm::ConstraintValues::ConnectionDistance)
      })
      .unwrap_or_default()
      * uniform_scale;
    let current_radius = if child.is_connector {
      connector_radius + connection_distance
    } else {
      radius + connection_distance
    };
    child.x = lo_i32(center_x + current_radius * radians.sin() - lo_i32(width / 2.0));
    // DrawingML cycle angles start at the top of the canvas and advance
    // clockwise in the slide coordinate system.  Y grows downwards, so the
    // cosine component is subtracted (LibreOffice's SmartArt cycle importer
    // uses the same centerY - radius * cos(angle) mapping).
    child.y = lo_i32(center_y - current_radius * radians.cos() - lo_i32(height / 2.0));
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

fn connector_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
) {
  if !node.is_connector {
    return;
  }
  node.connector_route = algorithm.connector_route;
  node.connector_dimension = algorithm.connector_dimension;
  node.connector_bend_at_end = algorithm.connector_bend_at_end;
  node.connector_begin_arrow = algorithm.connector_begin_arrow;
  node.connector_end_arrow = algorithm.connector_end_arrow;
  node.connector_begin_points = connector_point_set_name(algorithm.connector_begin_points);
  node.connector_end_points = connector_point_set_name(algorithm.connector_end_points);
  node.connector_source_node = algorithm.connector_source_node;
  node.connector_destination_node = algorithm.connector_destination_node;
  node.connector_route_shortest_distance = algorithm.connector_route_shortest_distance;
  let node_name = node.internal_name.clone();
  let base_properties = HashMap::from([
    (dgm::ConstraintValues::Width, node.width),
    (dgm::ConstraintValues::Height, node.height),
    (dgm::ConstraintValues::Left, 0.0),
    (dgm::ConstraintValues::Top, 0.0),
    (dgm::ConstraintValues::Right, node.width),
    (dgm::ConstraintValues::Bottom, node.height),
  ]);
  let mut properties = HashMap::from([
    (String::new(), base_properties.clone()),
    (node_name.clone(), base_properties),
  ]);
  let mut applicable_constraints: Vec<_> = constraints
    .iter()
    .filter(|constraint| {
      (constraint.for_name.is_empty() || constraint.for_name == node_name)
        && constraint
          .point_type
          .is_none_or(|point_type| diagram_element_type_matches(node.data_node_type, point_type))
        && constraint
          .relationship
          .is_none_or(|relationship| relationship != dgm::ConstraintRelationshipValues::Child)
    })
    .cloned()
    .collect();
  applicable_constraints.extend(node.direct_constraints.clone());
  solve_layout_constraints(&applicable_constraints, &mut properties);
  let resolved = |target| {
    let named_target = applicable_constraints
      .iter()
      .any(|constraint| constraint.for_name == node_name && constraint.target == target);
    let named = properties
      .get(node_name.as_str())
      .and_then(|properties| layout_property_value(properties, target));
    let parent = properties
      .get("")
      .and_then(|properties| layout_property_value(properties, target));
    if named_target {
      named.or(parent)
    } else {
      parent.or(named)
    }
  };
  let width = resolved(dgm::ConstraintValues::Width).unwrap_or(node.width);
  let height = resolved(dgm::ConstraintValues::Height).unwrap_or(node.height);
  node.x += (node.width - width) / 2.0;
  node.y += (node.height - height) / 2.0;
  node.width = width.max(0.0);
  node.height = height.max(0.0);
  node.connector_beginning_padding = resolved(dgm::ConstraintValues::BeginningPadding)
    .unwrap_or_default()
    .max(0.0);
  node.connector_end_padding = resolved(dgm::ConstraintValues::EndPadding)
    .unwrap_or_default()
    .max(0.0);
  node.connector_bending_distance = resolved(dgm::ConstraintValues::BendingDistance)
    .unwrap_or_default()
    .abs();
}

fn pyramid_layout_tree(node: &mut DiagramShapeNode, algorithm: LayoutAlgorithm) {
  if node.children.is_empty() || node.width == 0.0 || node.height == 0.0 {
    return;
  }
  let count = node.children.len();
  let child_height = node.height / count as f32;
  for (index, child) in node.children.iter_mut().enumerate() {
    child.x = 0.0;
    child.y = if algorithm.linear_direction == LinearDirection::Bottom {
      node.height - (index + 1) as f32 * child_height
    } else {
      index as f32 * child_height
    };
    child.width = node.width;
    child.height = child_height;
  }
}

fn hierarchy_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
) {
  if node.children.is_empty() || node.width == 0.0 || node.height == 0.0 {
    return;
  }

  let direction = if algorithm.kind == dgm::AlgorithmValues::HierarchyRoot
    || algorithm.child_direction == Some(dgm::ChildDirectionValues::Vertical)
  {
    LinearDirection::Top
  } else if algorithm.child_direction == Some(dgm::ChildDirectionValues::Horizontal)
    && matches!(
      algorithm.linear_direction,
      LinearDirection::Top | LinearDirection::Bottom
    )
  {
    LinearDirection::Left
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

  let node_name = node.internal_name.clone();
  let mut constraint_properties = HashMap::from([(
    String::new(),
    HashMap::from([
      (dgm::ConstraintValues::Width, node.width),
      (dgm::ConstraintValues::Height, node.height),
    ]),
  )]);
  solve_layout_constraints(constraints, &mut constraint_properties);
  let resolved_constraint = |target| {
    constraint_properties
      .get(node_name.as_str())
      .and_then(|properties| layout_property_value(properties, target))
      .or_else(|| {
        constraint_properties
          .get("")
          .and_then(|properties| layout_property_value(properties, target))
      })
  };
  let reference_extent = constraint_properties
    .values()
    .flat_map(|properties| {
      [
        layout_property_value(properties, dgm::ConstraintValues::Width),
        layout_property_value(properties, dgm::ConstraintValues::Height),
      ]
    })
    .flatten()
    .filter(|value| value.is_finite() && *value > f32::EPSILON)
    .fold(node.width.max(node.height), f32::max);
  let spacing_ratio = |target, fallback: f32| {
    resolved_constraint(target)
      .or_else(|| resolved_constraint(dgm::ConstraintValues::Spacing))
      .map(|spacing| spacing / reference_extent)
      .or_else(|| {
        constraints.iter().rev().find_map(|constraint| {
          (constraint.target == target
            && (constraint.for_name.is_empty() || constraint.for_name == node_name)
            && matches!(
              constraint.reference,
              dgm::ConstraintValues::Width | dgm::ConstraintValues::Height
            )
            && constraint.factor.is_finite())
          .then_some(constraint.factor)
        })
      })
      .unwrap_or(fallback)
      .clamp(-0.9, 4.0)
  };
  let primary_spacing = spacing_ratio(dgm::ConstraintValues::SiblingSpacing, 0.1);
  let secondary_spacing = spacing_ratio(dgm::ConstraintValues::SecondarySiblingSpacing, 0.3);
  let (space_width, space_height) =
    if matches!(direction, LinearDirection::Left | LinearDirection::Right) {
      (primary_spacing, secondary_spacing)
    } else {
      (secondary_spacing, primary_spacing)
    };
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
      child.x = child_cross_axis_offset(
        node.width,
        child.width,
        algorithm
          .child_alignment
          .or(algorithm.secondary_child_alignment),
        algorithm.node_horizontal_alignment,
        false,
      );
    } else {
      child.y = child_cross_axis_offset(
        node.height,
        child.height,
        algorithm
          .child_alignment
          .or(algorithm.secondary_child_alignment),
        algorithm.node_vertical_alignment,
        true,
      );
    }

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
  if direction == LinearDirection::Right {
    for child in &mut node.children {
      child.x = node.width - child.x - child.width;
    }
  }
  if direction == LinearDirection::Bottom {
    for child in &mut node.children {
      child.y = node.height - child.y - child.height;
    }
  }
  let alignment_offset = resolved_constraint(dgm::ConstraintValues::AlignmentOffset)
    .filter(|offset| offset.is_finite())
    .map(|offset| offset.clamp(0.0, 1.0));
  align_children_in_parent_with_offset(
    node,
    algorithm.hierarchy_horizontal_alignment,
    algorithm.hierarchy_vertical_alignment,
    alignment_offset,
  );
}

fn vertical_shapes_count(node: &DiagramShapeNode) -> usize {
  let Some(algorithm) = node.algorithms.last().cloned() else {
    return if node.is_connector { 0 } else { 1 };
  };
  if node.children.is_empty() {
    return if node.is_connector { 0 } else { 1 };
  }
  let direction = if algorithm.kind == dgm::AlgorithmValues::HierarchyRoot
    || algorithm.child_direction == Some(dgm::ChildDirectionValues::Vertical)
  {
    LinearDirection::Top
  } else if algorithm.child_direction == Some(dgm::ChildDirectionValues::Horizontal)
    && matches!(
      algorithm.linear_direction,
      LinearDirection::Top | LinearDirection::Bottom
    )
  {
    LinearDirection::Left
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

  let mut snake_constraints = expand_constraints_for_children(node, constraints);
  let mut properties_by_name: HashMap<String, HashMap<dgm::ConstraintValues, f32>> = HashMap::new();
  properties_by_name.insert(
    String::new(),
    HashMap::from([
      (dgm::ConstraintValues::Width, shape_width),
      (dgm::ConstraintValues::Height, shape_height),
    ]),
  );
  for child in &node.children {
    properties_by_name
      .entry(child.internal_name.clone())
      .or_insert_with(|| {
        HashMap::from([
          (dgm::ConstraintValues::Width, shape_width),
          (dgm::ConstraintValues::Height, shape_height),
        ])
      });
    for direct in &child.direct_constraints {
      let mut direct = direct.clone();
      if direct.for_name.is_empty() {
        direct.for_name.clone_from(&child.internal_name);
      }
      if direct.ref_for_name.is_empty() && direct.reference != dgm::ConstraintValues::None {
        direct.ref_for_name.clone_from(&child.internal_name);
      }
      snake_constraints.push(direct);
    }
  }
  solve_layout_constraints(&snake_constraints, &mut properties_by_name);

  let shape_sizes: Vec<(f32, f32)> = node
    .children
    .iter()
    .map(|child| {
      let properties = properties_by_name.get(child.internal_name.as_str());
      (
        properties
          .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Width))
          .unwrap_or(shape_width)
          .max(0.0),
        properties
          .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Height))
          .unwrap_or(shape_height)
          .max(0.0),
      )
    })
    .collect();
  let shape_widths: Vec<f32> = shape_sizes.iter().map(|size| size.0).collect();

  let spacing_value = properties_by_name
    .get("")
    .and_then(|properties| layout_property_value(properties, dgm::ConstraintValues::Spacing));
  let normalized_spacing = spacing_value
    .map(|spacing| spacing / shape_width.max(f32::EPSILON))
    .filter(|spacing| spacing.is_finite())
    .map(|spacing| spacing.clamp(-0.9, 4.0));
  let space_from_constraints = normalized_spacing.is_some();
  let space_from_constraint = normalized_spacing.unwrap_or(1.0);
  let (increment_x, increment_y) = match algorithm.grow_direction {
    GrowDirection::TopLeft => (1.0, 1.0),
    GrowDirection::TopRight => (-1.0, 1.0),
    GrowDirection::BottomLeft => (1.0, -1.0),
    GrowDirection::BottomRight => (-1.0, -1.0),
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
  if child_aspect_ratio != 0.0 && count as f32 <= child_aspect_ratio {
    rows = count;
  } else {
    for candidate_rows in 1..count {
      let candidate_columns = count.div_ceil(candidate_rows);
      rows = candidate_rows;
      columns = candidate_columns;
      let row_width: f32 = shape_widths.iter().take(candidate_columns).sum();
      if row_width != 0.0 && shape_height * candidate_rows as f32 / row_width >= grid_aspect_ratio {
        break;
      }
    }
  }
  match algorithm.breakpoint {
    dgm::BreakpointValues::Fixed => {
      columns = algorithm.breakpoint_fixed_value.min(count).max(1);
      rows = count.div_ceil(columns);
    }
    dgm::BreakpointValues::Balanced => {
      let canvas_aspect = (node.width / node.height.max(f32::EPSILON)).max(f32::EPSILON);
      columns = ((count as f32 * canvas_aspect).sqrt().ceil() as usize).clamp(1, count.max(1));
      rows = count.div_ceil(columns);
    }
    dgm::BreakpointValues::EndCanvas => {}
  }
  let max_row_width = shape_widths
    .chunks(columns.max(1))
    .map(|row| row.iter().sum())
    .fold(0.0_f32, f32::max);

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
  }

  let mut x = if increment_x == -1.0 {
    node.width - child_width
  } else {
    0.0
  };
  let mut y = if increment_y == -1.0 {
    node.height - child_height
  } else if space_from_constraints && algorithm.flow_direction == dgm::FlowDirectionValues::Column {
    child_height * space * 2.0
  } else {
    0.0
  };
  let start_x = x;
  let mut column_index = 0usize;
  let mut row_height = 0.0_f32;
  let widths_from_constraints = shape_widths
    .windows(2)
    .any(|widths| (widths[0] - widths[1]).abs() > f32::EPSILON);
  let row_width_sums: Vec<f32> = shape_widths
    .chunks(columns.max(1))
    .map(|row| row.iter().sum())
    .collect();
  for (index, child) in node.children.iter_mut().enumerate() {
    child.x = x;
    child.y = y;
    let mut current_width = child_width;
    let mut current_height = child_height;
    if widths_from_constraints && max_row_width != 0.0 {
      let row = index / columns.max(1);
      let row_length = count.saturating_sub(row * columns).min(columns);
      let gap = child_width * space;
      let available_width = (node.width - gap * row_length.saturating_sub(1) as f32).max(0.0);
      let row_width = row_width_sums.get(row).copied().unwrap_or(max_row_width);
      current_width = lo_i32(available_width * shape_widths[index] / row_width.max(f32::EPSILON));
      let raw_size = shape_sizes[index];
      if raw_size.0 > f32::EPSILON && raw_size.1 > f32::EPSILON {
        current_height = lo_i32(current_width * raw_size.1 / raw_size.0);
      }
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
  if algorithm.offset == dgm::OffsetValues::Offset {
    for (index, child) in node.children.iter_mut().enumerate() {
      let line = index / columns.max(1);
      if line.is_multiple_of(2) {
        continue;
      }
      child.x += increment_x * child.width * 0.5;
    }
  }
  if algorithm.flow_direction == dgm::FlowDirectionValues::Column {
    for child in &mut node.children {
      let old_x = child.x;
      let old_y = child.y;
      let old_width = child.width;
      let old_height = child.height;
      child.x = old_y / node.height.max(f32::EPSILON) * node.width;
      child.y = old_x / node.width.max(f32::EPSILON) * node.height;
      child.width = old_height / node.height.max(f32::EPSILON) * node.width;
      child.height = old_width / node.width.max(f32::EPSILON) * node.height;
    }
  }
}

fn lo_i32(value: f32) -> f32 {
  (value as i32) as f32
}

#[derive(Clone, Debug)]
struct DiagramNamedBounds {
  name: String,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
}

fn resolve_diagram_connector_targets(root: &mut DiagramShapeNode) {
  let mut bounds = Vec::new();
  collect_diagram_named_bounds(root, 0.0, 0.0, &mut bounds);
  resolve_diagram_connector_targets_in_node(root, 0.0, 0.0, &bounds);
}

fn collect_diagram_named_bounds(
  node: &DiagramShapeNode,
  parent_x: f32,
  parent_y: f32,
  output: &mut Vec<DiagramNamedBounds>,
) {
  let x = parent_x + node.x;
  let y = parent_y + node.y;
  if !node.internal_name.is_empty() {
    output.push(DiagramNamedBounds {
      name: node.internal_name.clone(),
      x,
      y,
      width: node.width,
      height: node.height,
    });
  }
  for child in &node.children {
    collect_diagram_named_bounds(child, x, y, output);
  }
}

fn resolve_diagram_connector_targets_in_node(
  node: &mut DiagramShapeNode,
  parent_x: f32,
  parent_y: f32,
  bounds: &[DiagramNamedBounds],
) {
  let x = parent_x + node.x;
  let y = parent_y + node.y;
  if node.is_connector {
    let connector_center = (x + node.width / 2.0, y + node.height / 2.0);
    let source = node
      .connector_source_node
      .as_deref()
      .and_then(|name| nearest_diagram_named_bounds(name, connector_center, bounds));
    let destination = node
      .connector_destination_node
      .as_deref()
      .and_then(|name| nearest_diagram_named_bounds(name, connector_center, bounds));
    let (default_start, default_end) =
      default_diagram_connector_points(node, x, y, connector_center);
    let source_target = destination
      .map(diagram_bounds_center)
      .unwrap_or(default_end);
    let destination_target = source.map(diagram_bounds_center).unwrap_or(default_start);
    node.connector_start_override = source.map(|source| {
      let point = diagram_bounds_attachment(
        source,
        node.connector_begin_points.as_deref(),
        source_target,
        node.connector_route_shortest_distance,
      );
      (point.0 - x, point.1 - y)
    });
    node.connector_end_override = destination.map(|destination| {
      let point = diagram_bounds_attachment(
        destination,
        node.connector_end_points.as_deref(),
        destination_target,
        node.connector_route_shortest_distance,
      );
      (point.0 - x, point.1 - y)
    });
  }
  for child in &mut node.children {
    resolve_diagram_connector_targets_in_node(child, x, y, bounds);
  }
}

fn nearest_diagram_named_bounds<'a>(
  name: &str,
  point: (f32, f32),
  bounds: &'a [DiagramNamedBounds],
) -> Option<&'a DiagramNamedBounds> {
  bounds
    .iter()
    .filter(|bounds| bounds.name == name)
    .min_by(|left, right| {
      let distance = |bounds: &DiagramNamedBounds| {
        let center = diagram_bounds_center(bounds);
        (center.0 - point.0).powi(2) + (center.1 - point.1).powi(2)
      };
      distance(left).total_cmp(&distance(right))
    })
}

fn diagram_bounds_center(bounds: &DiagramNamedBounds) -> (f32, f32) {
  (
    bounds.x + bounds.width / 2.0,
    bounds.y + bounds.height / 2.0,
  )
}

fn default_diagram_connector_points(
  node: &DiagramShapeNode,
  x: f32,
  y: f32,
  center: (f32, f32),
) -> ((f32, f32), (f32, f32)) {
  let length = node.width.max(node.height).max(1.0);
  let radians = node.connector_angle_deg.to_radians();
  let delta = (radians.cos() * length / 2.0, radians.sin() * length / 2.0);
  let bounds = DiagramNamedBounds {
    name: String::new(),
    x,
    y,
    width: node.width,
    height: node.height,
  };
  (
    diagram_bounds_attachment(
      &bounds,
      node.connector_begin_points.as_deref(),
      (center.0 - delta.0, center.1 - delta.1),
      false,
    ),
    diagram_bounds_attachment(
      &bounds,
      node.connector_end_points.as_deref(),
      (center.0 + delta.0, center.1 + delta.1),
      false,
    ),
  )
}

fn diagram_bounds_attachment(
  bounds: &DiagramNamedBounds,
  authored: Option<&str>,
  toward: (f32, f32),
  shortest: bool,
) -> (f32, f32) {
  let center = diagram_bounds_center(bounds);
  let top = (center.0, bounds.y);
  let bottom = (center.0, bounds.y + bounds.height);
  let left = (bounds.x, center.1);
  let right = (bounds.x + bounds.width, center.1);
  let top_left = (bounds.x, bounds.y);
  let top_right = (bounds.x + bounds.width, bounds.y);
  let bottom_left = (bounds.x, bounds.y + bounds.height);
  let bottom_right = (bounds.x + bounds.width, bounds.y + bounds.height);
  match authored {
    Some("tCtr") => top,
    Some("bCtr") => bottom,
    Some("ctr") => center,
    Some("midL") => left,
    Some("midR") => right,
    Some("tL") => top_left,
    Some("tR") => top_right,
    Some("bL") => bottom_left,
    Some("bR") => bottom_right,
    Some("midL midR") => {
      if (left.0 - toward.0).abs() <= (right.0 - toward.0).abs() {
        left
      } else {
        right
      }
    }
    Some("radial") | None if !shortest => {
      let dx = toward.0 - center.0;
      let dy = toward.1 - center.1;
      if dx.abs() <= f32::EPSILON && dy.abs() <= f32::EPSILON {
        return center;
      }
      let tx = if dx.abs() > f32::EPSILON {
        bounds.width.abs() / 2.0 / dx.abs()
      } else {
        f32::INFINITY
      };
      let ty = if dy.abs() > f32::EPSILON {
        bounds.height.abs() / 2.0 / dy.abs()
      } else {
        f32::INFINITY
      };
      let scale = tx.min(ty);
      (center.0 + dx * scale, center.1 + dy * scale)
    }
    Some("radial") | None | Some(_) => [
      top,
      bottom,
      left,
      right,
      top_left,
      top_right,
      bottom_left,
      bottom_right,
    ]
    .into_iter()
    .min_by(|left, right| {
      let distance =
        |point: (f32, f32)| (point.0 - toward.0).powi(2) + (point.1 - toward.1).powi(2);
      distance(*left).total_cmp(&distance(*right))
    })
    .unwrap_or(center),
  }
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
      connector_route: node.connector_route,
      connector_dimension: node.connector_dimension,
      connector_bend_at_end: node.connector_bend_at_end,
      connector_begin_arrow: node.connector_begin_arrow,
      connector_end_arrow: node.connector_end_arrow,
      connector_begin_points: node.connector_begin_points.clone(),
      connector_end_points: node.connector_end_points.clone(),
      connector_beginning_padding: node.connector_beginning_padding,
      connector_end_padding: node.connector_end_padding,
      connector_bending_distance: node.connector_bending_distance,
      connector_start_override: node
        .connector_start_override
        .map(|point| (x + point.0, y + point.1)),
      connector_end_override: node
        .connector_end_override
        .map(|point| (x + point.0, y + point.1)),
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

#[cfg(test)]
mod tests {
  use super::*;

  fn parameter(kind: dgm::ParameterIdValues, value: &str) -> dgm::Parameter {
    dgm::Parameter {
      r#type: kind,
      val: Some(value.to_string()),
    }
  }

  #[test]
  fn spatial_vertical_alignment_is_not_used_as_text_anchor() {
    let algorithm = dgm::Algorithm {
      r#type: dgm::AlgorithmValues::Linear,
      parameter: vec![
        parameter(dgm::ParameterIdValues::VerticalAlignment, "b"),
        parameter(dgm::ParameterIdValues::TextAnchorVertical, "t"),
      ],
      ..dgm::Algorithm::default()
    };
    let algorithm = layout_algorithm(&algorithm);
    assert_eq!(algorithm.vertical_alignment, Some(AxisAlignment::End));
    assert_eq!(
      algorithm.text_anchor_vertical,
      Some(dgm::TextAnchorVerticalValues::Top)
    );
  }

  #[test]
  fn connector_parameters_retain_route_ends_and_attachment_sets() {
    let algorithm = dgm::Algorithm {
      r#type: dgm::AlgorithmValues::Connector,
      parameter: vec![
        parameter(dgm::ParameterIdValues::ConnectionRoute, "curve"),
        parameter(dgm::ParameterIdValues::BendPoint, "beg"),
        parameter(dgm::ParameterIdValues::BeginningArrowheadStyle, "arr"),
        parameter(dgm::ParameterIdValues::EndStyle, "arr"),
        parameter(dgm::ParameterIdValues::BeginningPoints, "midL midR"),
        parameter(dgm::ParameterIdValues::EndPoints, "tCtr"),
        parameter(dgm::ParameterIdValues::SourceNode, "source"),
        parameter(dgm::ParameterIdValues::DestinationNode, "destination"),
        parameter(dgm::ParameterIdValues::ConnectorDimension, "2D"),
        parameter(dgm::ParameterIdValues::RouteShortestDistance, "true"),
      ],
      ..dgm::Algorithm::default()
    };
    let algorithm = layout_algorithm(&algorithm);
    assert_eq!(algorithm.connector_route, DiagramConnectorRoute::Curve);
    assert!(!algorithm.connector_bend_at_end);
    assert!(algorithm.connector_begin_arrow);
    assert!(algorithm.connector_end_arrow);
    assert_eq!(
      algorithm.connector_begin_points,
      ConnectorPointSet::MiddleLeftOrRight
    );
    assert_eq!(algorithm.connector_end_points, ConnectorPointSet::TopCenter);
    assert_eq!(algorithm.connector_source_node.as_deref(), Some("source"));
    assert_eq!(
      algorithm.connector_destination_node.as_deref(),
      Some("destination")
    );
    assert_eq!(
      algorithm.connector_dimension,
      dgm::ConnectorDimensionValues::TwoDimension
    );
    assert!(algorithm.connector_route_shortest_distance);
  }

  #[test]
  fn snake_parameters_retain_flow_breakpoint_and_offset() {
    let algorithm = dgm::Algorithm {
      r#type: dgm::AlgorithmValues::Snake,
      parameter: vec![
        parameter(dgm::ParameterIdValues::FlowDirection, "col"),
        parameter(dgm::ParameterIdValues::Breakpoint, "fixed"),
        parameter(dgm::ParameterIdValues::BreakpointFixedValue, "3"),
        parameter(dgm::ParameterIdValues::Offset, "off"),
        parameter(dgm::ParameterIdValues::ContinueDirection, "sameDir"),
      ],
      ..dgm::Algorithm::default()
    };
    let algorithm = layout_algorithm(&algorithm);
    assert_eq!(algorithm.flow_direction, dgm::FlowDirectionValues::Column);
    assert_eq!(algorithm.breakpoint, dgm::BreakpointValues::Fixed);
    assert_eq!(algorithm.breakpoint_fixed_value, 3);
    assert_eq!(algorithm.offset, dgm::OffsetValues::Offset);
    assert_eq!(
      algorithm.continue_direction,
      ContinueDirection::SameDirection
    );
  }

  #[test]
  fn named_connector_attachment_uses_the_requested_shape_edge() {
    let bounds = DiagramNamedBounds {
      name: "node".to_string(),
      x: 10.0,
      y: 20.0,
      width: 40.0,
      height: 30.0,
    };
    assert_eq!(
      diagram_bounds_attachment(&bounds, Some("bCtr"), (30.0, 100.0), false),
      (30.0, 50.0)
    );
    assert_eq!(
      diagram_bounds_attachment(&bounds, Some("midL midR"), (100.0, 35.0), true),
      (50.0, 35.0)
    );
  }
}
