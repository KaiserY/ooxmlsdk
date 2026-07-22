use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::sync::{Arc, OnceLock};

use image::codecs::jpeg::JpegEncoder;
use image::metadata::Orientation;
use image::{
  DynamicImage, GenericImageView, ImageDecoder, ImageFormat as RasterImageFormat, ImageReader,
  Rgba, imageops::FilterType,
};
use krilla::image::{BitsPerComponent, CustomImage, Image, ImageColorspace};
use rustc_hash::FxHashMap as HashMap;

use crate::error::{PdfError, Result};
use crate::options::PdfOptions;
use ooxmlsdk_layout::render::emf_wmf;

#[derive(Default)]
pub(super) struct ImageSet {
  rasters: HashMap<(usize, usize), Vec<CachedRaster>>,
  svgs: HashMap<(usize, usize), Arc<usvg::Tree>>,
}

struct CachedRaster {
  content_type: Option<String>,
  metafile_render_options: Option<emf_wmf::RenderOptions>,
  export_options: RasterExportOptions,
  image: Image,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RasterExportOptions {
  use_lossless_compression: bool,
  jpeg_quality: Option<u8>,
  max_size_px: Option<(u32, u32)>,
}

impl RasterExportOptions {
  fn new(options: &PdfOptions, display_width_pt: f32, display_height_pt: f32) -> Self {
    let max_size_px = options
      .images
      .reduce_resolution
      .then_some(options.images.max_resolution_dpi)
      .flatten()
      .filter(|dpi| *dpi > 50)
      .and_then(|dpi| {
        let width = display_pixels(display_width_pt, dpi)?;
        let height = display_pixels(display_height_pt, dpi)?;
        Some((width, height))
      });
    Self {
      use_lossless_compression: options.images.use_lossless_compression,
      jpeg_quality: options.effective_jpeg_quality(),
      max_size_px,
    }
  }
}

fn display_pixels(points: f32, dpi: u32) -> Option<u32> {
  if !points.is_finite() || points <= 0.0 {
    return None;
  }
  Some(((f64::from(points) * f64::from(dpi) / 72.0).round() as u32).max(1))
}

impl ImageSet {
  pub(super) fn raster(
    &mut self,
    data: &[u8],
    content_type: Option<&str>,
    options: &PdfOptions,
    metafile_render_options: Option<emf_wmf::RenderOptions>,
    display_width_pt: f32,
    display_height_pt: f32,
  ) -> Result<Image> {
    let export_options = RasterExportOptions::new(options, display_width_pt, display_height_pt);
    let key = image_data_key(data);
    if let Some(image) = self.rasters.get(&key).and_then(|images| {
      images.iter().find(|image| {
        image.content_type.as_deref() == content_type
          && image.metafile_render_options == metafile_render_options
          && image.export_options == export_options
      })
    }) {
      return Ok(image.image.clone());
    }
    let image = decode_image(data, content_type, export_options, metafile_render_options)?;
    self.rasters.entry(key).or_default().push(CachedRaster {
      content_type: content_type.map(str::to_string),
      metafile_render_options,
      export_options,
      image: image.clone(),
    });
    Ok(image)
  }

  pub(super) fn svg(&mut self, data: &[u8]) -> Result<Arc<usvg::Tree>> {
    let key = image_data_key(data);
    if let Some(tree) = self.svgs.get(&key) {
      return Ok(tree.clone());
    }
    let tree = Arc::new(
      usvg::Tree::from_data(
        data,
        &usvg::Options {
          fontdb: svg_font_database(),
          ..usvg::Options::default()
        },
      )
      .map_err(|err| PdfError::Krilla(format!("failed to decode SVG image: {err}")))?,
    );
    self.svgs.insert(key, tree.clone());
    Ok(tree)
  }
}

fn image_data_key(data: &[u8]) -> (usize, usize) {
  // ImageItem owns its Arc-backed bytes for the whole render. Using that stable
  // allocation identity avoids hashing large images on every repeated draw.
  (data.as_ptr() as usize, data.len())
}

fn svg_font_database() -> Arc<fontdb::Database> {
  static FONT_DATABASE: OnceLock<Arc<fontdb::Database>> = OnceLock::new();
  FONT_DATABASE
    .get_or_init(|| {
      let mut database = fontdb::Database::new();
      database.load_system_fonts();
      Arc::new(database)
    })
    .clone()
}

fn decode_image(
  data: &[u8],
  content_type: Option<&str>,
  export_options: RasterExportOptions,
  metafile_render_options: Option<emf_wmf::RenderOptions>,
) -> Result<Image> {
  let metafile_raster = match metafile_render_options {
    Some(render_options) => {
      emf_wmf::decode_metafile_as_raster_with_options(data, content_type, render_options)
    }
    None => emf_wmf::decode_metafile_as_raster(data, content_type),
  };
  if let Some(raster) = metafile_raster
    .map_err(|err| PdfError::Krilla(format!("failed to decode EMF/WMF image: {err}")))?
  {
    return match raster.content_type {
      "image/jpeg"
        if export_options.use_lossless_compression || export_options.jpeg_quality.is_some() =>
      {
        export_decoded_image(
          decode_dynamic_image(&raster.data, RasterImageFormat::Jpeg)?,
          RasterImageFormat::Jpeg,
          export_options,
        )
      }
      "image/jpeg" => Image::from_jpeg(raster.data.into(), true).map_err(PdfError::Krilla),
      "image/png" => {
        if let Some(quality) = export_options
          .jpeg_quality
          .filter(|_| !export_options.use_lossless_compression)
        {
          let raster = image::load_from_memory_with_format(&raster.data, RasterImageFormat::Png)
            .map_err(|err| {
              PdfError::Krilla(format!(
                "failed to decode EMF/WMF PNG for JPEG export: {err}"
              ))
            })?;
          let jpeg = encode_jpeg(raster, quality)?;
          return Image::from_jpeg(jpeg.into(), true).map_err(PdfError::Krilla);
        }
        let image = decode_png_relaxed(&raster.data)
          .map_err(|err| PdfError::Krilla(format!("failed to decode EMF/WMF PNG: {err}")))?;
        Image::from_custom(image, false).map_err(PdfError::Krilla)
      }
      content_type => Err(PdfError::Krilla(format!(
        "unsupported EMF/WMF raster content type: {content_type}"
      ))),
    };
  }

  let format = content_type
    .and_then(image_format_from_content_type)
    .or_else(|| image::guess_format(data).ok());

  if let Some(format) = format {
    let metadata = raster_metadata(data, format)?;
    let needs_orientation =
      metadata.is_some_and(|metadata| metadata.orientation != Orientation::NoTransforms);
    let needs_downsampling = metadata.is_some_and(|metadata| {
      export_options
        .max_size_px
        .is_some_and(|max_size| downsample_size(metadata.size, max_size).is_some())
    });
    let needs_compression_change = format == RasterImageFormat::Jpeg
      && (export_options.use_lossless_compression || export_options.jpeg_quality.is_some());
    if needs_orientation || needs_downsampling || needs_compression_change {
      return export_decoded_image(decode_dynamic_image(data, format)?, format, export_options);
    }
    if format == RasterImageFormat::Jpeg
      && let Ok(image) = Image::from_jpeg(data.to_vec().into(), true)
    {
      // Krilla reads and embeds the JPEG's native ICC profile while keeping
      // the compressed image stream intact.
      return Ok(image);
    }
    if format == RasterImageFormat::Png
      && let Ok(image) = Image::from_png(data.to_vec().into(), false)
    {
      return Ok(image);
    }
  }
  if matches!(format, Some(RasterImageFormat::Png))
    && let Ok(image) = decode_png_relaxed(data)
  {
    return Image::from_custom(image, false).map_err(PdfError::Krilla);
  }

  let format = format.ok_or_else(|| PdfError::Krilla("unknown raster image format".to_string()))?;
  let raster = decode_dynamic_image(data, format)?;
  export_decoded_image(raster, format, export_options)
}

fn raster_interpolation(format: RasterImageFormat) -> bool {
  // Word's fixed-format output marks photographic JPEG XObjects for smooth
  // interpolation while leaving lossless pixel graphics such as PNG
  // placeholders un-interpolated. Make that choice explicit instead of
  // inheriting one blanket backend default for every raster format.
  format == RasterImageFormat::Jpeg
}

#[derive(Clone, Copy, Debug)]
struct RasterMetadata {
  size: (u32, u32),
  orientation: Orientation,
}

fn raster_metadata(data: &[u8], format: RasterImageFormat) -> Result<Option<RasterMetadata>> {
  let mut decoder = match ImageReader::with_format(Cursor::new(data), format).into_decoder() {
    Ok(decoder) => decoder,
    Err(_) => return Ok(None),
  };
  let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);
  let mut size = decoder.dimensions();
  if matches!(
    orientation,
    Orientation::Rotate90
      | Orientation::Rotate270
      | Orientation::Rotate90FlipH
      | Orientation::Rotate270FlipH
  ) {
    size = (size.1, size.0);
  }
  Ok(Some(RasterMetadata { size, orientation }))
}

struct DecodedRasterImage {
  image: DynamicImage,
  icc_profile: Option<Vec<u8>>,
}

fn decode_dynamic_image(data: &[u8], format: RasterImageFormat) -> Result<DecodedRasterImage> {
  let mut decoder = ImageReader::with_format(Cursor::new(data), format)
    .into_decoder()
    .map_err(|err| PdfError::Krilla(format!("failed to open raster image: {err}")))?;
  let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);
  let icc_profile = decoder.icc_profile().unwrap_or_default();
  let mut image = DynamicImage::from_decoder(decoder)
    .map_err(|err| PdfError::Krilla(format!("failed to decode raster image: {err}")))?;
  image.apply_orientation(orientation);
  Ok(DecodedRasterImage { image, icc_profile })
}

fn export_decoded_image(
  mut raster: DecodedRasterImage,
  format: RasterImageFormat,
  export_options: RasterExportOptions,
) -> Result<Image> {
  let mut resized = false;
  if let Some(max_size) = export_options.max_size_px
    && let Some(target_size) = downsample_size(raster.image.dimensions(), max_size)
  {
    raster.image = raster
      .image
      .resize_exact(target_size.0, target_size.1, FilterType::Lanczos3);
    resized = true;
  }

  if format == RasterImageFormat::Jpeg
    && !export_options.use_lossless_compression
    && (resized || export_options.jpeg_quality.is_some())
  {
    let jpeg = encode_jpeg(raster.image, export_options.jpeg_quality.unwrap_or(90))?;
    return Image::from_jpeg_with_icc(jpeg.into(), raster.icc_profile.map(Into::into), true)
      .map_err(PdfError::Krilla);
  }

  Image::from_custom(
    PdfRasterImage::from_dynamic_with_icc(raster.image, raster.icc_profile),
    raster_interpolation(format),
  )
  .map_err(PdfError::Krilla)
}

fn downsample_size(size: (u32, u32), max_size: (u32, u32)) -> Option<(u32, u32)> {
  let (width, height) = size;
  let (max_width, max_height) = max_size;
  if width <= 50
    || height <= 50
    || max_width == 0
    || max_height == 0
    || (width <= max_width.saturating_add(4) && height <= max_height.saturating_add(4))
  {
    return None;
  }

  let scale =
    (f64::from(max_width) / f64::from(width)).min(f64::from(max_height) / f64::from(height));
  let target_width = (f64::from(width) * scale).round() as u32;
  let target_height = (f64::from(height) * scale).round() as u32;
  (target_width > 0 && target_height > 0).then_some((target_width, target_height))
}

fn encode_jpeg(image: image::DynamicImage, quality: u8) -> Result<Vec<u8>> {
  let rgb = image.to_rgb8();
  let (width, height) = rgb.dimensions();
  let mut jpeg = Vec::new();
  JpegEncoder::new_with_quality(&mut jpeg, quality)
    .encode(rgb.as_raw(), width, height, image::ExtendedColorType::Rgb8)
    .map_err(|err| PdfError::Krilla(format!("failed to encode JPEG image: {err}")))?;
  Ok(jpeg)
}

fn decode_png_relaxed(data: &[u8]) -> std::result::Result<PdfRasterImage, String> {
  let mut decoder = png::Decoder::new(Cursor::new(data));
  decoder.ignore_checksums(true);
  decoder.set_transformations(png::Transformations::normalize_to_color8());
  let mut reader = decoder.read_info().map_err(|err| err.to_string())?;
  let buffer_size = reader
    .output_buffer_size()
    .ok_or_else(|| "PNG output buffer size is unavailable".to_string())?;
  let mut buffer = vec![0; buffer_size];
  let info = reader
    .next_frame(&mut buffer)
    .map_err(|err| err.to_string())?;
  buffer.truncate(info.buffer_size());
  Ok(PdfRasterImage::from_png_frame(
    info.width,
    info.height,
    info.color_type,
    &buffer,
  ))
}

fn image_format_from_content_type(content_type: &str) -> Option<RasterImageFormat> {
  match content_type {
    "image/png" => Some(RasterImageFormat::Png),
    "image/jpeg" | "image/jpg" => Some(RasterImageFormat::Jpeg),
    "image/gif" => Some(RasterImageFormat::Gif),
    "image/webp" => Some(RasterImageFormat::WebP),
    _ => None,
  }
}

#[derive(Clone, Debug)]
struct PdfRasterImage {
  pixels: Arc<PdfRasterPixels>,
}

#[derive(Debug)]
struct PdfRasterPixels {
  width: u32,
  height: u32,
  rgb: Vec<u8>,
  alpha: Option<Vec<u8>>,
  icc_profile: Option<Vec<u8>>,
}

impl PdfRasterImage {
  fn from_dynamic_with_icc(image: image::DynamicImage, icc_profile: Option<Vec<u8>>) -> Self {
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8();
    let mut rgb = Vec::with_capacity(width as usize * height as usize * 3);
    let mut alpha = Vec::with_capacity(width as usize * height as usize);
    let mut opaque = true;

    for Rgba([r, g, b, a]) in rgba.pixels() {
      rgb.extend_from_slice(&[*r, *g, *b]);
      alpha.push(*a);
      opaque &= *a == u8::MAX;
    }

    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha: (!opaque).then_some(alpha),
        icc_profile,
      }),
    }
  }

  fn from_png_frame(width: u32, height: u32, color_type: png::ColorType, data: &[u8]) -> Self {
    let pixel_count = width as usize * height as usize;
    let mut rgb = Vec::with_capacity(pixel_count * 3);
    let mut alpha = Vec::with_capacity(pixel_count);
    let mut opaque = true;

    match color_type {
      png::ColorType::Grayscale => {
        for value in data {
          rgb.extend_from_slice(&[*value, *value, *value]);
        }
      }
      png::ColorType::GrayscaleAlpha => {
        for pixel in data.chunks_exact(2) {
          rgb.extend_from_slice(&[pixel[0], pixel[0], pixel[0]]);
          alpha.push(pixel[1]);
          opaque &= pixel[1] == u8::MAX;
        }
      }
      png::ColorType::Rgb => {
        rgb.extend_from_slice(data);
      }
      png::ColorType::Rgba => {
        for pixel in data.chunks_exact(4) {
          rgb.extend_from_slice(&pixel[..3]);
          alpha.push(pixel[3]);
          opaque &= pixel[3] == u8::MAX;
        }
      }
      png::ColorType::Indexed => {}
    }

    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha: (!opaque && !alpha.is_empty()).then_some(alpha),
        icc_profile: None,
      }),
    }
  }
}

impl Hash for PdfRasterImage {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.pixels.width.hash(state);
    self.pixels.height.hash(state);
    self.pixels.rgb.hash(state);
    self.pixels.alpha.hash(state);
    self.pixels.icc_profile.hash(state);
  }
}

impl CustomImage for PdfRasterImage {
  fn color_channel(&self) -> &[u8] {
    &self.pixels.rgb
  }

  fn alpha_channel(&self) -> Option<&[u8]> {
    self.pixels.alpha.as_deref()
  }

  fn bits_per_component(&self) -> BitsPerComponent {
    BitsPerComponent::Eight
  }

  fn size(&self) -> (u32, u32) {
    (self.pixels.width, self.pixels.height)
  }

  fn icc_profile(&self) -> Option<&[u8]> {
    self.pixels.icc_profile.as_deref()
  }

  fn color_space(&self) -> ImageColorspace {
    ImageColorspace::Rgb
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn custom_raster_preserves_icc_profile() {
    let profile = vec![0_u8; 128];
    let image =
      PdfRasterImage::from_dynamic_with_icc(DynamicImage::new_rgb8(1, 1), Some(profile.clone()));

    assert_eq!(CustomImage::icc_profile(&image), Some(profile.as_slice()));
  }

  #[test]
  fn image_set_reuses_matching_rasters_and_separates_metafile_options() {
    let mut jpeg = Vec::new();
    JpegEncoder::new(&mut jpeg)
      .encode(&[255, 0, 0], 1, 1, image::ExtendedColorType::Rgb8)
      .unwrap();
    let options = PdfOptions::default();
    let first = emf_wmf::RenderOptions {
      max_pixels: Some(1_000_000),
      ..emf_wmf::RenderOptions::default()
    };
    let second = emf_wmf::RenderOptions {
      max_pixels: Some(2_000_000),
      ..emf_wmf::RenderOptions::default()
    };
    let mut images = ImageSet::default();

    images
      .raster(&jpeg, Some("image/jpeg"), &options, Some(first), 72.0, 72.0)
      .unwrap();
    images
      .raster(&jpeg, Some("image/jpeg"), &options, Some(first), 72.0, 72.0)
      .unwrap();
    assert_eq!(images.rasters.values().map(Vec::len).sum::<usize>(), 1);

    images
      .raster(
        &jpeg,
        Some("image/jpeg"),
        &options,
        Some(second),
        72.0,
        72.0,
      )
      .unwrap();
    assert_eq!(images.rasters.values().map(Vec::len).sum::<usize>(), 2);
  }

  #[test]
  fn image_set_separates_resolution_requests() {
    let mut jpeg = Vec::new();
    JpegEncoder::new(&mut jpeg)
      .encode(
        &vec![127; 60 * 60 * 3],
        60,
        60,
        image::ExtendedColorType::Rgb8,
      )
      .unwrap();
    let mut options = PdfOptions::default();
    options.images.reduce_resolution = true;
    options.images.max_resolution_dpi = Some(72);
    let mut images = ImageSet::default();

    let full = images
      .raster(&jpeg, Some("image/jpeg"), &options, None, 60.0, 60.0)
      .unwrap();
    let reduced = images
      .raster(&jpeg, Some("image/jpeg"), &options, None, 30.0, 30.0)
      .unwrap();

    assert_eq!(full.size(), (60, 60));
    assert_eq!(reduced.size(), (30, 30));
    assert_eq!(images.rasters.values().map(Vec::len).sum::<usize>(), 2);
  }

  #[test]
  fn downsampling_uses_office_small_image_and_rounding_tolerances() {
    assert_eq!(downsample_size((50, 200), (25, 100)), None);
    assert_eq!(downsample_size((104, 104), (100, 100)), None);
    assert_eq!(downsample_size((105, 210), (100, 100)), Some((50, 100)));
  }

  #[test]
  fn jpeg_exif_orientation_is_applied_before_pdf_embedding() {
    let mut jpeg = Vec::new();
    JpegEncoder::new(&mut jpeg)
      .encode(
        &[255, 0, 0, 0, 0, 255],
        2,
        1,
        image::ExtendedColorType::Rgb8,
      )
      .unwrap();
    let exif = [
      b'E', b'x', b'i', b'f', 0, 0, b'I', b'I', 0x2a, 0, 8, 0, 0, 0, 1, 0, 0x12, 1, 3, 0, 1, 0, 0,
      0, 6, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut oriented = Vec::with_capacity(jpeg.len() + exif.len() + 4);
    oriented.extend_from_slice(&jpeg[..2]);
    oriented.extend_from_slice(&[0xff, 0xe1]);
    oriented.extend_from_slice(&u16::try_from(exif.len() + 2).unwrap().to_be_bytes());
    oriented.extend_from_slice(&exif);
    oriented.extend_from_slice(&jpeg[2..]);

    let image = decode_dynamic_image(&oriented, RasterImageFormat::Jpeg).unwrap();

    assert_eq!(image.image.dimensions(), (1, 2));
  }
}
