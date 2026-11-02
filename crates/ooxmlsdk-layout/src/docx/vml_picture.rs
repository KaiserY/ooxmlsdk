//! Word's legacy inline bitmap size, before fixed-output device mapping.
//!
//! MS-DOC PICF stores natural dimensions in twips and mx/my in tenths of a
//! percent. Native Word DOC readback agrees for independent 96/150/300/600-DPI
//! and anisotropic PNG/JPEG controls. Floating VML pictures bypass this conversion.

use std::io::Cursor;

use ooxmlsdk::schemas::{a, v};

use crate::common;

const PICTURE_FRAME_PATH: &str = "m@4@5l@4@11@9@11@9@5xe";
const PICTURE_FRAME_GUIDES: &[&str] = &[
  "if lineDrawn pixelLineWidth 0",
  "sum @0 1 0",
  "sum 0 0 @1",
  "prod @2 1 2",
  "prod @3 21600 pixelWidth",
  "prod @3 21600 pixelHeight",
  "sum @0 0 1",
  "prod @6 1 2",
  "prod @7 21600 pixelWidth",
  "sum @8 21600 0",
  "prod @7 21600 pixelHeight",
  "sum @10 21600 0",
];

/// Recognize the Office picture template, rather than its reusable spt alone.
/// A custom path can retain spt75 but has a different enclosing frame.
pub(super) fn apply_floating_picture_frame(
  image: &mut super::InlineImage,
  frame: &mut super::InlineShape,
  shape: &v::Shape,
  shape_type: Option<&v::Shapetype>,
) {
  let super::ImagePlacement::Floating(placement) = image.placement else {
    return;
  };
  if image.rotation_deg != 0.0
    || image.flip_horizontal
    || image.flip_vertical
    || frame.effects.is_some()
    || placement.alignment_extent.is_some()
    || placement
      .relative_width_pct
      .is_some_and(|value| value > 0.0)
    || placement
      .relative_height_pct
      .is_some_and(|value| value > 0.0)
  {
    return;
  }
  let Some(shape_type) = shape_type else {
    return;
  };
  let number = shape_type.optional_number.or_else(|| {
    shape
      .r#type
      .as_deref()
      .and_then(super::vml_shape_type_number)
  });
  if number != Some(75) {
    return;
  }
  let path = super::vml_shape_path(shape)
    .and_then(|path| path.value.as_deref())
    .or(shape.edge_path.as_deref())
    .or_else(|| super::vml_shapetype_path(shape_type).and_then(|path| path.value.as_deref()))
    .or(shape_type.edge_path.as_deref());
  if path != Some(PICTURE_FRAME_PATH)
    && path.and_then(super::vml_path_tokens) != super::vml_path_tokens(PICTURE_FRAME_PATH)
  {
    return;
  }
  let size = super::vml_coordinate_pair(
    shape
      .coordinate_size
      .as_deref()
      .or(shape_type.coordinate_size.as_deref()),
  );
  let origin = super::vml_coordinate_pair(
    shape
      .coordinate_origin
      .as_deref()
      .or(shape_type.coordinate_origin.as_deref()),
  );
  if size.unwrap_or((21600.0, 21600.0)) != (21600.0, 21600.0)
    || origin.unwrap_or((0.0, 0.0)) != (0.0, 0.0)
  {
    return;
  }
  let Some(formulas) =
    super::vml_shape_formulas(shape).or_else(|| super::vml_shapetype_formulas(shape_type))
  else {
    return;
  };
  if formulas.formula.len() != PICTURE_FRAME_GUIDES.len()
    || !formulas
      .formula
      .iter()
      .zip(PICTURE_FRAME_GUIDES)
      .all(|(formula, expected)| {
        formula.equation.as_deref().is_some_and(|equation| {
          equation
            .split_ascii_whitespace()
            .eq(expected.split_ascii_whitespace())
        })
      })
  {
    return;
  }
  let Some(width) = frame
    .stroke_override
    .as_deref()
    .map(|stroke| stroke.width.0)
    .or_else(|| frame.stroke.map(|stroke| stroke.width_pt))
    .filter(|width| width.is_finite() && *width >= 0.0)
  else {
    return;
  };
  let super::InlineShapeGeometry::Path { paths, preset, .. } = &mut frame.geometry else {
    return;
  };
  if paths.len() != 1 || !paths[0].stroke {
    return;
  }
  *preset = Some(a::ShapeTypeValues::Rectangle);
  // Native pen/axis controls distinguish this screen-device enclosure from
  // the 600-DPI painted path. Even an enabled zero-width pen reserves one
  // screen pixel. Existing floating metrics select the joint aligned frame;
  // absolute offsets keep the bitmap origin, independently of these outsets.
  const DEVICE_STEP_PT: f32 = 72.0 / 96.0;
  let outset = (width.max(DEVICE_STEP_PT) / DEVICE_STEP_PT).ceil() * DEVICE_STEP_PT;
  image.effect_left_pt = outset;
  image.effect_top_pt = outset;
  image.effect_right_pt = outset;
  image.effect_bottom_pt = outset;
}

pub(super) fn png_goal_twips(data: &[u8]) -> Option<(f64, f64)> {
  let reader = png::Decoder::new(Cursor::new(data)).read_info().ok()?;
  let info = reader.info();
  // Word's materialized PICF uses 96 DPI when pHYs is absent, unitless,
  // or zero. An explicit physical density still keeps its exact conversion:
  // 3780 pixels/meter is 96.012 DPI, not the default 96 DPI.
  let density = match info.pixel_dims {
    Some(density) if density.unit == png::Unit::Meter && density.xppu != 0 && density.yppu != 0 => {
      (
        f64::from(density.xppu) * 0.0254,
        f64::from(density.yppu) * 0.0254,
      )
    }
    _ => (96.0, 96.0),
  };
  let axis = |pixels: u32, dpi: f64| {
    let goal = (f64::from(pixels) * 1440.0 / dpi).round();
    (goal >= 1.0 && goal <= f64::from(i16::MAX)).then_some(goal)
  };
  Some((axis(info.width, density.0)?, axis(info.height, density.1)?))
}

fn jpeg_goal_twips(data: &[u8]) -> Option<(f64, f64)> {
  let (unit, horizontal, vertical) = common::drawingml_image_tile::jpeg_jfif_density(data)?;
  // Native Word PICF readback uses 96 DPI for unitless or zero-density JFIF,
  // and the authored physical density otherwise. Read only the JPEG headers;
  // recovering the layout size must not decode the photographic sample plane.
  let density = match unit {
    0 => (96.0, 96.0),
    1 | 2 if horizontal == 0 || vertical == 0 => (96.0, 96.0),
    1 => (f64::from(horizontal), f64::from(vertical)),
    2 => (f64::from(horizontal) * 2.54, f64::from(vertical) * 2.54),
    _ => return None,
  };
  let (width, height) = common::drawingml_image_tile::jpeg_dimensions(data)?;
  let axis = |pixels: u32, dpi: f64| {
    let goal = (f64::from(pixels) * 1440.0 / dpi).round();
    (goal >= 1.0 && goal <= f64::from(i16::MAX)).then_some(goal)
  };
  Some((axis(width, density.0)?, axis(height, density.1)?))
}

pub(super) fn bitmap_goal_twips(data: &[u8]) -> Option<(f64, f64)> {
  if data.starts_with(&[0xff, 0xd8]) {
    jpeg_goal_twips(data)
  } else {
    png_goal_twips(data)
  }
}

fn scaled_dimension_twips(authored_pt: f32, goal_twips: f64) -> Option<f64> {
  let authored_twips = (f64::from(authored_pt) * 20.0).round();
  let scale = (authored_twips * 1000.0 / goal_twips).round();
  if !(1.0..=f64::from(u16::MAX)).contains(&scale) {
    return None;
  }
  Some(goal_twips * scale / 1000.0)
}

#[cfg(test)]
fn scaled_dimension_pt(authored_pt: f32, goal_twips: f64) -> Option<f32> {
  scaled_dimension_twips(authored_pt, goal_twips).map(|twips| (twips.round() / 20.0) as f32)
}

pub(super) struct InlineBitmapSize {
  pub(super) layout_pt: (f32, f32),
  pub(super) paint_pt: Option<(f32, f32)>,
}

pub(super) fn inline_bitmap_size(data: &[u8], width: f32, height: f32) -> Option<InlineBitmapSize> {
  let (goal_x, goal_y) = bitmap_goal_twips(data)?;
  let width_twips = scaled_dimension_twips(width, goal_x)?;
  let height_twips = scaled_dimension_twips(height, goal_y)?;
  Some(InlineBitmapSize {
    layout_pt: (
      (width_twips.round() / 20.0) as f32,
      (height_twips.round() / 20.0) as f32,
    ),
    // JPEG density controls distinguish the PICF product from the rounded
    // character extent: 576 * 13177 / 1000 twips paints 3162 printer dots,
    // whereas first rounding that product to twips would paint 3163 dots.
    // Preserve it until the final printer mapping; paragraph advance still
    // uses integral twips. PNG retains its separately established path.
    paint_pt: data
      .starts_with(&[0xff, 0xd8])
      .then_some(((width_twips / 20.0) as f32, (height_twips / 20.0) as f32)),
  })
}

/// The legacy inline producer consumes a completed origin in integral Word
/// twips, realizes origin and extent independently on the 600-DPI printer,
/// then exports the two endpoints in integral twips.
/// In particular, rounding a width alone loses the origin's fractional twip.
pub(super) fn inline_paint_bounds(mut bounds: common::Rect) -> common::Rect {
  let axis = |origin: f32, extent: f32| {
    let origin_twips = f64::from((origin * 20.0).round());
    let first = (origin_twips / 2.4).round();
    let last = first + (f64::from(extent) / 0.12).round();
    let first_twips = (first * 2.4).round();
    let last_twips = (last * 2.4).round();
    (
      (first_twips / 20.0) as f32,
      ((last_twips - first_twips) / 20.0) as f32,
    )
  };
  (bounds.origin.x.0, bounds.size.width.0) = axis(bounds.origin.x.0, bounds.size.width.0);
  (bounds.origin.y.0, bounds.size.height.0) = axis(bounds.origin.y.0, bounds.size.height.0);
  bounds
}

#[cfg(test)]
pub(super) fn frame_test_image(weight: f32, changes: &[(&str, &str)]) -> super::InlineImage {
  use ooxmlsdk::sdk::SdkType;
  let formulas = PICTURE_FRAME_GUIDES
    .iter()
    .map(|value| format!("<v:f eqn=\"{value}\"/>"))
    .collect::<String>();
  let mut source = format!(
    r##"<w:pict xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"
    xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
    <v:shapetype id="_x0000_t75" o:spt="75" coordsize="21600,21600" filled="f" stroked="f" path="{PICTURE_FRAME_PATH}">
      <v:formulas>{formulas}</v:formulas><v:stroke joinstyle="miter"/>
    </v:shapetype>
    <v:shape type="#_x0000_t75" style="position:absolute;width:117.75pt;height:117.75pt;mso-position-horizontal:left"
      stroked="t" strokeweight="{weight}pt"><v:stroke dashstyle="dash"/><v:imagedata r:id="rId1"/></v:shape>
    </w:pict>"##
  );
  for (from, to) in changes {
    source = source.replace(from, to);
  }
  let picture = super::w::Picture::from_bytes(source.as_bytes()).unwrap();
  let mut catalog = super::ImageCatalog::default();
  catalog.by_relationship_id.insert(
    "rId1".into(),
    super::package::ImageResource {
      data: vec![1, 2, 3].into(),
      content_type: Some("image/png".into()),
    },
  );
  let shape_types = picture
    .picture_choice
    .iter()
    .filter_map(|item| match item {
      super::w::PictureChoice::Shapetype(value) => Some(value.as_ref()),
      _ => None,
    })
    .collect::<Vec<_>>();
  let shape = picture
    .picture_choice
    .iter()
    .find_map(|item| match item {
      super::w::PictureChoice::Shape(value) => Some(value.as_ref()),
      _ => None,
    })
    .unwrap();
  super::shape_image_with_style_and_shape_types(
    shape,
    shape.style.as_deref(),
    &catalog,
    &shape_types,
  )
  .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vml_picture_template_keeps_screen_enclosure_and_printer_pen_independent() {
    for (weight, outset, left, right) in [
      (0.0, 0.75, 0.0, 0.0),
      (0.25, 0.75, -0.24, 0.12),
      (0.74, 0.75, -0.48, 0.36),
      (0.75, 0.75, -0.48, 0.36),
      (0.76, 1.5, -0.48, 0.36),
      (1.0, 1.5, -0.60, 0.48),
      (1.25, 1.5, -0.72, 0.60),
      (1.5, 1.5, -0.84, 0.72),
      (3.0, 3.0, -1.56, 1.44),
    ] {
      let image = frame_test_image(weight, &[]);
      assert_eq!((image.width_pt, image.height_pt), (117.75, 117.75));
      assert_eq!(
        [
          image.effect_left_pt,
          image.effect_top_pt,
          image.effect_right_pt,
          image.effect_bottom_pt
        ],
        [outset; 4]
      );
      let frame = image.picture_frame.as_deref().unwrap();
      assert_eq!(
        frame.stroke.unwrap().width_pt,
        if weight == 0.0 {
          crate::units::OFFICE_VML_HAIRLINE_WIDTH_PT
        } else {
          weight
        },
      );
      let super::super::InlineShapeGeometry::Path { paths, preset, .. } = &frame.geometry else {
        panic!("picture template path")
      };
      assert_eq!(*preset, Some(a::ShapeTypeValues::Rectangle));
      let [
        common::PathCommand::MoveTo(a),
        _,
        common::PathCommand::LineTo(c),
        _,
        common::PathCommand::Close,
      ] = paths[0].commands.as_slice()
      else {
        panic!("four corner template")
      };
      assert!((a.x.0 - left).abs() < 0.01);
      assert!((c.x.0 - 117.75 - right).abs() < 0.01);
    }
    for changes in [
      vec![
        ("position:absolute;", ""),
        (";mso-position-horizontal:left", ""),
      ],
      vec![("stroked=\"t\"", "stroked=\"f\"")],
      vec![(PICTURE_FRAME_PATH, "m0,0l0,21600,21600,21600,21600,0xe")],
      vec![("prod @2 1 2", "prod @2 1 3")],
      vec![(
        "mso-position-horizontal:left",
        "mso-position-horizontal:left;rotation:33",
      )],
      vec![(
        "mso-position-horizontal:left",
        "mso-position-horizontal:left;mso-width-percent:500",
      )],
    ] {
      let image = frame_test_image(0.75, &changes);
      assert_eq!(
        [
          image.effect_left_pt,
          image.effect_top_pt,
          image.effect_right_pt,
          image.effect_bottom_pt
        ],
        [0.0; 4],
        "{changes:?}"
      );
    }
  }

  fn jpeg(unit: u8, horizontal: u16, vertical: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    image::codecs::jpeg::JpegEncoder::new(&mut bytes)
      .encode_image(&image::RgbImage::new(120, 106))
      .unwrap();
    assert_eq!(&bytes[6..11], b"JFIF\0");
    bytes[13] = unit;
    bytes[14..16].copy_from_slice(&horizontal.to_be_bytes());
    bytes[16..18].copy_from_slice(&vertical.to_be_bytes());
    bytes
  }

  #[test]
  fn vml_picture_jpeg_goals_and_scales_match_native_picf() {
    // Word's materialized DOC records, with the same authored 379.5x244.5pt
    // extent. Density changes the initial goals before mx/my quantization.
    for (unit, density, goals, expected) in [
      (1, [96, 96], [1800.0, 1590.0], [379.55, 244.45]),
      (1, [150, 150], [1152.0, 1018.0], [379.55, 244.5]),
      (1, [300, 300], [576.0, 509.0], [379.5, 244.5]),
      (1, [600, 600], [288.0, 254.0], [379.5, 244.5]),
      (1, [150, 300], [1152.0, 509.0], [379.55, 244.5]),
      (0, [1, 1], [1800.0, 1590.0], [379.55, 244.45]),
      (0, [96, 96], [1800.0, 1590.0], [379.55, 244.45]),
      (2, [96, 96], [709.0, 626.0], [379.5, 244.5]),
      (1, [0, 0], [1800.0, 1590.0], [379.55, 244.45]),
    ] {
      let data = jpeg(unit, density[0], density[1]);
      assert_eq!(bitmap_goal_twips(&data), Some((goals[0], goals[1])));
      assert_eq!(
        inline_bitmap_size(&data, 379.5, 244.5).map(|size| size.layout_pt),
        Some((expected[0], expected[1]))
      );
    }
    let data = jpeg(1, 96, 96);
    for (width, expected) in [
      (379.4, 379.45),
      (379.45, 379.45),
      (379.55, 379.55),
      (379.6, 379.6),
    ] {
      assert_eq!(
        inline_bitmap_size(&data, width, 244.5).map(|size| size.layout_pt),
        Some((expected, 244.45))
      );
    }
  }

  #[test]
  fn vml_picture_jpeg_paint_keeps_the_picf_product_until_printer_mapping() {
    let size = inline_bitmap_size(&jpeg(1, 300, 300), 379.5, 244.5).unwrap();
    assert_eq!(size.layout_pt, (379.5, 244.5));
    let (width, height) = size.paint_pt.unwrap();
    let bounds = inline_paint_bounds(common::Rect {
      origin: common::Point {
        x: common::Pt(85.1),
        y: common::Pt(76.8),
      },
      size: common::Size {
        width: common::Pt(width),
        height: common::Pt(height),
      },
    });
    assert_eq!(bounds.size.width.0, 379.4);
    assert_eq!(bounds.size.height.0, 244.45);
  }

  #[test]
  fn vml_picture_jpeg_import_keeps_floating_and_transformed_sizes() {
    use crate::docx::{ImageCatalog, package::ImageResource, v, vml_image_data};
    let mut catalog = ImageCatalog::default();
    catalog.by_relationship_id.insert(
      "rId1".into(),
      ImageResource {
        data: jpeg(1, 96, 96).into(),
        content_type: Some("image/jpeg".into()),
      },
    );
    let data = v::ImageData {
      relationship_id: Some("rId1".into()),
      ..Default::default()
    };
    for (prefix, expected) in [
      ("", (379.55, 244.45)),
      ("position:absolute;", (379.5, 244.5)),
      ("rotation:90;", (379.5, 244.5)),
      ("flip:x;", (379.5, 244.5)),
    ] {
      let style = format!("{prefix}width:379.5pt;height:244.5pt");
      let image = vml_image_data(&data, Some(&style), true, None, &catalog).unwrap();
      assert_eq!((image.width_pt, image.height_pt), expected);
    }
  }

  fn png(width: u32, height: u32, density: Option<png::PixelDimensions>) -> Vec<u8> {
    let mut bytes = Vec::new();
    {
      let mut encoder = png::Encoder::new(&mut bytes, width, height);
      encoder.set_color(png::ColorType::Grayscale);
      encoder.set_depth(png::BitDepth::Eight);
      encoder.set_pixel_dims(density);
      encoder
        .write_header()
        .unwrap()
        .write_image_data(&vec![0; (width * height) as usize])
        .unwrap();
    }
    bytes
  }

  #[test]
  fn vml_picture_physical_png_goals_match_native_density_controls() {
    for (ppu, right, left) in [
      ([5905, 5905], [3322.0, 2813.0], [2861.0, 2554.0]),
      ([3780, 3780], [5189.0, 4394.0], [4469.0, 3990.0]),
      ([5906, 5906], [3321.0, 2813.0], [2861.0, 2553.0]),
      ([11811, 11811], [1661.0, 1406.0], [1430.0, 1277.0]),
      ([23622, 23622], [830.0, 703.0], [715.0, 638.0]),
      ([5905, 11811], [3322.0, 1406.0], [2861.0, 1277.0]),
    ] {
      let density = Some(png::PixelDimensions {
        xppu: ppu[0],
        yppu: ppu[1],
        unit: png::Unit::Meter,
      });
      assert_eq!(
        png_goal_twips(&png(346, 293, density)),
        Some((right[0], right[1]))
      );
      assert_eq!(
        png_goal_twips(&png(298, 266, density)),
        Some((left[0], left[1]))
      );
    }
  }

  #[test]
  fn vml_picture_png_default_density_matches_native_picf() {
    // Word DOC readback for the same 264x300-pixel PNG, changing only pHYs.
    // Both 85.5pt and 85.55pt authored widths realize the same PICF scale.
    for density in [
      None,
      Some(png::PixelDimensions {
        xppu: 5906,
        yppu: 5906,
        unit: png::Unit::Unspecified,
      }),
      Some(png::PixelDimensions {
        xppu: 0,
        yppu: 0,
        unit: png::Unit::Meter,
      }),
    ] {
      let data = png(264, 300, density);
      assert_eq!(png_goal_twips(&data), Some((3960.0, 4500.0)));
      for width in [85.5, 85.55] {
        assert_eq!(
          inline_bitmap_size(&data, width, 94.5).unwrap().layout_pt,
          (85.55, 94.5)
        );
      }
    }
    let data = png(
      264,
      300,
      Some(png::PixelDimensions {
        xppu: 3780,
        yppu: 3780,
        unit: png::Unit::Meter,
      }),
    );
    assert_eq!(png_goal_twips(&data), Some((3960.0, 4499.0)));
  }

  #[test]
  fn vml_picture_png_import_preserves_floating_and_transformed_sizes() {
    use crate::docx::{ImageCatalog, package::ImageResource, v, vml_image_data};
    let mut catalog = ImageCatalog::default();
    for density in [
      None,
      Some(png::PixelDimensions {
        xppu: 5905,
        yppu: 5905,
        unit: png::Unit::Meter,
      }),
    ] {
      catalog.by_relationship_id.insert(
        "rId1".into(),
        ImageResource {
          data: png(298, 266, density).into(),
          content_type: Some("image/png".into()),
        },
      );
      let data = v::ImageData {
        relationship_id: Some("rId1".into()),
        ..Default::default()
      };
      for (prefix, expected) in [
        (
          "",
          if density.is_some() {
            (143.2, 127.2)
          } else {
            (143.05, 127.3)
          },
        ),
        ("position:absolute;", (143.15, 127.25)),
        ("rotation:90;", (143.15, 127.25)),
        ("flip:x;", (143.15, 127.25)),
      ] {
        let style = format!("{prefix}width:143.15pt;height:127.25pt");
        let image = vml_image_data(&data, Some(&style), true, None, &catalog).unwrap();
        assert_eq!((image.width_pt, image.height_pt), expected);
      }
    }
    assert_eq!(
      png_goal_twips(&png(
        1,
        1,
        Some(png::PixelDimensions {
          xppu: 5905,
          yppu: 5905,
          unit: png::Unit::Unspecified,
        })
      )),
      Some((15.0, 15.0))
    );
  }

  #[test]
  fn vml_picture_native_picf_density_scales() {
    // Actual Word PICF records: goals and independently observed mx/my.
    for (goal, authored, scale, realized) in [
      (3322.0, 127.25, 766.0, 127.25),
      (2813.0, 127.25, 905.0, 127.3),
      (2861.0, 143.15, 1001.0, 143.2),
      (2554.0, 127.25, 996.0, 127.2),
      (3322.0, 127.35, 767.0, 127.4),
      (1406.0, 127.25, 1810.0, 127.25),
      (1277.0, 127.25, 1993.0, 127.25),
      (830.0, 127.25, 3066.0, 127.25),
      (638.0, 127.25, 3989.0, 127.25),
      (5189.0, 127.25, 490.0, 127.15),
    ] {
      assert_eq!(
        ((authored as f64 * 20.0).round() * 1000.0 / goal).round(),
        scale
      );
      assert_eq!(scaled_dimension_pt(authored, goal), Some(realized));
    }
  }

  #[test]
  fn vml_picture_native_printer_endpoints() {
    for (x, y, width, height, expected) in [
      (
        85.06773,
        76.725,
        379.53,
        244.4625,
        [85.1, 76.8, 379.55, 244.45],
      ),
      (
        423.72998,
        118.39331,
        127.25,
        127.3,
        [423.7, 118.45, 127.2, 127.3],
      ),
      (
        36.033672,
        120.64331,
        143.2,
        127.2,
        [36.0, 120.6, 143.15, 127.2],
      ),
      (
        423.72998,
        118.32,
        127.4,
        127.3,
        [423.7, 118.3, 127.45, 127.35],
      ),
    ] {
      let actual = inline_paint_bounds(common::Rect {
        origin: common::Point {
          x: common::Pt(x),
          y: common::Pt(y),
        },
        size: common::Size {
          width: common::Pt(width),
          height: common::Pt(height),
        },
      });
      assert_eq!(
        [
          actual.origin.x.0,
          actual.origin.y.0,
          actual.size.width.0,
          actual.size.height.0
        ],
        expected
      );
    }
  }
}
