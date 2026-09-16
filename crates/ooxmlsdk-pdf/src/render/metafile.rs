use super::paint::{ImageCrop, ImageItem};
use crate::options::{PdfDocumentKind, PdfImageOptimizationPolicy, PdfOptimizeFor, PdfOptions};
use ooxmlsdk_layout::{common, units};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FixedOutputRasterAllocation {
  FloorExtent,
  PowerPointScreenEndpoints,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MetafilePlaybackRectangle {
  Canvas,
  PowerPointScreenEndpoints,
  ExcelVmlPicture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MetafileFixedOutputRasterProfile {
  raster_dpi: u32,
  allocation: FixedOutputRasterAllocation,
  playback_rectangle: MetafilePlaybackRectangle,
}

// The configured Office reference environment uses Windows ClearType
// contrast 1200. GDI applies SPI_GETFONTSMOOTHINGCONTRAST / 1000 as the
// device-space gamma when classic EMF/WMF text is replayed into a color DIB.
const OFFICE_REFERENCE_GDI_FONT_SMOOTHING_CONTRAST: u16 = 1200;

impl MetafileFixedOutputRasterProfile {
  fn playback_size(self, canvas_width: u32, canvas_height: u32) -> (u32, u32) {
    match self.playback_rectangle {
      MetafilePlaybackRectangle::Canvas => (canvas_width, canvas_height),
      MetafilePlaybackRectangle::PowerPointScreenEndpoints => (
        canvas_width.saturating_sub(1).max(1),
        canvas_height.saturating_sub(1).max(1),
      ),
      MetafilePlaybackRectangle::ExcelVmlPicture => (canvas_width, canvas_height),
    }
  }

  fn text_playback_size(self, canvas_width: u32, canvas_height: u32) -> (u32, u32) {
    let playback = self.playback_size(canvas_width, canvas_height);
    match self.playback_rectangle {
      // Excel allocates N color pixels but exposes the inclusive right
      // endpoint to the GDI font mapper for VML pictures.
      MetafilePlaybackRectangle::ExcelVmlPicture => (canvas_width.saturating_add(1), playback.1),
      _ => playback,
    }
  }

  fn monochrome_text_playback_size(self, canvas_width: u32, canvas_height: u32) -> (u32, u32) {
    match self.playback_rectangle {
      // The one-bit transparency surface retains the ordinary destination
      // rectangle even when the Excel color plane uses the inclusive axis.
      MetafilePlaybackRectangle::ExcelVmlPicture => self.playback_size(canvas_width, canvas_height),
      _ => self.text_playback_size(canvas_width, canvas_height),
    }
  }
}

fn fixed_output_raster_pixels(
  points: f32,
  visible_fraction: f32,
  raster_dpi: u32,
  allocation: FixedOutputRasterAllocation,
) -> u32 {
  let print_dots =
    (points.max(0.0) / visible_fraction) * units::OFFICE_FIXED_OUTPUT_DPI / units::POINTS_PER_INCH;
  let nearest_print_dot = print_dots.round();
  let print_grid_slack = f32::EPSILON * print_dots.abs().max(1.0) * 8.0;
  let print_dots = if (print_dots - nearest_print_dot).abs() <= print_grid_slack {
    nearest_print_dot
  } else {
    print_dots
  };
  let raster_pixels = print_dots * raster_dpi as f32 / units::OFFICE_FIXED_OUTPUT_DPI;
  let raster_pixels = match allocation {
    FixedOutputRasterAllocation::FloorExtent => raster_pixels.floor(),
    FixedOutputRasterAllocation::PowerPointScreenEndpoints => raster_pixels.ceil() - 1.0,
  };
  raster_pixels.clamp(1.0, u32::MAX as f32) as u32
}

fn fixed_output_raster_profile(
  image: &ImageItem<'_>,
  options: &PdfOptions,
) -> MetafileFixedOutputRasterProfile {
  if matches!(
    options.images.optimization_policy,
    PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Xlsx)
  ) && options.optimize_for == PdfOptimizeFor::Screen
    && image.metafile_fixed_output_profile == common::MetafileFixedOutputProfile::ExcelVmlPicture
  {
    return MetafileFixedOutputRasterProfile {
      raster_dpi: units::CSS_PIXELS_PER_INCH as u32,
      allocation: FixedOutputRasterAllocation::FloorExtent,
      playback_rectangle: MetafilePlaybackRectangle::ExcelVmlPicture,
    };
  }

  match options.images.optimization_policy {
    PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Xlsx) => {
      MetafileFixedOutputRasterProfile {
        raster_dpi: units::OFFICE_FIXED_OUTPUT_RASTER_DPI as u32,
        allocation: FixedOutputRasterAllocation::FloorExtent,
        playback_rectangle: MetafilePlaybackRectangle::Canvas,
      }
    }
    PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(_) => match options.optimize_for {
      PdfOptimizeFor::Print => MetafileFixedOutputRasterProfile {
        raster_dpi: units::OFFICE_FIXED_OUTPUT_RASTER_DPI as u32,
        allocation: FixedOutputRasterAllocation::FloorExtent,
        playback_rectangle: MetafilePlaybackRectangle::Canvas,
      },
      PdfOptimizeFor::Screen => MetafileFixedOutputRasterProfile {
        raster_dpi: units::CSS_PIXELS_PER_INCH as u32,
        allocation: FixedOutputRasterAllocation::PowerPointScreenEndpoints,
        playback_rectangle: MetafilePlaybackRectangle::PowerPointScreenEndpoints,
      },
    },
    PdfImageOptimizationPolicy::Requested if options.images.reduce_resolution => {
      MetafileFixedOutputRasterProfile {
        raster_dpi: options
          .images
          .max_resolution_dpi
          .unwrap_or(units::OFFICE_FIXED_OUTPUT_RASTER_DPI as u32)
          .clamp(72, units::OFFICE_FIXED_OUTPUT_RASTER_DPI as u32),
        allocation: FixedOutputRasterAllocation::FloorExtent,
        playback_rectangle: MetafilePlaybackRectangle::Canvas,
      }
    }
    PdfImageOptimizationPolicy::Requested => MetafileFixedOutputRasterProfile {
      raster_dpi: units::OFFICE_FIXED_OUTPUT_RASTER_DPI as u32,
      allocation: FixedOutputRasterAllocation::FloorExtent,
      playback_rectangle: MetafilePlaybackRectangle::Canvas,
    },
  }
}

pub(super) fn render_options_for_image(
  image: &ImageItem<'_>,
  options: &PdfOptions,
) -> ooxmlsdk_layout::render::emf_wmf::RenderOptions {
  let raster_profile = fixed_output_raster_profile(image, options);
  let raster_dpi = raster_profile.raster_dpi;
  let powerpoint_screen_fixed_output = matches!(
    options.images.optimization_policy,
    PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx)
  ) && options.optimize_for == PdfOptimizeFor::Screen;
  let visible_width = (1.0 - image.crop.left - image.crop.right).max(f32::EPSILON);
  let visible_height = (1.0 - image.crop.top - image.crop.bottom).max(f32::EPSILON);
  let target_size = if image.metafile_semantic_text_includes_raster_backdrop {
    None
  } else {
    Some((
      fixed_output_raster_pixels(
        image.width_pt,
        visible_width,
        raster_dpi,
        raster_profile.allocation,
      ),
      fixed_output_raster_pixels(
        image.height_pt,
        visible_height,
        raster_dpi,
        raster_profile.allocation,
      ),
    ))
  };
  let target_pixels = target_size
    .map(|(width, height)| width.saturating_mul(height))
    .unwrap_or_default();
  let raster_budget_pixels = raster_dpi
    .saturating_mul(raster_dpi)
    .saturating_mul(64)
    .max(target_pixels)
    .min(16_000_000);
  let transparent_background = powerpoint_screen_fixed_output
    || image.metafile_background_color.is_some()
    || image.metafile_semantic_text_includes_raster_backdrop
    || (image.semantic_metafile_text
      && ooxmlsdk_layout::render::emf_wmf::metafile_text_requires_raster_backdrop(
        &image.data,
        image.content_type.as_deref(),
      ));
  let playback_size =
    target_size.map(|(width, height)| raster_profile.playback_size(width, height));
  let text_playback_size =
    target_size.map(|(width, height)| raster_profile.text_playback_size(width, height));
  let monochrome_text_playback_size =
    target_size.map(|(width, height)| raster_profile.monochrome_text_playback_size(width, height));

  ooxmlsdk_layout::render::emf_wmf::RenderOptions {
    target_width_px: target_size.map(|size| size.0),
    target_height_px: target_size.map(|size| size.1),
    max_pixels: Some(raster_budget_pixels),
    font_smoothing_contrast: matches!(
      options.images.optimization_policy,
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(_)
    )
    .then_some(OFFICE_REFERENCE_GDI_FONT_SMOOTHING_CONTRAST),
    playback_width_px: playback_size.map(|size| size.0),
    playback_height_px: playback_size.map(|size| size.1),
    text_playback_width_px: text_playback_size.map(|size| size.0),
    text_playback_height_px: text_playback_size.map(|size| size.1),
    monochrome_text_playback_width_px: monochrome_text_playback_size.map(|size| size.0),
    monochrome_text_playback_height_px: monochrome_text_playback_size.map(|size| size.1),
    transparent_background,
    background_color: None,
    monochrome_dib_palette_override: image.metafile_monochrome_dib_palette_override,
    filter_high_frequency_pattern_brushes: matches!(
      options.images.optimization_policy,
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Xlsx)
    ) || image
      .metafile_semantic_text_includes_raster_backdrop
      || !matches!(
        options.images.optimization_policy,
        PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(_)
      )
      || options.optimize_for != PdfOptimizeFor::Screen,
    suppress_text: image.metafile_semantic_text_includes_raster_backdrop,
    suppress_solid_pattern_rects: image.metafile_semantic_text_includes_raster_backdrop,
    suppress_bitmap_layers: image.metafile_semantic_text_includes_raster_backdrop,
    wmf_external_header: image.metafile_external_header,
  }
}

pub(super) fn is_metafile_image(image: &ImageItem<'_>) -> bool {
  image.content_type.as_deref().is_some_and(|content_type| {
    let content_type = content_type.to_ascii_lowercase();
    content_type.contains("emf") || content_type.contains("wmf")
  }) || image.metafile_monochrome_dib_palette_override.is_some()
    || image.metafile_background_color.is_some()
    || image.metafile_external_header.is_some()
    || image.semantic_metafile_text
    || image.metafile_semantic_text_includes_raster_backdrop
    || ooxmlsdk_layout::render::emf_wmf::metafile_physical_size(
      &image.data,
      image.content_type.as_deref(),
    )
    .is_some()
}

pub(super) fn native_paint_size(image: &ImageItem<'_>) -> Option<(f32, f32)> {
  if !image.metafile_native_size
    || image.metafile_background_color.is_some()
    || image.crop != ImageCrop::default()
  {
    return None;
  }
  let physical = ooxmlsdk_layout::render::emf_wmf::metafile_physical_size(
    &image.data,
    image.content_type.as_deref(),
  )?;
  let width_pixel_pt = physical.width_pt / physical.natural_width_px.max(1) as f32;
  let height_pixel_pt = physical.height_pt / physical.natural_height_px.max(1) as f32;
  ((image.width_pt - physical.width_pt).abs() <= width_pixel_pt
    && (image.height_pt - physical.height_pt).abs() <= height_pixel_pt)
    .then_some((physical.width_pt, physical.height_pt))
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;

  use super::*;

  fn test_image() -> ImageItem<'static> {
    ImageItem {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: 72.0,
      height_pt: 36.0,
      crop: ImageCrop {
        left: 0.25,
        right: 0.25,
        ..ImageCrop::default()
      },
      clip_path: &[],
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      data: Cow::Borrowed(&[]),
      content_type: Some(Cow::Borrowed("image/emf")),
      blip_compression_state: common::BlipCompressionState::Unspecified,
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      metafile_external_header: None,
      metafile_fixed_output_profile: common::MetafileFixedOutputProfile::Default,
      alt_text: None,
      hyperlink_url: None,
      semantic_metafile_text: false,
      metafile_semantic_text_includes_raster_backdrop: false,
      signature_line: None,
      metafile_native_size: true,
    }
  }

  #[test]
  fn metafile_raster_size_preserves_every_office_fixed_output_profile() {
    let image = test_image();
    let fixed_output = render_options_for_image(&image, &PdfOptions::default());
    assert_eq!(fixed_output.target_width_px, Some(400));
    assert_eq!(fixed_output.target_height_px, Some(100));
    assert_eq!(fixed_output.playback_width_px, Some(400));
    assert_eq!(fixed_output.playback_height_px, Some(100));
    assert_eq!(fixed_output.font_smoothing_contrast, None);
    assert!(!fixed_output.transparent_background);

    let mut options = PdfOptions::default();
    options.images.reduce_resolution = true;
    let reduced = render_options_for_image(&image, &options);
    assert_eq!(reduced.target_width_px, Some(400));
    assert_eq!(reduced.target_height_px, Some(100));

    options.images.max_resolution_dpi = Some(96);
    let low_ceiling = render_options_for_image(&image, &options);
    assert_eq!(low_ceiling.target_width_px, Some(192));
    assert_eq!(low_ceiling.target_height_px, Some(48));
    assert_eq!(low_ceiling.playback_width_px, Some(192));
    assert_eq!(low_ceiling.playback_height_px, Some(48));

    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx);
    options.optimize_for = PdfOptimizeFor::Screen;
    let powerpoint_screen = render_options_for_image(&image, &options);
    assert_eq!(powerpoint_screen.target_width_px, Some(191));
    assert_eq!(powerpoint_screen.target_height_px, Some(47));
    assert_eq!(powerpoint_screen.playback_width_px, Some(190));
    assert_eq!(powerpoint_screen.playback_height_px, Some(46));
    assert_eq!(powerpoint_screen.font_smoothing_contrast, Some(1200));
    assert!(powerpoint_screen.transparent_background);
    assert!(!powerpoint_screen.filter_high_frequency_pattern_brushes);

    let mut full_slide = image.clone();
    full_slide.width_pt = 720.0;
    full_slide.height_pt = 540.0;
    full_slide.crop = ImageCrop::default();
    let full_slide = render_options_for_image(&full_slide, &options);
    assert_eq!(full_slide.target_width_px, Some(959));
    assert_eq!(full_slide.target_height_px, Some(719));
    assert_eq!(full_slide.playback_width_px, Some(958));
    assert_eq!(full_slide.playback_height_px, Some(718));

    options.optimize_for = PdfOptimizeFor::Print;
    let powerpoint_print = render_options_for_image(&image, &options);
    assert_eq!(powerpoint_print.target_width_px, Some(400));
    assert_eq!(powerpoint_print.target_height_px, Some(100));
    assert_eq!(powerpoint_print.font_smoothing_contrast, Some(1200));
    assert!(!powerpoint_print.transparent_background);
    assert!(powerpoint_print.filter_high_frequency_pattern_brushes);

    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Xlsx);
    options.optimize_for = PdfOptimizeFor::Screen;
    let excel_screen = render_options_for_image(&image, &options);
    assert_eq!(excel_screen.target_width_px, Some(400));
    assert_eq!(excel_screen.target_height_px, Some(100));
    assert_eq!(excel_screen.playback_width_px, Some(400));
    assert_eq!(excel_screen.playback_height_px, Some(100));

    let mut excel_vml = image.clone();
    excel_vml.metafile_fixed_output_profile = common::MetafileFixedOutputProfile::ExcelVmlPicture;
    let excel_vml = render_options_for_image(&excel_vml, &options);
    assert_eq!(excel_vml.target_width_px, Some(192));
    assert_eq!(excel_vml.target_height_px, Some(48));
    assert_eq!(excel_vml.text_playback_width_px, Some(193));
    assert_eq!(excel_vml.monochrome_text_playback_width_px, Some(192));

    let mut vml = image;
    vml.width_pt = 77.25;
    vml.height_pt = 49.5;
    vml.crop = ImageCrop::default();
    vml.metafile_background_color = Some([255, 0, 0]);
    let vml_options = render_options_for_image(&vml, &PdfOptions::default());
    assert_eq!(vml_options.target_width_px, Some(214));
    assert_eq!(vml_options.target_height_px, Some(137));
    assert!(vml_options.transparent_background);

    vml.width_pt = 14.76;
    vml.height_pt = 26.76;
    let printer_grid = render_options_for_image(&vml, &PdfOptions::default());
    assert_eq!(printer_grid.target_width_px, Some(41));
    assert_eq!(printer_grid.target_height_px, Some(74));

    vml.width_pt = 14.759;
    let below_grid = render_options_for_image(&vml, &PdfOptions::default());
    assert_eq!(below_grid.target_width_px, Some(40));
  }
}
