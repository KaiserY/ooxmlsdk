//! Word's inline-picture BRCs, separate from the VML drawing outline.
use super::{BorderStyle, ImagePlacement, InlineImage, ParagraphBordersModel, RgbColor, v, w, w10};

fn style(
  kind: Option<w10::BorderValues>,
  width: Option<i64>,
  shadow: bool,
  color: Option<&str>,
) -> Option<BorderStyle> {
  use w::BorderValues as W;
  use w10::BorderValues as V;
  let kind = match kind? {
    V::None => return None,
    V::Single => W::Single,
    V::Thick => W::Thick,
    V::Double => W::Double,
    V::Hairline => W::Single,
    V::Dot => W::Dotted,
    V::Dash => W::Dashed,
    V::DotDash => W::DotDash,
    V::DashDotDot => W::DotDotDash,
    V::Triple => W::Triple,
    V::ThinThickSmall => W::ThinThickSmallGap,
    V::ThickThinSmall => W::ThickThinSmallGap,
    V::ThickBetweenThinSmall => W::ThinThickThinSmallGap,
    V::ThinThick => W::ThinThickMediumGap,
    V::ThickThin => W::ThickThinMediumGap,
    V::ThickBetweenThin => W::ThinThickThinMediumGap,
    V::ThinThickLarge => W::ThinThickLargeGap,
    V::ThickThinLarge => W::ThickThinLargeGap,
    V::ThickBetweenThinLarge => W::ThinThickThinLargeGap,
    V::Wave => W::Wave,
    V::DoubleWave => W::DoubleWave,
    V::DashedSmall => W::DashSmallGap,
    V::DashDotStroked => W::DashDotStroked,
    V::ThreeDEmboss => W::ThreeDEmboss,
    V::ThreeDEngrave => W::ThreeDEngrave,
    V::HtmlOutset => W::Outset,
    V::HtmlInset => W::Inset,
  };
  // CT_Border stores eighth-points. Actual native omitted-width controls
  // retain the one-dot BRC default, independently of w:pBdr's default width.
  let width = width
    .and_then(|value| u32::try_from(value).ok())
    .unwrap_or(1);
  let mut border = super::border_style(kind, Some(width.max(1)), None, None, None)?;
  border.shadow = shadow;
  border.color = color.and_then(super::parse_vml_color).unwrap_or_default();
  Some(border)
}

pub(super) fn device_bands(border: BorderStyle) -> [i64; 5] {
  super::border_bands::device_widths(border).unwrap_or_else(|| {
    let twips = (f64::from(border.width_pt) * 20.0).floor() as i64;
    [(twips * 5 / 12).max(1), 0, 0, 0, 0]
  })
}

pub(super) fn band_colors(border: BorderStyle) -> [Option<RgbColor>; 5] {
  if super::border_3d::band_widths(border).is_some() {
    let [a, b, c] = super::border_3d::band_colors(border);
    [Some(a), Some(b), Some(c), None, None]
  } else {
    [
      Some(border.color),
      None,
      Some(border.color),
      None,
      Some(border.color),
    ]
  }
}

pub(super) fn physical_extent(border: Option<BorderStyle>) -> f32 {
  border.map_or(0.0, |border| {
    let dots: i64 = device_bands(border).into_iter().sum();
    let twips = (dots as f64 * 2.4 * if border.shadow { 2.0 } else { 1.0 }).round();
    twips as f32 / 20.0
  })
}

pub(super) fn layout_height_adjustment(borders: &ParagraphBordersModel) -> f32 {
  [borders.top, borders.bottom]
    .into_iter()
    .map(|border| {
      let logical = border.map_or(0.0, |border| {
        let width = super::border_3d::band_widths(border).map_or_else(
          || {
            super::border_bands::widths(border)
              .map_or(border.width_pt, |bands| bands.into_iter().sum())
          },
          |bands| bands.into_iter().sum(),
        );
        width * if border.shadow { 2.0 } else { 1.0 }
      });
      logical - physical_extent(border)
    })
    .sum()
}

fn attach(image: &mut InlineImage, borders: ParagraphBordersModel) {
  if borders.is_empty() || !matches!(image.placement, ImagePlacement::Inline) {
    // Native absolute-floating counterparts in modes12/15 ignore w10 BRCs.
    // Their independent v:stroke and standard picture template still apply.
    return;
  }
  image.effect_left_pt += physical_extent(borders.left);
  image.effect_top_pt += physical_extent(borders.top);
  image.effect_right_pt += physical_extent(borders.right);
  image.effect_bottom_pt += physical_extent(borders.bottom);
  image.vml_picture_borders = Some(Box::new(borders));
}

macro_rules! importer {
  ($name:ident, $host:ident, $choices:ident, $choice:ident) => {
    pub(super) fn $name(image: &mut InlineImage, host: &v::$host) {
      let mut borders = ParagraphBordersModel::default();
      for choice in &host.$choices {
        match choice {
          v::$choice::TopBorder(border) => {
            borders.top = style(
              border.r#type,
              border.width,
              border.shadow.is_some_and(|v| v.as_bool()),
              host.border_top_color.as_deref(),
            );
          }
          v::$choice::RightBorder(border) => {
            borders.right = style(
              border.r#type,
              border.width,
              border.shadow.is_some_and(|v| v.as_bool()),
              host.border_right_color.as_deref(),
            );
          }
          v::$choice::BottomBorder(border) => {
            borders.bottom = style(
              border.r#type,
              border.width,
              border.shadow.is_some_and(|v| v.as_bool()),
              host.border_bottom_color.as_deref(),
            );
          }
          v::$choice::LeftBorder(border) => {
            borders.left = style(
              border.r#type,
              border.width,
              border.shadow.is_some_and(|v| v.as_bool()),
              host.border_left_color.as_deref(),
            );
          }
          _ => {}
        }
      }
      attach(image, borders);
    }
  };
}

importer!(apply_shape, Shape, shape_choice, ShapeChoice);
importer!(
  apply_image_file,
  ImageFile,
  image_file_choice,
  ImageFileChoice
);
importer!(
  apply_rectangle,
  Rectangle,
  rectangle_choice,
  RectangleChoice
);
importer!(
  apply_round_rectangle,
  RoundRectangle,
  round_rectangle_choice,
  RoundRectangleChoice
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vml_picture_border_source_retains_each_side_color_style_shadow_and_flow() {
    let border_xml = "<w10:bordertop type=\"double\" width=\"6\"/><w10:borderleft type=\"single\" width=\"12\"/><w10:borderbottom type=\"single\" width=\"18\" shadow=\"t\"/><w10:borderright type=\"none\" width=\"24\"/>";
    let image = super::super::vml_picture::frame_test_image(
      0.0,
      &[
        (
          "xmlns:o=",
          "xmlns:w10=\"urn:schemas-microsoft-com:office:word\" xmlns:o=",
        ),
        ("position:absolute;", ""),
        (";mso-position-horizontal:left", ""),
        (
          "stroked=\"t\"",
          "stroked=\"f\" o:bordertopcolor=\"red\" o:borderleftcolor=\"blue\"",
        ),
        ("</v:shape>", &format!("{border_xml}</v:shape>")),
      ],
    );
    let borders = image.vml_picture_borders.as_deref().unwrap();
    assert_eq!(borders.top.unwrap().color, RgbColor { r: 255, g: 0, b: 0 });
    assert_eq!(borders.left.unwrap().color, RgbColor { r: 0, g: 0, b: 255 });
    assert_eq!(device_bands(borders.top.unwrap()), [6, 6, 6, 0, 0]);
    assert!(borders.bottom.unwrap().shadow);
    assert!(borders.right.is_none());
    assert_eq!(
      [
        image.effect_left_pt,
        image.effect_top_pt,
        image.effect_right_pt,
        image.effect_bottom_pt
      ],
      [1.45, 2.15, 0.0, 4.3]
    );
  }

  #[test]
  fn vml_picture_border_omission_none_and_native_floating_opposites_are_inert() {
    let xml = "<w10:bordertop type=\"single\" width=\"6\"/>";
    let image = super::super::vml_picture::frame_test_image(
      0.0,
      &[
        (
          "xmlns:o=",
          "xmlns:w10=\"urn:schemas-microsoft-com:office:word\" xmlns:o=",
        ),
        ("stroked=\"t\"", "stroked=\"f\""),
        ("</v:shape>", &format!("{xml}</v:shape>")),
      ],
    );
    assert!(image.vml_picture_borders.is_none());
    assert_eq!(image.effect_top_pt, 0.0);
    assert!(style(None, Some(6), false, None).is_none());
    assert!(style(Some(w10::BorderValues::None), Some(18), false, None).is_none());
    let default = style(Some(w10::BorderValues::Single), None, false, None).unwrap();
    assert_eq!(device_bands(default), [1, 0, 0, 0, 0]);
  }
}
