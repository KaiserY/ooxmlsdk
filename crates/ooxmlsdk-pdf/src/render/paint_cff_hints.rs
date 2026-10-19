//! Horizontal stem placement for private monochrome CFF glyphs.
//!
//! Skrifa's CFF sink applies the horizontal-stem (Y) hints while scaling X.
//! Native GDI CFF outline controls place vertical-stem centers on whole or
//! half pixels according to the rounded stem width. Retain the outline's
//! width, interpolate between active stem edges, and keep Skrifa's Y result.
//! Type2 vstem/hintmask ownership follows Adobe's Type2 specification; the
//! edge interpolation follows the same piecewise mapping as CF2 hint maps.
use resvg::tiny_skia::{Path, PathBuilder, PathSegment};
use skrifa::{
  FontRef,
  raw::{
    TableProvider,
    ps::{cff::CffFontRef, cs::CommandSink, transform::FontMatrix},
    types::{Fixed, GlyphId},
  },
};

#[derive(Clone, Copy)]
struct Stem {
  index: usize,
  min: f64,
  max: f64,
}

struct HorizontalHints {
  scale: f64,
  hint_count: usize,
  stems: Vec<Stem>,
  mask: Option<Vec<u8>>,
  anchors: Vec<(f64, f64)>,
  dirty: bool,
  unsupported: bool,
  path: PathBuilder,
}

impl HorizontalHints {
  fn map(&mut self, coordinate: Fixed) -> f32 {
    if self.dirty {
      self.anchors.clear();
      for stem in &self.stems {
        if self.mask.as_ref().is_some_and(|mask| {
          mask
            .get(stem.index / 8)
            .is_none_or(|byte| byte & (0x80 >> (stem.index & 7)) == 0)
        }) {
          continue;
        }
        let width = stem.max - stem.min;
        if width <= 0.0 {
          continue;
        }
        let odd = width.round() as i64 & 1;
        let phase = odd as f64 * 0.5;
        let center = ((stem.min + stem.max) * 0.5 - phase).round() + phase;
        self.anchors.push((stem.min, center - width * 0.5));
        self.anchors.push((stem.max, center + width * 0.5));
      }
      self.anchors.sort_by(|a, b| a.0.total_cmp(&b.0));
      self.dirty = false;
    }
    let x = f64::from(coordinate.to_f32()) * self.scale;
    let upper = self.anchors.partition_point(|anchor| anchor.0 <= x);
    match (
      upper
        .checked_sub(1)
        .and_then(|index| self.anchors.get(index)),
      self.anchors.get(upper),
    ) {
      (Some(&(a, b)), Some(&(c, d))) if c > a => (b + (x - a) * (d - b) / (c - a)) as f32,
      (Some(&(a, b)), _) | (_, Some(&(a, b))) => (x + b - a) as f32,
      _ => x as f32,
    }
  }

  fn y(&self, y: Fixed) -> f32 {
    (f64::from(y.to_f32()) * self.scale) as f32
  }
}

impl CommandSink for HorizontalHints {
  fn hstem(&mut self, _: Fixed, _: Fixed) {
    self.hint_count += 1;
  }
  fn vstem(&mut self, min: Fixed, max: Fixed) {
    if self.hint_count >= 96 {
      self.unsupported = true;
      return;
    }
    self.stems.push(Stem {
      index: self.hint_count,
      min: f64::from(min.to_f32()) * self.scale,
      max: f64::from(max.to_f32()) * self.scale,
    });
    self.hint_count += 1;
    self.dirty = true;
  }
  fn hint_mask(&mut self, mask: &[u8]) {
    self.mask = Some(mask.to_vec());
    self.dirty = true;
  }
  fn counter_mask(&mut self, _: &[u8]) {
    // Counter groups need joint stem placement; do not partially realize one
    // with independent centers and silently change its counters.
    self.unsupported = true;
  }
  fn clear_hints(&mut self) {
    self.hint_count = 0;
    self.stems.clear();
    self.mask = None;
    self.dirty = true;
  }
  fn move_to(&mut self, x: Fixed, y: Fixed) {
    let x = self.map(x);
    self.path.move_to(x, self.y(y));
  }
  fn line_to(&mut self, x: Fixed, y: Fixed) {
    let x = self.map(x);
    self.path.line_to(x, self.y(y));
  }
  fn curve_to(&mut self, a: Fixed, b: Fixed, c: Fixed, d: Fixed, e: Fixed, f: Fixed) {
    let a = self.map(a);
    let c = self.map(c);
    let e = self.map(e);
    self.path.cubic_to(a, self.y(b), c, self.y(d), e, self.y(f));
  }
  fn close(&mut self) {
    self.path.close();
  }
}

pub(super) fn horizontal(
  font: &FontRef<'_>,
  ppem: u32,
  glyph: u32,
  vertical: &Path,
) -> Option<Path> {
  let table = font.cff().ok()?;
  let cff = CffFontRef::new_cff(
    table.offset_data().as_bytes(),
    0,
    Some(i32::from(font.head().ok()?.units_per_em())),
  )
  .ok()?;
  let glyph = GlyphId::new(glyph);
  let subfont = cff.subfont(cff.subfont_index(glyph)?, &[]).ok()?;
  let transform = cff.transform(&subfont, Some(ppem as f32));
  if transform.matrix != FontMatrix::IDENTITY {
    return None;
  }
  // The CFF scaler's hint-space conversion preserves a16.16 scale; its
  // eventual outline-to-26.6 truncation is a separate boundary.
  let scale = f64::from(transform.scale?.to_bits().saturating_add(32) / 64) / 65536.0;
  let mut hints = HorizontalHints {
    scale,
    hint_count: 0,
    stems: Vec::new(),
    mask: None,
    anchors: Vec::new(),
    dirty: true,
    unsupported: false,
    path: PathBuilder::new(),
  };
  cff
    .evaluate_charstring(&subfont, glyph, &[], &mut hints)
    .ok()?;
  if hints.unsupported {
    return None;
  }
  let horizontal = hints.path.finish()?;
  let mut x = horizontal.segments();
  let mut combined = PathBuilder::new();
  for y in vertical.segments() {
    match (x.next()?, y) {
      (PathSegment::MoveTo(x), PathSegment::MoveTo(y)) => combined.move_to(x.x, y.y),
      (PathSegment::LineTo(x), PathSegment::LineTo(y)) => combined.line_to(x.x, y.y),
      (PathSegment::CubicTo(a, b, c), PathSegment::CubicTo(d, e, f)) => {
        combined.cubic_to(a.x, d.y, b.x, e.y, c.x, f.y)
      }
      (PathSegment::Close, PathSegment::Close) => combined.close(),
      _ => return None,
    }
  }
  if x.next().is_some() {
    return None;
  }
  combined.finish()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn hints(ppem: u32) -> HorizontalHints {
    HorizontalHints {
      scale: f64::from(Fixed::from_f64(f64::from(ppem) / 1000.0).to_f32()),
      hint_count: 0,
      stems: Vec::new(),
      mask: None,
      anchors: Vec::new(),
      dirty: true,
      unsupported: false,
      path: PathBuilder::new(),
    }
  }

  #[test]
  fn cff_font_vstem_centers_follow_native_whole_and_half_pixel_phases() {
    // Native GGO_NATIVE readbacks, including a second size and an independent
    // synthetic face. Width preservation and center placement are separate.
    for (left, right, ppem, center) in [
      (51, 153, 400, 40.500122_f32),
      (58, 177, 400, 47.000145),
      (51, 148, 800, 80.000305),
    ] {
      let mut hints = hints(ppem);
      let left = Fixed::from_i32(left);
      let right = Fixed::from_i32(right);
      hints.vstem(left, right);
      assert!(((hints.map(left) + hints.map(right)) / 2.0 - center).abs() < 0.001);
    }
  }

  #[test]
  fn cff_font_hint_masks_count_both_axes_and_clear_between_components() {
    let mut hints = hints(400);
    hints.hstem(Fixed::ZERO, Fixed::from_i32(700));
    hints.vstem(Fixed::from_i32(51), Fixed::from_i32(153));
    hints.vstem(Fixed::from_i32(307), Fixed::from_i32(409));
    hints.hint_mask(&[0x40]);
    let isolated = hints.map(Fixed::from_i32(307));
    hints.hint_mask(&[0x60]);
    let both = hints.map(Fixed::from_i32(307));
    assert!((both - 123.1).abs() < 0.001);
    assert!(both - isolated > 0.5);
    hints.clear_hints();
    assert_eq!(hints.map(Fixed::ZERO), 0.0);
    assert!((hints.map(Fixed::from_i32(307)) - 122.8).abs() < 0.01);
  }
}
