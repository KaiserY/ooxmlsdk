//! Conservative broad phase for the next wavefront event.
//!
//! Between events, every vertex follows an affine trajectory. The first known
//! edge collapse bounds the search interval: a split later than it cannot win.
//! A vertex and a moving stretch can meet only if their swept boxes overlap.
//! This index removes impossible pairs; the caller retains all original event
//! predicates and restores source order before resolving simultaneous events.

use super::{ActiveVertex, POINT_EPSILON, Point};

#[derive(Clone, Copy)]
struct Bounds {
  min: Point,
  max: Point,
}

impl Bounds {
  fn vertex(vertex: ActiveVertex, start: f64, end: f64) -> Option<Self> {
    let a = vertex.position(start);
    let b = vertex.position(end);
    [a.x, a.y, b.x, b.y]
      .into_iter()
      .all(f64::is_finite)
      .then_some(Self {
        min: Point {
          x: a.x.min(b.x),
          y: a.y.min(b.y),
        },
        max: Point {
          x: a.x.max(b.x),
          y: a.y.max(b.y),
        },
      })
  }

  fn union(self, other: Self) -> Self {
    Self {
      min: Point {
        x: self.min.x.min(other.min.x),
        y: self.min.y.min(other.min.y),
      },
      max: Point {
        x: self.max.x.max(other.max.x),
        y: self.max.y.max(other.max.y),
      },
    }
  }

  fn expanded(mut self) -> Self {
    // Match the narrow phase's point-to-stretch allowance, plus an outward
    // roundoff bound for its subtraction, multiply and addition. This only
    // admits extra candidates; it never changes the collision tolerance.
    let magnitude = self
      .min
      .x
      .abs()
      .max(self.min.y.abs())
      .max(self.max.x.abs())
      .max(self.max.y.abs());
    let padding = POINT_EPSILON * 16.0 + magnitude * (f64::EPSILON * 8.0);
    self.min.x = (self.min.x - padding).next_down();
    self.min.y = (self.min.y - padding).next_down();
    self.max.x = (self.max.x + padding).next_up();
    self.max.y = (self.max.y + padding).next_up();
    self
  }

  fn intersects(self, other: Self) -> bool {
    self.min.x <= other.max.x
      && other.min.x <= self.max.x
      && self.min.y <= other.max.y
      && other.min.y <= self.max.y
  }
}

struct Node {
  bounds: Bounds,
  start: usize,
  end: usize,
  children: Option<(usize, usize)>,
}

pub(super) struct SweepIndex {
  boxes: Vec<Bounds>,
  order: Vec<usize>,
  nodes: Vec<Node>,
  start: f64,
  end: f64,
}

impl SweepIndex {
  pub(super) fn new(
    vertices: &[ActiveVertex],
    live: &[usize],
    start: f64,
    end: f64,
  ) -> Option<Self> {
    if live.len() < 32 || !start.is_finite() || !end.is_finite() || end < start {
      return None;
    }
    let boxes = live
      .iter()
      .map(|&index| {
        let first = vertices[index];
        Some(
          Bounds::vertex(first, start, end)?
            .union(Bounds::vertex(vertices[first.next], start, end)?)
            .expanded(),
        )
      })
      .collect::<Option<Vec<_>>>()?;
    let mut result = Self {
      boxes,
      order: (0..live.len()).collect(),
      nodes: Vec::with_capacity(live.len()),
      start,
      end,
    };
    result.build(0, live.len());
    Some(result)
  }

  fn build(&mut self, start: usize, end: usize) -> usize {
    let bounds = self.order[start + 1..end]
      .iter()
      .fold(self.boxes[self.order[start]], |bounds, &index| {
        bounds.union(self.boxes[index])
      });
    let index = self.nodes.len();
    self.nodes.push(Node {
      bounds,
      start,
      end,
      children: None,
    });
    if end - start > 8 {
      let x_axis = bounds.max.x - bounds.min.x >= bounds.max.y - bounds.min.y;
      let boxes = &self.boxes;
      let center = |index: usize| {
        let bounds = boxes[index];
        if x_axis {
          bounds.min.x * 0.5 + bounds.max.x * 0.5
        } else {
          bounds.min.y * 0.5 + bounds.max.y * 0.5
        }
      };
      let middle = (start + end) / 2;
      self.order[start..end].select_nth_unstable_by(middle - start, |&left, &right| {
        center(left).total_cmp(&center(right))
      });
      let left = self.build(start, middle);
      let right = self.build(middle, end);
      self.nodes[index].children = Some((left, right));
    }
    index
  }

  pub(super) fn candidates(&self, vertex: ActiveVertex, output: &mut Vec<usize>) {
    output.clear();
    let Some(bounds) = Bounds::vertex(vertex, self.start, self.end) else {
      output.extend(0..self.order.len());
      return;
    };
    self.visit(0, bounds, output);
    output.sort_unstable();
  }

  fn visit(&self, index: usize, bounds: Bounds, output: &mut Vec<usize>) {
    let node = &self.nodes[index];
    if !node.bounds.intersects(bounds) {
      return;
    }
    if let Some((left, right)) = node.children {
      self.visit(left, bounds, output);
      self.visit(right, bounds, output);
    } else {
      output.extend(
        self.order[node.start..node.end]
          .iter()
          .copied()
          .filter(|&index| self.boxes[index].intersects(bounds)),
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn swept_index_retains_endpoint_hits_and_source_order() {
    let vertices = (0..96)
      .map(|index| ActiveVertex {
        alive: true,
        previous: (index + 95) % 96,
        next: (index + 1) % 96,
        left_edge: index,
        right_edge: index,
        anchor: Point {
          x: (index % 12) as f64 * 8.0,
          y: (index / 12) as f64 * 7.0,
        },
        anchor_time: 0.25,
        velocity: Point {
          x: (index % 3) as f64 - 1.0,
          y: (index % 5) as f64 - 2.0,
        },
        source_vertex: None,
      })
      .collect::<Vec<_>>();
    let live = (0..vertices.len()).collect::<Vec<_>>();
    let index = SweepIndex::new(&vertices, &live, 0.5, 3.0).unwrap();
    let mut candidates = Vec::new();
    for (edge_index, &vertex) in vertices.iter().enumerate() {
      for time in [0.5, 0.5_f64.next_up(), 1.75, 3.0_f64.next_down(), 3.0] {
        let hit = vertex.position(time);
        let query = ActiveVertex {
          anchor: hit,
          anchor_time: time,
          velocity: Point { x: 4.0, y: -3.0 },
          ..vertex
        };
        index.candidates(query, &mut candidates);
        assert!(candidates.contains(&edge_index));
        assert!(candidates.windows(2).all(|pair| pair[0] < pair[1]));
      }
    }
  }
}
