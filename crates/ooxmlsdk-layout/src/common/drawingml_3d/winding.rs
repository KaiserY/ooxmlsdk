//! Exact winding queries over the edges of an inset cap.
//!
//! As in WPF's geometry scanner, only edges active at the query height can
//! contribute to winding. Queries here need not arrive in scan order (the cap
//! can be projected), so retain bounded height buckets instead of a mutable
//! sweep list. Buckets select candidates only: no vertex, crossing predicate,
//! sample location or fill convention is quantized.

type Edge = [(f32, f32); 2];

pub(super) struct BoundaryWinding {
  edges: Vec<Edge>,
  buckets: Vec<Vec<usize>>,
  top: f64,
  scale: f64,
}

impl BoundaryWinding {
  pub(super) fn new(edges: Vec<Edge>) -> Self {
    let mut result = Self {
      edges,
      buckets: Vec::new(),
      top: 0.0,
      scale: 0.0,
    };
    if result.edges.len() < 32
      || result
        .edges
        .iter()
        .flatten()
        .any(|&(x, y)| !x.is_finite() || !y.is_finite())
    {
      return result;
    }
    let (top, bottom) = result.edges.iter().flatten().fold(
      (f64::INFINITY, f64::NEG_INFINITY),
      |(top, bottom), &(_, y)| (top.min(f64::from(y)), bottom.max(f64::from(y))),
    );
    if bottom <= top {
      return result;
    }
    let count = result.edges.len().div_ceil(16).min(128);
    result.top = top;
    result.scale = count as f64 / (bottom - top);
    result.buckets = vec![Vec::new(); count];
    for (index, &[first, second]) in result.edges.iter().enumerate() {
      if first.1 == second.1 {
        continue;
      }
      let first_bucket = result.bucket(first.1.min(second.1));
      let last_bucket = result.bucket(first.1.max(second.1));
      // Include both endpoints' buckets. The monotone mapping then contains
      // every intermediate query height; the original half-open Y test below
      // resolves exact boundary ownership, including bucket boundaries.
      for bucket in &mut result.buckets[first_bucket..=last_bucket] {
        bucket.push(index);
      }
    }
    result
  }

  fn bucket(&self, y: f32) -> usize {
    (((f64::from(y) - self.top) * self.scale) as usize).min(self.buckets.len() - 1)
  }

  pub(super) fn contains(&self, point: (f32, f32)) -> bool {
    if self.buckets.is_empty() || !point.1.is_finite() {
      return self
        .edges
        .iter()
        .map(|edge| edge_winding(edge, point))
        .sum::<i32>()
        != 0;
    }
    self.buckets[self.bucket(point.1)]
      .iter()
      .map(|&index| edge_winding(&self.edges[index], point))
      .sum::<i32>()
      != 0
  }
}

fn edge_winding(&[first, second]: &Edge, point: (f32, f32)) -> i32 {
  if first.1 <= point.1 {
    if second.1 > point.1 && super::text_surface_edge(first, second, point) > 0.0 {
      return 1;
    }
  } else if second.1 <= point.1 && super::text_surface_edge(first, second, point) < 0.0 {
    return -1;
  }
  0
}

#[cfg(test)]
mod tests {
  use super::{BoundaryWinding, Edge, edge_winding};

  fn check(edges: Vec<Edge>, points: impl IntoIterator<Item = (f32, f32)>) {
    let boundary = BoundaryWinding::new(edges);
    for point in points {
      let expected = boundary
        .edges
        .iter()
        .map(|edge| edge_winding(edge, point))
        .sum::<i32>()
        != 0;
      assert_eq!(boundary.contains(point), expected, "point={point:?}");
    }
  }

  #[test]
  fn buckets_preserve_winding_at_vertices_holes_and_adjacent_floats() {
    let mut edges = Vec::new();
    for index in 0..64 {
      let x = index as f32 * 3.125 - 100.0;
      let y = (index % 7) as f32 * 0.375 - 1.0;
      for (inset, reverse) in [(0.0, false), (0.25, true)] {
        let mut points = [
          (x + inset, y + inset),
          (x + 2.0 - inset, y + inset),
          (x + 1.25, y + 3.0 - inset),
          (x + inset, y + 3.0 - inset),
        ];
        if reverse {
          points.reverse();
        }
        edges.extend((0..4).map(|i| [points[i], points[(i + 1) % 4]]));
      }
    }
    let mut queries = Vec::new();
    for &[a, b] in &edges {
      for point in [a, b, ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5)] {
        for x in [point.0.next_down(), point.0, point.0.next_up()] {
          for y in [point.1.next_down(), point.1, point.1.next_up()] {
            queries.push((x, y));
          }
        }
      }
    }
    queries.extend((-100..101).flat_map(|x| (-30..81).map(move |y| (x as f32, y as f32 / 10.0))));
    check(edges, queries);
  }

  #[test]
  fn buckets_preserve_degenerate_and_nonfinite_queries() {
    for edges in [
      Vec::new(),
      vec![[(0.0, 1.0), (2.0, 1.0)]; 64],
      vec![[(0.0, -f32::MAX), (1.0, f32::MAX)]; 64],
      vec![[(0.0, f32::NAN), (1.0, f32::INFINITY)]; 64],
    ] {
      check(
        edges,
        [
          (0.0, 0.0),
          (1.0, 1.0),
          (0.0, f32::NAN),
          (0.0, f32::INFINITY),
          (0.0, f32::NEG_INFINITY),
        ],
      );
    }
  }
}
