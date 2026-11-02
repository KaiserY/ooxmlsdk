//! Word legacy page brushes use a fixed physical period, independent of the
//! paper and VML target-screen hint. Linear brushes remain vector gradients;
//! radial brushes use repeated raster tiles.

use image::{ImageEncoder, codecs::png::PngEncoder};

use super::{ImageCrop, InlineShapeImageFill, InlineShapeImageFillMode, common, o, v};
use crate::common::drawingml_gradient;

/// One physical period of Word's legacy page brush. Repetition is resolved
/// against each output page, so paper dimensions never stretch the brush.
#[derive(Clone, Debug)]
pub(crate) struct LinearGradient {
  gradient: common::GradientFill<'static>,
}

pub(super) fn linear_gradient(background: &super::w::DocumentBackground) -> Option<LinearGradient> {
  background.color.as_ref()?;
  let shape = background.background.as_deref()?;
  if !super::vml_background_fill_enabled(shape) {
    return None;
  }
  let fill = shape.fill.as_deref()?;
  if fill.r#type != Some(v::FillTypeValues::Gradient)
    || fill
      .fill_extended_properties
      .as_ref()
      .and_then(|fill| fill.r#type)
      .is_some()
  {
    return None;
  }
  let model = crate::xlsx::object_resources::vml_background_model(shape);
  let common::Fill::Gradient(mut gradient) =
    super::word_vml_shape_common_fill(&model, kurbo::Affine::IDENTITY)
  else {
    return None;
  };
  let focus = model
    .fill_focus
    .as_deref()
    .and_then(crate::xlsx::parse_vml_ratio)
    .unwrap_or(0.0);
  let authored_angle = model
    .fill_angle
    .as_deref()
    .and_then(|s| s.parse::<f32>().ok())
    .filter(|a| a.is_finite())
    .unwrap_or(0.0);
  let angle = gradient.angle_degrees.unwrap_or_default();
  let mut direction = angle.rem_euclid(180.0);
  if direction > 90.0 {
    direction -= 180.0;
  }
  let explicit =
    !crate::xlsx::vml_gradient_intermediate_stops(model.fill_colors.as_deref()).is_empty();
  if !explicit && (angle - direction).rem_euclid(360.0) > 90.0 {
    gradient.stops.reverse();
    for stop in &mut gradient.stops {
      stop.position = 1.0 - stop.position;
    }
  }
  if explicit && focus.abs() > 0.0 && focus.abs() < 1.0 {
    let mut stops = Vec::with_capacity(gradient.stops.len() * 2 - 1);
    for stop in &gradient.stops {
      let mut stop = stop.clone();
      stop.position *= 0.5;
      stops.push(stop);
    }
    for stop in gradient.stops.iter().rev().skip(1) {
      let mut stop = stop.clone();
      stop.position = 1.0 - stop.position * 0.5;
      stops.push(stop);
    }
    gradient.stops = stops;
  }
  // Word's horizontal preset uses the small brush; custom angled/axial
  // fills use the general 123x92.28pt period. Target-screen is a UI hint.
  let (width, height) =
    if authored_angle.rem_euclid(180.0) == 0.0 && (focus == 0.0 || focus.abs() == 1.0) {
      (61.44, 46.08)
    } else {
      (123.0, 92.28)
    };
  gradient.line = super::vml_gradient::transformed_line(
    Some(direction),
    kurbo::Affine::scale_non_uniform(width, height),
  );
  gradient.angle_degrees = Some(direction);
  for stop in &mut gradient.stops {
    stop.color.a = 255;
  }
  Some(LinearGradient { gradient })
}

impl LinearGradient {
  pub(super) fn for_page(&self, width: f32, height: f32) -> common::GradientFill<'static> {
    let mut gradient = self.gradient.clone();
    let (start, end) = gradient.line.expect("linear page brush");
    let dx = end.x.0 - start.x.0;
    let dy = end.y.0 - start.y.0;
    let length2 = dx * dx + dy * dy;
    let project = |x: f32, y: f32| ((x - start.x.0) * dx + (y - start.y.0) * dy) / length2;
    let positions = [
      project(0.0, 0.0),
      project(width, 0.0),
      project(0.0, height),
      project(width, height),
    ];
    let mut first = positions.into_iter().fold(f32::INFINITY, f32::min).floor() as i32;
    // Word's vertical page preset retains the preceding period. This also
    // preserves the native sampled-function spacing at the first boundary.
    if dx.abs() < f32::EPSILON && dy > 0.0 && first == 0 {
      first = -1;
    }
    let last = positions
      .into_iter()
      .fold(f32::NEG_INFINITY, f32::max)
      .ceil() as i32;
    let count = (last - first).clamp(1, 4096) as usize;
    let source = &self.gradient.stops;
    let uniform = source.len() > 1
      && source
        .iter()
        .enumerate()
        .all(|(i, s)| (s.position - i as f32 / (source.len() - 1) as f32).abs() < 1e-6);
    let mut stops = Vec::with_capacity(count * source.len());
    for repetition in 0..count {
      let reversed = (first + repetition as i32).rem_euclid(2) != 0;
      for i in 0..source.len() {
        let mut stop = source[if reversed { source.len() - 1 - i } else { i }].clone();
        stop.position = if uniform {
          // Sampled PDF functions retain both sides of a tile boundary.
          (repetition * source.len() + i) as f32 / (count * source.len() - 1) as f32
        } else {
          (repetition as f32
            + if reversed {
              1.0 - stop.position
            } else {
              stop.position
            })
            / count as f32
        };
        stops.push(stop);
      }
    }
    gradient.stops = stops;
    gradient.line = Some((
      common::Point {
        x: common::Pt(start.x.0 + dx * first as f32),
        y: common::Pt(start.y.0 + dy * first as f32),
      },
      common::Point {
        x: common::Pt(start.x.0 + dx * (first as f32 + count as f32)),
        y: common::Pt(start.y.0 + dy * (first as f32 + count as f32)),
      },
    ));
    gradient
  }
}

/// Word prints a page picture as a repeating bitmap at the 600-DPI reference
/// device, even though ordinary VML frame fills stretch inside their host.
/// Its pixel dimensions, not image density or VML size/aspect, own the period.
/// Explicit origin/position retain their image/host-space phase. Keep ordinary
/// tile fills and their natural-size policy separate.
pub(super) fn frame_picture_texture(
  fill: &v::Fill,
  images: &super::ImageCatalog,
) -> Option<InlineShapeImageFill> {
  if fill.r#type != Some(v::FillTypeValues::Frame) {
    return None;
  }
  let mut image = super::vml_fill_image(fill, fill.r#type, images)?;
  // Read only dimensions here. The PDF image policy owns decoding,
  // downsampling and compression once the physical tile is known.
  let (width, height) =
    crate::common::drawingml_image_tile::jpeg_dimensions(&image.data).or_else(|| {
      image::ImageReader::new(std::io::Cursor::new(&image.data))
        .with_guessed_format()
        .ok()?
        .into_dimensions()
        .ok()
    })?;
  let reference_pixel_pt = crate::units::POINTS_PER_INCH / 600.0;
  image.mode = InlineShapeImageFillMode::Tile {
    size: Some(format!(
      "{}pt,{}pt",
      width as f32 * reference_pixel_pt,
      height as f32 * reference_pixel_pt
    )),
    origin: fill.origin.clone(),
    position: fill.position.clone(),
  };
  Some(image)
}

pub(super) fn radial_gradient_texture(background: &v::Background) -> Option<InlineShapeImageFill> {
  let fill = background.fill.as_deref()?;
  // ECMA-376 Part 4 §19.2.2.13: o:fill overrides the parent v:fill type.
  let extended = fill
    .fill_extended_properties
    .as_ref()
    .and_then(|fill| fill.r#type);
  let corner = match extended {
    Some(o::FillValues::GradientCenter) => true,
    Some(o::FillValues::GradientRadial) => false,
    None if fill.r#type == Some(v::FillTypeValues::GradientRadial) => false,
    _ => return None,
  };
  let mut model = crate::xlsx::object_resources::vml_background_model(background);
  model.fill_type = Some(v::FillTypeValues::GradientRadial);
  let common::Fill::Gradient(mut gradient) =
    super::word_vml_shape_common_fill(&model, kurbo::Affine::IDENTITY)
  else {
    return None;
  };
  let focus = gradient.path?.fill_to;
  let explicit_stops =
    !crate::xlsx::vml_gradient_intermediate_stops(model.fill_colors.as_deref()).is_empty();
  if explicit_stops {
    // Native page-brush controls: an explicit color array ignores method,
    // but focus still reverses the progression. Shape gradients own a
    // separate policy; do not change their shared import here.
    gradient.interpolation = common::GradientInterpolation::LinearSrgb;
    if model
      .fill_focus
      .as_deref()
      .and_then(crate::xlsx::parse_vml_ratio)
      .unwrap_or(0.0)
      .abs()
      <= 0.5
    {
      gradient.stops.reverse();
      for stop in &mut gradient.stops {
        stop.position = 1.0 - stop.position;
      }
    }
  }
  let stops = if !explicit_stops && fill.method == Some(v::FillMethodValues::Linear) {
    // The raster page brush's explicit linear method is gamma-correct RGB,
    // whereas none is byte-linear. Resolve once, not with pow per pixel.
    let start = gradient.stops.first()?;
    let end = gradient.stops.last()?;
    (0..=256)
      .map(|step| {
        let t = step as f32 / 256.0;
        common::GradientStop {
          position: t,
          color: drawingml_gradient::gamma_correct_color(start.color, end.color, t),
          scheme: None,
        }
      })
      .collect()
  } else {
    drawingml_gradient::resolved_stops(&gradient)
  };
  // Word fixed-output controls across all target-screen values, screen/print
  // quality and doubled paper dimensions retain these brush grids. The
  // corner profile has an 81x61 raster over 61.44x46.08pt. The general radial
  // profile retains the far edge and uses 164x123 over 123x92.28pt. An exact
  // center focus selects the latter even with the gradientCenter extension.
  let centered = focus.left == 0.5 && focus.top == 0.5 && focus.right == 0.5 && focus.bottom == 0.5;
  let (width, height, tile_width, tile_height) = if corner && !centered {
    (81, 61, 61.44, 46.08)
  } else {
    (164, 123, 123.0, 92.28)
  };
  let xs: Vec<_> = (0..width)
    .map(|x| rectangle_axis_position(x as f32 / width as f32, focus.left, focus.right))
    .collect();
  let ys: Vec<_> = (0..height)
    .map(|y| rectangle_axis_position(y as f32 / height as f32, focus.top, focus.bottom))
    .collect();
  let mut rgb = Vec::with_capacity(width * height * 3);
  for y in ys {
    for &x in &xs {
      let color = drawingml_gradient::sample(&stops, x.max(y));
      // Page gradients print opaque; fill opacity does not create a shape
      // transparency group (native opacity controls).
      rgb.extend_from_slice(&[color.r, color.g, color.b]);
    }
  }
  let mut png = Vec::new();
  PngEncoder::new(&mut png)
    .write_image(
      &rgb,
      width as u32,
      height as u32,
      image::ExtendedColorType::Rgb8,
    )
    .ok()?;
  Some(InlineShapeImageFill {
    data: png.into(),
    content_type: Some("image/png".into()),
    blip_compression_state: common::BlipCompressionState::default(),
    crop: ImageCrop::default(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    rotate_with_shape: false,
    mode: InlineShapeImageFillMode::Tile {
      size: Some(format!("{tile_width}pt,{tile_height}pt")),
      origin: None,
      position: None,
    },
  })
}

fn rectangle_axis_position(value: f32, near: f32, far: f32) -> f32 {
  if value < near {
    (near - value) / near
  } else if value > 1.0 - far {
    (value - (1.0 - far)) / far
  } else {
    0.0
  }
  .clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::sdk::SdkType;

  fn background(extra: &str, fill: &str, extension: &str) -> super::super::w::DocumentBackground {
    super::super::w::DocumentBackground::from_bytes(
      format!(
        r##"
      <w:background xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
        xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"
        w:color="000082"><v:background fillcolor="#000082" {extra}>
        <v:fill color2="#ff8200" type="gradientRadial" {fill}>{extension}</v:fill>
      </v:background></w:background>"##
      )
      .as_bytes(),
    )
    .unwrap()
  }

  fn render(
    background: &super::super::w::DocumentBackground,
  ) -> (InlineShapeImageFill, image::RgbImage) {
    let texture =
      super::super::document_background_texture(background, &super::super::ImageCatalog::default())
        .unwrap();
    let image = image::load_from_memory(&texture.data).unwrap().to_rgb8();
    (texture, image)
  }

  const EXT: &str = r#"<o:fill type="gradientCenter"/>"#;

  #[test]
  fn frame_page_picture_uses_reference_pixels_and_ignores_shape_fill_geometry() {
    for (width, height) in [(8, 8), (128, 128), (1024, 512)] {
      let mut bytes = Vec::new();
      PngEncoder::new(&mut bytes)
        .write_image(
          &vec![128; (width * height * 3) as usize],
          width,
          height,
          image::ExtendedColorType::Rgb8,
        )
        .unwrap();
      let mut images = super::super::ImageCatalog::default();
      images.by_relationship_id.insert(
        "picture".into(),
        super::super::package::ImageResource {
          data: bytes.into(),
          content_type: Some("image/png".into()),
        },
      );
      for extra in [
        "",
        r#"size="60pt,40pt" aspect="atMost" origin=".5,.5" position=".5,.5" opacity="50%""#,
      ] {
        let source=super::super::w::DocumentBackground::from_bytes(format!(r#"
          <w:background xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:v="urn:schemas-microsoft-com:vml"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" w:color="FFFFFF">
          <v:background><v:fill type="frame" r:id="picture" {extra}/></v:background></w:background>"#).as_bytes()).unwrap();
        let texture = super::super::document_background_texture(&source, &images).unwrap();
        let InlineShapeImageFillMode::Tile {
          size,
          origin,
          position,
        } = texture.mode
        else {
          panic!("frame page picture must repeat");
        };
        let (w, h) = super::super::vml_point_pair(size.as_deref().unwrap()).unwrap();
        assert!((w - width as f32 * 0.12).abs() < 0.0001);
        assert!((h - height as f32 * 0.12).abs() < 0.0001);
        let phase = (!extra.is_empty()).then_some(".5,.5");
        assert_eq!(origin.as_deref(), phase);
        assert_eq!(position.as_deref(), phase);
        assert_eq!(texture.data, images.by_relationship_id["picture"].data);
      }
    }
  }

  #[test]
  fn linear_page_brush_repeats_without_stretching_and_keeps_activation() {
    let source = |extra: &str, fill: &str| {
      super::super::w::DocumentBackground::from_bytes(format!(r##"
      <w:background xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
        xmlns:v="urn:schemas-microsoft-com:vml" w:color="FF00FF">
        <v:background fillcolor="fuchsia" {extra}><v:fill type="gradient" color2="lime" {fill}/></v:background>
      </w:background>"##).as_bytes()).unwrap()
    };
    let background = source("", r#"angle="225" focus="50%""#);
    let brush = linear_gradient(&background).unwrap();
    for (width, height, count) in [
      (419.52, 297.0, 4),
      (839.16, 594.0, 7),
      (297.0, 419.52, 4),
      (297.0, 297.0, 3),
    ] {
      let gradient = brush.for_page(width, height);
      assert_eq!(gradient.stops.len(), count * 511);
      let (start, end) = gradient.line.unwrap();
      assert!((start.x.0 - 17.201569).abs() < 0.001);
      assert!((start.y.0 + 12.905373).abs() < 0.001);
      assert!((end.x.0 - start.x.0 - count as f32 * 88.59686).abs() < 0.001);
      for index in [0, 510, 511] {
        assert_eq!(
          gradient.stops[index].color,
          common::Color {
            r: 255,
            g: 0,
            b: 255,
            a: 255
          }
        );
      }
      assert_eq!(
        gradient.stops[255].color,
        common::Color {
          r: 0,
          g: 255,
          b: 0,
          a: 255
        }
      );
    }
    let vertical = linear_gradient(&source("", r#"angle="0" focus="50%""#))
      .unwrap()
      .for_page(419.52, 297.0);
    assert_eq!(vertical.stops.len(), 2555);
    assert!((vertical.line.unwrap().0.y.0 + 92.28).abs() < 0.001);
    for (extra, fill) in [
      (r#"fill="f""#, r#"angle="225" focus="50%""#),
      ("", r#"on="f" angle="225" focus="50%""#),
    ] {
      assert!(linear_gradient(&source(extra, fill)).is_none());
    }
    let mut absent = background;
    absent.color = None;
    assert!(linear_gradient(&absent).is_none());
  }

  #[test]
  fn radial_page_gradient_uses_a_fixed_tile_and_honors_activation() {
    for screen in [
      "",
      r#"o:targetscreensize="544,376""#,
      r#"o:targetscreensize="1152,862""#,
    ] {
      let mut bg = background(screen, r#"focus="100%" method="none""#, EXT);
      let (texture, image) = render(&bg);
      assert_eq!(image.dimensions(), (81, 61));
      assert!(
        matches!(texture.mode, InlineShapeImageFillMode::Tile{size:Some(ref size),..} if size=="61.44pt,46.08pt")
      );
      assert_eq!(image.get_pixel(0, 0).0, [0, 0, 130]);
      // Native independent endpoint control midpoint:126,64,66.
      assert_eq!(image.get_pixel(40, 0).0, [126, 64, 66]);
      bg.color = None;
      assert!(
        super::super::document_background_texture(&bg, &super::super::ImageCatalog::default())
          .is_none()
      );
    }
    for (host, fill) in [(r#"fill="f""#, ""), ("", r#"on="f""#)] {
      let bg = background(host, fill, EXT);
      assert!(
        super::super::document_background_texture(&bg, &super::super::ImageCatalog::default())
          .is_none()
      );
    }
  }

  #[test]
  fn radial_page_gradient_keeps_corner_focus_and_extended_type_ownership() {
    for (position, corner) in [
      ("0,0", [0, 0, 130]),
      ("1,0", [255, 130, 0]),
      ("0,1", [255, 130, 0]),
      ("1,1", [255, 130, 0]),
    ] {
      let bg = background(
        "",
        &format!(r#"focus="100%" method="none" focusposition="{position}""#),
        EXT,
      );
      let (_, image) = render(&bg);
      assert_eq!(image.get_pixel(0, 0).0, corner);
    }
    for (position, extension) in [("0,0", ""), (".5,.5", EXT)] {
      let bg = background(
        "",
        &format!(r#"focus="100%" method="none" focusposition="{position}""#),
        extension,
      );
      let (texture, image) = render(&bg);
      assert_eq!(image.dimensions(), (164, 123));
      assert!(
        matches!(texture.mode,InlineShapeImageFillMode::Tile{size:Some(ref size),..} if size=="123pt,92.28pt")
      );
    }
    let mut bg = background("", r#"focus="100%" method="none""#, EXT);
    bg.background
      .as_mut()
      .unwrap()
      .fill
      .as_mut()
      .unwrap()
      .r#type = Some(v::FillTypeValues::Gradient);
    assert_eq!(render(&bg).1.dimensions(), (81, 61));
  }

  #[test]
  fn radial_page_gradient_explicit_stops_own_interpolation_and_focus_reversal() {
    let attrs = r##"colors="0 #204060;.25 #80A0C0;1 #C0A040" focus="100%""##;
    let (_, base) = render(&background("", &format!(r#"{attrs} method="none""#), EXT));
    for method in ["linear", "sigma", "any"] {
      let (_, actual) = render(&background(
        "",
        &format!(r#"{attrs} method="{method}" opacity="50%""#),
        EXT,
      ));
      assert_eq!(actual, base);
    }
    assert_eq!(base.get_pixel(0, 0).0, [32, 64, 96]);
    let (_, reversed) = render(&background("", &attrs.replace("100%", "0%"), EXT));
    assert_eq!(reversed.get_pixel(0, 0).0, [192, 160, 64]);
  }
}
