use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::sync::Arc;

use image::codecs::jpeg::JpegEncoder;
use image::{GenericImageView, ImageFormat as RasterImageFormat, Rgba};
use krilla::image::{BitsPerComponent, CustomImage, Image, ImageColorspace};

use crate::error::{PdfError, Result};
use crate::options::PdfOptions;
use ooxmlsdk_layout::render::emf_wmf;

pub(super) fn decode_image(
  data: &[u8],
  content_type: Option<&str>,
  options: &PdfOptions,
) -> Result<Image> {
  if let Some(raster) = emf_wmf::decode_metafile_as_raster(data, content_type)
    .map_err(|err| PdfError::Krilla(format!("failed to decode EMF/WMF image: {err}")))?
  {
    return match raster.content_type {
      "image/jpeg" => Image::from_jpeg(raster.data.into(), false).map_err(PdfError::Krilla),
      "image/png" => {
        if let Some(quality) = options.effective_jpeg_quality() {
          let raster = image::load_from_memory_with_format(&raster.data, RasterImageFormat::Png)
            .map_err(|err| {
              PdfError::Krilla(format!(
                "failed to decode EMF/WMF PNG for JPEG export: {err}"
              ))
            })?;
          let jpeg = encode_jpeg(raster, quality)?;
          return Image::from_jpeg(jpeg.into(), false).map_err(PdfError::Krilla);
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

  if matches!(format, Some(RasterImageFormat::Jpeg))
    && let Ok(image) = Image::from_jpeg(data.to_vec().into(), false)
  {
    return Ok(image);
  }
  if matches!(format, Some(RasterImageFormat::Png))
    && let Ok(image) = decode_png_relaxed(data)
  {
    return Image::from_custom(image, false).map_err(PdfError::Krilla);
  }

  let raster = match format {
    Some(format) => image::load_from_memory_with_format(data, format),
    None => image::load_from_memory(data),
  };

  let raster =
    raster.map_err(|err| PdfError::Krilla(format!("failed to decode raster image: {err}")))?;
  Image::from_custom(PdfRasterImage::from_dynamic(raster), false).map_err(PdfError::Krilla)
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
}

impl PdfRasterImage {
  fn from_dynamic(image: image::DynamicImage) -> Self {
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
    None
  }

  fn color_space(&self) -> ImageColorspace {
    ImageColorspace::Rgb
  }
}
