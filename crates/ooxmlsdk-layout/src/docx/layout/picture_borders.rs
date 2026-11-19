//! Legacy inline-picture borders: per-side BRC printer bands and shadow copies.
use super::*;
use crate::docx::{ParagraphBordersModel, vml_picture_borders};

pub(super) fn content_top(
  image: &crate::docx::InlineImage,
  metrics: InlineImageMetrics,
  frame_top: f32,
) -> f32 {
  let unchanged = frame_top + metrics.content_offset_y_pt;
  let Some(borders) = image.vml_picture_borders.as_deref() else {
    return unchanged;
  };
  // A complete vertical BRC frame follows the fitted logical baseline.
  // Native single-edge opposites retain a different realization at some
  // printer phases; keep their established edge mapping independent.
  if ![borders.top, borders.bottom]
    .into_iter()
    .all(|border| border.is_some_and(|border| native_brush(border) && !border.shadow))
    || image.rotation_deg != 0.0
    || image.flip_horizontal
    || image.flip_vertical
    || image.inline_frame_axes_swapped
    || image.inline_baseline_gap_pt.is_some()
    || image.crop != ImageCrop::default()
    || image.height_pt <= 0.0
  {
    return unchanged;
  }
  let wmf = emfsdk::detect_format(&image.data) == Some(emfsdk::Format::Wmf);
  let raw_height = match image.picture_scaled_height_pt {
    Some(height) => height * (metrics.content_height_pt / image.height_pt),
    None if wmf => metrics.content_height_pt,
    None => return unchanged,
  };
  // PICF keeps goal*scale finer than its integral-twip character size.
  // The inline picture fits against the completed logical BRC baseline;
  // the bitmap extent and lower printer brush are converted independently.
  // Native side/phase controls distinguish this from snapping the top.
  let span = raw_height + (metrics.frame_height_pt - metrics.content_height_pt);
  let dots_per_point = WORD_FIXED_OUTPUT_PRINTER_DPI as f32 / 72.0;
  let baseline = if wmf {
    // A WMF's local device span follows its separately realized origin.
    // Keep the imported FLOAT's side of a half-dot tie: multiplying by an
    // already rounded FLOAT DPI ratio can erase it (180.35pt frame controls).
    let origin =
      (f64::from(frame_top) * WORD_FIXED_OUTPUT_PRINTER_DPI as f64 / 72.0).round_ties_even() as f32;
    origin + (span * dots_per_point).floor()
  } else {
    ((frame_top + span) * dots_per_point).round_ties_even()
  };
  let top = baseline
    - (raw_height * dots_per_point).round_ties_even()
    - (metrics.content_bottom_gap_pt * dots_per_point).round_ties_even();
  if wmf {
    (f64::from(top) * 72.0 / WORD_FIXED_OUTPUT_PRINTER_DPI as f64) as f32
  } else {
    // The bitmap writer consumes integral Word twips. Preserve the chosen
    // device row when crossing that boundary, rather than snapping twice.
    ((f64::from(top) * 2.4).round() / 20.0) as f32
  }
}

#[derive(Clone, Copy, Default)]
struct Bands {
  widths: [i64; 5],
  colors: [Option<RgbColor>; 5],
  shadow: bool,
}

impl Bands {
  fn new(border: Option<BorderStyle>) -> Self {
    border
      .filter(|border| native_brush(*border))
      .map_or_else(Self::default, |border| Self {
        widths: vml_picture_borders::device_bands(border),
        colors: vml_picture_borders::band_colors(border),
        shadow: border.shadow,
      })
  }

  fn extent(self) -> i64 {
    self.widths.into_iter().sum()
  }
}

fn native_brush(border: BorderStyle) -> bool {
  border.dash_pattern == BorderDashPattern::Solid && border.relief.is_none()
}

struct Painter<'a> {
  items: &'a mut Vec<PageItem>,
  device: Option<wordprocessing_table_device::PageCoordinates>,
}

impl Painter<'_> {
  fn fill(&mut self, (left, top, right, bottom): (i64, i64, i64, i64), color: RgbColor) {
    if right <= left || bottom <= top {
      return;
    }
    let bounds = self.device.map_or_else(
      || {
        (
          left as f32 * 0.12,
          top as f32 * 0.12,
          (right - left) as f32 * 0.12,
          (bottom - top) as f32 * 0.12,
        )
      },
      |device| device.map_device_rectangle((left, top, right - left, bottom - top)),
    );
    self.items.push(PageItem::Fill(FillItem {
      x_pt: bounds.0,
      y_pt: bounds.1,
      width_pt: bounds.2,
      height_pt: bounds.3,
      paint: ShadingPaint::Solid(color),
    }));
  }

  fn band(&mut self, bounds: (i64, i64, i64, i64), color: RgbColor, shadow: i64) {
    self.fill(bounds, color);
    if shadow > 0 {
      self.fill(
        (
          bounds.0 + shadow,
          bounds.1 + shadow,
          bounds.2 + shadow,
          bounds.3 + shadow,
        ),
        color,
      );
    }
  }

  fn horizontal(&mut self, bands: Bands, bounds: (i64, i64, i64, i64), trailing: bool) {
    let (left, top, right, bottom) = bounds;
    let mut offset = 0;
    for (width, color) in bands.widths.into_iter().zip(bands.colors) {
      let y = if trailing {
        bottom - offset - width
      } else {
        top + offset
      };
      if let Some(color) = color {
        self.band(
          (left, y, right, y + width),
          color,
          if trailing && bands.shadow {
            bands.extent()
          } else {
            0
          },
        );
      }
      offset += width;
    }
  }

  fn vertical(&mut self, bands: Bands, bounds: (i64, i64, i64, i64), trailing: bool) {
    let (left, top, right, bottom) = bounds;
    let mut offset = 0;
    for (width, color) in bands.widths.into_iter().zip(bands.colors) {
      let x = if trailing {
        right - offset - width
      } else {
        left + offset
      };
      if let Some(color) = color {
        self.band(
          (x, top, x + width, bottom),
          color,
          if trailing && bands.shadow {
            bands.extent()
          } else {
            0
          },
        );
      }
      offset += width;
    }
  }

  fn corner(
    &mut self,
    h: Bands,
    v: Bands,
    bounds: (i64, i64, i64, i64),
    right: bool,
    bottom: bool,
  ) {
    if h.widths != v.widths {
      self.horizontal(h, bounds, bottom);
      return;
    }
    let (left, top, far_x, far_y) = bounds;
    let mut offset = 0;
    for (index, width) in h.widths.into_iter().enumerate() {
      let x = if right {
        far_x - offset - width
      } else {
        left + offset
      };
      let y = if bottom {
        far_y - offset - width
      } else {
        top + offset
      };
      let (x0, x1) = if right {
        (left, far_x - offset)
      } else {
        (left + offset, far_x)
      };
      let (y0, y1) = if bottom {
        (top, far_y - offset)
      } else {
        (top + offset, far_y)
      };
      if let Some(color) = v.colors[index] {
        self.band(
          (x, y0, x + width, y1),
          color,
          if right && v.shadow { v.extent() } else { 0 },
        );
      }
      if let Some(color) = h.colors[index] {
        self.band(
          (x0, y, x1, y + width),
          color,
          if bottom && h.shadow { h.extent() } else { 0 },
        );
      }
      offset += width;
    }
  }
}

pub(super) fn paint(
  items: &mut Vec<PageItem>,
  setup: PageSetup,
  (x, y, width, height): (f32, f32, f32, f32),
  borders: ParagraphBordersModel,
  image: &crate::docx::InlineImage,
) {
  let start = items.len();
  let [top, right, bottom, left] =
    [borders.top, borders.right, borders.bottom, borders.left].map(Bands::new);
  let coordinate = wordprocessing_table_device::source_coordinate;
  // The PICF image rectangle owns the inner frame; there is no paragraph
  // text inset. Reserve BRC widths in layout, then realize each actual edge.
  let inner = (
    coordinate(x),
    coordinate(y),
    coordinate(x + width),
    coordinate(y + height),
  );
  let outer = (
    inner.0 - left.extent(),
    inner.1 - top.extent(),
    inner.2 + right.extent(),
    inner.3 + bottom.extent(),
  );
  let mut painter = Painter {
    items,
    device: wordprocessing_table_device::page_coordinates(setup.width_pt, setup.height_pt),
  };
  painter.corner(
    top,
    left,
    (outer.0, outer.1, inner.0, inner.1),
    false,
    false,
  );
  painter.horizontal(top, (inner.0, outer.1, inner.2, inner.1), false);
  painter.corner(
    top,
    right,
    (inner.2, outer.1, outer.2, inner.1),
    true,
    false,
  );
  painter.corner(
    bottom,
    left,
    (outer.0, inner.3, inner.0, outer.3),
    false,
    true,
  );
  painter.horizontal(bottom, (inner.0, inner.3, inner.2, outer.3), true);
  painter.corner(
    bottom,
    right,
    (inner.2, inner.3, outer.2, outer.3),
    true,
    true,
  );
  painter.vertical(left, (outer.0, inner.1, inner.0, inner.3), false);
  painter.vertical(right, (inner.2, inner.1, outer.2, inner.3), true);

  // Dashed and relief borders retain the existing Word styled-rule painter;
  // the native solid/compound brush must not turn them into solid bands.
  let mut styled = None;
  for (source, points) in [
    (borders.top, (x, y, x + width, y)),
    (borders.right, (x + width, y, x + width, y + height)),
    (borders.bottom, (x, y + height, x + width, y + height)),
    (borders.left, (x, y, x, y + height)),
  ] {
    if let Some(border) = source.filter(|border| !native_brush(*border)) {
      let page = styled.get_or_insert_with(|| empty_page(setup, 0));
      push_styled_line(page, points.0, points.1, points.2, points.3, border);
    }
  }
  if let Some(page) = styled {
    painter.items.extend(page.items);
  }

  if image.rotation_deg != 0.0 || image.flip_horizontal || image.flip_vertical {
    let center = (f64::from(x + width / 2.0), f64::from(y + height / 2.0));
    let transform = Affine::translate((-center.0, -center.1))
      .then_scale_non_uniform(
        if image.flip_horizontal { -1.0 } else { 1.0 },
        if image.flip_vertical { -1.0 } else { 1.0 },
      )
      .then_rotate(f64::from(image.rotation_deg).to_radians())
      .then_translate(center.into());
    for item in &mut painter.items[start..] {
      let PageItem::Fill(fill) = item else { continue };
      let ShadingPaint::Solid(color) = fill.paint else {
        continue;
      };
      let commands = common::drawingml_geometry::transform_commands(
        vec![
          common::PathCommand::MoveTo(common_point(fill.x_pt, fill.y_pt)),
          common::PathCommand::LineTo(common_point(fill.x_pt + fill.width_pt, fill.y_pt)),
          common::PathCommand::LineTo(common_point(
            fill.x_pt + fill.width_pt,
            fill.y_pt + fill.height_pt,
          )),
          common::PathCommand::LineTo(common_point(fill.x_pt, fill.y_pt + fill.height_pt)),
          common::PathCommand::Close,
        ],
        transform,
      );
      *item = PageItem::path(common::PathItem {
        bounds: drawing_path_bounds(&commands).unwrap(),
        points: Vec::new(),
        commands,
        closed: true,
        fill: common::Fill::Solid(common_rgb(color, 1.0)),
        stroke: None,
      });
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vml_picture_border_brush_retains_opaque_corners_and_the_double_gap() {
    let single = crate::docx::border_style(
      crate::docx::w::BorderValues::Single,
      Some(6),
      None,
      None,
      None,
    )
    .unwrap();
    let double = crate::docx::border_style(
      crate::docx::w::BorderValues::Double,
      Some(6),
      None,
      None,
      None,
    )
    .unwrap();
    let mut items = Vec::new();
    let device = wordprocessing_table_device::page_coordinates(612.0, 792.0);
    let mut painter = Painter {
      items: &mut items,
      device,
    };
    painter.corner(
      Bands::new(Some(single)),
      Bands::new(Some(single)),
      (1200, 1166, 1206, 1172),
      false,
      false,
    );
    assert_eq!(painter.items.len(), 2);
    painter.horizontal(Bands::new(Some(double)), (1218, 1634, 1685, 1652), false);
    let [PageItem::Fill(first), PageItem::Fill(second)] = &painter.items[2..] else {
      panic!("two native rails")
    };
    assert!((first.height_pt - 0.72).abs() < 0.0241);
    assert!((second.y_pt - first.y_pt - 1.44).abs() < 0.0241);
  }
}
