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
  pub(crate) order: usize,
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
  if let Some(layout) = layout
    && let Some(mut tree) = build_diagram_shape_tree(data, layout, colors, accent_fill, bounds)
  {
    layout_diagram_shape_tree(&mut tree);
    let mut shapes = Vec::new();
    flatten_diagram_shape_tree(&tree, bounds.x, bounds.y, &mut shapes);
    return shapes;
  }

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
      linear_layout(
        &nodes,
        bounds,
        algorithm.reverse,
        algorithm.kind == dgm::AlgorithmValues::Linear,
      )
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
  depth: usize,
}

#[derive(Clone, Debug)]
struct DiagramShapeNode {
  internal_name: String,
  text: String,
  fill: RgbColor,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  algorithm: Option<LayoutAlgorithm>,
  child_order: dgm::ChildOrderValues,
  has_geometry: bool,
  hidden_geometry: bool,
  is_connector: bool,
  order: usize,
  constraints: Vec<DiagramConstraint>,
  rules: Vec<DiagramRule>,
  children: Vec<DiagramShapeNode>,
}

#[derive(Clone, Debug)]
struct DiagramConstraint {
  for_name: String,
  ref_for_name: String,
  factor: f32,
  value: f32,
  target: dgm::ConstraintValues,
  reference: dgm::ConstraintValues,
  operator: dgm::BoolOperatorValues,
  r#for: Option<dgm::ConstraintRelationshipValues>,
  point_type: Option<dgm::ElementValues>,
}

#[derive(Clone, Debug)]
struct DiagramRule {
  for_name: String,
}

#[derive(Clone, Copy, Debug)]
struct LayoutAlgorithm {
  kind: dgm::AlgorithmValues,
  reverse: bool,
  linear_direction: LinearDirection,
  secondary_linear_direction: Option<LinearDirection>,
  start_angle: f32,
  span_angle: f32,
  center_shape_mapping_first_node: bool,
  aspect_ratio: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LinearDirection {
  #[default]
  FromLeft,
  FromRight,
  FromTop,
  FromBottom,
}

fn ordered_document_nodes(
  data: &dgm::DataModelRoot,
  layout: Option<&dgm::LayoutDefinition>,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
) -> Vec<DiagramNode> {
  let points = &data.point_list.point;
  if let Some(nodes) = ordered_presentation_nodes(
    data,
    layout,
    layout_node_metrics(layout),
    colors,
    fallback_fill,
  ) {
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
      depth: 0,
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

pub(crate) fn presentation_point_list_orders(data: &dgm::DataModelRoot) -> HashMap<String, usize> {
  let data_orders: HashMap<&str, usize> = data
    .point_list
    .point
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

fn document_node_depths(data: &dgm::DataModelRoot) -> HashMap<String, usize> {
  let mut depths = HashMap::new();
  let Some(root_id) = data
    .point_list
    .point
    .iter()
    .find(|point| point.r#type == Some(dgm::PointValues::Document))
    .map(|point| point.model_id.as_str())
  else {
    return depths;
  };
  let Some(connections) = data.connection_list.as_ref() else {
    return depths;
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
  append_document_node_depths(
    root_id,
    0,
    &point_by_id,
    &children_by_source,
    &mut visited,
    &mut depths,
  );
  depths
}

fn append_document_node_depths<'a>(
  parent_id: &str,
  parent_depth: usize,
  point_by_id: &HashMap<&'a str, &'a dgm::Point>,
  children_by_source: &HashMap<&'a str, Vec<&'a dgm::Connection>>,
  visited: &mut HashSet<&'a str>,
  depths: &mut HashMap<String, usize>,
) {
  let Some(children) = children_by_source.get(parent_id) else {
    return;
  };
  for connection in children {
    let child_id = connection.destination_id.as_str();
    if !visited.insert(child_id) {
      continue;
    }
    let depth = parent_depth + 1;
    if let Some(point) = point_by_id.get(child_id)
      && point
        .r#type
        .is_none_or(|kind| matches!(kind, dgm::PointValues::Node | dgm::PointValues::Assistant))
    {
      depths.insert(child_id.to_string(), depth);
    }
    append_document_node_depths(
      child_id,
      depth,
      point_by_id,
      children_by_source,
      visited,
      depths,
    );
  }
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
  layout: Option<&dgm::LayoutDefinition>,
  mut metrics: LayoutNodeMetrics,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
) -> Option<Vec<DiagramNode>> {
  let connections = data.connection_list.as_ref()?;
  metrics.data_orders = document_node_orders(data);
  metrics.data_depths = document_node_depths(data);
  let point_by_id: HashMap<&str, &dgm::Point> = data
    .point_list
    .point
    .iter()
    .map(|point| (point.model_id.as_str(), point))
    .collect();
  let mut children_by_source: HashMap<&str, Vec<&dgm::Connection>> = HashMap::new();
  let mut data_by_presentation: HashMap<&str, Vec<(u32, &dgm::Point)>> = HashMap::new();
  let mut points_by_presentation_name: HashMap<&str, Vec<&dgm::Point>> = HashMap::new();
  for point in &data.point_list.point {
    if let Some(name) = presentation_name(point) {
      points_by_presentation_name
        .entry(name)
        .or_default()
        .push(point);
    }
  }
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
          data_by_presentation
            .entry(presentation_point.model_id.as_str())
            .or_default()
            .push((connection.destination_position, data_point));
        }
      }
      _ => {}
    }
  }
  let data_by_presentation: HashMap<&str, Vec<&dgm::Point>> = data_by_presentation
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

  let root_point = associated_data_point(root_presentation, &point_by_id)?;
  let mut for_each_by_name = HashMap::new();
  if let Some(layout) = layout {
    collect_for_each_refs_from_layout_node(&layout.layout_node, &mut for_each_by_name);
  }
  let mut visitor = DiagramShapeCreationVisitor {
    point_by_id: &point_by_id,
    points_by_presentation_name: &points_by_presentation_name,
    data_by_presentation: &data_by_presentation,
    for_each_by_name: &for_each_by_name,
    connections,
    metrics: &metrics,
    colors,
    fallback_fill,
    current_point: root_point,
    current_index: 0,
    current_step: 1,
    current_count: 1,
    nodes: Vec::new(),
    tree: None,
    parent_path: Vec::new(),
    tree_root_mapped: false,
  };
  if let Some(layout) = layout {
    visitor.visit_layout_node(&layout.layout_node);
    return Some(visitor.nodes);
  }

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

fn build_diagram_shape_tree(
  data: &dgm::DataModelRoot,
  layout: &dgm::LayoutDefinition,
  colors: Option<&DiagramStyleColors>,
  fallback_fill: RgbColor,
  bounds: DiagramBounds,
) -> Option<DiagramShapeNode> {
  let connections = data.connection_list.as_ref()?;
  let mut metrics = layout_node_metrics(Some(layout));
  metrics.data_orders = document_node_orders(data);
  metrics.data_depths = document_node_depths(data);
  let point_by_id: HashMap<&str, &dgm::Point> = data
    .point_list
    .point
    .iter()
    .map(|point| (point.model_id.as_str(), point))
    .collect();
  let mut data_by_presentation: HashMap<&str, Vec<(u32, &dgm::Point)>> = HashMap::new();
  let mut points_by_presentation_name: HashMap<&str, Vec<&dgm::Point>> = HashMap::new();
  for point in &data.point_list.point {
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
        .push((connection.destination_position, data_point));
    }
  }
  let data_by_presentation: HashMap<&str, Vec<&dgm::Point>> = data_by_presentation
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
  let root_presentation = data.point_list.point.iter().find(|point| {
    point.r#type == Some(dgm::PointValues::Presentation)
      && associated_data_point(point, &point_by_id)
        .is_some_and(|point| point.r#type == Some(dgm::PointValues::Document))
  })?;
  let root_point = associated_data_point(root_presentation, &point_by_id)?;
  let mut for_each_by_name = HashMap::new();
  collect_for_each_refs_from_layout_node(&layout.layout_node, &mut for_each_by_name);
  let mut visitor = DiagramShapeCreationVisitor {
    point_by_id: &point_by_id,
    points_by_presentation_name: &points_by_presentation_name,
    data_by_presentation: &data_by_presentation,
    for_each_by_name: &for_each_by_name,
    connections,
    metrics: &metrics,
    colors,
    fallback_fill,
    current_point: root_point,
    current_index: 0,
    current_step: 1,
    current_count: 1,
    nodes: Vec::new(),
    tree: Some(DiagramShapeNode {
      internal_name: String::new(),
      text: String::new(),
      fill: fallback_fill,
      x: 0.0,
      y: 0.0,
      width: bounds.width,
      height: bounds.height,
      algorithm: None,
      child_order: dgm::ChildOrderValues::Bottom,
      has_geometry: false,
      hidden_geometry: false,
      is_connector: false,
      order: usize::MAX,
      constraints: Vec::new(),
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

struct DiagramShapeCreationVisitor<'a> {
  point_by_id: &'a HashMap<&'a str, &'a dgm::Point>,
  points_by_presentation_name: &'a HashMap<&'a str, Vec<&'a dgm::Point>>,
  data_by_presentation: &'a HashMap<&'a str, Vec<&'a dgm::Point>>,
  for_each_by_name: &'a HashMap<&'a str, &'a dgm::ForEach>,
  connections: &'a dgm::ConnectionList,
  metrics: &'a LayoutNodeMetrics,
  colors: Option<&'a DiagramStyleColors>,
  fallback_fill: RgbColor,
  current_point: &'a dgm::Point,
  current_index: usize,
  current_step: usize,
  current_count: usize,
  nodes: Vec<DiagramNode>,
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
      && self.current_index + self.current_step >= self.current_count
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
      .unwrap_or(children);
    let count = children.min(requested_count);
    let step = for_each
      .step
      .as_ref()
      .and_then(|steps| steps.first())
      .copied()
      .unwrap_or(1)
      .max(1) as usize;
    let old_index = self.current_index;
    let old_step = self.current_step;
    let old_count = self.current_count;
    self.current_step = step;
    self.current_count = count;
    let mut index = 0usize;
    while index < count {
      self.current_index = index;
      for choice in &for_each.for_each_choice {
        self.visit_for_each_choice(choice);
      }
      index += step;
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
        self.max_depth_for_condition(self.current_point),
        branch.val.parse::<i32>().unwrap_or_default(),
      ),
      dgm::FunctionValues::Depth
      | dgm::FunctionValues::Position
      | dgm::FunctionValues::ReversePosition
      | dgm::FunctionValues::PositionEven
      | dgm::FunctionValues::PositionOdd => true,
    }
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
    let Some(mut node_id) = presentation_association_id(self.current_point) else {
      return 0;
    };
    if branch.axis.as_ref().is_some_and(|axes| {
      axes.len() == 2 && axes[0] == dgm::AxisValues::Child && axes[1] == dgm::AxisValues::Child
    }) && let Some(child_id) =
      self.navigate_connection(dgm::ConnectionValues::ParentOf, node_id, true)
    {
      node_id = child_id;
    }
    self
      .connections
      .connection
      .iter()
      .filter(|connection| {
        connection
          .r#type
          .is_none_or(|kind| kind == dgm::ConnectionValues::ParentOf)
          && connection.source_id == node_id
      })
      .count() as i32
  }

  fn max_depth_for_condition(&self, point: &dgm::Point) -> i32 {
    let Some(node_id) = presentation_association_id(point) else {
      return 0;
    };
    self.max_depth_from_node(node_id, &mut HashSet::new())
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
    let explicit_data_points = self
      .data_by_presentation
      .get(presentation_point.model_id.as_str());
    let fallback_data_point = explicit_data_points.is_none().then(|| {
      self
        .metrics
        .allows_associated_text_fallback(name)
        .then(|| associated_data_point(presentation_point, self.point_by_id))
        .flatten()
    });
    let data_points: Vec<_> = explicit_data_points
      .into_iter()
      .flatten()
      .copied()
      .chain(fallback_data_point.into_iter().flatten())
      .collect();
    let mut text = String::new();
    let mut order = usize::MAX;
    let mut depth = usize::MAX;
    for data_point in &data_points {
      let data_text = point_text(data_point);
      if !data_text.is_empty() {
        if !text.is_empty() {
          text.push('\n');
        }
        text.push_str(data_text.as_str());
      }
      order = order.min(
        self
          .metrics
          .data_orders
          .get(data_point.model_id.as_str())
          .copied()
          .unwrap_or(usize::MAX),
      );
      depth = depth.min(
        self
          .metrics
          .data_depths
          .get(data_point.model_id.as_str())
          .copied()
          .unwrap_or_default(),
      );
    }
    if self.metrics.is_text_shape(name) && !text.is_empty() {
      self.nodes.push(DiagramNode {
        text: text.clone(),
        fill: diagram_node_fill(Some(presentation_point), self.colors, self.fallback_fill),
        preferred_height: self.metrics.preferred_heights.get(name).copied(),
        height_width_factor: self.metrics.height_width_factors.get(name).copied(),
        order,
        depth: if depth == usize::MAX { 0 } else { depth },
      });
    }
    if self.tree.is_none() {
      return None;
    }
    let has_geometry = shape_atom.is_some();
    let hidden_geometry = shape_atom
      .and_then(|shape| shape.hide_geometry)
      .map(bool::from)
      .unwrap_or(false);
    let is_connector = shape_atom
      .and_then(|shape| shape.r#type.as_deref())
      .is_some_and(|shape_type| shape_type == "conn");
    let child = DiagramShapeNode {
      internal_name: name.to_string(),
      text,
      fill: diagram_node_fill(Some(presentation_point), self.colors, self.fallback_fill),
      x: 0.0,
      y: 0.0,
      width: 0.0,
      height: 0.0,
      algorithm: self.active_algorithm(layout_node),
      child_order: layout_node.child_order.unwrap_or_default(),
      has_geometry,
      hidden_geometry,
      is_connector,
      order,
      constraints: self.active_constraints(layout_node),
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

  fn active_algorithm(&self, node: &'a dgm::LayoutNode) -> Option<LayoutAlgorithm> {
    for choice in &node.layout_node_choice {
      match choice {
        dgm::LayoutNodeChoice::Algorithm(algorithm) => return Some(layout_algorithm(algorithm)),
        dgm::LayoutNodeChoice::Choose(choose) => {
          if let Some(algorithm) = self.active_algorithm_in_choose(choose) {
            return Some(algorithm);
          }
        }
        _ => {}
      }
    }
    None
  }

  fn active_algorithm_in_choose(&self, choose: &'a dgm::Choose) -> Option<LayoutAlgorithm> {
    for branch in &choose.diagram_choose_if {
      if self.choose_if_decision(branch) {
        for choice in &branch.diagram_choose_if_choice {
          match choice {
            dgm::DiagramChooseIfChoice::Algorithm(algorithm) => {
              return Some(layout_algorithm(algorithm));
            }
            dgm::DiagramChooseIfChoice::Choose(choose) => {
              if let Some(algorithm) = self.active_algorithm_in_choose(choose) {
                return Some(algorithm);
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
          dgm::DiagramChooseElseChoice::Algorithm(algorithm) => {
            return Some(layout_algorithm(algorithm));
          }
          dgm::DiagramChooseElseChoice::Choose(choose) => {
            if let Some(algorithm) = self.active_algorithm_in_choose(choose) {
              return Some(algorithm);
            }
          }
          _ => {}
        }
      }
    }
    None
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

fn append_presentation_nodes<'a>(
  parent_id: &str,
  point_by_id: &HashMap<&'a str, &'a dgm::Point>,
  data_by_presentation: &HashMap<&'a str, Vec<&'a dgm::Point>>,
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
      {
        let explicit_data_points = data_by_presentation.get(child_id);
        let fallback_data_point = explicit_data_points.is_none().then(|| {
          metrics
            .allows_associated_text_fallback(name)
            .then(|| associated_data_point(presentation_point, point_by_id))
            .flatten()
        });
        let mut data_points = explicit_data_points
          .into_iter()
          .flatten()
          .copied()
          .chain(fallback_data_point.into_iter().flatten());
        for data_point in &mut data_points {
          let text = point_text(data_point);
          if !text.is_empty() {
            nodes.push(DiagramNode {
              text,
              fill: diagram_node_fill(Some(*presentation_point), colors, fallback_fill),
              preferred_height: metrics.preferred_heights.get(name).copied(),
              height_width_factor: metrics.height_width_factors.get(name).copied(),
              order: metrics
                .data_orders
                .get(data_point.model_id.as_str())
                .copied()
                .unwrap_or(usize::MAX),
              depth: metrics
                .data_depths
                .get(data_point.model_id.as_str())
                .copied()
                .unwrap_or_default(),
            });
          }
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
      dgm::LayoutNodeChoice::Constraints(constraints) => Some(constraints.as_ref()),
      _ => None,
    })
    .flat_map(|constraints| parse_constraints(constraints, true))
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
    operator: constraint.operator.unwrap_or_default(),
    r#for: constraint.r#for,
    point_type: constraint.point_type,
  })
}

fn direct_rules(node: &dgm::LayoutNode) -> Vec<DiagramRule> {
  node
    .layout_node_choice
    .iter()
    .filter_map(|choice| match choice {
      dgm::LayoutNodeChoice::RuleList(rules) => Some(rules.as_ref()),
      _ => None,
    })
    .flat_map(parse_rules)
    .collect()
}

fn parse_rules(rules: &dgm::RuleList) -> Vec<DiagramRule> {
  rules
    .rule
    .iter()
    .filter_map(|rule| {
      let for_name = rule.for_name.clone().unwrap_or_default();
      (!for_name.is_empty()).then_some(DiagramRule { for_name })
    })
    .collect()
}

#[derive(Clone, Debug, Default)]
struct LayoutNodeMetrics {
  shape_names: HashSet<String>,
  text_names: HashSet<String>,
  text_axes: HashMap<String, Vec<dgm::AxisValues>>,
  preferred_heights: HashMap<String, f32>,
  height_width_factors: HashMap<String, f32>,
  data_orders: HashMap<String, usize>,
  data_depths: HashMap<String, usize>,
}

impl LayoutNodeMetrics {
  fn is_text_shape(&self, name: &str) -> bool {
    if self.text_names.is_empty() {
      self.shape_names.contains(name)
    } else {
      self.text_names.contains(name)
    }
  }

  fn allows_associated_text_fallback(&self, name: &str) -> bool {
    self.text_axes.get(name).is_none_or(|axes| {
      axes.is_empty()
        || axes.iter().any(|axis| {
          matches!(
            axis,
            dgm::AxisValues::_Self | dgm::AxisValues::DescendantOrSelf
          )
        })
    })
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
  let text_axes = node
    .layout_node_choice
    .iter()
    .filter_map(|choice| match choice {
      dgm::LayoutNodeChoice::PresentationOf(presentation_of) => Some(presentation_of),
      _ => None,
    })
    .next()
    .and_then(|presentation_of| presentation_of.axis.clone());
  let has_rendered_text_axis = text_axes.as_ref().is_none_or(|axes| {
    axes.is_empty()
      || axes.iter().any(|axis| {
        matches!(
          axis,
          dgm::AxisValues::_Self | dgm::AxisValues::Descendant | dgm::AxisValues::DescendantOrSelf
        )
      })
  });
  if has_shape && let Some(name) = node.name.as_deref() {
    metrics.shape_names.insert(name.to_string());
    if has_text_algorithm && has_rendered_text_axis {
      metrics.text_names.insert(name.to_string());
      if let Some(axes) = text_axes {
        metrics.text_axes.insert(name.to_string(), axes);
      }
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
          fill: diagram_node_fill(
            presentation_by_data.get(child_id).copied(),
            colors,
            fallback_fill,
          ),
          preferred_height: None,
          height_width_factor: None,
          order: nodes.len(),
          depth: depth as usize,
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
    .and_then(|fills| {
      if fills.is_empty() {
        None
      } else {
        Some(fills[style_index % fills.len()])
      }
    })
    .unwrap_or(fallback_fill)
}

pub(crate) fn point_text(point: &dgm::Point) -> String {
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
    .and_then(|value| (value != "none").then(|| linear_direction_from_value(value)));
  let reverse = algorithm.parameter.iter().any(|parameter| {
    matches!(
      (parameter.r#type, parameter.val.as_deref()),
      (
        dgm::ParameterIdValues::GrowDirection,
        Some("tR" | "bR" | "rT" | "rB")
      )
    )
  }) || matches!(
    linear_direction,
    LinearDirection::FromRight | LinearDirection::FromBottom
  );
  LayoutAlgorithm {
    kind: algorithm.r#type,
    reverse,
    linear_direction,
    secondary_linear_direction,
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
    aspect_ratio: algorithm_parameter_f32(algorithm, dgm::ParameterIdValues::AspectRatio)
      .unwrap_or_default(),
  }
}

impl LayoutAlgorithm {
  fn with_reversed(mut self, reversed: bool) -> Self {
    self.reverse |= reversed;
    self
  }
}

fn linear_direction_from_value(value: &str) -> LinearDirection {
  match value {
    "fromR" => LinearDirection::FromRight,
    "fromT" => LinearDirection::FromTop,
    "fromB" => LinearDirection::FromBottom,
    _ => LinearDirection::FromLeft,
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
  let constraints = root.constraints.clone();
  let rules = root.rules.clone();
  layout_diagram_shape_node(root, &constraints, &rules);
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
  if let Some(algorithm) = node.algorithm {
    layout_shape_children(node, algorithm, &constraints, &rules);
  }
  for child in &mut node.children {
    layout_diagram_shape_node(child, &constraints, &rules);
  }
  if node.child_order == dgm::ChildOrderValues::Top {
    node.children.reverse();
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
    dgm::AlgorithmValues::Text | dgm::AlgorithmValues::Space | dgm::AlgorithmValues::Connector => {}
    dgm::AlgorithmValues::Pyramid => linear_layout_tree(node, algorithm, constraints, rules),
  }
}

impl DiagramShapeNode {
  fn is_empty_group(&self) -> bool {
    self.text.is_empty() && self.children.is_empty() && !self.has_geometry
  }
}

fn composite_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  constraints: &[DiagramConstraint],
) {
  let mut properties: HashMap<String, HashMap<dgm::ConstraintValues, f32>> = HashMap::new();
  let parent_width = if algorithm.aspect_ratio == 1.0 {
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
    LinearDirection::FromLeft | LinearDirection::FromRight
  );
  let reverse = matches!(
    algorithm.linear_direction,
    LinearDirection::FromRight | LinearDirection::FromBottom
  );
  let mut properties: HashMap<String, HashMap<dgm::ConstraintValues, f32>> = HashMap::new();
  for constraint in constraints
    .iter()
    .filter(|constraint| !constraint.for_name.is_empty())
  {
    if constraint.target == dgm::ConstraintValues::Width {
      properties
        .entry(constraint.for_name.clone())
        .or_default()
        .insert(dgm::ConstraintValues::Width, node.width * constraint.factor);
    }
    if constraint.target == dgm::ConstraintValues::Height {
      properties
        .entry(constraint.for_name.clone())
        .or_default()
        .insert(
          dgm::ConstraintValues::Height,
          node.height * constraint.factor,
        );
    }
  }
  let mut space = 0.0;
  for constraint in constraints {
    if matches!(constraint.for_name.as_str(), "sp" | "space" | "sibTrans") {
      if horizontal && constraint.target == dgm::ConstraintValues::Width {
        space = node.width * constraint.factor;
      }
      if !horizontal && constraint.target == dgm::ConstraintValues::Height {
        space = node.height * constraint.factor;
      }
    }
  }
  let shrink_names: HashSet<&str> = rules.iter().map(|rule| rule.for_name.as_str()).collect();
  if shrink_names.is_empty() {
    node.children.retain(|child| !child.is_empty_group());
  }
  let count = node.children.len().max(1) as f32;
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
  total_primary += (count - 1.0).max(0.0) * space;
  let scale = if horizontal && total_primary > node.width {
    node.width / total_primary
  } else if !horizontal && total_primary > node.height {
    node.height / total_primary
  } else {
    1.0
  };
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
      width *= scale;
      height *= scale;
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
      if reverse {
        cursor -= space * scale;
      } else {
        cursor += space * scale;
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
      if reverse {
        cursor -= space * scale;
      } else {
        cursor += space * scale;
      }
    }
  }
}

fn cycle_layout_tree(node: &mut DiagramShapeNode, algorithm: LayoutAlgorithm) {
  if node.children.is_empty() {
    return;
  }
  let center_x = node.width / 2.0;
  let center_y = node.height / 2.0;
  let child_width = node.width / 4.0;
  let child_height = node.height / 4.0;
  let radius = ((node.width - child_width) / 2.0).min((node.height - child_height) / 2.0);
  let mut start = 0usize;
  if algorithm.center_shape_mapping_first_node
    && let Some(center) = node.children.first_mut()
  {
    center.x = center_x - child_width / 2.0;
    center.y = center_y - child_height / 2.0;
    center.width = child_width;
    center.height = child_height;
    start = 1;
  }
  let count = node.children.len().saturating_sub(start);
  if count == 0 {
    return;
  }
  for (index, child) in node.children.iter_mut().skip(start).enumerate() {
    let angle = (index as f32) * algorithm.span_angle / count as f32 + algorithm.start_angle;
    let radians = angle.to_radians();
    let (width, height, current_radius) = if child.is_connector {
      (node.width / 12.0, node.height / 12.0, radius)
    } else {
      (child_width, child_height, radius)
    };
    child.x = center_x + current_radius * radians.sin() - width / 2.0;
    child.y = center_y - current_radius * radians.cos() - height / 2.0;
    child.width = width;
    child.height = height;
  }
}

fn hierarchy_layout_tree(node: &mut DiagramShapeNode, algorithm: LayoutAlgorithm) {
  let direction = if algorithm.kind == dgm::AlgorithmValues::HierarchyRoot {
    LinearDirection::FromTop
  } else {
    algorithm.linear_direction
  };
  let algorithm = LayoutAlgorithm {
    linear_direction: direction,
    ..algorithm
  };
  linear_layout_tree(node, algorithm, &[], &[]);
}

fn snake_layout_tree(
  node: &mut DiagramShapeNode,
  algorithm: LayoutAlgorithm,
  _constraints: &[DiagramConstraint],
) {
  if node.children.is_empty() {
    return;
  }
  let count = node.children.len();
  let space = 0.3_f32;
  let aspect_ratio = 0.54_f32;
  let mut columns = 1usize;
  let mut rows = 1usize;
  let mut max_row_width = 0.0_f32;
  for candidate_rows in 1..count {
    let candidate_columns = count.div_ceil(candidate_rows);
    let row_width = node.width * candidate_columns as f32;
    let total_height = node.height * candidate_rows as f32;
    if row_width > 0.0 && total_height / row_width >= aspect_ratio {
      columns = candidate_columns;
      rows = candidate_rows;
      max_row_width = row_width;
      break;
    }
  }
  if max_row_width == 0.0 {
    columns = count;
    rows = 1;
  }
  let child_width = node.width / (columns as f32 + (columns.saturating_sub(1) as f32) * space);
  let child_height = (child_width * aspect_ratio)
    .min(node.height / (rows as f32 + (rows.saturating_sub(1) as f32) * space));
  let step_x = child_width + child_width * space;
  let step_y = child_height + child_height * space;
  let start_x = if algorithm.reverse {
    node.width - child_width
  } else {
    0.0
  };
  let inc_x = if algorithm.reverse { -1.0 } else { 1.0 };
  let mut x = start_x;
  let mut y = 0.0;
  let mut column_index = 0usize;
  for (index, child) in node.children.iter_mut().enumerate() {
    child.x = x;
    child.y = y;
    child.width = child_width;
    child.height = child_height;

    let next_index = index + 1;
    if next_index % columns == 0 || ((next_index / columns) + 1) != rows {
      x += inc_x * step_x;
    }

    column_index += 1;
    if column_index == columns {
      if (next_index + 1) % columns != 0
        && (next_index + 1) >= 3
        && ((next_index + 1) / columns + 1) == rows
        && count != rows * columns
      {
        x = start_x + inc_x * step_x / 2.0;
      } else {
        x = start_x;
      }
      y += step_y;
      column_index = 0;
    }

    if next_index % columns != 0 && next_index >= 3 && ((next_index / columns) + 1) == rows {
      x += inc_x * step_x;
    }
  }
}

fn flatten_diagram_shape_tree(
  node: &DiagramShapeNode,
  offset_x: f32,
  offset_y: f32,
  shapes: &mut Vec<DiagramShape>,
) {
  let x = offset_x + node.x;
  let y = offset_y + node.y;
  if !node.hidden_geometry && (node.has_geometry || !node.text.trim().is_empty()) {
    shapes.push(DiagramShape {
      x,
      y,
      width: node.width,
      height: node.height,
      text: node.text.clone(),
      fill: node.fill,
      order: node.order,
    });
  }
  for child in &node.children {
    flatten_diagram_shape_tree(child, x, y, shapes);
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

fn linear_layout(
  nodes: &[DiagramNode],
  bounds: DiagramBounds,
  reverse: bool,
  allow_hierarchy: bool,
) -> Vec<DiagramShape> {
  if allow_hierarchy
    && nodes
      .iter()
      .map(|node| node.depth)
      .max()
      .unwrap_or_default()
      > nodes
        .iter()
        .map(|node| node.depth)
        .min()
        .unwrap_or_default()
  {
    return hierarchical_linear_layout(nodes, bounds, reverse);
  }
  grid_layout(nodes, bounds, nodes.len().max(1), 1, reverse)
}

fn hierarchical_linear_layout(
  nodes: &[DiagramNode],
  bounds: DiagramBounds,
  reverse: bool,
) -> Vec<DiagramShape> {
  // Source: LibreOffice CompositeAlg + LinAlg: child layout nodes are sized
  // inside their parent composite, and linear layout spreads the composites
  // along the primary axis. This preserves that parent/descendant grouping for
  // the flattened render model until the full shape tree visitor is in place.
  let min_depth = nodes
    .iter()
    .map(|node| node.depth)
    .min()
    .unwrap_or_default();
  let max_depth = nodes
    .iter()
    .map(|node| node.depth)
    .max()
    .unwrap_or(min_depth);
  let rows = (max_depth - min_depth + 1).max(1);
  let mut groups = Vec::<Vec<&DiagramNode>>::new();
  for node in nodes {
    if node.depth == min_depth || groups.is_empty() {
      groups.push(vec![node]);
    } else if let Some(group) = groups.last_mut() {
      group.push(node);
    }
  }
  let columns = groups.len().max(1);
  let gap = (bounds.width.min(bounds.height) * 0.04).clamp(4.0, 18.0);
  let cell_width = (bounds.width - gap * (columns.saturating_sub(1) as f32)) / columns as f32;
  let cell_height = (bounds.height - gap * (rows.saturating_sub(1) as f32)) / rows as f32;
  let mut shapes = Vec::new();
  for (group_index, group) in groups.iter().enumerate() {
    let column = if reverse {
      columns - 1 - group_index
    } else {
      group_index
    };
    for node in group {
      let row = node.depth.saturating_sub(min_depth).min(rows - 1);
      let x = bounds.x + column as f32 * (cell_width + gap);
      let shape_height = node
        .height_width_factor
        .map(|factor| cell_width * factor)
        .or(node.preferred_height)
        .map(|height| height.max(18.0).min(cell_height))
        .unwrap_or_else(|| (cell_height * 0.72).max(18.0));
      let y = bounds.y + row as f32 * (cell_height + gap) + (cell_height - shape_height) / 2.0;
      shapes.push(DiagramShape {
        x,
        y,
        width: cell_width.max(1.0),
        height: shape_height,
        text: node.text.clone(),
        fill: node.fill,
        order: node.order,
      });
    }
  }
  shapes
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
        order: node.order,
      }
    })
    .collect()
}
