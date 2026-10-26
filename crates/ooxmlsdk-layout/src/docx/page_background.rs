//! Word's legacy radial page brush is a repeated raster, independent of the
//! paper and the VML target-screen hint. Keep this export owner separate from
//! ordinary shape gradients, which have different interpolation defaults.

use image::{ImageEncoder, codecs::png::PngEncoder};

use super::{ImageCrop, InlineShapeImageFill, InlineShapeImageFillMode, common, o, v};
use crate::common::drawingml_gradient;

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
