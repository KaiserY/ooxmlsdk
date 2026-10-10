//! Word's pie label layout, in chart cells (0.75pt), x-right/y-up.
//!
//! Plain best-fit labels have a stepped line outline, five integer fitting
//! widths and a dependent callout. The layout cost measures those shapes,
//! rather than treating every label as an axis-aligned bounding rectangle.
//! Constants and state transitions below follow observation-only native
//! LLPieLabel2/LLPieCallout/LLLayout controls; drawing and layout are separate.

const EPS: f64 = 1.0e-6;
// CHART's degree conversion uses this decimal constant, including at arc
// endpoints. Retaining it matters for tangencies and deterministic layout.
const NATIVE_PI: f64 = 3_141_592_700.0 / 1_000_000_000.0;

pub(super) type Point = (f64, f64);
pub(super) type Bounds = [f64; 4];

#[derive(Clone, Debug)]
pub(super) struct Sector {
  pub center: Point,
  pub radius: f64,
  pub start: f64,
  pub end: f64,
  pub arc: Point,
  endpoints: [Point; 2],
  bounds: Bounds,
  projected: Option<ProjectedSector>,
}

#[derive(Clone, Debug)]
pub(super) struct ProjectedSector {
  pub center: Point,
  pub endpoints: [Point; 2],
  pub arc: Point,
  pub outline: Vec<Point>,
  pub ellipse_center: Point,
  pub ellipse_radii: Point,
  pub inner_direction: Point,
}

impl Sector {
  pub fn new(center: Point, radius: f64, start: f64, end: f64) -> Self {
    let at = |angle: f64| {
      let a = angle * NATIVE_PI / 180.0;
      (center.0 + radius * a.cos(), center.1 + radius * a.sin())
    };
    let endpoints = [at(start), at(end)];
    let mut corners = vec![center, endpoints[0], endpoints[1]];
    for quadrant in (start / 90.0).ceil() as i32..=(end / 90.0).floor() as i32 {
      corners.push(at(f64::from(quadrant) * 90.0));
    }
    Self {
      center,
      radius,
      start,
      end,
      arc: at((start + end) * 0.5),
      endpoints,
      bounds: point_bounds(&corners),
      projected: None,
    }
  }

  pub fn projected(start: f64, end: f64, geometry: ProjectedSector) -> Self {
    Self {
      center: geometry.center,
      // The fitting-width height guard is measured in source model units,
      // independently of the projected ellipse's size in chart cells.
      radius: 100.0,
      start,
      end,
      arc: geometry.arc,
      endpoints: geometry.endpoints,
      bounds: point_bounds(&geometry.outline),
      projected: Some(geometry),
    }
  }

  pub fn is_projected(&self) -> bool {
    self.projected.is_some()
  }

  pub fn projected_inner_translation(&self, vertices: &[Point], margin: f64) -> Option<Point> {
    let projected = self.projected.as_ref()?;
    if vertices.is_empty() || self.end - self.start < EPS {
      return None;
    }
    let a = sub(self.endpoints[0], self.center);
    let b = sub(self.endpoints[1], self.center);
    let determinant = cross(a, b);
    let translation = if determinant < EPS {
      // A wide wedge has no intersection of two inward radial half planes.
      // Native LLPieSlice3DElement aligns the nearest text edge to its hub.
      let nearest = vertices
        .windows(2)
        .map(|edge| segment_nearest(self.center, edge[0], edge[1]))
        .min_by(|first, second| first.0.total_cmp(&second.0))?;
      sub(self.center, nearest.1)
    } else {
      let mut alpha = f64::INFINITY;
      let mut beta = f64::INFINITY;
      for &point in vertices {
        let relative = sub(point, self.center);
        alpha = alpha.min(cross(relative, b) / determinant);
        beta = beta.min(cross(a, relative) / determinant);
      }
      sub(scale(a, -alpha), scale(b, beta))
    };
    let direction = unit(projected.inner_direction);
    let (rx, ry) = projected.ellipse_radii;
    if rx <= EPS || ry <= EPS || dot(direction, direction) < EPS {
      return None;
    }
    let (rx_squared, ry_squared) = (rx * rx, ry * ry);
    let quadratic = direction.0.powi(2) * ry_squared + direction.1.powi(2) * rx_squared;
    let mut clearance = f64::INFINITY;
    for &point in vertices {
      let q = sub(add(point, translation), projected.ellipse_center);
      let constant = q.0.powi(2) * ry_squared + q.1.powi(2) * rx_squared - rx_squared * ry_squared;
      if constant > 0.0 {
        return None;
      }
      let linear = q.0 * direction.0 * ry_squared + q.1 * direction.1 * rx_squared;
      clearance =
        clearance.min((-linear + (linear * linear - quadratic * constant).sqrt()) / quadratic);
    }
    (clearance >= margin).then(|| add(translation, scale(direction, clearance - margin)))
  }

  fn contains_angle(&self, p: Point) -> bool {
    let angle = (p.1 - self.center.1).atan2(p.0 - self.center.0) * 180.0 / NATIVE_PI;
    (angle - self.start).rem_euclid(360.0) <= self.end - self.start + EPS
  }

  fn contains(&self, p: Point) -> bool {
    squared(p, self.center) <= self.radius * self.radius + EPS && self.contains_angle(p)
  }
}

#[derive(Clone, Debug)]
pub(super) struct TextShape {
  pub width: i32,
  pub bounds: Bounds,
  pub vertices: Vec<Point>,
  pub attachments: Vec<u8>,
  pub lines: usize,
  pub overflowed_word: bool,
}

#[derive(Clone, Debug)]
pub(super) struct Label {
  pub sector: usize,
  pub bounds: Bounds,
  pub shapes: Vec<TextShape>,
  pub shape: usize,
  pub movable: bool,
  pub locked_width: bool,
  pub initial_word_overflow: bool,
  pub original_width: i32,
  pub minimum_width: i32,
  pub maximum_width: i32,
  pub leader: bool,
  force_leader: bool,
  leader_points: Vec<Point>,
  leader_visible: bool,
  version: u64,
  leader_version: u64,
}

impl Label {
  pub fn new(
    sector: usize,
    bounds: Bounds,
    shapes: Vec<TextShape>,
    shape: usize,
    movable: bool,
    leader: bool,
  ) -> Self {
    let original_width = shapes[shape].width;
    let maximum_width = shapes.iter().map(|shape| shape.width).max().unwrap_or(20);
    let minimum_width = shapes.iter().map(|shape| shape.width).min().unwrap_or(20);
    Self {
      sector,
      bounds,
      shapes,
      shape,
      movable,
      locked_width: false,
      initial_word_overflow: false,
      original_width,
      minimum_width,
      maximum_width,
      leader,
      force_leader: false,
      leader_points: Vec::new(),
      leader_visible: false,
      version: 0,
      leader_version: 0,
    }
  }

  fn outline(&self) -> Outline {
    let shape = &self.shapes[self.shape];
    Outline {
      bounds: self.bounds,
      vertices: shape
        .vertices
        .iter()
        .map(|&(x, y)| (x + self.bounds[0], y + self.bounds[1]))
        .collect(),
      closed: true,
      rectangle: shape.lines == 1,
    }
  }

  fn callout(&self) -> Outline {
    Outline {
      bounds: point_bounds(&self.leader_points),
      vertices: self.leader_points.clone(),
      closed: false,
      rectangle: false,
    }
  }

  fn word_overflow(&self) -> bool {
    self.shapes[self.shape].overflowed_word
      || (self.initial_word_overflow && self.shapes[self.shape].width == self.original_width)
  }

  pub fn visible_leader(&self) -> Option<&[Point]> {
    self.leader_visible.then_some(&self.leader_points)
  }
}

#[derive(Clone)]
struct Outline {
  bounds: Bounds,
  vertices: Vec<Point>,
  closed: bool,
  rectangle: bool,
}

impl Outline {
  fn rectangle(bounds: Bounds) -> Self {
    Self {
      bounds,
      vertices: rectangle_vertices(bounds).to_vec(),
      closed: true,
      rectangle: true,
    }
  }
}

fn add(a: Point, b: Point) -> Point {
  (a.0 + b.0, a.1 + b.1)
}
fn sub(a: Point, b: Point) -> Point {
  (a.0 - b.0, a.1 - b.1)
}
fn scale(a: Point, t: f64) -> Point {
  (a.0 * t, a.1 * t)
}
fn dot(a: Point, b: Point) -> f64 {
  a.0 * b.0 + a.1 * b.1
}
fn cross(a: Point, b: Point) -> f64 {
  a.0 * b.1 - a.1 * b.0
}
fn squared(a: Point, b: Point) -> f64 {
  let v = sub(a, b);
  dot(v, v)
}
fn unit(a: Point) -> Point {
  let length = a.0.hypot(a.1);
  if length > EPS {
    scale(a, 1.0 / length)
  } else {
    (0.0, 0.0)
  }
}
fn point_bounds(points: &[Point]) -> Bounds {
  let mut bounds = [
    f64::INFINITY,
    f64::INFINITY,
    f64::NEG_INFINITY,
    f64::NEG_INFINITY,
  ];
  for &(x, y) in points {
    bounds[0] = bounds[0].min(x);
    bounds[1] = bounds[1].min(y);
    bounds[2] = bounds[2].max(x);
    bounds[3] = bounds[3].max(y);
  }
  bounds
}
fn rectangle_vertices(b: Bounds) -> [Point; 5] {
  [
    (b[0], b[1]),
    (b[0], b[3]),
    (b[2], b[3]),
    (b[2], b[1]),
    (b[0], b[1]),
  ]
}
fn rectangle_center(b: Bounds) -> Point {
  ((b[0] + b[2]) * 0.5, (b[1] + b[3]) * 0.5)
}
fn rectangle_distance(a: Bounds, b: Bounds) -> f64 {
  let dx = (a[0] - b[2]).max(b[0] - a[2]).max(0.0);
  let dy = (a[1] - b[3]).max(b[1] - a[3]).max(0.0);
  dx * dx + dy * dy
}
fn rectangle_ratio(a: Bounds, b: Bounds) -> f64 {
  let area = ((a[2] - a[0]) * (a[3] - a[1])).min((b[2] - b[0]) * (b[3] - b[1]));
  if area > EPS {
    (a[2].min(b[2]) - a[0].max(b[0])).max(0.0) * (a[3].min(b[3]) - a[1].max(b[1])).max(0.0) / area
  } else {
    0.0
  }
}
fn rectangle_nearest(b: Bounds, p: Point) -> (f64, Point) {
  let q = (p.0.clamp(b[0], b[2]), p.1.clamp(b[1], b[3]));
  (squared(p, q), q)
}
fn rectangle_contains(b: Bounds, p: Point) -> bool {
  b[0] <= p.0 && p.0 <= b[2] && b[1] <= p.1 && p.1 <= b[3]
}
fn segment_nearest(p: Point, a: Point, b: Point) -> (f64, Point, f64) {
  let v = sub(b, a);
  let length = dot(v, v);
  let t = if length >= EPS {
    (dot(sub(p, a), v) / length).clamp(0.0, 1.0)
  } else {
    0.0
  };
  let q = add(a, scale(v, t));
  (squared(p, q), q, t)
}
fn segments_nearest(a: Point, b: Point, c: Point, d: Point) -> (f64, Point, Point, f64, f64) {
  let v = sub(b, a);
  let w = sub(d, c);
  let den = cross(v, w);
  if den.abs() >= EPS {
    let q = sub(c, a);
    let t = cross(q, w) / den;
    let u = cross(q, v) / den;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
      return (0.0, add(a, scale(v, t)), add(c, scale(w, u)), t, u);
    }
  }
  let (z, q, t) = segment_nearest(a, c, d);
  let mut best = (z, a, q, 0.0, t);
  {
    let (z, q, t) = segment_nearest(b, c, d);
    if z < best.0 {
      best = (z, b, q, 1.0, t);
    }
  }
  for (p, endpoint) in [(c, 0.0), (d, 1.0)] {
    let (z, q, t) = segment_nearest(p, a, b);
    if z < best.0 {
      best = (z, q, p, t, endpoint);
    }
  }
  best
}
fn segment_clip(a: Point, b: Point, r: Bounds) -> Option<[Point; 2]> {
  let v = sub(b, a);
  let mut lower: f64 = 0.0;
  let mut upper: f64 = 1.0;
  for (p, q, lo, hi) in [(a.0, v.0, r[0], r[2]), (a.1, v.1, r[1], r[3])] {
    if q.abs() < EPS {
      if p < lo || p > hi {
        return None;
      }
    } else {
      let t = (lo - p) / q;
      let u = (hi - p) / q;
      lower = lower.max(t.min(u));
      upper = upper.min(t.max(u));
    }
  }
  (lower <= upper).then(|| [add(a, scale(v, lower)), add(a, scale(v, upper))])
}
fn boundary_inside_length(poly: &Outline, r: Bounds) -> f64 {
  poly
    .vertices
    .windows(2)
    .filter_map(|edge| segment_clip(edge[0], edge[1], r))
    .map(|[a, b]| squared(a, b).sqrt())
    .sum()
}
fn polygon_contains(poly: &Outline, p: Point) -> bool {
  let mut inside = false;
  for edge in poly.vertices.windows(2) {
    let [a, b] = [edge[0], edge[1]];
    if segment_nearest(p, a, b).0 < EPS {
      return true;
    }
    if (a.1 > p.1) != (b.1 > p.1) && p.0 < (b.0 - a.0) * (p.1 - a.1) / (b.1 - a.1) + a.0 {
      inside = !inside;
    }
  }
  inside
}

struct SegmentDistance {
  squared: f64,
  segment: Point,
  sector: Point,
  intersections: Vec<f64>,
}

fn sector_segment(s: &Sector, a: Point, b: Point, biased: bool) -> SegmentDistance {
  let (z, p, _) = segment_nearest(s.center, a, b);
  let radius_squared = s.radius * s.radius;
  if squared(s.center, a) > radius_squared
    && squared(s.center, b) > radius_squared
    && z > radius_squared
  {
    if biased {
      let (z, p, _) = segment_nearest(s.arc, a, b);
      return SegmentDistance {
        squared: z,
        segment: p,
        sector: s.arc,
        intersections: Vec::new(),
      };
    }
    if s.contains_angle(p) {
      // The native outside-circle fast path retains the center distance;
      // other sector paths measure the actual arc clearance.
      return SegmentDistance {
        squared: z,
        segment: p,
        sector: add(s.center, scale(sub(p, s.center), s.radius / z.sqrt())),
        intersections: Vec::new(),
      };
    }
  }
  let mut best = SegmentDistance {
    squared: f64::INFINITY,
    segment: a,
    sector: s.center,
    intersections: Vec::new(),
  };
  for endpoint in s.endpoints {
    let (z, p, q, t, _) = segments_nearest(a, b, s.center, endpoint);
    if z < EPS {
      best.intersections.push(t);
    }
    if z < best.squared {
      best.squared = z;
      best.segment = p;
      best.sector = q;
    }
  }
  let v = sub(b, a);
  let q = sub(a, s.center);
  let aa = dot(v, v);
  if aa >= EPS {
    let bb = 2.0 * dot(v, q);
    let cc = dot(q, q) - radius_squared;
    let discriminant = bb * bb - 4.0 * aa * cc;
    if discriminant >= 0.0 {
      for t in [
        (-bb - discriminant.sqrt()) / (2.0 * aa),
        (-bb + discriminant.sqrt()) / (2.0 * aa),
      ] {
        if (0.0..=1.0).contains(&t) && s.contains_angle(add(a, scale(v, t))) {
          best.intersections.push(t);
        }
      }
    }
  }
  if !best.intersections.is_empty() {
    best.squared = 0.0;
    best.intersections.sort_by(f64::total_cmp);
    return best;
  }
  let radial = z.sqrt();
  if radial <= s.radius && s.contains_angle(p) {
    let clearance = (radial - s.radius).powi(2);
    if clearance < best.squared {
      best.squared = clearance;
      best.segment = p;
      best.sector = add(
        s.center,
        scale(
          sub(p, s.center),
          if radial > EPS { s.radius / radial } else { 0.0 },
        ),
      );
    }
  }
  best
}

#[derive(Clone, Copy)]
struct Distance {
  squared: f64,
  first: Point,
  second: Point,
  ratio: f64,
}

fn sector_rectangle(s: &Sector, b: Bounds, biased: bool) -> Distance {
  let (z, p) = rectangle_nearest(b, s.center);
  let radius_squared = s.radius * s.radius;
  let mut result = if z > radius_squared && biased {
    let (z, p) = rectangle_nearest(b, s.arc);
    return Distance {
      squared: z,
      first: s.arc,
      second: p,
      ratio: 0.0,
    };
  } else if s.contains_angle(p) {
    if z > radius_squared {
      let radial = z.sqrt();
      Distance {
        // LLPieContent's rectangle query retains the signed difference
        // between squared radii, even though its nearest point lies on
        // the arc. The biased label query above uses arc-to-box distance.
        squared: z - radius_squared,
        first: add(s.center, scale(sub(p, s.center), s.radius / radial)),
        second: p,
        ratio: 0.0,
      }
    } else {
      Distance {
        squared: z - radius_squared,
        first: s.center,
        second: p,
        ratio: 0.0,
      }
    }
  } else {
    let mut best = Distance {
      squared: f64::INFINITY,
      first: s.center,
      second: p,
      ratio: 0.0,
    };
    for edge in rectangle_vertices(b).windows(2) {
      let q = sector_segment(s, edge[0], edge[1], false);
      if q.squared < best.squared {
        best.squared = q.squared;
        best.first = q.sector;
        best.second = q.segment;
      }
    }
    best
  };
  let area = (s.bounds[2] - s.bounds[0]) * (s.bounds[3] - s.bounds[1]);
  if area > EPS {
    result.ratio =
      rectangle_ratio(s.bounds, b) * radius_squared * NATIVE_PI * (s.end - s.start) / 360.0 / area;
  }
  if z <= radius_squared && s.contains_angle(p) {
    for endpoint in s.endpoints {
      if let Some([a, b]) = segment_clip(s.center, endpoint, b) {
        result.ratio += 1.0 - squared(a, s.center).min(squared(b, s.center)) / radius_squared;
      }
    }
  }
  result
}

fn sector_outline(s: &Sector, n: &Outline, biased: bool, signed: bool) -> Distance {
  if let Some(projected) = &s.projected {
    return projected_sector_outline(s.bounds, projected, n, biased);
  }
  if n.rectangle {
    return sector_rectangle(s, n.bounds, biased);
  }
  let mut best = Distance {
    squared: f64::INFINITY,
    first: s.center,
    second: s.center,
    ratio: 0.0,
  };
  let mut first_hit = None;
  let mut inside_length = 0.0;
  let mut perimeter = 0.0;
  let radius_squared = s.radius * s.radius;
  let mut radial = [radius_squared; 2];
  for edge in n.vertices.windows(2) {
    let [a, b] = [edge[0], edge[1]];
    let q = sector_segment(s, a, b, biased);
    if first_hit.is_none() && q.squared < best.squared {
      best.squared = q.squared;
      best.first = q.sector;
      best.second = q.segment;
    }
    if first_hit.is_none()
      && let Some(&t) = q.intersections.first()
    {
      first_hit = Some(add(a, scale(sub(b, a), t)));
    }
    for (index, endpoint) in s.endpoints.iter().enumerate() {
      let (z, p, _, _, _) = segments_nearest(a, b, s.center, *endpoint);
      if z < EPS && squared(*endpoint, p) >= EPS {
        radial[index] = radial[index].min(squared(p, s.center));
      }
    }
    let length = squared(a, b).sqrt();
    perimeter += length;
    let mut cuts = q.intersections;
    cuts.extend([0.0, 1.0]);
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    for interval in cuts.windows(2) {
      if s.contains(add(a, scale(sub(b, a), (interval[0] + interval[1]) * 0.5))) {
        inside_length += length * (interval[1] - interval[0]);
      }
    }
  }
  if let Some(p) = first_hit {
    best.squared = 0.0;
    best.first = p;
    best.second = p;
  } else if signed && n.vertices.last().is_some_and(|&p| s.contains(p)) {
    best.squared = -best.squared;
  }
  best.ratio = (if perimeter > EPS {
    inside_length / perimeter
  } else {
    0.0
  })
  .max(inside_length / s.radius)
    + radial
      .into_iter()
      .map(|r| 1.0 - r / radius_squared)
      .sum::<f64>();
  best
}

fn projected_sector_outline(
  bounds: Bounds,
  sector: &ProjectedSector,
  label: &Outline,
  biased: bool,
) -> Distance {
  // The 3-D owner first keeps a positive bounding-box distance. Only
  // overlapping boxes require its projected top/bottom outline query.
  let mut distance = Distance {
    squared: rectangle_distance(bounds, label.bounds),
    first: sector.arc,
    second: sector.arc,
    ratio: 0.0,
  };
  if distance.squared <= 0.0 {
    let outline = Outline {
      bounds,
      vertices: sector.outline.clone(),
      closed: true,
      rectangle: false,
    };
    if label.rectangle {
      // The rectangular path queries the sector's edges first.
      let closest = outlines_nearest(&outline, label);
      distance.first = closest.first;
      distance.second = closest.second;
    } else {
      // The native polygon path visits the text edges first; the two
      // intersection outputs retain first and second crossings separately.
      let closest = outlines_nearest(label, &outline);
      distance.first = closest.second;
      distance.second = closest.first;
    }
    (distance.squared, distance.ratio) = outlines_distance(&outline, label);
  }
  if biased && distance.squared > 0.0 {
    distance.first = sector.arc;
    if label.rectangle {
      (distance.squared, distance.second) = rectangle_nearest(label.bounds, sector.arc);
    } else {
      distance.squared = f64::INFINITY;
      for edge in label.vertices.windows(2) {
        let (squared, point, _) = segment_nearest(sector.arc, edge[0], edge[1]);
        if squared < distance.squared {
          distance.squared = squared;
          distance.second = point;
        }
      }
    }
  }
  distance
}

fn outlines_nearest(first: &Outline, second: &Outline) -> Distance {
  let mut result = Distance {
    squared: f64::INFINITY,
    first: (0.0, 0.0),
    second: (0.0, 0.0),
    ratio: 0.0,
  };
  let mut crossing = false;
  for a in first.vertices.windows(2) {
    for b in second.vertices.windows(2) {
      let (squared, first, second, _, _) = segments_nearest(a[0], a[1], b[0], b[1]);
      if squared < EPS {
        if crossing {
          result.second = first;
          return result;
        }
        crossing = true;
        result.squared = 0.0;
        result.first = first;
        result.second = first;
      } else if !crossing && squared < result.squared {
        result.squared = squared;
        result.first = first;
        result.second = second;
      }
    }
  }
  result
}

fn outlines_distance(a: &Outline, b: &Outline) -> (f64, f64) {
  if a.rectangle && b.rectangle {
    return (
      rectangle_distance(a.bounds, b.bounds),
      rectangle_ratio(a.bounds, b.bounds),
    );
  }
  let mut best = f64::INFINITY;
  for first in a.vertices.windows(2) {
    for second in b.vertices.windows(2) {
      best = best.min(segments_nearest(first[0], first[1], second[0], second[1]).0);
    }
  }
  if a.rectangle || b.rectangle {
    let (rect, poly) = if a.rectangle { (a, b) } else { (b, a) };
    if best > EPS
      && (poly
        .vertices
        .first()
        .is_some_and(|&p| rectangle_contains(rect.bounds, p))
        || (poly.closed && polygon_contains(poly, (rect.bounds[0], rect.bounds[1]))))
    {
      return (-best, 1.0);
    }
    let diagonal = (rect.bounds[2] - rect.bounds[0]).hypot(rect.bounds[3] - rect.bounds[1]);
    return (
      best,
      if diagonal > EPS {
        boundary_inside_length(poly, rect.bounds) / diagonal
      } else {
        0.0
      },
    );
  }
  let ratio = if a.closed && b.closed {
    rectangle_ratio(a.bounds, b.bounds)
  } else if a.closed || b.closed {
    let (poly, line) = if a.closed { (a, b) } else { (b, a) };
    let diagonal = (poly.bounds[2] - poly.bounds[0]).hypot(poly.bounds[3] - poly.bounds[1]);
    if diagonal > EPS {
      boundary_inside_length(line, poly.bounds) / diagonal
    } else {
      0.0
    }
  } else if best < EPS {
    1.0
  } else {
    0.0
  };
  (best, ratio)
}

fn label_attachment(label: &Label, outline: &Outline, nearest: Point) -> (Point, Point) {
  let mut closest = f64::INFINITY;
  let mut chosen = None;
  let mut projection = None;
  for (index, (&p, &flags)) in outline
    .vertices
    .iter()
    .zip(&label.shapes[label.shape].attachments)
    .enumerate()
  {
    if flags & 1 != 0 {
      let z = squared(nearest, p);
      if z < closest {
        chosen = Some(index);
        if z <= 25.0 {
          break;
        }
        closest = z;
      }
    }
    if flags & 2 != 0
      && let Some(&next) = outline.vertices.get(index + 1)
    {
      let (z, q, t) = segment_nearest(nearest, p, next);
      if z < 1.0 && t > EPS && t < 1.0 - EPS {
        chosen = Some(index);
        projection = Some(q);
        break;
      }
    }
  }
  if let Some(index) = chosen
    && let Some(&next) = outline.vertices.get(index + 1)
  {
    let p = outline.vertices[index];
    let edge = sub(next, p);
    let normal = unit((edge.1, -edge.0));
    (
      projection.unwrap_or_else(|| add(p, scale(normal, 2.0))),
      normal,
    )
  } else {
    (nearest, (0.0, 0.0))
  }
}

fn update_leader(label: &mut Label, sector: &Sector) {
  if !label.leader {
    return;
  }
  let outline = label.outline();
  let nearest = sector_outline(sector, &outline, true, true);
  let (end, normal) = label_attachment(label, &outline, nearest.second);
  let elbow = add(end, scale(normal, 6.0));
  let mut points = vec![nearest.first, end];
  if squared(nearest.first, end) >= 36.0 && dot(sub(nearest.first, elbow), sub(end, elbow)) <= 0.0 {
    points.insert(1, elbow);
  }
  label.leader_visible = nearest.squared > if label.force_leader { 0.0 } else { 64.0 };
  label.leader_points = points;
  label.leader_version += 1;
}

fn label_local_cost(label: &Label, sector: &Sector, maximum_distance: f64) -> f64 {
  let nearest = sector_outline(sector, &label.outline(), true, false);
  let distance = nearest.squared.max(0.0).sqrt();
  let mut cost = if nearest.squared < 9.0 {
    0.2 * (3.0 - distance) / 3.0
  } else {
    0.2 * (distance - 3.0).powi(2) / maximum_distance.powi(2)
  };
  if !label.leader_visible {
    let angle = (sector.start + sector.end) * 0.5;
    let near = if (15.0..=75.0).contains(&(angle % 90.0).abs()) {
      nearest.second
    } else {
      rectangle_center(label.bounds)
    };
    let radial = unit(sub(sector.arc, sector.center));
    let limit = 1.0 - dot(unit(sub(sector.endpoints[0], sector.center)), radial);
    if limit > EPS {
      let cosine = dot(unit(sub(near, sector.center)), radial);
      cost += (0.1 * (1.0 - cosine).powi(2) / limit.powi(2)).min(10.0);
    }
  }
  let shape = &label.shapes[label.shape];
  if shape.lines > 1 {
    let width = label.bounds[2] - label.bounds[0];
    let height = label.bounds[3] - label.bounds[1];
    let aspect = if width > EPS { height / width } else { 1.0 };
    cost += (shape.lines as f64 * 0.5
      + if label.word_overflow() { 500.0 } else { 0.0 }
      + (aspect.powi(2) - 0.4).clamp(0.0, 10.0))
      / 5.0
      * 0.5;
  }
  cost
}

fn leader_local_cost(label: &Label, sector: &Sector, maximum_distance: f64) -> f64 {
  if !label.leader_visible {
    return 0.0;
  }
  let points = &label.leader_points;
  let length: f64 = points
    .windows(2)
    .map(|edge| squared(edge[0], edge[1]).sqrt())
    .sum();
  let delta = unit(sub(points[1], points[0]));
  let radial = unit(sub(sector.arc, sector.center));
  let angle = 1.0 - dot(delta, radial);
  let angle = if (0.0..=1.0).contains(&angle) {
    angle
  } else {
    1.0
  };
  let mut cost = 0.25 * (0.2 + ((sector.end - sector.start) / 60.0).min(0.8)) * angle
    + 0.75 * ((length - 20.0) / (maximum_distance - 20.0).max(20.0)).powi(2);
  let end = *points.last().unwrap();
  let dx = (points[0].0 - end.0).abs();
  let width = label.bounds[2] - label.bounds[0];
  if dx > EPS && dx < 6.0 {
    // Generated Word labels have text lines; the vertical-callout penalty
    // is ten times the ordinary connector penalty for such a dependency.
    cost += 0.5;
  } else if dx <= EPS && width > EPS {
    let offset = (rectangle_center(label.bounds).0 - end.0).abs();
    if offset <= width {
      cost += 0.1 * offset / (width * 0.5);
    }
  }
  cost
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Object {
  Sector(usize),
  Label(usize),
  Leader(usize),
  Obstacle(usize),
}

#[derive(Clone, Copy)]
struct CachedPair {
  versions: (u64, u64),
  value: (f64, f64),
}

struct Layout<'a> {
  sectors: &'a [Sector],
  labels: &'a mut [Label],
  obstacles: Vec<Outline>,
  objects: Vec<Object>,
  cache: Vec<Option<CachedPair>>,
  maximum_distance: f64,
  seed: u32,
  chart: Bounds,
}

impl Layout<'_> {
  fn visible(&self, object: Object) -> bool {
    match object {
      Object::Leader(i) => self.labels[i].leader_visible,
      _ => true,
    }
  }
  fn fixed(object: Object) -> bool {
    matches!(object, Object::Sector(_) | Object::Obstacle(_))
  }
  fn owner(object: Object) -> Option<usize> {
    match object {
      Object::Label(i) | Object::Leader(i) => Some(i),
      _ => None,
    }
  }
  fn version(&self, object: Object) -> u64 {
    match object {
      Object::Label(i) => self.labels[i].version,
      Object::Leader(i) => self.labels[i].leader_version,
      _ => 0,
    }
  }
  fn bounds(&self, object: Object) -> Bounds {
    match object {
      Object::Sector(i) => self.sectors[i].bounds,
      Object::Label(i) => self.labels[i].bounds,
      Object::Leader(i) => point_bounds(&self.labels[i].leader_points),
      Object::Obstacle(i) => self.obstacles[i].bounds,
    }
  }
  fn outline(&self, object: Object) -> Outline {
    match object {
      Object::Label(i) => self.labels[i].outline(),
      Object::Leader(i) => self.labels[i].callout(),
      Object::Obstacle(i) => self.obstacles[i].clone(),
      Object::Sector(_) => unreachable!(),
    }
  }
  fn distance(&mut self, a: usize, b: usize, gap: f64) -> (f64, f64) {
    let first = self.objects[a];
    let second = self.objects[b];
    let versions = (self.version(first), self.version(second));
    let index = a * self.objects.len() + b;
    if let Some(cached) = self.cache[index]
      && cached.versions == versions
    {
      return cached.value;
    }
    let bounds_distance = rectangle_distance(self.bounds(first), self.bounds(second));
    let value = if bounds_distance > gap * gap + EPS {
      (bounds_distance, 0.0)
    } else if let Object::Sector(i) = first {
      let d = sector_outline(&self.sectors[i], &self.outline(second), false, false);
      (d.squared, d.ratio)
    } else if let Object::Sector(i) = second {
      let d = sector_outline(&self.sectors[i], &self.outline(first), false, false);
      (d.squared, d.ratio)
    } else {
      outlines_distance(&self.outline(first), &self.outline(second))
    };
    self.cache[index] = Some(CachedPair { versions, value });
    value
  }
  fn excluded(&self, a: Object, b: Object) -> bool {
    if a == b || !self.visible(a) || !self.visible(b) {
      return true;
    }
    if let Some(i) = Self::owner(a) {
      if matches!(a, Object::Leader(_))
        && matches!(b, Object::Sector(s) if self.labels[i].sector == s)
      {
        return true;
      }
      if Self::owner(b) == Some(i) {
        return true;
      }
    }
    if let Some(i) = Self::owner(b)
      && matches!(b, Object::Leader(_))
      && matches!(a, Object::Sector(s) if self.labels[i].sector == s)
    {
      return true;
    }
    Self::fixed(a) && Self::fixed(b)
  }
  fn ordering(&self, a: usize, b: usize) -> bool {
    let first = &self.labels[a];
    let second = &self.labels[b];
    let p = self.sectors[first.sector].arc;
    let r = self.sectors[second.sector].arc;
    squared(p, r) >= EPS
      && segments_nearest(
        p,
        rectangle_center(first.bounds),
        r,
        rectangle_center(second.bounds),
      )
      .0 < EPS
  }
  fn pair(&mut self, a: usize, b: usize, gap: f64) -> f64 {
    let first = self.objects[a];
    let second = self.objects[b];
    if self.excluded(first, second) {
      return 0.0;
    }
    let (distance, ratio) = self.distance(a, b, gap);
    let leader = matches!(first, Object::Leader(_)) || matches!(second, Object::Leader(_));
    let mut cost = if distance < EPS {
      let mut cost = 0.35 + 0.65 * ratio;
      if leader {
        cost *= if Self::fixed(first) || Self::fixed(second) {
          0.84
        } else if matches!(first, Object::Leader(_)) && matches!(second, Object::Leader(_)) {
          0.19
        } else {
          0.43
        };
      }
      cost
    } else if distance < 12.25 {
      0.05 * (3.5 - distance.sqrt()) / 3.5
    } else {
      0.0
    };
    if let (Object::Label(i), Object::Label(j)) = (first, second)
      && self.ordering(i, j)
    {
      cost += 0.42;
    }
    cost
  }
  fn local(&self, i: usize) -> f64 {
    let label = &self.labels[i];
    10.0
      * (label_local_cost(label, &self.sectors[label.sector], self.maximum_distance)
        + leader_local_cost(label, &self.sectors[label.sector], self.maximum_distance))
  }
  fn cost(&mut self, selected: Option<usize>) -> f64 {
    let mut cost = 0.0;
    for i in 0..self.labels.len() {
      if selected.is_none_or(|chosen| chosen == i) {
        cost += self.local(i);
      }
    }
    for a in 0..self.objects.len() {
      for b in a + 1..self.objects.len() {
        let first = self.objects[a];
        let second = self.objects[b];
        if selected.is_some_and(|i| Self::owner(first) != Some(i) && Self::owner(second) != Some(i))
        {
          continue;
        }
        // Sectors and their labels share the diagram group. A title is a
        // separate chart group; this weight belongs to group ownership,
        // independently of the object's fixed/movable constraint flag.
        let group_weight =
          if matches!(first, Object::Obstacle(_)) || matches!(second, Object::Obstacle(_)) {
            2.5
          } else {
            1.0
          };
        cost += self.pair(a, b, 3.5) * 100.0 * group_weight;
      }
    }
    cost
  }
  fn random(&mut self) -> f64 {
    self.seed = self.seed.wrapping_mul(0x343fd).wrapping_add(0x269ec3);
    f64::from((self.seed >> 16) & 0x7fff) / 32768.0
  }
  fn set_position(&mut self, index: usize, left: f64, bottom: f64, shape: usize) -> bool {
    let label = &mut self.labels[index];
    let text = &label.shapes[shape];
    let width = text.bounds[2] - text.bounds[0];
    let height = text.bounds[3] - text.bounds[1];
    let x = left.clamp(self.chart[0], (self.chart[2] - width).max(self.chart[0]));
    let y = bottom.clamp(self.chart[1], (self.chart[3] - height).max(self.chart[1]));
    label.bounds = [x, y, x + width, y + height];
    label.shape = shape;
    label.version += 1;
    update_leader(label, &self.sectors[label.sector]);
    x != left || y != bottom
  }
  fn radial_position(&mut self, index: usize, distance: f64, angle: f64, width: i32) -> bool {
    let label = &self.labels[index];
    let shape = label
      .shapes
      .iter()
      .position(|shape| shape.width == width)
      .unwrap_or(label.shape);
    let bounds = label.shapes[shape].bounds;
    let sector = &self.sectors[label.sector];
    let original_angle = (((sector.start + sector.end) * 0.5).trunc() as i32).rem_euclid(360);
    let a = angle * NATIVE_PI / 180.0;
    let x = sector.arc.0 + distance * a.cos()
      - if (120..=239).contains(&original_angle) {
        bounds[2]
      } else if (60..=299).contains(&original_angle) {
        bounds[2] * 0.5
      } else {
        0.0
      };
    let y = sector.arc.1 + distance * a.sin()
      - if (210..=329).contains(&original_angle) {
        bounds[3]
      } else if (30..=149).contains(&original_angle) {
        0.0
      } else {
        bounds[3] * 0.5
      };
    self.set_position(index, x, y, shape)
  }
  fn mutate(&mut self, index: usize) {
    for _ in 0..=5 {
      let angle_unit = self.random();
      let label = &self.labels[index];
      let (min, max, locked) = (label.minimum_width, label.maximum_width, label.locked_width);
      let width = if locked {
        max
      } else {
        min + (max - min) * (self.random() * 5.0) as i32 / 4
      };
      let sector = &self.sectors[self.labels[index].sector];
      let sweep = sector.end - sector.start;
      let original_angle = (sector.start + sector.end) * 0.5;
      let mut span = 150.0;
      if sweep < 10.0 {
        span += (175.0 - span) / ((sweep - 2.5).max(0.0) + 1.0);
      }
      let distance = self.random() * self.maximum_distance * self.random();
      let angle = original_angle - span * 0.5 + angle_unit * span;
      if !self.radial_position(index, distance, angle, width) {
        break;
      }
    }
  }
  fn anchor(&self, index: usize) -> Point {
    let label = &self.labels[index];
    if label.leader_visible
      && let Some(&end) = label.leader_points.last()
    {
      return end;
    }
    let outline = label.outline();
    let nearest = sector_outline(&self.sectors[label.sector], &outline, true, false);
    label_attachment(label, &outline, nearest.second).0
  }
  fn translate(&mut self, index: usize, shift: Point) {
    let label = &mut self.labels[index];
    let mut bounds = [
      label.bounds[0] + shift.0,
      label.bounds[1] + shift.1,
      label.bounds[2] + shift.0,
      label.bounds[3] + shift.1,
    ];
    for axis in 0..2 {
      let correction = if bounds[axis] < self.chart[axis] {
        self.chart[axis] - bounds[axis]
      } else if bounds[axis + 2] > self.chart[axis + 2] {
        self.chart[axis + 2] - bounds[axis + 2]
      } else {
        0.0
      };
      bounds[axis] += correction;
      bounds[axis + 2] += correction;
    }
    label.bounds = bounds;
    label.version += 1;
    update_leader(label, &self.sectors[label.sector]);
  }
  fn swap_positions(&mut self, a: usize, b: usize) {
    let first = self.labels[a].bounds;
    let second = self.labels[b].bounds;
    let center_a = rectangle_center(first);
    let center_b = rectangle_center(second);
    let anchor_a = self.anchor(a);
    let anchor_b = self.anchor(b);
    let offset_a = anchor_a.0 - center_a.0;
    let offset_b = anchor_b.0 - center_b.0;
    let sign = |value: f64| i32::from(value > 0.0) - i32::from(value < 0.0);
    let (dx_a, dx_b) = if offset_a.abs() < (first[2] - first[0]) * 0.25
      && offset_b.abs() < (second[2] - second[0]) * 0.25
    {
      let shift = center_b.0 - center_a.0;
      (shift, -shift)
    } else if sign(offset_a) != sign(offset_b) {
      (
        anchor_b.0 - (anchor_a.0 - offset_a * 2.0),
        anchor_a.0 - (anchor_b.0 - offset_b * 2.0),
      )
    } else {
      let shift = anchor_b.0 - anchor_a.0;
      (shift, -shift)
    };
    let dy = anchor_b.1 - anchor_a.1;
    self.translate(a, (dx_a, dy));
    self.translate(b, (dx_b, -dy));
  }
  fn swap_neighbors(&mut self, moving: &[usize], final_pass: bool) -> bool {
    if moving.len() < 2 {
      return false;
    }
    let mut improved = false;
    for position in 0..moving.len() {
      let a = moving[position];
      let b = moving[(position + 1) % moving.len()];
      let crossed = self.ordering(a, b);
      if crossed && !final_pass {
        let previous = (self.labels[a].clone(), self.labels[b].clone());
        let previous_cache = self.cache.clone();
        self.swap_positions(a, b);
        if !self.ordering(a, b) {
          // The native pass commits this ordering repair without reporting
          // a cost improvement to the annealing loop.
          continue;
        }
        self.labels[a] = previous.0;
        self.labels[b] = previous.1;
        self.cache = previous_cache;
      }
      let collision = [
        (Object::Leader(a), Object::Leader(b)),
        (Object::Leader(a), Object::Label(b)),
        (Object::Leader(b), Object::Label(a)),
      ]
      .into_iter()
      .any(|(first, second)| {
        let exists = |object| match object {
          Object::Leader(i) => self.labels[i].leader,
          _ => true,
        };
        // This geometric crossing query includes hidden callouts. Their
        // visibility is considered by the subsequent complete cost query.
        exists(first)
          && exists(second)
          && outlines_distance(&self.outline(first), &self.outline(second)).1 > 0.0
      });
      if !collision {
        continue;
      }
      let old_cost = self.cost(Some(a)) + self.cost(Some(b));
      let previous = (self.labels[a].clone(), self.labels[b].clone());
      let previous_cache = self.cache.clone();
      self.swap_positions(a, b);
      let new_cost = self.cost(Some(a)) + self.cost(Some(b));
      let crossed_after = self.ordering(a, b);
      if old_cost * if crossed { 1.5 } else { 1.0 }
        > new_cost * if crossed_after { 1.75 } else { 1.0 }
      {
        improved = true;
      } else {
        self.labels[a] = previous.0;
        self.labels[b] = previous.1;
        self.cache = previous_cache;
      }
    }
    improved
  }
  fn initialize(&mut self, moving: &[usize]) {
    // The cluster pass asks for zero-gap distances. Geometry caches retain
    // those conservative box distances until either shape changes version.
    let mut forced = Vec::with_capacity(moving.len());
    let mut area = 0.0;
    for &i in moving {
      let label = &self.labels[i];
      area += (label.bounds[2] - label.bounds[0]) * (label.bounds[3] - label.bounds[1]);
      let mut pair_cost = 0.0;
      for a in 0..self.objects.len() {
        if self.objects[a] != Object::Label(i) {
          continue;
        }
        for b in 0..self.objects.len() {
          let weight = if matches!(self.objects[b], Object::Obstacle(_)) {
            2.5
          } else {
            1.0
          };
          pair_cost += self.pair(b, a, 0.0) * weight;
        }
      }
      forced.push(pair_cost * 100.0 > 175.0 && pair_cost * 100.0 <= 900.0);
    }
    let mut longest = 0;
    let mut run = 0;
    for &force in forced.iter().cycle().take(forced.len() * 2) {
      run = if force { run + 1 } else { 0 };
      longest = longest.max(run.min(moving.len()));
    }
    for (&i, &force) in moving.iter().zip(&forced) {
      self.labels[i].force_leader = force;
    }
    let count = moving.len() as f64;
    let mean_area = area / count;
    if count > 15.0 || mean_area > 1500.0 || longest > 2 {
      self.maximum_distance = (200.0
        * (1.0
          + ((count - 15.0) / 100.0).max(0.0)
          + ((mean_area - 1500.0) / 2000.0).max(0.0)
          + ((longest as f64 - 2.0) / 10.0).max(0.0)))
      .trunc()
      .min(500.0);
    }
  }
}

pub(super) fn place(sectors: &[Sector], labels: &mut [Label], obstacles: &[Bounds], chart: Bounds) {
  let moving: Vec<_> = (0..labels.len())
    .rev()
    .filter(|&i| labels[i].movable)
    .collect();
  if moving.is_empty() {
    return;
  }
  for label in labels.iter_mut() {
    update_leader(label, &sectors[label.sector]);
  }
  let mut objects: Vec<_> = (0..sectors.len()).rev().map(Object::Sector).collect();
  for i in (0..labels.len()).rev() {
    objects.push(Object::Label(i));
    if labels[i].leader {
      objects.push(Object::Leader(i));
    }
  }
  objects.extend((0..obstacles.len()).map(Object::Obstacle));
  let count = objects.len();
  let mut layout = Layout {
    sectors,
    labels,
    obstacles: obstacles.iter().copied().map(Outline::rectangle).collect(),
    objects,
    cache: vec![None; count * count],
    maximum_distance: 200.0,
    seed: 0,
    chart,
  };
  layout.initialize(&moving);
  let mut total = layout.cost(None);
  let mut best = f64::INFINITY;
  let maximum_iterations = (moving.len().min(50) * 1000).min(20_000);
  let best_window = (maximum_iterations as f64 / (10 + moving.len() / 4) as f64).max(500.0);
  let worse_window = best_window / 10.0;
  let cooling = 1.0 - 1.0 / (moving.len() as f64 * 25.0);
  let mut temperature = 2.0;
  let mut last_best = 0;
  let mut last_worse = 0;
  for iteration in 0..maximum_iterations {
    if iteration as f64 - last_best as f64 >= best_window
      && iteration as f64 - last_worse as f64 >= worse_window
    {
      break;
    }
    let index = moving[iteration % moving.len()];
    let previous = layout.labels[index].clone();
    let old_cost = layout.cost(Some(index));
    let previous_cache = layout.cache.clone();
    layout.mutate(index);
    let chance = layout.random();
    let new_cost = layout.cost(Some(index));
    let proposed_total = total + new_cost - old_cost;
    if proposed_total < best {
      best = proposed_total;
      last_best = iteration;
    }
    let delta = new_cost - old_cost;
    if delta > 0.0 && chance < 1.0 - (-delta / temperature).exp() {
      layout.labels[index] = previous;
      layout.cache = previous_cache;
    } else {
      if delta > 0.0 {
        last_worse = iteration;
      }
      total = proposed_total;
      if last_worse == iteration && layout.swap_neighbors(&moving, false) {
        total = layout.cost(None);
      }
    }
    temperature *= cooling;
  }
  layout.swap_neighbors(&moving, true);
  // Native final cleanup retries the preferred radial gap at the selected
  // text width, accepting only a strict reduction in the complete cost.
  for &index in &moving {
    let previous = layout.labels[index].clone();
    let old_cost = layout.cost(Some(index));
    let previous_cache = layout.cache.clone();
    let sector = &sectors[previous.sector];
    let width = previous.shapes[previous.shape].width;
    layout.radial_position(index, 3.0, (sector.start + sector.end) * 0.5, width);
    if layout.cost(Some(index)) >= old_cost {
      layout.labels[index] = previous;
      layout.cache = previous_cache;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn projected_pie_label_fitting_matches_native_narrow_and_wide_sectors() {
    // Observation-only LLPieSlice3DElement controls in chart cells. The
    // last case changes only the values to make a 295.2-degree wedge;
    // its nearest-edge branch must remain distinct from a narrow wedge.
    let radii = (154.113_070_866_141_73, 73.132_703_412_073_5);
    for (angles, center, endpoints, ellipse, direction, vertices, expected) in [
      (
        (334.784_270_890_223_9, 361.130_529_765_155_64),
        (303.351_706_036_745_4, 180.357_900_262_467_18),
        [
          (456.531_758_530_183_7, 147.258_162_729_658_8),
          (458.091_233_595_800_53, 181.785_196_850_393_7),
        ],
        (303.351_706_036_745_4, 168.832_335_958_005_25),
        (0.995_150_671_587_402_7, -0.098_362_293_787_516_4),
        [
          (424.706_036_745_406_9, 147.329_674_143_000_1),
          (424.706_036_745_406_9, 131.056_288_316_228_43),
          (467.072_965_879_265_15, 131.056_288_316_228_43),
          (467.072_965_879_265_15, 114.782_902_489_456_77),
          (483.366_299_212_598_5, 114.782_902_489_456_77),
          (483.366_299_212_598_5, 131.056_288_316_228_43),
          (525.733_333_333_333_3, 131.056_288_316_228_43),
          (525.733_333_333_333_3, 147.329_674_143_000_1),
          (424.706_036_745_406_9, 147.329_674_143_000_1),
        ],
        None,
      ),
      (
        (242.179_137_083_560_9, 334.784_270_890_223_9),
        (278.042_414_698_162_7, 166.541_522_309_711_3),
        [
          (193.209_973_753_280_85, 87.343_307_086_614_17),
          (434.227_191_601_049_87, 131.385_721_784_776_9),
        ],
        (278.042_414_698_162_7, 153.946_456_692_913_4),
        (0.585_055_267_907_392_2, -0.810_993_423_828_954_1),
        [
          (300.360_192_011_780_9, 41.260_237_090_630_97),
          (300.360_192_011_780_9, 24.986_851_263_859_318),
          (327.577_094_898_92, 24.986_851_263_859_318),
          (327.577_094_898_92, 8.713_465_437_087_663),
          (350.630_428_232_253_34, 8.713_465_437_087_663),
          (350.630_428_232_253_34, 24.986_851_263_859_318),
          (377.847_331_119_392_44, 24.986_851_263_859_318),
          (377.847_331_119_392_44, 41.260_237_090_630_97),
          (300.360_192_011_780_9, 41.260_237_090_630_97),
        ],
        Some((-16.141_509_557_618_694, 81.800_148_719_871_03)),
      ),
      (
        (90.0, 242.179_137_083_560_9),
        (228.310_866_141_732_28, 188.364_514_435_695_53),
        [
          (233.210_393_700_787_4, 249.197_270_341_207_34),
          (140.173_018_372_703_4, 116.710_026_246_719_17),
        ],
        (228.310_761_154_855_64, 177.429_501_312_335_96),
        (-0.993_480_663_403_029, 0.114_000_751_946_105_5),
        [
          (47.678_952_911_377_72, 221.594_652_506_505_3),
          (47.678_952_911_377_72, 205.321_266_679_733_64),
          (54.212_391_231_587_695, 205.321_266_679_733_64),
          (54.212_391_231_587_695, 189.047_880_852_961_98),
          (77.265_724_564_921_03, 189.047_880_852_961_98),
          (77.265_724_564_921_03, 205.321_266_679_733_64),
          (83.799_162_885_131, 205.321_266_679_733_64),
          (83.799_162_885_131, 221.594_652_506_505_3),
          (47.678_952_911_377_72, 221.594_652_506_505_3),
        ],
        Some((69.697_262_081_479_44, 3.466_395_377_882_54)),
      ),
      (
        (90.0, 385.2),
        (244.029_606_299_212_6, 168.534_803_149_606_3),
        [
          (246.955_800_524_934_38, 234.349_291_338_582_67),
          (380.371_023_622_047_23, 198.952_545_931_758_52),
        ],
        (244.029_606_299_212_6, 156.098_057_742_782_15),
        (-0.807_659_218_386_142_7, -0.589_649_545_879_486_7),
        [
          (107.562_651_003_833_11, 55.698_617_174_886_09),
          (107.562_651_003_833_11, 39.425_231_348_114_44),
          (114.096_089_324_043_08, 39.425_231_348_114_44),
          (114.096_089_324_043_08, 23.151_845_521_342_78),
          (137.149_422_657_376_42, 23.151_845_521_342_78),
          (137.149_422_657_376_42, 39.425_231_348_114_44),
          (143.682_860_977_586_4, 39.425_231_348_114_44),
          (143.682_860_977_586_4, 55.698_617_174_886_09),
          (107.562_651_003_833_11, 55.698_617_174_886_09),
        ],
        Some((48.552_945_722_653_476, 75.022_972_533_124_69)),
      ),
    ] {
      let sector = Sector::projected(
        angles.0,
        angles.1,
        ProjectedSector {
          center,
          endpoints,
          arc: center,
          outline: vec![center, endpoints[0], endpoints[1], center],
          ellipse_center: ellipse,
          ellipse_radii: radii,
          inner_direction: direction,
        },
      );
      let actual = sector.projected_inner_translation(&vertices, 5.0);
      assert_eq!(actual.is_some(), expected.is_some());
      if let (Some(actual), Some(expected)) = (actual, expected) {
        assert!(
          (actual.0 - expected.0).abs() < 1.0e-9,
          "{actual:?} vs {expected:?}"
        );
        assert!(
          (actual.1 - expected.1).abs() < 1.0e-9,
          "{actual:?} vs {expected:?}"
        );
      }
    }
  }

  #[test]
  fn projected_pie_distance_retains_box_clearance_and_callout_anchor() {
    let sector = Sector::projected(
      0.0,
      90.0,
      ProjectedSector {
        center: (0.0, 0.0),
        endpoints: [(8.0, 0.0), (0.0, 8.0)],
        arc: (5.0, 5.0),
        outline: vec![(0.0, 0.0), (8.0, 0.0), (0.0, 8.0), (0.0, 0.0)],
        ellipse_center: (0.0, 0.0),
        ellipse_radii: (8.0, 8.0),
        inner_direction: (1.0, 1.0),
      },
    );
    let label = Outline::rectangle([8.5, 0.0, 10.5, 2.0]);
    // Even a sub-gap separation uses the native 3-D bounding-box query;
    // its biased callout query independently measures from the arc anchor.
    assert_eq!(sector_outline(&sector, &label, false, false).squared, 0.25);
    let biased = sector_outline(&sector, &label, true, false);
    assert_eq!(biased.first, (5.0, 5.0));
    assert_eq!(biased.second, (8.5, 2.0));
    assert_eq!(biased.squared, 21.25);
  }

  #[test]
  fn word_pie_outline_distance_preserves_multiline_steps() {
    let a = Outline {
      bounds: [0.0, 0.0, 100.0, 40.0],
      vertices: vec![
        (0.0, 40.0),
        (0.0, 20.0),
        (45.0, 20.0),
        (45.0, 0.0),
        (55.0, 0.0),
        (55.0, 20.0),
        (100.0, 20.0),
        (100.0, 40.0),
        (0.0, 40.0),
      ],
      closed: true,
      rectangle: false,
    };
    let b = Outline::rectangle([10.0, 2.0, 20.0, 12.0]);
    assert_eq!(rectangle_distance(a.bounds, b.bounds), 0.0);
    assert_eq!(outlines_distance(&a, &b), (64.0, 0.0));
    let c = Outline::rectangle([48.0, 2.0, 52.0, 12.0]);
    assert_eq!(outlines_distance(&a, &c).1, 1.0);
  }

  #[test]
  fn word_pie_sector_rectangle_overlap_uses_its_wedge_area() {
    let sector = Sector::new((0.0, 0.0), 100.0, 0.0, 90.0);
    let result = sector_rectangle(&sector, [90.0, 90.0, 110.0, 110.0], false);
    assert!(result.squared > 0.0);
    assert!((result.ratio - NATIVE_PI / 16.0).abs() < 1.0e-6);
  }

  #[test]
  fn word_pie_rectangle_distance_keeps_native_squared_radius_difference() {
    let sector = Sector::new((0.0, 0.0), 100.0, 0.0, 90.0);
    let bounds = [60.0, 80.5, 80.0, 100.0];
    let clearance = sector_rectangle(&sector, bounds, false);
    // A small geometric gap must not become a near-collision penalty:
    // the rectangle query returns 60² + 80.5² - 100², not (r - 100)².
    assert_eq!(clearance.squared, 80.25);
    assert!((squared(clearance.first, sector.center) - 10_000.0).abs() < EPS);
    assert_ne!(
      clearance.squared,
      squared(clearance.first, clearance.second)
    );
    let biased = sector_rectangle(&sector, bounds, true);
    assert!((biased.squared - squared(biased.first, biased.second)).abs() < EPS);
  }

  #[test]
  fn word_pie_callout_visibility_uses_distance_even_for_inward_first_segment() {
    let sector = Sector::new((0.0, 0.0), 100.0, 0.0, 90.0);
    let shape = TextShape {
      width: 20,
      bounds: [0.0, 0.0, 20.0, 10.0],
      vertices: rectangle_vertices([0.0, 0.0, 20.0, 10.0]).to_vec(),
      attachments: vec![0; 5],
      lines: 1,
      overflowed_word: false,
    };
    let mut label = Label::new(0, [-100.0, 30.0, -80.0, 40.0], vec![shape], 0, true, true);
    update_leader(&mut label, &sector);
    assert!(label.leader_visible);
    assert!(
      dot(
        sub(label.leader_points[1], label.leader_points[0]),
        sub(sector.arc, sector.center)
      ) < 0.0
    );
  }

  #[test]
  fn word_pie_random_schedule_is_independent_of_platform_rng() {
    let mut labels = [];
    let mut layout = Layout {
      sectors: &[],
      labels: &mut labels,
      obstacles: Vec::new(),
      objects: Vec::new(),
      cache: Vec::new(),
      maximum_distance: 200.0,
      seed: 0,
      chart: [0.0; 4],
    };
    let first: Vec<_> = (0..5).map(|_| (layout.random() * 32768.0) as u32).collect();
    assert_eq!(first, [38, 7719, 21238, 2437, 8855]);
    assert_eq!(layout.seed, 2727824503);
    for _ in 5..21 {
      layout.random();
    }
    assert_eq!(layout.seed, 3522190791);
  }
}
