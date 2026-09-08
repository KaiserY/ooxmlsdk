//! Event-driven direct inset cells for DrawingML bevel meshes.
//!
//! Microsoft's direct-inset contract computes all wavefront changes before it
//! applies a bevel profile.  A cell below is the region swept by one live
//! source-edge stretch between two changes to its endpoint trajectories.
//! Events elsewhere in the wavefront must not subdivide that unchanged face:
//! its endpoints remain affine in inset time, and retaining one maximal cell
//! preserves both the intended interpolation domain and a compact mesh.
//! Keeping the event time on both cell boundaries lets the caller traverse
//! the same graph in either direction for folded profiles without rebuilding
//! polygon offsets.

use std::collections::HashMap;

const TIME_EPSILON: f64 = 1.0e-7;
const POINT_EPSILON: f64 = 1.0e-6;
const PARALLEL_EPSILON: f64 = 1.0e-12;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct DirectInsetEdgeId {
  pub(super) contour_index: usize,
  pub(super) edge_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct DirectInsetVertexId {
  pub(super) contour_index: usize,
  pub(super) vertex_index: usize,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct DirectInsetCellEndpoint {
  pub(super) point: (f32, f32),
  /// Original flattened-outline vertex carried by this wavefront trajectory.
  /// Event-created vertices have no such owner: their newly adjacent source
  /// faces must emit independent normals rather than borrowing a stale
  /// normal from either endpoint of the original edge.
  pub(super) source_vertex: Option<DirectInsetVertexId>,
  /// Internal wavefront trajectory used to coalesce consecutive intervals
  /// that belong to the same unchanged source-face stretch.
  trajectory_vertex: usize,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct DirectInsetCellBoundary {
  pub(super) inset: f32,
  pub(super) endpoints: [DirectInsetCellEndpoint; 2],
}

#[derive(Clone, Copy, Debug)]
pub(super) struct DirectInsetCell {
  pub(super) source_edge: DirectInsetEdgeId,
  pub(super) outer: DirectInsetCellBoundary,
  pub(super) inner: DirectInsetCellBoundary,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn from_f32(point: (f32, f32)) -> Self {
    Self {
      x: f64::from(point.0),
      y: f64::from(point.1),
    }
  }

  fn to_f32(self) -> (f32, f32) {
    (self.x as f32, self.y as f32)
  }

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }

  fn scale(self, factor: f64) -> Self {
    Self {
      x: self.x * factor,
      y: self.y * factor,
    }
  }

  fn dot(self, other: Self) -> f64 {
    self.x * other.x + self.y * other.y
  }

  fn cross(self, other: Self) -> f64 {
    self.x * other.y - self.y * other.x
  }

  fn length(self) -> f64 {
    self.x.hypot(self.y)
  }

  fn distance(self, other: Self) -> f64 {
    self.sub(other).length()
  }
}

#[derive(Clone, Copy, Debug)]
struct SourceEdge {
  id: DirectInsetEdgeId,
  tangent: Point,
  inward: Point,
  line_constant: f64,
}

#[derive(Clone, Copy, Debug)]
struct ActiveVertex {
  alive: bool,
  previous: usize,
  next: usize,
  left_edge: usize,
  right_edge: usize,
  anchor: Point,
  anchor_time: f64,
  velocity: Point,
  source_vertex: Option<DirectInsetVertexId>,
}

impl ActiveVertex {
  fn position(self, inset: f64) -> Point {
    self
      .anchor
      .add(self.velocity.scale(inset - self.anchor_time))
  }
}

#[derive(Clone, Copy, Debug)]
enum DirectInsetEventKind {
  Edge { first: usize, second: usize },
  Split { vertex: usize, edge_start: usize },
}

#[derive(Clone, Copy, Debug)]
struct DirectInsetEvent {
  inset: f64,
  point: Point,
  kind: DirectInsetEventKind,
}

struct DirectInsetGraph {
  contours: Vec<Vec<Point>>,
  source_edges: Vec<SourceEdge>,
  vertices: Vec<ActiveVertex>,
  solid_on_right: bool,
  cells: Vec<DirectInsetCell>,
  open_cells: HashMap<(usize, usize, usize), usize>,
}

impl DirectInsetGraph {
  fn new(contours: &[&[(f32, f32)]], solid_on_right: bool) -> Option<Self> {
    let mut source_edges = Vec::new();
    let mut vertices = Vec::new();
    let mut stored_contours = Vec::with_capacity(contours.len());

    for (contour_index, contour) in contours.iter().enumerate() {
      if contour.len() < 3 {
        continue;
      }
      let points = contour
        .iter()
        .copied()
        .map(Point::from_f32)
        .collect::<Vec<_>>();
      let edge_start = source_edges.len();
      for edge_index in 0..points.len() {
        let start = points[edge_index];
        let end = points[(edge_index + 1) % points.len()];
        let direction = end.sub(start);
        let length = direction.length();
        if !length.is_finite() || length <= POINT_EPSILON {
          return None;
        }
        let tangent = direction.scale(length.recip());
        let inward = if solid_on_right {
          Point {
            x: -tangent.y,
            y: tangent.x,
          }
        } else {
          Point {
            x: tangent.y,
            y: -tangent.x,
          }
        };
        source_edges.push(SourceEdge {
          id: DirectInsetEdgeId {
            contour_index,
            edge_index,
          },
          tangent,
          inward,
          line_constant: inward.dot(start),
        });
      }

      let vertex_start = vertices.len();
      for index in 0..points.len() {
        let previous = vertex_start + (index + points.len() - 1) % points.len();
        let next = vertex_start + (index + 1) % points.len();
        let left_edge = edge_start + (index + points.len() - 1) % points.len();
        let right_edge = edge_start + index;
        let velocity = Self::vertex_velocity(
          source_edges[left_edge].inward,
          source_edges[right_edge].inward,
        )?;
        vertices.push(ActiveVertex {
          alive: true,
          previous,
          next,
          left_edge,
          right_edge,
          anchor: points[index],
          anchor_time: 0.0,
          velocity,
          source_vertex: Some(DirectInsetVertexId {
            contour_index,
            vertex_index: index,
          }),
        });
      }
      stored_contours.push(points);
    }

    (!source_edges.is_empty()).then_some(Self {
      contours: stored_contours,
      source_edges,
      vertices,
      solid_on_right,
      cells: Vec::new(),
      open_cells: HashMap::new(),
    })
  }

  fn vertex_velocity(left: Point, right: Point) -> Option<Point> {
    let determinant = left.cross(right);
    if determinant.abs() > PARALLEL_EPSILON {
      let velocity = Point {
        x: (right.y - left.y) / determinant,
        y: (left.x - right.x) / determinant,
      };
      return (velocity.x.is_finite() && velocity.y.is_finite()).then_some(velocity);
    }

    // Consecutive chords produced while flattening a smooth source curve can
    // be exactly collinear.  Their two moving lines are the same constraint;
    // use its unit-speed normal.  Opposed lines describe a collapsed
    // two-edge loop and cannot form a live wavefront vertex.
    if left.dot(right) <= 0.0 {
      return None;
    }
    let sum = left.add(right);
    let length = sum.length();
    if length <= PARALLEL_EPSILON {
      return None;
    }
    let direction = sum.scale(length.recip());
    let speed = left.dot(direction);
    (speed.abs() > PARALLEL_EPSILON).then(|| direction.scale(speed.recip()))
  }

  fn create_vertex(
    &mut self,
    previous: usize,
    next: usize,
    left_edge: usize,
    right_edge: usize,
    point: Point,
    inset: f64,
  ) -> Option<usize> {
    let velocity = Self::vertex_velocity(
      self.source_edges[left_edge].inward,
      self.source_edges[right_edge].inward,
    )?;
    let index = self.vertices.len();
    self.vertices.push(ActiveVertex {
      alive: true,
      previous,
      next,
      left_edge,
      right_edge,
      anchor: point,
      anchor_time: inset,
      velocity,
      source_vertex: None,
    });
    Some(index)
  }

  fn live_vertices(&self) -> impl Iterator<Item = usize> + '_ {
    self
      .vertices
      .iter()
      .enumerate()
      .filter_map(|(index, vertex)| vertex.alive.then_some(index))
  }

  fn is_reflex(&self, vertex: usize) -> bool {
    let vertex = self.vertices[vertex];
    let turn = self.source_edges[vertex.left_edge]
      .tangent
      .cross(self.source_edges[vertex.right_edge].tangent);
    if self.solid_on_right {
      turn < -POINT_EPSILON
    } else {
      turn > POINT_EPSILON
    }
  }

  fn point_inside_or_on_boundary(&self, point: Point) -> bool {
    let mut winding = 0_i32;
    for contour in &self.contours {
      for index in 0..contour.len() {
        let first = contour[index];
        let second = contour[(index + 1) % contour.len()];
        let edge = second.sub(first);
        let length_squared = edge.dot(edge);
        if length_squared > 0.0 {
          let parameter = point.sub(first).dot(edge) / length_squared;
          if (-POINT_EPSILON..=1.0 + POINT_EPSILON).contains(&parameter) {
            let nearest = first.add(edge.scale(parameter.clamp(0.0, 1.0)));
            if nearest.distance(point) <= POINT_EPSILON {
              return true;
            }
          }
        }
        if first.y <= point.y {
          if second.y > point.y && edge.cross(point.sub(first)) > 0.0 {
            winding += 1;
          }
        } else if second.y <= point.y && edge.cross(point.sub(first)) < 0.0 {
          winding -= 1;
        }
      }
    }
    winding != 0
  }

  fn event_is_better(candidate: DirectInsetEvent, current: Option<DirectInsetEvent>) -> bool {
    let Some(current) = current else {
      return true;
    };
    if candidate.inset < current.inset - TIME_EPSILON {
      return true;
    }
    if (candidate.inset - current.inset).abs() > TIME_EPSILON {
      return false;
    }
    // Collapse zero-length stretches before processing a vertex/edge hit at
    // the same inset.  The subsequent full rescan then observes the updated
    // live stretch and turns endpoint hits into deterministic merge events.
    matches!(candidate.kind, DirectInsetEventKind::Edge { .. })
      && matches!(current.kind, DirectInsetEventKind::Split { .. })
  }

  fn earliest_event(&self, now: f64) -> Option<DirectInsetEvent> {
    let mut earliest = None;
    for first_index in self.live_vertices() {
      let first = self.vertices[first_index];
      let second_index = first.next;
      let second = self.vertices[second_index];
      if !second.alive || first.right_edge != second.left_edge {
        continue;
      }
      let tangent = self.source_edges[first.right_edge].tangent;
      let separation = second.position(now).sub(first.position(now)).dot(tangent);
      let closing_speed = second.velocity.sub(first.velocity).dot(tangent);
      let event_delta = if separation.abs() <= POINT_EPSILON {
        0.0
      } else if separation > 0.0 && closing_speed < -PARALLEL_EPSILON {
        -separation / closing_speed
      } else {
        continue;
      };
      if event_delta < -TIME_EPSILON {
        continue;
      }
      let inset = (now + event_delta).max(now);
      let first_point = first.position(inset);
      let second_point = second.position(inset);
      if first_point.distance(second_point) > POINT_EPSILON * 16.0 {
        continue;
      }
      let candidate = DirectInsetEvent {
        inset,
        point: first_point.add(second_point).scale(0.5),
        kind: DirectInsetEventKind::Edge {
          first: first_index,
          second: second_index,
        },
      };
      if Self::event_is_better(candidate, earliest) {
        earliest = Some(candidate);
      }
    }

    for vertex_index in self.live_vertices() {
      if !self.is_reflex(vertex_index) {
        continue;
      }
      let vertex = self.vertices[vertex_index];
      let vertex_now = vertex.position(now);
      for edge_start_index in self.live_vertices() {
        let edge_start = self.vertices[edge_start_index];
        let edge_end_index = edge_start.next;
        if vertex_index == edge_start_index
          || vertex_index == edge_end_index
          || edge_start.right_edge == vertex.left_edge
          || edge_start.right_edge == vertex.right_edge
        {
          continue;
        }
        let source_edge = self.source_edges[edge_start.right_edge];
        let denominator = source_edge.inward.dot(vertex.velocity) - 1.0;
        if denominator.abs() <= PARALLEL_EPSILON {
          continue;
        }
        let numerator = source_edge.line_constant + now - source_edge.inward.dot(vertex_now);
        let event_delta = numerator / denominator;
        if event_delta < -TIME_EPSILON {
          continue;
        }
        let inset = (now + event_delta).max(now);
        if earliest.is_some_and(|event| inset > event.inset + TIME_EPSILON) {
          continue;
        }
        let point = vertex.position(inset);
        let stretch_start = edge_start.position(inset);
        let stretch_end = self.vertices[edge_end_index].position(inset);
        let stretch = stretch_end.sub(stretch_start);
        let length_squared = stretch.dot(stretch);
        if length_squared <= POINT_EPSILON * POINT_EPSILON {
          continue;
        }
        let parameter = point.sub(stretch_start).dot(stretch) / length_squared;
        if !(-POINT_EPSILON..=1.0 + POINT_EPSILON).contains(&parameter) {
          continue;
        }
        let nearest = stretch_start.add(stretch.scale(parameter.clamp(0.0, 1.0)));
        if nearest.distance(point) > POINT_EPSILON * 16.0
          || !self.point_inside_or_on_boundary(point)
        {
          continue;
        }
        let candidate = DirectInsetEvent {
          inset,
          point,
          kind: DirectInsetEventKind::Split {
            vertex: vertex_index,
            edge_start: edge_start_index,
          },
        };
        if Self::event_is_better(candidate, earliest) {
          earliest = Some(candidate);
        }
      }
    }
    earliest
  }

  fn record_interval(&mut self, outer: f64, inner: f64) {
    if inner <= outer + TIME_EPSILON {
      return;
    }
    let live = self.live_vertices().collect::<Vec<_>>();
    for first_index in live {
      let first = self.vertices[first_index];
      let second = self.vertices[first.next];
      if !second.alive || first.right_edge != second.left_edge {
        continue;
      }
      let outer_points = [first.position(outer), second.position(outer)];
      let inner_points = [first.position(inner), second.position(inner)];
      if outer_points[0].distance(outer_points[1]) <= POINT_EPSILON
        && inner_points[0].distance(inner_points[1]) <= POINT_EPSILON
      {
        continue;
      }
      let boundary = |inset: f64, points: [Point; 2]| DirectInsetCellBoundary {
        inset: inset as f32,
        endpoints: [
          DirectInsetCellEndpoint {
            point: points[0].to_f32(),
            source_vertex: first.source_vertex,
            trajectory_vertex: first_index,
          },
          DirectInsetCellEndpoint {
            point: points[1].to_f32(),
            source_vertex: second.source_vertex,
            trajectory_vertex: first.next,
          },
        ],
      };
      let outer_boundary = boundary(outer, outer_points);
      let inner_boundary = boundary(inner, inner_points);
      let key = (first.right_edge, first_index, first.next);
      if let Some(&cell_index) = self.open_cells.get(&key)
        && self.cells.get(cell_index).is_some_and(|cell| {
          (f64::from(cell.inner.inset) - outer).abs() <= POINT_EPSILON
            && cell.inner.endpoints[0].trajectory_vertex == first_index
            && cell.inner.endpoints[1].trajectory_vertex == first.next
        })
      {
        self.cells[cell_index].inner = inner_boundary;
        continue;
      }
      let cell_index = self.cells.len();
      self.cells.push(DirectInsetCell {
        source_edge: self.source_edges[first.right_edge].id,
        outer: outer_boundary,
        inner: inner_boundary,
      });
      self.open_cells.insert(key, cell_index);
    }
  }

  fn remove_short_loops(&mut self) {
    let live = self.live_vertices().collect::<Vec<_>>();
    let mut seen = vec![false; self.vertices.len()];
    for start in live {
      if seen[start] || !self.vertices[start].alive {
        continue;
      }
      let mut loop_vertices = Vec::new();
      let mut current = start;
      loop {
        if current >= self.vertices.len() || !self.vertices[current].alive || seen[current] {
          break;
        }
        seen[current] = true;
        loop_vertices.push(current);
        current = self.vertices[current].next;
        if current == start {
          break;
        }
      }
      if current != start || loop_vertices.len() > 2 {
        continue;
      }
      for vertex in loop_vertices {
        self.vertices[vertex].alive = false;
      }
    }
  }

  fn remove_loop(&mut self, start: usize) {
    if !self.vertices.get(start).is_some_and(|vertex| vertex.alive) {
      return;
    }
    let mut current = start;
    for _ in 0..=self.vertices.len() {
      if !self.vertices[current].alive {
        break;
      }
      let next = self.vertices[current].next;
      self.vertices[current].alive = false;
      current = next;
      if current == start {
        break;
      }
    }
  }

  fn loop_is_degenerate_at(&self, start: usize, inset: f64) -> bool {
    if !self.vertices.get(start).is_some_and(|vertex| vertex.alive) {
      return true;
    }
    let mut current = start;
    let mut points = Vec::new();
    for _ in 0..=self.vertices.len() {
      if !self.vertices[current].alive {
        return false;
      }
      points.push(self.vertices[current].position(inset));
      current = self.vertices[current].next;
      if current == start {
        break;
      }
    }
    if current != start || points.len() < 3 {
      return true;
    }

    let mut twice_area = 0.0;
    let mut perimeter = 0.0;
    for index in 0..points.len() {
      let first = points[index];
      let second = points[(index + 1) % points.len()];
      twice_area += first.cross(second);
      perimeter += first.distance(second);
    }
    twice_area.abs() <= POINT_EPSILON * perimeter.max(1.0)
  }

  fn chain_is_degenerate_at(&self, start: usize, end: usize, inset: f64) -> bool {
    let Some(chain) = self.chain_vertices(start, end) else {
      return false;
    };
    let points = chain
      .into_iter()
      .map(|index| self.vertices[index].position(inset))
      .collect::<Vec<_>>();

    let mut twice_area = 0.0;
    let mut perimeter = 0.0;
    for index in 0..points.len() {
      let first = points[index];
      let second = points[(index + 1) % points.len()];
      twice_area += first.cross(second);
      perimeter += first.distance(second);
    }
    twice_area.abs() <= POINT_EPSILON * perimeter.max(1.0)
  }

  fn chain_vertices(&self, start: usize, end: usize) -> Option<Vec<usize>> {
    if !self.vertices.get(start).is_some_and(|vertex| vertex.alive)
      || !self.vertices.get(end).is_some_and(|vertex| vertex.alive)
    {
      return None;
    }
    let mut current = start;
    let mut chain = Vec::new();
    for _ in 0..=self.vertices.len() {
      if !self.vertices[current].alive {
        return None;
      }
      chain.push(current);
      if current == end {
        return Some(chain);
      }
      current = self.vertices[current].next;
    }
    None
  }

  fn process_vertex_merge(
    &mut self,
    first_index: usize,
    second_index: usize,
    point: Point,
    inset: f64,
  ) -> bool {
    if first_index == second_index
      || !self
        .vertices
        .get(first_index)
        .is_some_and(|vertex| vertex.alive)
      || !self
        .vertices
        .get(second_index)
        .is_some_and(|vertex| vertex.alive)
    {
      return false;
    }
    let first = self.vertices[first_index];
    let second = self.vertices[second_index];
    if first.previous == second_index
      || first.next == second_index
      || second.previous == first_index
      || second.next == first_index
    {
      return false;
    }

    // At an endpoint hit, the two wavefront vertices coincide and their four
    // half-edges reconnect crosswise.  Treating it as an interior split first
    // creates a zero-length edge and repeatedly rediscovers the same event.
    // This is the patent's vertex-merge change, distinct from polygon split.
    let first_branch = Self::vertex_velocity(
      self.source_edges[first.left_edge].inward,
      self.source_edges[second.right_edge].inward,
    );
    let second_branch = Self::vertex_velocity(
      self.source_edges[second.left_edge].inward,
      self.source_edges[first.right_edge].inward,
    );
    let first_previous = first.previous;
    let first_next = first.next;
    let second_previous = second.previous;
    let second_next = second.next;
    match (first_branch, second_branch) {
      (None, None) => {
        // Two opposed front pairs have met.  The affected wavefront component
        // has collapsed to zero area (for example, an aligned rectangular
        // counter meeting its outer rectangle) and contributes no later cell.
        self.remove_loop(first_index);
        self.remove_loop(second_index);
        return true;
      }
      (Some(_), None) => {
        // A simultaneous edge collapse can make only one crosswise branch
        // finite.  Keep that branch and discard the other path only when the
        // complete path has zero area at this event.  This is the one-sided
        // vertex merge produced when an expanding counter becomes tangent to
        // the shrinking outer wavefront.
        if !self.chain_is_degenerate_at(first_next, second_previous, inset) {
          return false;
        }
        let Some(replacement) = self.create_vertex(
          first_previous,
          second_next,
          first.left_edge,
          second.right_edge,
          point,
          inset,
        ) else {
          return false;
        };
        self.vertices[first_index].alive = false;
        self.vertices[second_index].alive = false;
        self.vertices[first_previous].next = replacement;
        self.vertices[second_next].previous = replacement;
        self.vertices[second_previous].next = first_next;
        self.vertices[first_next].previous = second_previous;
        self.remove_loop(first_next);
        return true;
      }
      (None, Some(_)) => {
        if !self.chain_is_degenerate_at(second_next, first_previous, inset) {
          return false;
        }
        let Some(replacement) = self.create_vertex(
          second_previous,
          first_next,
          second.left_edge,
          first.right_edge,
          point,
          inset,
        ) else {
          return false;
        };
        self.vertices[first_index].alive = false;
        self.vertices[second_index].alive = false;
        self.vertices[second_previous].next = replacement;
        self.vertices[first_next].previous = replacement;
        self.vertices[first_previous].next = second_next;
        self.vertices[second_next].previous = first_previous;
        self.remove_loop(second_next);
        return true;
      }
      (Some(_), Some(_)) => {}
    }

    let Some(first_replacement) = self.create_vertex(
      first_previous,
      second_next,
      first.left_edge,
      second.right_edge,
      point,
      inset,
    ) else {
      return false;
    };
    let Some(second_replacement) = self.create_vertex(
      second_previous,
      first_next,
      second.left_edge,
      first.right_edge,
      point,
      inset,
    ) else {
      self.vertices[first_replacement].alive = false;
      return false;
    };
    self.vertices[first_index].alive = false;
    self.vertices[second_index].alive = false;
    self.vertices[first_previous].next = first_replacement;
    self.vertices[second_next].previous = first_replacement;
    self.vertices[second_previous].next = second_replacement;
    self.vertices[first_next].previous = second_replacement;
    self.remove_short_loops();
    true
  }

  fn process_opposed_edge_overlap(
    &mut self,
    first_index: usize,
    second_index: usize,
    point: Point,
    inset: f64,
  ) -> bool {
    let first = self.vertices[first_index];
    let second = self.vertices[second_index];
    let mut left_index = first.previous;
    let mut right_index = second.next;
    let previous_point = self.vertices[left_index].position(inset);
    let next_point = self.vertices[right_index].position(inset);
    let left_span = point.sub(previous_point);
    let right_span = next_point.sub(point);
    let left_length = left_span.length();
    let right_length = right_span.length();
    let scale = left_length.max(right_length).max(1.0);
    if left_length <= POINT_EPSILON
      || right_length <= POINT_EPSILON
      || left_span.cross(right_span).abs() > POINT_EPSILON * scale
      || left_span.dot(right_span) >= 0.0
    {
      return false;
    }

    // Oppositely moving parallel fronts cancel over their common interval;
    // only the portion owned by the longer source edge remains live.  Rebuild
    // the endpoint whose adjacent source edge changes so its trajectory is
    // derived from the surviving pair rather than from a stale bisector.  A
    // curved outline commonly produces a whole alternating collinear chain;
    // reduce that chain before creating either endpoint.
    let mut survivor_edge;
    let mut stable_left;
    if left_length > right_length + POINT_EPSILON {
      survivor_edge = first.left_edge;
      stable_left = true;
    } else if right_length > left_length + POINT_EPSILON {
      survivor_edge = second.right_edge;
      stable_left = false;
    } else {
      return false;
    }

    for _ in 0..=self.vertices.len() {
      let left = self.vertices[left_index];
      let right = self.vertices[right_index];
      let left_point = left.position(inset);
      let right_point = right.position(inset);
      if stable_left {
        if Self::vertex_velocity(
          self.source_edges[survivor_edge].inward,
          self.source_edges[right.right_edge].inward,
        )
        .is_some()
        {
          let Some(dead) = self.chain_vertices(left.next, right_index) else {
            return false;
          };
          let right_next = right.next;
          let Some(replacement) = self.create_vertex(
            left_index,
            right_next,
            survivor_edge,
            right.right_edge,
            right_point,
            inset,
          ) else {
            return false;
          };
          for index in dead {
            self.vertices[index].alive = false;
          }
          self.vertices[left_index].next = replacement;
          self.vertices[right_next].previous = replacement;
          self.remove_short_loops();
          return true;
        }

        let right_next_index = right.next;
        if right_next_index == left_index {
          return false;
        }
        let right_next_point = self.vertices[right_next_index].position(inset);
        let survivor_span = right_point.sub(left_point);
        let external_span = right_next_point.sub(right_point);
        let survivor_length = survivor_span.length();
        let external_length = external_span.length();
        let scale = survivor_length.max(external_length).max(1.0);
        if survivor_length <= POINT_EPSILON
          || external_length <= POINT_EPSILON
          || survivor_span.cross(external_span).abs() > POINT_EPSILON * scale
          || survivor_span.dot(external_span) >= 0.0
        {
          return false;
        }
        if (survivor_length - external_length).abs() <= POINT_EPSILON {
          let merge_point = left_point.add(right_next_point).scale(0.5);
          return self.process_vertex_merge(left_index, right_next_index, merge_point, inset);
        }
        right_index = right_next_index;
        if external_length > survivor_length {
          survivor_edge = right.right_edge;
          stable_left = false;
        }
      } else {
        if Self::vertex_velocity(
          self.source_edges[left.left_edge].inward,
          self.source_edges[survivor_edge].inward,
        )
        .is_some()
        {
          let Some(dead) = self.chain_vertices(left_index, right.previous) else {
            return false;
          };
          let left_previous = left.previous;
          let Some(replacement) = self.create_vertex(
            left_previous,
            right_index,
            left.left_edge,
            survivor_edge,
            left_point,
            inset,
          ) else {
            return false;
          };
          for index in dead {
            self.vertices[index].alive = false;
          }
          self.vertices[left_previous].next = replacement;
          self.vertices[right_index].previous = replacement;
          self.remove_short_loops();
          return true;
        }

        let left_previous_index = left.previous;
        if left_previous_index == right_index {
          return false;
        }
        let left_previous_point = self.vertices[left_previous_index].position(inset);
        let external_span = left_point.sub(left_previous_point);
        let survivor_span = right_point.sub(left_point);
        let external_length = external_span.length();
        let survivor_length = survivor_span.length();
        let scale = survivor_length.max(external_length).max(1.0);
        if survivor_length <= POINT_EPSILON
          || external_length <= POINT_EPSILON
          || external_span.cross(survivor_span).abs() > POINT_EPSILON * scale
          || external_span.dot(survivor_span) >= 0.0
        {
          return false;
        }
        if (survivor_length - external_length).abs() <= POINT_EPSILON {
          let merge_point = left_previous_point.add(right_point).scale(0.5);
          return self.process_vertex_merge(left_previous_index, right_index, merge_point, inset);
        }
        left_index = left_previous_index;
        if external_length > survivor_length {
          survivor_edge = left.left_edge;
          stable_left = true;
        }
      }
    }
    false
  }

  fn process_edge_event(
    &mut self,
    first_index: usize,
    second_index: usize,
    point: Point,
    inset: f64,
  ) -> bool {
    if !self
      .vertices
      .get(first_index)
      .is_some_and(|vertex| vertex.alive)
      || !self
        .vertices
        .get(second_index)
        .is_some_and(|vertex| vertex.alive)
      || self.vertices[first_index].next != second_index
    {
      return false;
    }
    let first = self.vertices[first_index];
    let second = self.vertices[second_index];
    let previous = first.previous;
    let next = second.next;
    let Some(replacement) = self.create_vertex(
      previous,
      next,
      first.left_edge,
      second.right_edge,
      point,
      inset,
    ) else {
      // When the neighbouring source edges are opposed, the disappearing
      // stretch cannot be replaced by a finite wavefront vertex.  This is a
      // terminal edge-collapse only if the complete live loop has zero area
      // at the event (a rectangle reaching its medial ridge, for example).
      // A non-degenerate loop needs a grouped simultaneous-event update, so
      // leave it untouched and let the caller use its conservative fallback.
      if self.loop_is_degenerate_at(first_index, inset) {
        self.remove_loop(first_index);
        return true;
      }
      let previous_point = self.vertices[previous].position(inset);
      let next_point = self.vertices[next].position(inset);
      if previous_point.distance(next_point) <= POINT_EPSILON * 16.0 {
        let merge_point = previous_point.add(next_point).scale(0.5);
        return self.process_vertex_merge(previous, next, merge_point, inset);
      }
      return self.process_opposed_edge_overlap(first_index, second_index, point, inset);
    };
    self.vertices[first_index].alive = false;
    self.vertices[second_index].alive = false;
    self.vertices[previous].next = replacement;
    self.vertices[next].previous = replacement;
    self.remove_short_loops();
    true
  }

  /// Resolves the degenerate form of a split event where one new crosswise
  /// vertex has opposed incident edges and therefore no finite velocity.
  ///
  /// The two opposed stretches cancel over their common interval.  The part
  /// belonging to only one stretch remains in the wavefront, and cancellation
  /// continues across an alternating collinear chain until a finite pair of
  /// outside edges is reached.  The independent, non-opposed crosswise branch
  /// is emitted at the original hit point at the same time.
  fn process_opposed_split_overlap(
    &mut self,
    event_vertex: usize,
    junction_vertices: [usize; 2],
    junction_edges: [usize; 2],
    overlap_vertices: [usize; 2],
    mut meeting_point: Point,
    inset: f64,
  ) -> bool {
    let [junction_previous, junction_next] = junction_vertices;
    let [junction_left_edge, junction_right_edge] = junction_edges;
    let [mut left_index, mut right_index] = overlap_vertices;
    let junction_point = meeting_point;
    let mut dead = vec![event_vertex];
    let mut terminal = None;

    for _ in 0..=self.vertices.len() {
      let Some((&left, &right)) = self
        .vertices
        .get(left_index)
        .zip(self.vertices.get(right_index))
      else {
        return false;
      };
      if !left.alive
        || !right.alive
        || left_index == right_index
        || dead.contains(&left_index)
        || dead.contains(&right_index)
      {
        return false;
      }

      let left_point = left.position(inset);
      let right_point = right.position(inset);
      let left_span = meeting_point.sub(left_point);
      let right_span = right_point.sub(meeting_point);
      let left_length = left_span.length();
      let right_length = right_span.length();
      let scale = left_length.max(right_length).max(1.0);
      if left_length <= POINT_EPSILON
        || right_length <= POINT_EPSILON
        || left_span.cross(right_span).abs() > POINT_EPSILON * scale
        || left_span.dot(right_span) >= 0.0
        || Self::vertex_velocity(
          self.source_edges[left.right_edge].inward,
          self.source_edges[right.left_edge].inward,
        )
        .is_some()
      {
        return false;
      }

      if left_length > right_length + POINT_EPSILON {
        let survivor_edge = left.right_edge;
        if Self::vertex_velocity(
          self.source_edges[survivor_edge].inward,
          self.source_edges[right.right_edge].inward,
        )
        .is_some()
        {
          dead.push(right_index);
          terminal = Some((
            left_index,
            right.next,
            survivor_edge,
            right.right_edge,
            right_point,
          ));
          break;
        }
        dead.push(right_index);
        meeting_point = right_point;
        right_index = right.next;
        continue;
      }

      if right_length > left_length + POINT_EPSILON {
        let survivor_edge = right.left_edge;
        if Self::vertex_velocity(
          self.source_edges[left.left_edge].inward,
          self.source_edges[survivor_edge].inward,
        )
        .is_some()
        {
          dead.push(left_index);
          terminal = Some((
            left.previous,
            right_index,
            left.left_edge,
            survivor_edge,
            left_point,
          ));
          break;
        }
        dead.push(left_index);
        meeting_point = left_point;
        left_index = left.previous;
        continue;
      }

      let coincident_point = left_point.add(right_point).scale(0.5);
      if Self::vertex_velocity(
        self.source_edges[left.left_edge].inward,
        self.source_edges[right.right_edge].inward,
      )
      .is_some()
      {
        dead.push(left_index);
        dead.push(right_index);
        terminal = Some((
          left.previous,
          right.next,
          left.left_edge,
          right.right_edge,
          coincident_point,
        ));
        break;
      }
      dead.push(left_index);
      dead.push(right_index);
      meeting_point = coincident_point;
      left_index = left.previous;
      right_index = right.next;
    }

    let Some((terminal_previous, terminal_next, terminal_left_edge, terminal_right_edge, point)) =
      terminal
    else {
      return false;
    };
    let live_endpoints = [
      junction_previous,
      junction_next,
      terminal_previous,
      terminal_next,
    ];
    if live_endpoints.into_iter().any(|index| {
      dead.contains(&index) || !self.vertices.get(index).is_some_and(|vertex| vertex.alive)
    }) {
      return false;
    }

    let Some(junction) = self.create_vertex(
      junction_previous,
      junction_next,
      junction_left_edge,
      junction_right_edge,
      junction_point,
      inset,
    ) else {
      return false;
    };
    let Some(terminal) = self.create_vertex(
      terminal_previous,
      terminal_next,
      terminal_left_edge,
      terminal_right_edge,
      point,
      inset,
    ) else {
      self.vertices[junction].alive = false;
      return false;
    };

    dead.sort_unstable();
    dead.dedup();
    for index in dead {
      self.vertices[index].alive = false;
    }
    self.vertices[junction_previous].next = junction;
    self.vertices[junction_next].previous = junction;
    self.vertices[terminal_previous].next = terminal;
    self.vertices[terminal_next].previous = terminal;
    self.remove_short_loops();
    true
  }

  fn process_split_event(
    &mut self,
    vertex_index: usize,
    edge_start_index: usize,
    point: Point,
    inset: f64,
  ) -> bool {
    if !self
      .vertices
      .get(vertex_index)
      .is_some_and(|vertex| vertex.alive)
      || !self
        .vertices
        .get(edge_start_index)
        .is_some_and(|vertex| vertex.alive)
    {
      return false;
    }
    let vertex = self.vertices[vertex_index];
    let edge_start = self.vertices[edge_start_index];
    let edge_end_index = edge_start.next;
    if vertex_index == edge_start_index
      || vertex_index == edge_end_index
      || !self.vertices[edge_end_index].alive
    {
      return false;
    }
    let edge_start_point = edge_start.position(inset);
    let edge_end_point = self.vertices[edge_end_index].position(inset);
    if point.distance(edge_start_point) <= POINT_EPSILON * 16.0 {
      return self.process_vertex_merge(vertex_index, edge_start_index, point, inset);
    }
    if point.distance(edge_end_point) <= POINT_EPSILON * 16.0 {
      return self.process_vertex_merge(vertex_index, edge_end_index, point, inset);
    }

    let previous = vertex.previous;
    let next = vertex.next;
    let hit_edge = edge_start.right_edge;
    let first_is_finite = Self::vertex_velocity(
      self.source_edges[vertex.left_edge].inward,
      self.source_edges[hit_edge].inward,
    )
    .is_some();
    let second_is_finite = Self::vertex_velocity(
      self.source_edges[hit_edge].inward,
      self.source_edges[vertex.right_edge].inward,
    )
    .is_some();
    match (first_is_finite, second_is_finite) {
      (false, true) => {
        return self.process_opposed_split_overlap(
          vertex_index,
          [edge_start_index, next],
          [hit_edge, vertex.right_edge],
          [previous, edge_end_index],
          point,
          inset,
        );
      }
      (true, false) => {
        return self.process_opposed_split_overlap(
          vertex_index,
          [previous, edge_end_index],
          [vertex.left_edge, hit_edge],
          [edge_start_index, next],
          point,
          inset,
        );
      }
      (false, false) => return false,
      (true, true) => {}
    }
    let Some(first_split) = self.create_vertex(
      previous,
      edge_end_index,
      vertex.left_edge,
      hit_edge,
      point,
      inset,
    ) else {
      return false;
    };
    let Some(second_split) = self.create_vertex(
      edge_start_index,
      next,
      hit_edge,
      vertex.right_edge,
      point,
      inset,
    ) else {
      self.vertices[first_split].alive = false;
      return false;
    };

    self.vertices[vertex_index].alive = false;
    self.vertices[previous].next = first_split;
    self.vertices[edge_end_index].previous = first_split;
    self.vertices[edge_start_index].next = second_split;
    self.vertices[next].previous = second_split;
    self.remove_short_loops();
    true
  }

  fn build(mut self, maximum_inset: f64) -> Option<Vec<DirectInsetCell>> {
    if !maximum_inset.is_finite() || maximum_inset <= TIME_EPSILON {
      return Some(Vec::new());
    }
    let mut now = 0.0;
    let mut event_count = 0_usize;
    let event_limit = self.vertices.len().saturating_mul(8).max(64);
    while self.live_vertices().next().is_some() && now < maximum_inset - TIME_EPSILON {
      let Some(event) = self.earliest_event(now) else {
        self.record_interval(now, maximum_inset);
        now = maximum_inset;
        break;
      };
      if !event.inset.is_finite() || event.inset > maximum_inset + TIME_EPSILON {
        self.record_interval(now, maximum_inset);
        now = maximum_inset;
        break;
      }
      if event.inset + TIME_EPSILON < now {
        return None;
      }
      let event_inset = event.inset.clamp(now, maximum_inset);
      self.record_interval(now, event_inset);
      match event.kind {
        DirectInsetEventKind::Edge { first, second } => {
          if !self.process_edge_event(first, second, event.point, event_inset) {
            return None;
          }
        }
        DirectInsetEventKind::Split { vertex, edge_start } => {
          if !self.process_split_event(vertex, edge_start, event.point, event_inset) {
            return None;
          }
        }
      }
      now = event_inset;
      event_count += 1;
      if event_count > event_limit {
        return None;
      }
    }
    if now < maximum_inset - TIME_EPSILON {
      self.record_interval(now, maximum_inset);
    }
    Some(self.cells)
  }
}

/// Computes the complete, truncated direct-inset graph and returns its swept
/// source-edge cells.  `contours` retain their original winding; the caller's
/// `solid_on_right` flag selects the common material-side normal for outer
/// outlines and counters alike.
pub(super) fn direct_inset_cells(
  contours: &[&[(f32, f32)]],
  solid_on_right: bool,
  maximum_inset: f32,
) -> Option<Vec<DirectInsetCell>> {
  DirectInsetGraph::new(contours, solid_on_right)?.build(f64::from(maximum_inset))
}

#[cfg(test)]
mod tests {
  use super::{DirectInsetEdgeId, DirectInsetVertexId, direct_inset_cells};

  fn boundary_points(cell: &super::DirectInsetCell, inner: bool) -> [(f32, f32); 2] {
    let boundary = if inner { cell.inner } else { cell.outer };
    boundary.endpoints.map(|endpoint| endpoint.point)
  }

  #[test]
  fn rectangle_cells_stop_on_the_first_medial_ridge() {
    let rectangle = [(0.0, 0.0), (10.0, 0.0), (10.0, 4.0), (0.0, 4.0)];
    let cells = direct_inset_cells(&[&rectangle], true, 8.0).expect("rectangle graph");

    assert_eq!(cells.len(), 4);
    assert!(
      cells
        .iter()
        .all(|cell| (cell.inner.inset - 2.0).abs() < 1.0e-5)
    );
    let top = cells
      .iter()
      .find(|cell| {
        cell.source_edge
          == DirectInsetEdgeId {
            contour_index: 0,
            edge_index: 0,
          }
      })
      .expect("top cell");
    assert_eq!(boundary_points(top, false), [(0.0, 0.0), (10.0, 0.0)]);
    assert_eq!(boundary_points(top, true), [(2.0, 2.0), (8.0, 2.0)]);
    assert_eq!(
      top.outer.endpoints.map(|endpoint| endpoint.source_vertex),
      [
        Some(DirectInsetVertexId {
          contour_index: 0,
          vertex_index: 0,
        }),
        Some(DirectInsetVertexId {
          contour_index: 0,
          vertex_index: 1,
        }),
      ]
    );
  }

  #[test]
  fn rectangle_truncation_keeps_one_cell_per_source_edge() {
    let rectangle = [(0.0, 0.0), (10.0, 0.0), (10.0, 4.0), (0.0, 4.0)];
    let cells = direct_inset_cells(&[&rectangle], true, 1.0).expect("truncated graph");

    assert_eq!(cells.len(), 4);
    assert!(
      cells
        .iter()
        .all(|cell| (cell.inner.inset - 1.0).abs() < 1.0e-5)
    );
    let top = &cells[0];
    assert_eq!(boundary_points(top, true), [(1.0, 1.0), (9.0, 1.0)]);
  }

  #[test]
  fn counter_wavefront_merges_with_the_outer_outline() {
    let outer = [(0.0, 0.0), (12.0, 0.0), (12.0, 12.0), (0.0, 12.0)];
    let counter = [(4.0, 4.0), (4.0, 8.0), (8.0, 8.0), (8.0, 4.0)];
    let cells = direct_inset_cells(&[&outer, &counter], true, 3.0).expect("counter graph");

    assert!(!cells.is_empty());
    assert!(cells.iter().any(|cell| cell.source_edge.contour_index == 0));
    assert!(cells.iter().any(|cell| cell.source_edge.contour_index == 1));
    assert!(cells.iter().any(|cell| cell.inner.inset < 3.0 - 1.0e-5));
    assert!(
      cells
        .iter()
        .all(|cell| cell.outer.inset <= cell.inner.inset)
    );
  }

  #[test]
  fn nonparallel_counter_vertex_merges_into_an_outer_edge() {
    let outer = [(0.0, 0.0), (12.0, 0.0), (12.0, 12.0), (0.0, 12.0)];
    let counter = [(6.0, 3.0), (3.0, 6.0), (6.0, 9.0), (9.0, 6.0)];
    let cells = direct_inset_cells(&[&outer, &counter], true, 2.0).expect("diamond merge");
    let expected_merge = 3.0 / (1.0 + 2.0_f32.sqrt());

    assert!(cells.iter().any(|cell| {
      (cell.inner.inset - expected_merge).abs() < 1.0e-4 && cell.source_edge.contour_index == 0
    }));
    assert!(cells.iter().any(|cell| {
      cell.outer.inset >= expected_merge - 1.0e-4 && cell.source_edge.contour_index == 1
    }));
  }

  #[test]
  fn partially_overlapping_counter_merge_keeps_the_surviving_branch() {
    let outer = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (0.0, 20.0)];
    let counter = [(5.0, 4.0), (5.0, 8.0), (10.0, 8.0), (10.0, 4.0)];
    let cells = direct_inset_cells(&[&outer, &counter], true, 3.0)
      .expect("partially overlapping counter graph");

    assert!(cells.iter().any(|cell| cell.inner.inset < 3.0 - 1.0e-5));
    assert!(
      cells
        .iter()
        .any(|cell| { cell.outer.inset >= 2.0 - 1.0e-5 && cell.source_edge.contour_index == 0 })
    );
    assert!(
      cells
        .iter()
        .any(|cell| { cell.outer.inset >= 2.0 - 1.0e-5 && cell.source_edge.contour_index == 1 })
    );
  }

  #[test]
  fn opposed_split_overlap_is_independent_of_counter_start_vertex() {
    let outer = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (0.0, 20.0)];
    // Same clockwise counter as the preceding test, rotated so the other
    // endpoint of its top-moving edge is visited first. This exercises the
    // symmetric split branch without changing any geometry.
    let counter = [(10.0, 4.0), (5.0, 4.0), (5.0, 8.0), (10.0, 8.0)];
    let cells = direct_inset_cells(&[&outer, &counter], true, 3.0)
      .expect("rotated partially overlapping counter graph");

    assert!(cells.iter().any(|cell| cell.inner.inset < 3.0 - 1.0e-5));
    for contour_index in 0..=1 {
      assert!(cells.iter().any(|cell| {
        cell.outer.inset >= 2.0 - 1.0e-5 && cell.source_edge.contour_index == contour_index
      }));
    }
  }

  #[test]
  fn equal_opposed_split_overlap_continues_across_the_far_corner() {
    let outer = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (0.0, 20.0)];
    // At inset 2 the complete counter bottom and the outer top-right corner
    // coincide. Their equal horizontal overlap ends in a second opposed
    // vertical pair, so the cancellation must continue to the next finite
    // pair instead of dropping the graph.
    let counter = [(5.0, 4.0), (5.0, 8.0), (16.0, 8.0), (16.0, 4.0)];
    let cells = direct_inset_cells(&[&outer, &counter], true, 2.5)
      .expect("equal opposed-overlap counter graph");

    assert!(cells.iter().any(|cell| cell.inner.inset < 2.5 - 1.0e-5));
    for contour_index in 0..=1 {
      assert!(cells.iter().any(|cell| {
        cell.outer.inset >= 2.0 - 1.0e-5 && cell.source_edge.contour_index == contour_index
      }));
    }
  }

  #[test]
  fn reflex_vertex_split_changes_the_live_source_edge_stretches() {
    let concave = [
      (0.0, 0.0),
      (12.0, 0.0),
      (12.0, 12.0),
      (8.0, 12.0),
      (8.0, 5.0),
      (6.0, 7.0),
      (4.0, 5.0),
      (4.0, 12.0),
      (0.0, 12.0),
    ];
    let cells = direct_inset_cells(&[&concave], true, 3.0).expect("concave split");

    assert!(!cells.is_empty());
    assert!(cells.iter().any(|cell| cell.inner.inset < 3.0 - 1.0e-5));
    assert!(cells.iter().any(|cell| {
      cell.outer.inset > 0.0
        && cell
          .outer
          .endpoints
          .iter()
          .any(|endpoint| endpoint.source_vertex.is_none())
    }));
    assert!(
      cells
        .iter()
        .all(|cell| cell.outer.inset <= cell.inner.inset)
    );
  }
}
