//! Raster-source side surfaces, with material opacity separate from coverage.

use image::{Rgba, RgbaImage};

use super::{blend_over, text_surface_edge, text_surface_triangle_covers_sample};

pub(super) struct SourceEdge {
  pub first: (f32, f32),
  pub second: (f32, f32),
  pub normal: [f32; 2],
  pub facing_hint: [f32; 2],
  pub source_pixel: [u32; 2],
}

#[derive(Clone, Copy)]
pub(super) struct Vertex {
  pub point: (f32, f32),
  pub depth: f32,
}

pub(super) struct Face {
  pub vertices: [Vertex; 4],
  pub color: Rgba<u8>,
}

struct Triangle {
  vertices: [Vertex; 3],
  color: Rgba<u8>,
  area: f32,
  bounds: [u32; 4],
}

#[derive(Clone, Copy)]
struct Fragment {
  depth: f32,
  color: Rgba<u8>,
}

// Use 4x4 coverage samples, the same density as tiny-skia path_aa's
// SUPERSAMPLE_SHIFT=2, without claiming identical edge quantization. Resolve
// after surface compositing: resolving each quad first creates coverage seams.
const GRID: usize = 4;
const SAMPLES: usize = GRID * GRID;
const TILE: u32 = 16;

pub(super) fn composite_source_coverage(destination: &mut RgbaImage, faces: &mut [Face]) {
  // Keep automatic source-fringe reconstruction distinct from authored
  // material transparency. The existing fallback accumulates the source
  // coverage twice in fully covered interiors. Apply that scalar operation
  // before rasterization, not by drawing the geometry twice with different
  // antialias modes. Geometric edges and adjacent cells must resolve once.
  // This preserves the fallback's interior rule, not a claim that its source
  // texture realization already matches Office for every outline width.
  for face in faces.iter_mut() {
    let alpha = u32::from(face.color[3]);
    face.color[3] = (alpha + ((255 - alpha) * alpha + 127) / 255) as u8;
  }
  composite(destination, faces);
}

pub(super) fn composite(destination: &mut RgbaImage, faces: &[Face]) {
  let (width, height) = destination.dimensions();
  if width == 0 || height == 0 || faces.is_empty() {
    return;
  }
  let columns = width.div_ceil(TILE);
  let rows = height.div_ceil(TILE);
  let mut bins = vec![Vec::new(); columns as usize * rows as usize];
  let mut triangles = Vec::new();
  for face in faces {
    if face.color[3] == 0
      || face
        .vertices
        .iter()
        .any(|v| !v.point.0.is_finite() || !v.point.1.is_finite() || !v.depth.is_finite())
    {
      continue;
    }
    for indices in [[0, 1, 2], [2, 3, 0]] {
      let vertices = indices.map(|i| face.vertices[i]);
      let area = text_surface_edge(vertices[0].point, vertices[1].point, vertices[2].point);
      if area.abs() <= 1.0e-8 || !area.is_finite() {
        continue;
      }
      let x = vertices.map(|v| v.point.0);
      let y = vertices.map(|v| v.point.1);
      let bounds = [
        x.into_iter()
          .fold(f32::INFINITY, f32::min)
          .floor()
          .clamp(0.0, width as f32) as u32,
        y.into_iter()
          .fold(f32::INFINITY, f32::min)
          .floor()
          .clamp(0.0, height as f32) as u32,
        x.into_iter()
          .fold(f32::NEG_INFINITY, f32::max)
          .ceil()
          .clamp(0.0, width as f32) as u32,
        y.into_iter()
          .fold(f32::NEG_INFINITY, f32::max)
          .ceil()
          .clamp(0.0, height as f32) as u32,
      ];
      if bounds[0] >= bounds[2] || bounds[1] >= bounds[3] {
        continue;
      }
      let index = triangles.len();
      triangles.push(Triangle {
        vertices,
        color: face.color,
        area,
        bounds,
      });
      for ty in bounds[1] / TILE..=(bounds[3] - 1) / TILE {
        for tx in bounds[0] / TILE..=(bounds[2] - 1) / TILE {
          bins[(ty * columns + tx) as usize].push(index);
        }
      }
    }
  }

  // Storage is limited to a tile's sample lists, not sixteen full-size
  // framebuffer copies. Reuse allocations across tiles, including overlap.
  let mut samples: Vec<Vec<Fragment>> = vec![Vec::new(); TILE as usize * TILE as usize * SAMPLES];
  for (bin_index, bin) in bins.iter().enumerate().filter(|(_, bin)| !bin.is_empty()) {
    for sample in &mut samples {
      sample.clear();
    }
    let left = bin_index as u32 % columns * TILE;
    let top = bin_index as u32 / columns * TILE;
    let right = (left + TILE).min(width);
    let bottom = (top + TILE).min(height);
    for &index in bin {
      let triangle = &triangles[index];
      let [a, b, c] = triangle.vertices;
      for y in top.max(triangle.bounds[1])..bottom.min(triangle.bounds[3]) {
        for x in left.max(triangle.bounds[0])..right.min(triangle.bounds[2]) {
          let pixel = ((y - top) * TILE + x - left) as usize * SAMPLES;
          for sy in 0..GRID {
            for sx in 0..GRID {
              let point = (
                x as f32 + (sx as f32 + 0.5) / GRID as f32,
                y as f32 + (sy as f32 + 0.5) / GRID as f32,
              );
              if !text_surface_triangle_covers_sample(
                a.point,
                b.point,
                c.point,
                point,
                triangle.area,
              ) {
                continue;
              }
              let wa = text_surface_edge(b.point, c.point, point) / triangle.area;
              let wb = text_surface_edge(c.point, a.point, point) / triangle.area;
              let depth = a.depth * wa + b.depth * wb + c.depth * (1.0 - wa - wb);
              samples[pixel + sy * GRID + sx].push(Fragment {
                depth,
                color: triangle.color,
              });
            }
          }
        }
      }
    }
    for y in top..bottom {
      for x in left..right {
        let offset = ((y - top) * TILE + x - left) as usize * SAMPLES;
        let mut sum = [0.0_f32; 4];
        for sample in &mut samples[offset..offset + SAMPLES] {
          // Reciprocal viewpoint distance is used for perspective cameras;
          // both depth representations increase toward the observer.
          sample.sort_unstable_by(|a, b| a.depth.total_cmp(&b.depth));
          let mut premultiplied = [0.0_f32; 4];
          for fragment in sample {
            let alpha = f32::from(fragment.color[3]) / 255.0;
            for (channel, value) in premultiplied[..3].iter_mut().enumerate() {
              *value = f32::from(fragment.color[channel]) * alpha + *value * (1.0 - alpha);
            }
            premultiplied[3] = alpha + premultiplied[3] * (1.0 - alpha);
          }
          for (sum, value) in sum.iter_mut().zip(premultiplied) {
            *sum += value;
          }
        }
        if sum[3] > 0.0 {
          let color = Rgba([
            (sum[0] / sum[3]).round().clamp(0.0, 255.0) as u8,
            (sum[1] / sum[3]).round().clamp(0.0, 255.0) as u8,
            (sum[2] / sum[3]).round().clamp(0.0, 255.0) as u8,
            (sum[3] * 255.0 / SAMPLES as f32).round().clamp(0.0, 255.0) as u8,
          ]);
          blend_over(destination.get_pixel_mut(x, y), color);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn authored_opacity_is_independent_of_front_fringe() {
    use super::super::ExtrusionOpacitySource;
    for source in [0, 1, 64, 128, 192, 255] {
      for alpha in [0, 64, 128, 191, 255] {
        assert_eq!(ExtrusionOpacitySource::Color.alpha(source, alpha), alpha);
        assert_eq!(
          ExtrusionOpacitySource::SourceMask.alpha(source, alpha),
          ((u16::from(source) * u16::from(alpha) + 127) / 255) as u8,
        );
      }
    }
  }

  fn quad(left: f32, right: f32, depth: [f32; 2], color: [u8; 4]) -> Face {
    Face {
      vertices: [(left, 0.0), (right, 0.0), (right, 20.0), (left, 20.0)].map(|point| Vertex {
        depth: if point.0 == left { depth[0] } else { depth[1] },
        point,
      }),
      color: Rgba(color),
    }
  }

  #[test]
  fn adjacent_translucent_cells_resolve_once_across_tiles() {
    let faces = (0..32)
      .map(|x| quad(x as f32, x as f32 + 1.0, [0.0; 2], [20, 80, 140, 128]))
      .collect::<Vec<_>>();
    let mut image = RgbaImage::new(32, 20);
    composite(&mut image, &faces);
    assert!(
      image
        .pixels()
        .all(|pixel| *pixel == Rgba([20, 80, 140, 128]))
    );
  }

  #[test]
  fn automatic_coverage_is_invariant_under_coplanar_subdivision() {
    for alpha in [64, 128, 192, 255] {
      let color = [20, 80, 140, alpha];
      let mut whole = [quad(0.0, 32.0, [0.0; 2], color)];
      let mut split = [
        quad(0.0, 12.375, [0.0; 2], color),
        quad(12.375, 32.0, [0.0; 2], color),
      ];
      let mut a = RgbaImage::new(32, 20);
      let mut b = RgbaImage::new(32, 20);
      composite_source_coverage(&mut a, &mut whole);
      composite_source_coverage(&mut b, &mut split);
      assert_eq!(a, b, "source coverage {alpha}");
    }
  }

  #[test]
  fn transparency_follows_actual_surface_overlap() {
    for alpha in [0, 64, 128, 191, 255] {
      let faces = [
        quad(0.0, 20.0, [0.0; 2], [0, 100, 50, alpha]),
        quad(10.0, 30.0, [1.0; 2], [0, 100, 50, alpha]),
      ];
      let mut image = RgbaImage::new(32, 20);
      composite(&mut image, &faces);
      assert_eq!(image.get_pixel(5, 5)[3], alpha);
      let expected = (f32::from(alpha) * (2.0 - f32::from(alpha) / 255.0)).round() as u8;
      assert_eq!(image.get_pixel(15, 5)[3], expected);
      assert_eq!(image.get_pixel(25, 5)[3], alpha);
      assert_eq!(image.get_pixel(31, 5)[3], 0);
    }
  }

  #[test]
  fn crossing_faces_sort_at_each_sample_not_by_face_average() {
    let a = quad(0.0, 32.0, [-1.0, 1.0], [255, 0, 0, 255]);
    let b = quad(0.0, 32.0, [1.0, -1.0], [0, 0, 255, 255]);
    let mut image = RgbaImage::new(32, 20);
    composite(&mut image, &[a, b]);
    assert_eq!(*image.get_pixel(4, 5), Rgba([0, 0, 255, 255]));
    assert_eq!(*image.get_pixel(27, 5), Rgba([255, 0, 0, 255]));
  }
}
