use skrifa::{
  FontRef, GlyphId, MetadataProvider,
  instance::Size,
  outline::{DrawSettings, OutlinePen, pen::ControlBoundsPen},
  raw::TableProvider,
};

use crate::fonts::FontFaceData;
use crate::text_metrics::ShapedGlyphBounds;

/// VML trim fits the outline that the PDF and raster painters synthesize,
/// rather than the upright face's stored glyph rectangle. Shear actual points:
/// transforming the rectangle would also include corners absent from the glyph.
pub(super) fn synthetic_italic_bounds(
  face: &FontFaceData,
  glyph_id: u32,
) -> Option<ShapedGlyphBounds> {
  if !face.synthetic_italic {
    return None;
  }
  let font = FontRef::from_index(face.data.as_ref(), face.index).ok()?;
  let upem = f32::from(font.head().ok()?.units_per_em());
  if upem <= 0.0 {
    return None;
  }
  let outline = font.outline_glyphs().get(GlyphId::new(glyph_id))?;
  let mut pen = ItalicBoundsPen::default();
  outline
    .draw(
      DrawSettings::unhinted(Size::unscaled(), face.location()),
      &mut pen,
    )
    .ok()?;
  let bounds = pen.0.bounding_box()?;
  Some(ShapedGlyphBounds {
    x_min_em: bounds.x_min / upem,
    y_min_em: bounds.y_min / upem,
    x_max_em: bounds.x_max / upem,
    y_max_em: bounds.y_max / upem,
  })
}

#[derive(Default)]
struct ItalicBoundsPen(ControlBoundsPen);

impl OutlinePen for ItalicBoundsPen {
  fn move_to(&mut self, x: f32, y: f32) {
    self.0.move_to(x + y / 3.0, y);
  }

  fn line_to(&mut self, x: f32, y: f32) {
    self.0.line_to(x + y / 3.0, y);
  }

  fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
    self.0.quad_to(cx + cy / 3.0, cy, x + y / 3.0, y);
  }

  fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
    self
      .0
      .curve_to(cx0 + cy0 / 3.0, cy0, cx1 + cy1 / 3.0, cy1, x + y / 3.0, y);
  }

  fn close(&mut self) {
    self.0.close();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn italic_fit_uses_actual_outline_points_instead_of_empty_rectangle_corners() {
    let mut pen = ItalicBoundsPen::default();
    // The upper-right corner (8, 9) is outside this L-shaped outline.
    for (index, (x, y)) in [
      (0.0, 0.0),
      (8.0, 0.0),
      (8.0, 1.0),
      (1.0, 1.0),
      (1.0, 9.0),
      (0.0, 9.0),
    ]
    .into_iter()
    .enumerate()
    {
      if index == 0 {
        pen.move_to(x, y);
      } else {
        pen.line_to(x, y);
      }
    }
    pen.close();
    let bounds = pen.0.bounding_box().unwrap();
    assert_eq!((bounds.x_min, bounds.y_min, bounds.y_max), (0.0, 0.0, 9.0));
    assert_eq!(bounds.x_max, 8.0 + 1.0 / 3.0);
    assert!(bounds.x_max < 8.0 + 9.0 / 3.0);
  }
}
