use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::sync::{Arc, OnceLock};

use image::codecs::png::PngEncoder;
use image::metadata::Orientation;
use image::{
  ColorType, DynamicImage, GenericImageView, ImageDecoder, ImageEncoder,
  ImageFormat as RasterImageFormat, ImageReader, Rgba, imageops::FilterType,
};
use jpeg_encoder::{
  ColorType as JpegColorType, Encoder as JpegEncoder, ImageBuffer as JpegImageBuffer,
  JpegColorType as JpegComponentColorType, SamplingFactor, rgb_to_ycbcr,
};
use krilla::image::{BitsPerComponent, CustomImage, Image, ImageColorspace};
use rustc_hash::FxHashMap as HashMap;

use crate::error::{PdfError, Result};
use crate::options::PdfOptions;
use ooxmlsdk_layout::render::emf_wmf;

const WORD_STATIC_3D_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-static-3d+png";

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
      usvg::Tree::from_data(data, svg_options())
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

fn svg_options() -> &'static usvg::Options<'static> {
  static OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();
  OPTIONS.get_or_init(|| {
    let mut options = usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    options
  })
}

fn decode_image(
  data: &[u8],
  content_type: Option<&str>,
  export_options: RasterExportOptions,
  metafile_render_options: Option<emf_wmf::RenderOptions>,
) -> Result<Image> {
  if content_type.is_some_and(|content_type| {
    content_type.eq_ignore_ascii_case(WORD_STATIC_3D_BITMAP_CONTENT_TYPE)
  }) {
    return export_wordprocessing_static_3d_image(
      decode_dynamic_image(data, RasterImageFormat::Png)?,
      export_options,
    );
  }

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
        let image = decode_png_relaxed(&raster.data)
          .map_err(|err| PdfError::Krilla(format!("failed to decode EMF/WMF PNG: {err}")))?;
        // Office fixed output keeps generated metafile previews lossless and
        // marks their image XObjects `/Interpolate false`. Do not apply the
        // DOCX photographic-JPEG policy to a GDI replay; it would also discard
        // a reconstructed soft mask.
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

fn export_wordprocessing_static_3d_image(
  mut raster: DecodedRasterImage,
  export_options: RasterExportOptions,
) -> Result<Image> {
  if let Some(max_size) = export_options.max_size_px
    && let Some(target_size) = downsample_size(raster.image.dimensions(), max_size)
  {
    raster.image = raster
      .image
      .resize_exact(target_size.0, target_size.1, FilterType::Lanczos3);
  }

  if export_options.use_lossless_compression {
    return Image::from_custom(
      PdfRasterImage::from_dynamic_with_icc(raster.image, raster.icc_profile),
      true,
    )
    .map_err(PdfError::Krilla);
  }

  let rgba = raster.image.to_rgba8();
  let premultiplied = apply_black_matte(&rgba);
  let quality = export_options.jpeg_quality.unwrap_or(75);
  let jpeg = encode_office_h2v2_jpeg(&premultiplied, quality)?;
  let compressed_rgb = decode_dynamic_image(&jpeg, RasterImageFormat::Jpeg)?
    .image
    .to_rgb8();
  // Office attaches `/Matte [0 0 0]` to the separate SMask. Krilla does not
  // currently expose that image-dictionary entry, so bake the mathematically
  // equivalent black-matte removal into the decoded color samples before the
  // backend writes its ordinary RGB+SMask image.
  let rgb = remove_black_matte(compressed_rgb, &rgba);
  Image::from_custom(
    PdfRasterImage::from_rgb_with_alpha(rgb, &rgba, raster.icc_profile),
    true,
  )
  .map_err(PdfError::Krilla)
}

fn apply_black_matte(image: &image::RgbaImage) -> image::RgbaImage {
  image::RgbaImage::from_fn(image.width(), image.height(), |x, y| {
    let pixel = image.get_pixel(x, y);
    let alpha = u16::from(pixel[3]);
    Rgba([
      ((u16::from(pixel[0]) * alpha + 127) / 255) as u8,
      ((u16::from(pixel[1]) * alpha + 127) / 255) as u8,
      ((u16::from(pixel[2]) * alpha + 127) / 255) as u8,
      pixel[3],
    ])
  })
}

fn remove_black_matte(
  mut image: image::RgbImage,
  alpha_source: &image::RgbaImage,
) -> image::RgbImage {
  debug_assert_eq!(image.dimensions(), alpha_source.dimensions());
  for (pixel, source) in image.pixels_mut().zip(alpha_source.pixels()) {
    let alpha = u16::from(source[3]);
    for channel in &mut pixel.0 {
      *channel = if alpha == 0 {
        0
      } else {
        ((u16::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8
      };
    }
  }
  image
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
  let width = u16::try_from(width)
    .map_err(|_| PdfError::Krilla("JPEG width exceeds 65535 pixels".to_string()))?;
  let height = u16::try_from(height)
    .map_err(|_| PdfError::Krilla("JPEG height exceeds 65535 pixels".to_string()))?;
  let mut jpeg = Vec::new();
  let mut encoder = JpegEncoder::new(&mut jpeg, quality);
  // Office fixed-format JPEG XObjects use conventional 4:2:0 chroma
  // subsampling. `image`'s encoder currently emits 4:4:4 regardless of the
  // requested quality, so use an encoder with an explicit sampling contract.
  encoder.set_sampling_factor(SamplingFactor::R_4_2_0);
  encoder
    .encode(rgb.as_raw(), width, height, JpegColorType::Rgb)
    .map_err(|err| PdfError::Krilla(format!("failed to encode JPEG image: {err}")))?;
  Ok(jpeg)
}

struct H2V2BoxRgbImage<'a> {
  data: &'a [u8],
  width: u16,
  height: u16,
}

impl H2V2BoxRgbImage<'_> {
  fn ycbcr(&self, x: u16, y: u16) -> (u8, u8, u8) {
    let index = (usize::from(y) * usize::from(self.width) + usize::from(x)) * 3;
    rgb_to_ycbcr(self.data[index], self.data[index + 1], self.data[index + 2])
  }

  fn h2v2_chroma(&self, x: u16, y: u16) -> (u8, u8) {
    let next_x = x.saturating_add(1).min(self.width - 1);
    let next_y = y.saturating_add(1).min(self.height - 1);
    let (_, cb00, cr00) = self.ycbcr(x, y);
    let (_, cb01, cr01) = self.ycbcr(next_x, y);
    let (_, cb10, cr10) = self.ycbcr(x, next_y);
    let (_, cb11, cr11) = self.ycbcr(next_x, next_y);
    // libjpeg's h2v2_downsample alternates the rounding bias so exact halves
    // do not introduce a systematic upward bias. The pattern restarts on
    // each output row. jpeg-encoder samples the even input coordinates for
    // 4:2:0, so put the filtered value at those coordinates.
    let bias = if (x / 2).is_multiple_of(2) { 1 } else { 2 };
    let average = |a: u8, b: u8, c: u8, d: u8| {
      ((u16::from(a) + u16::from(b) + u16::from(c) + u16::from(d) + bias) >> 2) as u8
    };
    (
      average(cb00, cb01, cb10, cb11),
      average(cr00, cr01, cr10, cr11),
    )
  }
}

impl JpegImageBuffer for H2V2BoxRgbImage<'_> {
  fn get_jpeg_color_type(&self) -> JpegComponentColorType {
    JpegComponentColorType::Ycbcr
  }

  fn width(&self) -> u16 {
    self.width
  }

  fn height(&self) -> u16 {
    self.height
  }

  fn fill_buffers(&self, y: u16, buffers: &mut [Vec<u8>; 4]) {
    for x in 0..self.width {
      let (luma, mut cb, mut cr) = self.ycbcr(x, y);
      if x.is_multiple_of(2) && y.is_multiple_of(2) {
        (cb, cr) = self.h2v2_chroma(x, y);
      }
      buffers[0].push(luma);
      buffers[1].push(cb);
      buffers[2].push(cr);
    }
  }
}

fn encode_office_h2v2_jpeg(image: &image::RgbaImage, quality: u8) -> Result<Vec<u8>> {
  let rgb = DynamicImage::ImageRgba8(image.clone()).to_rgb8();
  let width = u16::try_from(rgb.width())
    .map_err(|_| PdfError::Krilla("JPEG width exceeds 65535 pixels".to_string()))?;
  let height = u16::try_from(rgb.height())
    .map_err(|_| PdfError::Krilla("JPEG height exceeds 65535 pixels".to_string()))?;
  let mut jpeg = Vec::new();
  let mut encoder = JpegEncoder::new(&mut jpeg, quality);
  encoder.set_sampling_factor(SamplingFactor::R_4_2_0);
  encoder
    .encode_image(H2V2BoxRgbImage {
      data: rgb.as_raw(),
      width,
      height,
    })
    .map_err(|err| PdfError::Krilla(format!("failed to encode Office JPEG image: {err}")))?;
  Ok(jpeg)
}

/// Reproduce PowerPoint's fixed-output treatment of bitmap pixels lifted from
/// an ActiveX WMF preview: JPEG-compress the WMF DIB color plane while
/// preserving its alpha plane exactly.
pub(super) fn powerpoint_activex_bitmap_png(data: &[u8], quality: u8) -> Result<Vec<u8>> {
  let original = decode_dynamic_image(data, RasterImageFormat::Png)?
    .image
    .to_rgba8();
  let (width, height) = original.dimensions();
  let jpeg = encode_office_h2v2_jpeg(&original, quality)?;
  let recompressed = decode_dynamic_image(&jpeg, RasterImageFormat::Jpeg)?
    .image
    .to_rgb8();
  let opaque = original.pixels().all(|pixel| pixel[3] == u8::MAX);
  let mut output = Vec::new();
  if opaque {
    PngEncoder::new(&mut output)
      .write_image(recompressed.as_raw(), width, height, ColorType::Rgb8.into())
      .map_err(|err| PdfError::Krilla(format!("failed to encode ActiveX bitmap PNG: {err}")))?;
  } else {
    let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
    for (color, source) in recompressed.pixels().zip(original.pixels()) {
      rgba.extend_from_slice(&[color[0], color[1], color[2], source[3]]);
    }
    PngEncoder::new(&mut output)
      .write_image(&rgba, width, height, ColorType::Rgba8.into())
      .map_err(|err| PdfError::Krilla(format!("failed to encode ActiveX bitmap PNG: {err}")))?;
  }
  Ok(output)
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

  fn from_rgb_with_alpha(
    rgb: image::RgbImage,
    alpha_source: &image::RgbaImage,
    icc_profile: Option<Vec<u8>>,
  ) -> Self {
    debug_assert_eq!(rgb.dimensions(), alpha_source.dimensions());
    let (width, height) = rgb.dimensions();
    let alpha = alpha_source
      .pixels()
      .map(|pixel| pixel[3])
      .collect::<Vec<_>>();
    let opaque = alpha.iter().all(|alpha| *alpha == u8::MAX);
    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb: rgb.into_raw(),
        alpha: (!opaque).then_some(alpha),
        icc_profile,
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
  use image::codecs::jpeg::JpegEncoder as ImageJpegEncoder;

  #[test]
  fn custom_raster_preserves_icc_profile() {
    let profile = vec![0_u8; 128];
    let image =
      PdfRasterImage::from_dynamic_with_icc(DynamicImage::new_rgb8(1, 1), Some(profile.clone()));

    assert_eq!(CustomImage::icc_profile(&image), Some(profile.as_slice()));
  }

  #[test]
  fn office_h2v2_jpeg_averages_each_chroma_block() {
    let data = [
      255, 0, 0, 0, 255, 0, // red, green
      0, 0, 255, 255, 255, 255, // blue, white
    ];
    let image = H2V2BoxRgbImage {
      data: &data,
      width: 2,
      height: 2,
    };
    let converted = [
      rgb_to_ycbcr(255, 0, 0),
      rgb_to_ycbcr(0, 255, 0),
      rgb_to_ycbcr(0, 0, 255),
      rgb_to_ycbcr(255, 255, 255),
    ];
    let average = |component: usize| {
      ((converted
        .iter()
        .map(|pixel| u16::from([pixel.0, pixel.1, pixel.2][component]))
        .sum::<u16>()
        + 1)
        >> 2) as u8
    };

    assert_eq!(image.h2v2_chroma(0, 0), (average(1), average(2)));
  }

  #[test]
  fn word_static_3d_black_matte_round_trip_preserves_alpha_contract() {
    let source = image::RgbaImage::from_raw(2, 1, vec![146, 208, 80, 92, 255, 127, 64, 0]).unwrap();

    let premultiplied = apply_black_matte(&source);
    assert_eq!(premultiplied.get_pixel(0, 0).0, [53, 75, 29, 92]);
    assert_eq!(premultiplied.get_pixel(1, 0).0, [0, 0, 0, 0]);

    let restored = remove_black_matte(
      image::RgbImage::from_fn(2, 1, |x, _| {
        let pixel = premultiplied.get_pixel(x, 0);
        image::Rgb([pixel[0], pixel[1], pixel[2]])
      }),
      &source,
    );
    assert_eq!(restored.get_pixel(0, 0).0, [147, 208, 80]);
    assert_eq!(restored.get_pixel(1, 0).0, [0, 0, 0]);
  }

  #[test]
  fn powerpoint_activex_jpeg_round_trip_preserves_binary_alpha() {
    let source = [
      240, 20, 10, 255, // opaque red
      10, 220, 30, 0, // transparent green
    ];
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(&source, 2, 1, ColorType::Rgba8.into())
      .unwrap();

    let output = powerpoint_activex_bitmap_png(&png, 75).unwrap();
    let output = image::load_from_memory_with_format(&output, RasterImageFormat::Png)
      .unwrap()
      .to_rgba8();

    assert_eq!(output.dimensions(), (2, 1));
    assert_eq!(output.get_pixel(0, 0)[3], 255);
    assert_eq!(output.get_pixel(1, 0)[3], 0);
  }

  #[test]
  fn image_set_reuses_matching_rasters_and_separates_metafile_options() {
    let mut jpeg = Vec::new();
    ImageJpegEncoder::new(&mut jpeg)
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
    ImageJpegEncoder::new(&mut jpeg)
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
    ImageJpegEncoder::new(&mut jpeg)
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

  #[test]
  fn jpeg_export_uses_office_four_two_zero_sampling() {
    let jpeg = encode_jpeg(DynamicImage::new_rgb8(16, 16), 75).unwrap();
    let sof = jpeg
      .windows(2)
      .position(|marker| marker == [0xff, 0xc0])
      .expect("baseline JPEG start-of-frame marker");

    assert_eq!(jpeg[sof + 11], 0x22, "luma sampling factors");
    assert_eq!(jpeg[sof + 14], 0x11, "Cb sampling factors");
    assert_eq!(jpeg[sof + 17], 0x11, "Cr sampling factors");
  }
}
