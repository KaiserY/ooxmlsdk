use std::collections::{HashMap, HashSet};

use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;

use crate::docx::RgbColor;

#[derive(Clone, Debug)]
pub(crate) struct DiagramShape {
  pub(crate) x: f32,
  pub(crate) y: f32,
  pub(crate) width: f32,
  pub(crate) height: f32,
  pub(crate) text: String,
  pub(crate) fill: RgbColor,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct DiagramStyleColors {
  pub(crate) fill_by_label: HashMap<String, Vec<RgbColor>>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct DiagramBounds {
  pub(crate) x: f32,
  pub(crate) y: f32,
  pub(crate) width: f32,
  pub(crate) height: f32,
}

pub(crate) fn layout_shapes(
  data: &dgm::DataModelRoot,
  layout: Option<&dgm::LayoutDefinition>,
  colors: Option<&DiagramStyleColors>,
  bounds: DiagramBounds,
  accent_fill: RgbColor,
) -> Vec<DiagramShape> {
  // Source: LibreOffice oox/source/drawingml/diagram/diagram.cxx
  // SmartArtDiagram::createShapeHierarchyFromModel() creates shapes from the
  // diagram data model, then applies layout atoms. This is the shared entry
  // point for that same model-to-shapes stage.
  let mut nodes = ordered_document_nodes(data, layout, colors, accent_fill);
  nodes.retain(|node| !node.text.is_empty());
  nodes.sort_by_key(|node| node.order);
  if nodes.is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
    return Vec::new();
  }

  match root_algorithm(data, layout) {
    Some(algorithm) if algorithm.kind == dgm::AlgorithmValues::Snake => {
      snake_layout(&nodes, bounds, algorithm.reverse)
    }
    Some(algorithm)
      if matches!(
        algorithm.kind,
        dgm::AlgorithmValues::Linear | dgm::AlgorithmValues::Composite
      ) =>
    {
      linear_layout(&nodes, bounds, algorithm.reverse)
    }
    Some(algorithm) if algorithm.kind == dgm::AlgorithmValues::Cycle => {
      cycle_layout(&nodes, bounds, algorithm.reverse)
    }
    _ => snake_layout(&nodes, bounds, false),
  }
}

#[derive(Clone, Debug)]
struct DiagramNode {
  text: String,
  fill: RgbColor,
  preferred_height: Option<f32>,
  height_width_factor: Option<f32>,
  order: usize,
}

#[derive(Clone, Copy, Debug)]
struct LayoutAlgorithm {
  kind: dgm::AlgorithmValues,
  reverse: bool,
}

fn ordered_document_nodes(
  data: &dgm::DataModelRoot,
  layout: Option<&dgm::LayoutDefinition>,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
) -> Vec<DiagramNode> {
  let points = &data.point_list.point;
  if let Some(nodes) =
    ordered_presentation_nodes(data, layout_node_metrics(layout), colors, fallback_fill)
    && !nodes.is_empty()
  {
    return nodes;
  }

  let Some(root_id) = points
    .iter()
    .find(|point| point.r#type == Some(dgm::PointValues::Document))
    .map(|point| point.model_id.as_str().to_string())
  else {
    return Vec::new();
  };

  let mut nodes = Vec::new();
  if let Some(connections) = data.connection_list.as_ref() {
    let point_by_id: HashMap<&str, &dgm::Point> = points
      .iter()
      .map(|point| (point.model_id.as_str(), point))
      .collect();
    let presentation_by_data = presentation_points_by_data(points, connections);
    let mut children_by_source: HashMap<&str, Vec<&dgm::Connection>> = HashMap::new();
    for connection in &connections.connection {
      if connection
        .r#type
        .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
      {
        children_by_source
          .entry(connection.source_id.as_str())
          .or_default()
          .push(connection);
      }
    }
    for child_connections in children_by_source.values_mut() {
      child_connections.sort_by_key(|connection| connection.source_position);
    }
    let mut visited = HashSet::new();
    append_document_nodes(
      root_id.as_str(),
      0,
      &point_by_id,
      &children_by_source,
      &presentation_by_data,
      &mut visited,
      &mut nodes,
      colors,
      fallback_fill,
    );
  }
  if !nodes.is_empty() {
    return nodes;
  }

  points
    .iter()
    .filter(|point| {
      point
        .r#type
        .is_none_or(|kind| kind == dgm::PointValues::Node)
    })
    .map(|point| DiagramNode {
      text: point_text(point),
      fill: fallback_fill,
      preferred_height: None,
      height_width_factor: None,
      order: usize::MAX,
    })
    .collect()
}

fn presentation_points_by_data<'a>(
  points: &'a [dgm::Point],
  connections: &'a dgm::ConnectionList,
) -> HashMap<&'a str, &'a dgm::Point> {
  let point_by_id: HashMap<&str, &dgm::Point> = points
    .iter()
    .map(|point| (point.model_id.as_str(), point))
    .collect();
  connections
    .connection
    .iter()
    .filter(|connection| connection.r#type == Some(dgm::ConnectionValues::PresentationOf))
    .filter_map(|connection| {
      Some((
        connection.source_id.as_str(),
        *point_by_id.get(connection.destination_id.as_str())?,
      ))
    })
    .collect()
}

fn document_node_orders(data: &dgm::DataModelRoot) -> HashMap<String, usize> {
  let mut orders = HashMap::new();
  let Some(root_id) = data
    .point_list
    .point
    .iter()
    .find(|point| point.r#type == Some(dgm::PointValues::Document))
    .map(|point| point.model_id.as_str())
  else {
    return orders;
  };
  let Some(connections) = data.connection_list.as_ref() else {
    return orders;
  };
  let point_by_id: HashMap<&str, &dgm::Point> = data
    .point_list
    .point
    .iter()
    .map(|point| (point.model_id.as_str(), point))
    .collect();
  let mut children_by_source: HashMap<&str, Vec<&dgm::Connection>> = HashMap::new();
  for connection in &connections.connection {
    if connection
      .r#type
      .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
    {
      children_by_source
        .entry(connection.source_id.as_str())
        .or_default()
        .push(connection);
    }
  }
  for children in children_by_source.values_mut() {
    children.sort_by_key(|connection| connection.source_position);
  }
  let mut visited = HashSet::new();
  append_document_node_orders(
    root_id,
    &point_by_id,
    &children_by_source,
    &mut visited,
    &mut orders,
  );
  orders
}

fn append_document_node_orders<'a>(
  parent_id: &str,
  point_by_id: &HashMap<&'a str, &'a dgm::Point>,
  children_by_source: &HashMap<&'a str, Vec<&'a dgm::Connection>>,
  visited: &mut HashSet<&'a str>,
  orders: &mut HashMap<String, usize>,
) {
  let Some(children) = children_by_source.get(parent_id) else {
    return;
  };
  for connection in children {
    let child_id = connection.destination_id.as_str();
    if !visited.insert(child_id) {
      continue;
    }
    if let Some(point) = point_by_id.get(child_id)
      && point
        .r#type
        .is_none_or(|kind| matches!(kind, dgm::PointValues::Node | dgm::PointValues::Assistant))
    {
      orders.insert(child_id.to_string(), orders.len());
    }
    append_document_node_orders(child_id, point_by_id, children_by_source, visited, orders);
  }
}

fn ordered_presentation_nodes(
  data: &dgm::DataModelRoot,
  mut metrics: LayoutNodeMetrics,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
) -> Option<Vec<DiagramNode>> {
  let connections = data.connection_list.as_ref()?;
  metrics.data_orders = document_node_orders(data);
  let point_by_id: HashMap<&str, &dgm::Point> = data
    .point_list
    .point
    .iter()
    .map(|point| (point.model_id.as_str(), point))
    .collect();
  let mut children_by_source: HashMap<&str, Vec<&dgm::Connection>> = HashMap::new();
  let mut data_by_presentation: HashMap<&str, &dgm::Point> = HashMap::new();
  for connection in &connections.connection {
    match connection.r#type {
      Some(dgm::ConnectionValues::PresentationParentOf) => children_by_source
        .entry(connection.source_id.as_str())
        .or_default()
        .push(connection),
      Some(dgm::ConnectionValues::PresentationOf) => {
        if let (Some(data_point), Some(presentation_point)) = (
          point_by_id.get(connection.source_id.as_str()),
          point_by_id.get(connection.destination_id.as_str()),
        ) {
          data_by_presentation.insert(presentation_point.model_id.as_str(), data_point);
        }
      }
      _ => {}
    }
  }
  for children in children_by_source.values_mut() {
    children.sort_by_key(|connection| connection.source_position);
  }

  let root_presentation = data
    .point_list
    .point
    .iter()
    .find(|point| {
      point.r#type == Some(dgm::PointValues::Presentation)
        && associated_data_point(point, &point_by_id)
          .is_some_and(|point| point.r#type == Some(dgm::PointValues::Document))
    })
    .or_else(|| {
      data.point_list.point.iter().find(|point| {
        point.r#type == Some(dgm::PointValues::Presentation)
          && presentation_name(point).is_some_and(|name| metrics.is_text_shape(name))
      })
    })?;

  let mut nodes = Vec::new();
  let mut visited = HashSet::new();
  append_presentation_nodes(
    root_presentation.model_id.as_str(),
    &point_by_id,
    &data_by_presentation,
    &children_by_source,
    &metrics,
    &mut visited,
    &mut nodes,
    colors,
    fallback_fill,
  );
  Some(nodes)
}

fn append_presentation_nodes<'a>(
  parent_id: &str,
  point_by_id: &HashMap<&'a str, &'a dgm::Point>,
  data_by_presentation: &HashMap<&'a str, &'a dgm::Point>,
  children_by_source: &HashMap<&'a str, Vec<&'a dgm::Connection>>,
  metrics: &LayoutNodeMetrics,
  visited: &mut HashSet<&'a str>,
  nodes: &mut Vec<DiagramNode>,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
) {
  let Some(child_connections) = children_by_source.get(parent_id) else {
    return;
  };
  for connection in child_connections {
    let child_id = connection.destination_id.as_str();
    if !visited.insert(child_id) {
      continue;
    }
    if let Some(presentation_point) = point_by_id.get(child_id) {
      if let Some(name) = presentation_name(presentation_point)
        && metrics.is_text_shape(name)
        && let Some(data_point) = data_by_presentation
          .get(child_id)
          .copied()
          .or_else(|| associated_data_point(presentation_point, point_by_id))
      {
        let text = point_text(data_point);
        if !text.is_empty() {
          nodes.push(DiagramNode {
            text,
            fill: diagram_node_fill(Some(presentation_point), colors, fallback_fill),
            preferred_height: metrics.preferred_heights.get(name).copied(),
            height_width_factor: metrics.height_width_factors.get(name).copied(),
            order: metrics
              .data_orders
              .get(data_point.model_id.as_str())
              .copied()
              .unwrap_or(usize::MAX),
          });
        }
      }
      append_presentation_nodes(
        child_id,
        point_by_id,
        data_by_presentation,
        children_by_source,
        metrics,
        visited,
        nodes,
        colors,
        fallback_fill,
      );
    }
  }
}

fn presentation_name(point: &dgm::Point) -> Option<&str> {
  point
    .property_set
    .as_deref()
    .and_then(|properties| properties.presentation_name.as_deref())
}

fn associated_data_point<'a>(
  presentation_point: &dgm::Point,
  point_by_id: &HashMap<&'a str, &'a dgm::Point>,
) -> Option<&'a dgm::Point> {
  presentation_point
    .property_set
    .as_deref()
    .and_then(|properties| properties.presentation_element_id.as_deref())
    .and_then(|id| point_by_id.get(id))
    .copied()
}

#[derive(Clone, Debug, Default)]
struct LayoutNodeMetrics {
  shape_names: HashSet<String>,
  text_names: HashSet<String>,
  preferred_heights: HashMap<String, f32>,
  height_width_factors: HashMap<String, f32>,
  data_orders: HashMap<String, usize>,
}

impl LayoutNodeMetrics {
  fn is_text_shape(&self, name: &str) -> bool {
    if self.text_names.is_empty() {
      self.shape_names.contains(name)
    } else {
      self.text_names.contains(name)
    }
  }
}

fn layout_node_metrics(layout: Option<&dgm::LayoutDefinition>) -> LayoutNodeMetrics {
  let mut metrics = LayoutNodeMetrics::default();
  if let Some(layout) = layout {
    collect_layout_node_metrics(&layout.layout_node, &mut metrics);
  }
  metrics
}

fn collect_layout_node_metrics(node: &dgm::LayoutNode, metrics: &mut LayoutNodeMetrics) {
  let has_shape = node
    .layout_node_choice
    .iter()
    .any(|choice| matches!(choice, dgm::LayoutNodeChoice::Shape(_)));
  let has_text_algorithm = node.layout_node_choice.iter().any(|choice| {
    matches!(
      choice,
      dgm::LayoutNodeChoice::Algorithm(algorithm)
        if algorithm.r#type == dgm::AlgorithmValues::Text
    )
  });
  let has_self_presentation = node
    .layout_node_choice
    .iter()
    .filter_map(|choice| match choice {
      dgm::LayoutNodeChoice::PresentationOf(presentation_of) => Some(presentation_of),
      _ => None,
    })
    .next()
    .is_none_or(|presentation_of| {
      presentation_of.axis.as_ref().is_none_or(|axes| {
        axes.is_empty() || axes.iter().any(|axis| *axis == dgm::AxisValues::_Self)
      })
    });
  if has_shape && let Some(name) = node.name.as_deref() {
    metrics.shape_names.insert(name.to_string());
    if has_text_algorithm && has_self_presentation {
      metrics.text_names.insert(name.to_string());
    }
  }
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
  let mut font_sizes: HashMap<String, f32> = HashMap::new();
  let node_name = node.name.as_deref();
  for choice in &node.layout_node_choice {
    let dgm::LayoutNodeChoice::Constraints(constraints) = choice else {
      continue;
    };
    for constraint in &constraints.constraint {
      if constraint.r#type == dgm::ConstraintValues::Height
        && constraint.reference_type == Some(dgm::ConstraintValues::Width)
        && let Some(factor) = constraint.fact
        && let Some(name) = constraint.for_name.as_deref().or(node_name)
      {
        metrics
          .height_width_factors
          .insert(name.to_string(), factor as f32);
      }
      if constraint.r#type == dgm::ConstraintValues::PrimaryFontSize
        && let (Some(name), Some(value)) = (constraint.for_name.as_deref(), constraint.val)
      {
        font_sizes.insert(name.to_string(), value as f32);
      }
    }
    for constraint in &constraints.constraint {
      if constraint.r#type == dgm::ConstraintValues::Height
        && constraint.reference_type == Some(dgm::ConstraintValues::PrimaryFontSize)
        && let (Some(name), Some(factor)) = (constraint.for_name.as_deref(), constraint.fact)
        && let Some(font_size) = font_sizes.get(name)
      {
        metrics
          .preferred_heights
          .insert(name.to_string(), font_size * factor as f32);
      }
    }
  }
}

fn append_document_nodes<'a>(
  parent_id: &str,
  depth: u32,
  point_by_id: &HashMap<&'a str, &'a dgm::Point>,
  children_by_source: &HashMap<&'a str, Vec<&'a dgm::Connection>>,
  presentation_by_data: &HashMap<&'a str, &'a dgm::Point>,
  visited: &mut HashSet<&'a str>,
  nodes: &mut Vec<DiagramNode>,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
) {
  let Some(child_connections) = children_by_source.get(parent_id) else {
    return;
  };
  for connection in child_connections {
    let child_id = connection.destination_id.as_str();
    if !visited.insert(child_id) {
      continue;
    }
    if let Some(point) = point_by_id.get(child_id) {
      if point
        .r#type
        .is_none_or(|kind| matches!(kind, dgm::PointValues::Node | dgm::PointValues::Assistant))
      {
        nodes.push(DiagramNode {
          text: point_text(point),
          fill: diagram_node_fill(presentation_by_data.get(child_id), colors, fallback_fill),
          preferred_height: None,
          height_width_factor: None,
          order: nodes.len(),
        });
      }
      append_document_nodes(
        child_id,
        depth.saturating_add(1),
        point_by_id,
        children_by_source,
        presentation_by_data,
        visited,
        nodes,
        colors,
        fallback_fill,
      );
    }
  }
}

fn diagram_node_fill(
  presentation_point: Option<&&dgm::Point>,
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
    .and_then(|fills| {
      if fills.is_empty() {
        None
      } else {
        Some(fills[style_index % fills.len()])
      }
    })
    .unwrap_or(fallback_fill)
}

fn point_text(point: &dgm::Point) -> String {
  point
    .text_body
    .as_deref()
    .map(text_body_text)
    .unwrap_or_default()
    .trim()
    .to_string()
}

fn text_body_text(text_body: &dgm::TextBody) -> String {
  let mut text = String::new();
  for paragraph in &text_body.paragraph {
    if !text.is_empty() {
      text.push('\n');
    }
    for choice in &paragraph.paragraph_choice {
      match choice {
        ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::ParagraphChoice::Run(
          run,
        ) => text.push_str(run.text.as_str()),
        ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::ParagraphChoice::Field(
          field,
        ) => {
          if let Some(field_text) = field.text.as_ref() {
            text.push_str(field_text.as_str());
          }
        }
        ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::ParagraphChoice::Break(
          _,
        ) => text.push('\n'),
        ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::ParagraphChoice::TextMath(
          _,
        ) => {}
      }
    }
  }
  text
}

fn root_algorithm(
  data: &dgm::DataModelRoot,
  layout: Option<&dgm::LayoutDefinition>,
) -> Option<LayoutAlgorithm> {
  let reversed = document_direction(data) == Some(dgm::DirectionValues::Reversed);
  layout.and_then(|layout| first_algorithm(&layout.layout_node, reversed))
}

fn document_direction(data: &dgm::DataModelRoot) -> Option<dgm::DirectionValues> {
  data
    .point_list
    .point
    .iter()
    .filter_map(|point| point.property_set.as_ref())
    .filter_map(|properties| properties.presentation_layout_variables.as_deref())
    .filter_map(|variables| variables.direction.as_ref())
    .filter_map(|direction| direction.val)
    .find(|direction| *direction == dgm::DirectionValues::Reversed)
}

fn first_algorithm(node: &dgm::LayoutNode, reversed: bool) -> Option<LayoutAlgorithm> {
  for choice in &node.layout_node_choice {
    match choice {
      dgm::LayoutNodeChoice::Algorithm(algorithm) => {
        return Some(layout_algorithm(algorithm).with_reversed(reversed));
      }
      dgm::LayoutNodeChoice::Choose(choose) => {
        if !reversed {
          for branch in &choose.diagram_choose_if {
            if let Some(algorithm) = first_algorithm_in_if(branch) {
              return Some(algorithm.with_reversed(reversed));
            }
          }
        }
        if let Some(branch) = choose.diagram_choose_else.as_ref()
          && let Some(algorithm) = first_algorithm_in_else(branch)
        {
          return Some(algorithm.with_reversed(reversed));
        }
      }
      _ => {}
    }
  }
  None
}

fn first_algorithm_in_if(branch: &dgm::DiagramChooseIf) -> Option<LayoutAlgorithm> {
  for choice in &branch.diagram_choose_if_choice {
    if let dgm::DiagramChooseIfChoice::Algorithm(algorithm) = choice {
      return Some(layout_algorithm(algorithm));
    }
  }
  None
}

fn first_algorithm_in_else(branch: &dgm::DiagramChooseElse) -> Option<LayoutAlgorithm> {
  for choice in &branch.diagram_choose_else_choice {
    if let dgm::DiagramChooseElseChoice::Algorithm(algorithm) = choice {
      return Some(layout_algorithm(algorithm));
    }
  }
  None
}

fn layout_algorithm(algorithm: &dgm::Algorithm) -> LayoutAlgorithm {
  let reverse = algorithm.parameter.iter().any(|parameter| {
    matches!(
      (parameter.r#type, parameter.val.as_deref()),
      (
        dgm::ParameterIdValues::GrowDirection,
        Some("tR" | "bR" | "rT" | "rB")
      ) | (
        dgm::ParameterIdValues::LinearDirection,
        Some("fromR" | "fromB")
      )
    )
  });
  LayoutAlgorithm {
    kind: algorithm.r#type,
    reverse,
  }
}

impl LayoutAlgorithm {
  fn with_reversed(mut self, reversed: bool) -> Self {
    self.reverse |= reversed;
    self
  }
}

fn snake_layout(nodes: &[DiagramNode], bounds: DiagramBounds, reverse: bool) -> Vec<DiagramShape> {
  let columns = if nodes.len() <= 3 {
    nodes.len().max(1)
  } else {
    2
  };
  let rows = nodes.len().div_ceil(columns).max(1);
  grid_layout(nodes, bounds, columns, rows, reverse)
}

fn linear_layout(nodes: &[DiagramNode], bounds: DiagramBounds, reverse: bool) -> Vec<DiagramShape> {
  grid_layout(nodes, bounds, nodes.len().max(1), 1, reverse)
}

fn cycle_layout(nodes: &[DiagramNode], bounds: DiagramBounds, reverse: bool) -> Vec<DiagramShape> {
  let columns = ((nodes.len() as f32).sqrt().ceil() as usize).max(1);
  let rows = nodes.len().div_ceil(columns).max(1);
  grid_layout(nodes, bounds, columns, rows, reverse)
}

fn grid_layout(
  nodes: &[DiagramNode],
  bounds: DiagramBounds,
  columns: usize,
  rows: usize,
  reverse: bool,
) -> Vec<DiagramShape> {
  let gap = (bounds.width.min(bounds.height) * 0.04).clamp(4.0, 18.0);
  let cell_width = (bounds.width - gap * (columns.saturating_sub(1) as f32)) / columns as f32;
  let cell_height = (bounds.height - gap * (rows.saturating_sub(1) as f32)) / rows as f32;
  nodes
    .iter()
    .enumerate()
    .map(|(index, node)| {
      let column = if reverse {
        columns - 1 - index % columns
      } else {
        index % columns
      };
      let row = index / columns;
      let x = bounds.x + column as f32 * (cell_width + gap);
      let shape_height = node
        .height_width_factor
        .map(|factor| cell_width * factor)
        .or(node.preferred_height)
        .map(|height| height.max(18.0).min(cell_height))
        .unwrap_or_else(|| (cell_height * 0.72).max(18.0));
      let y = bounds.y + row as f32 * (cell_height + gap) + (cell_height - shape_height) / 2.0;
      DiagramShape {
        x,
        y,
        width: cell_width.max(1.0),
        height: shape_height,
        text: node.text.clone(),
        fill: node.fill,
      }
    })
    .collect()
}
