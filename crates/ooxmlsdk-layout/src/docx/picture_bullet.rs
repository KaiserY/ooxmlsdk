//! Word picture-numbering autosizing, measured in configured PDF/EMF controls.
use std::io::Cursor;

/// MS-DOC PICF owns the natural bitmap dimensions in integral twips. Word
/// picture-numbering controls use that height as the autosizing reference,
/// retaining the authored shape's scale and aspect ratio.
pub(super) fn intrinsic_height_pt(data: &[u8]) -> Option<f64> {
  let reader = image::ImageReader::new(Cursor::new(data))
    .with_guessed_format()
    .ok()?;
  match reader.format()? {
    image::ImageFormat::Png => {
      if let Some((_, height)) = super::vml_picture::png_goal_twips(data) {
        return Some(height / 20.0);
      }
    }
    image::ImageFormat::Gif => {}
    // Other picture producers retain their existing size owner until their
    // physical-density contract has been established independently.
    _ => return None,
  }
  let (_, height) = reader.into_dimensions().ok()?;
  (height > 0).then_some(f64::from(height) * 72.0 / 96.0)
}

pub(super) fn scale(authored_height: f32, intrinsic_height: Option<f64>, font_size: f32) -> f64 {
  // Native font2/3/4 and font9..24 controls establish the two-point minimum
  // and internal leading. Font/run/mark controls distinguish label font
  // ownership from the first visible body run.
  let cell = (f64::from(font_size) - 2.0).max(2.0);
  cell / intrinsic_height.unwrap_or(f64::from(authored_height))
}

/// Bitmap DrawImagePoints already exports absolute endpoints in twips;
/// it does not use the font's later 3000-DPI replay transform.
pub(super) fn paint_axis(first_source: i64, extent_source: i64) -> (f32, f32) {
  let first = (first_source as f64 * 2.4).round();
  let last = ((first_source + extent_source) as f64 * 2.4).round();
  ((first / 20.0) as f32, ((last - first) / 20.0) as f32)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_picture_bullet_extents_keep_authored_scale_and_bitmap_density() {
    // Configured Word PDFs: independent label fonts, shape heights and GIF
    // pixel heights/PNG densities. Expectations are observed printer dots.
    for (font, authored, intrinsic, expected) in [
      (2.0, 13.5, 13.5, 17),
      (3.0, 13.5, 13.5, 17),
      (9.0, 13.5, 13.5, 58),
      (10.0, 13.5, 13.5, 67),
      (12.0, 13.5, 13.5, 83),
      (18.0, 13.5, 13.5, 133),
      (24.0, 13.5, 13.5, 183),
      (12.0, 6.0, 13.5, 37),
      (12.0, 36.0, 13.5, 222),
      (12.0, 13.5, 27.0, 42),
      (12.0, 13.5, 54.0, 21),
      (12.0, 13.5, 18.0, 63),
      (12.0, 13.5, 9.0, 125),
    ] {
      let extent = (f64::from(authored) * scale(authored, Some(intrinsic), font) / 0.12).round();
      assert_eq!(extent as i64, expected);
    }
  }

  #[test]
  fn native_picture_bullet_bitmap_endpoints_are_absolute_twips() {
    // Actual configured PDF EnumEnhMetaFile callback, original/6pt/font9.
    assert_eq!(paint_axis(622, 83), (74.65, 9.95));
    assert_eq!(paint_axis(482, 83), (57.85, 9.95));
    assert_eq!(paint_axis(528, 37), (63.35, 4.45));
    assert_eq!(paint_axis(485, 58), (58.2, 6.95));
    assert_eq!(paint_axis(622, 67), (74.65, 8.05));
  }
}
