use std::io::Write;

use flate2::{Compression, write::ZlibEncoder};
use pdf_writer::types::Predictor;
use pdf_writer::{Filter, Finish, Name, Pdf, Ref};
use rustc_hash::FxHashMap as HashMap;

use super::image::{
  DirectRasterColorSpace, DirectRasterEncoding, DirectRasterImage, PreparedRasterImage,
};
use crate::error::{PdfError, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RegisteredImage {
  pub(super) name: Vec<u8>,
  pub(super) id: Ref,
}

#[derive(Debug)]
struct ImageObject {
  registered: RegisteredImage,
  soft_mask_id: Option<Ref>,
  icc_id: Option<Ref>,
  image: PreparedRasterImage,
}

#[derive(Debug)]
struct IccObject {
  id: Ref,
  color_space: DirectRasterColorSpace,
  profile: Vec<u8>,
}

/// Document-wide image registry for the direct serializer.
///
/// Pages own only resource-name mappings. Encoded streams, soft masks, and ICC
/// profiles are emitted once per prepared image, matching PDF's XObject model
/// and avoiding repeated hashing or compression of large pixel planes.
#[derive(Debug, Default)]
pub(super) struct DirectImageSet {
  by_identity: HashMap<usize, usize>,
  images: Vec<ImageObject>,
  icc_profiles: Vec<IccObject>,
}

impl DirectImageSet {
  pub(super) fn register(
    &mut self,
    image: PreparedRasterImage,
    mut allocate: impl FnMut() -> Result<Ref>,
  ) -> Result<RegisteredImage> {
    if let Some(&index) = self.by_identity.get(&image.identity()) {
      return Ok(self.images[index].registered.clone());
    }

    validate_image(image.direct())?;
    let index = self.images.len();
    let registered = RegisteredImage {
      name: format!("Im{index}").into_bytes(),
      id: allocate()?,
    };
    let soft_mask_id = image.direct().alpha().map(|_| allocate()).transpose()?;
    let icc_id = match image.direct().icc_profile() {
      Some(profile) => {
        Some(self.register_icc(image.direct().color_space, profile, &mut allocate)?)
      }
      None => None,
    };
    self.by_identity.insert(image.identity(), index);
    self.images.push(ImageObject {
      registered: registered.clone(),
      soft_mask_id,
      icc_id,
      image,
    });
    Ok(registered)
  }

  fn register_icc(
    &mut self,
    color_space: DirectRasterColorSpace,
    profile: &[u8],
    allocate: &mut impl FnMut() -> Result<Ref>,
  ) -> Result<Ref> {
    if let Some(existing) = self
      .icc_profiles
      .iter()
      .find(|existing| existing.color_space == color_space && existing.profile == profile)
    {
      return Ok(existing.id);
    }
    let id = allocate()?;
    self.icc_profiles.push(IccObject {
      id,
      color_space,
      profile: profile.to_vec(),
    });
    Ok(id)
  }

  pub(super) fn write_objects(&self, pdf: &mut Pdf) -> Result<()> {
    for profile in &self.icc_profiles {
      let compressed = deflate(&profile.profile)?;
      validate_stream_length(compressed.len())?;
      let mut object = pdf.icc_profile(profile.id, &compressed);
      object.filter(Filter::FlateDecode);
      object.n(profile.color_space.components());
      write_alternate_color_space(object.alternate(), profile.color_space);
      object.finish();
    }
    for image in &self.images {
      write_image_object(pdf, image)?;
    }
    Ok(())
  }
}

fn validate_image(image: &DirectRasterImage) -> Result<()> {
  let width = usize::try_from(image.width)
    .map_err(|_| PdfError::Writer("image width exceeds the address space".to_string()))?;
  let height = usize::try_from(image.height)
    .map_err(|_| PdfError::Writer("image height exceeds the address space".to_string()))?;
  if width == 0 || height == 0 {
    return Err(PdfError::Writer(
      "PDF image dimensions must be positive".to_string(),
    ));
  }
  i32::try_from(image.width)
    .and_then(|_| i32::try_from(image.height))
    .map_err(|_| PdfError::Writer("image dimensions exceed the PDF integer range".to_string()))?;
  if !matches!(image.bits_per_component, 1 | 2 | 4 | 8 | 16) {
    return Err(PdfError::Writer(format!(
      "unsupported image component depth {}",
      image.bits_per_component
    )));
  }

  match &image.encoding {
    DirectRasterEncoding::Sampled { pixels } => {
      if image.bits_per_component != 8 || image.color_space != DirectRasterColorSpace::Rgb {
        return Err(PdfError::Writer(
          "sampled raster transport currently requires 8-bit RGB".to_string(),
        ));
      }
      let pixel_count = width
        .checked_mul(height)
        .ok_or_else(|| PdfError::Writer("image sample count overflows usize".to_string()))?;
      let color_length = pixel_count
        .checked_mul(3)
        .ok_or_else(|| PdfError::Writer("image color plane length overflows usize".to_string()))?;
      if pixels.width != image.width
        || pixels.height != image.height
        || pixels.rgb.len() != color_length
        || pixels
          .alpha
          .as_ref()
          .is_some_and(|alpha| alpha.len() != pixel_count)
      {
        return Err(PdfError::Writer(
          "sampled raster planes do not match their declared dimensions".to_string(),
        ));
      }
    }
    DirectRasterEncoding::Dct {
      data,
      invert_cmyk,
      alpha,
      ..
    } => {
      if image.bits_per_component != 8 {
        return Err(PdfError::Writer(
          "JPEG raster transport currently requires 8-bit components".to_string(),
        ));
      }
      if *invert_cmyk && image.color_space != DirectRasterColorSpace::Cmyk {
        return Err(PdfError::Writer(
          "JPEG component inversion requires a CMYK color space".to_string(),
        ));
      }
      if data.is_empty() {
        return Err(PdfError::Writer("JPEG image stream is empty".to_string()));
      }
      if let Some(alpha) = alpha {
        let pixel_count = width
          .checked_mul(height)
          .ok_or_else(|| PdfError::Writer("image sample count overflows usize".to_string()))?;
        if image.color_space != DirectRasterColorSpace::Rgb || alpha.len() != pixel_count {
          return Err(PdfError::Writer(
            "JPEG soft-mask samples require matching 8-bit RGB dimensions".to_string(),
          ));
        }
      }
    }
    DirectRasterEncoding::IndexedPng { data, palette, .. } => {
      let entries = palette.len() / 3;
      if image.color_space != DirectRasterColorSpace::Rgb
        || !matches!(image.bits_per_component, 1 | 2 | 4 | 8)
        || data.is_empty()
        || palette.is_empty()
        || palette.len() % 3 != 0
        || entries > (1_usize << image.bits_per_component)
      {
        return Err(PdfError::Writer(
          "indexed PNG stream has an invalid palette or IDAT payload".to_string(),
        ));
      }
    }
  }
  if let Some(matte) = image.matte {
    if image.alpha().is_none() || image.color_space != DirectRasterColorSpace::Rgb {
      return Err(PdfError::Writer(
        "image Matte requires an RGB image with a soft mask".to_string(),
      ));
    }
    if matte
      .into_iter()
      .any(|component| !component.is_finite() || !(0.0..=1.0).contains(&component))
    {
      return Err(PdfError::Writer(
        "image Matte components must be finite DeviceRGB values".to_string(),
      ));
    }
  }
  Ok(())
}

fn write_image_object(pdf: &mut Pdf, object: &ImageObject) -> Result<()> {
  let image = object.image.direct();
  let encoded;
  let (stream, filter) = match &image.encoding {
    DirectRasterEncoding::Sampled { pixels } => {
      encoded = deflate(&pixels.rgb)?;
      (encoded.as_slice(), Filter::FlateDecode)
    }
    DirectRasterEncoding::Dct { data, .. } => (data.as_ref(), Filter::DctDecode),
    DirectRasterEncoding::IndexedPng { data, .. } => (data.as_ref(), Filter::FlateDecode),
  };
  validate_stream_length(stream.len())?;

  let mut writer = pdf.image_xobject(object.registered.id, stream);
  writer.filter(filter);
  writer.width(image.width as i32);
  writer.height(image.height as i32);
  writer.bits_per_component(i32::from(image.bits_per_component));
  if image.interpolate {
    writer.interpolate(true);
  }

  match &image.encoding {
    DirectRasterEncoding::IndexedPng { palette, .. } => {
      let high = i32::try_from(palette.len() / 3 - 1)
        .map_err(|_| PdfError::Writer("indexed image palette is too large".to_string()))?;
      if let Some(icc_id) = object.icc_id {
        let mut space = writer.insert(Name(b"ColorSpace")).array();
        space.item(Name(b"Indexed"));
        let mut base = space.push().array();
        base.item(Name(b"ICCBased"));
        base.item(icc_id);
        base.finish();
        space.item(high);
        space.item(pdf_writer::Str(palette));
        space.finish();
      } else {
        writer
          .color_space()
          .indexed(Name(b"DeviceRGB"), high, palette);
      }
      let mut parameters = writer.decode_parms();
      parameters
        .predictor(Predictor::PngOptimum)
        .colors(1)
        .bits_per_component(i32::from(image.bits_per_component))
        .columns(image.width as i32);
      parameters.finish();
    }
    DirectRasterEncoding::Sampled { .. } | DirectRasterEncoding::Dct { .. } => {
      if let Some(icc_id) = object.icc_id {
        writer.color_space().icc_based(icc_id);
      } else {
        write_device_color_space(writer.color_space(), image.color_space);
      }
    }
  }
  if let DirectRasterEncoding::Dct {
    invert_cmyk: true, ..
  } = &image.encoding
  {
    writer.decode(
      std::iter::repeat_n([1.0_f32, 0.0], image.color_space.components() as usize).flatten(),
    );
  }
  if let Some(mask_id) = object.soft_mask_id {
    writer.s_mask(mask_id);
  }
  writer.finish();

  if let (Some(mask_id), Some(alpha)) = (object.soft_mask_id, image.alpha()) {
    let encoded_alpha = deflate(alpha)?;
    validate_stream_length(encoded_alpha.len())?;
    let mut mask = pdf.image_xobject(mask_id, &encoded_alpha);
    mask.filter(Filter::FlateDecode);
    mask.width(image.width as i32);
    mask.height(image.height as i32);
    mask.color_space().device_gray();
    mask.bits_per_component(i32::from(image.bits_per_component));
    if image.soft_mask_interpolate {
      mask.interpolate(true);
    }
    if let Some(matte) = image.matte {
      mask.matte(matte);
    }
    mask.finish();
  }
  Ok(())
}

fn write_device_color_space(
  color_space: pdf_writer::writers::ColorSpace<'_>,
  value: DirectRasterColorSpace,
) {
  match value {
    DirectRasterColorSpace::Gray => color_space.device_gray(),
    DirectRasterColorSpace::Rgb => color_space.device_rgb(),
    DirectRasterColorSpace::Cmyk => color_space.device_cmyk(),
  }
}

fn write_alternate_color_space(
  color_space: pdf_writer::writers::ColorSpace<'_>,
  value: DirectRasterColorSpace,
) {
  write_device_color_space(color_space, value);
}

fn deflate(data: &[u8]) -> Result<Vec<u8>> {
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(6));
  encoder
    .write_all(data)
    .map_err(|error| PdfError::Writer(format!("failed to compress image stream: {error}")))?;
  encoder
    .finish()
    .map_err(|error| PdfError::Writer(format!("failed to finish image stream: {error}")))
}

fn validate_stream_length(length: usize) -> Result<()> {
  i32::try_from(length)
    .map(|_| ())
    .map_err(|_| PdfError::Writer("image stream exceeds the PDF integer range".to_string()))
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use pdf_writer::Settings;

  use super::*;
  use crate::render::image::PdfRasterPixels;

  fn prepared(direct: DirectRasterImage) -> PreparedRasterImage {
    PreparedRasterImage::new(direct)
  }

  fn sampled_with_icc(
    alpha: Option<Vec<u8>>,
    matte: Option<[f32; 3]>,
    icc_profile: Option<Vec<u8>>,
  ) -> PreparedRasterImage {
    let pixels = Arc::new(PdfRasterPixels {
      width: 2,
      height: 2,
      rgb: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255],
      alpha,
      icc_profile,
    });
    prepared(DirectRasterImage {
      width: 2,
      height: 2,
      color_space: DirectRasterColorSpace::Rgb,
      bits_per_component: 8,
      encoding: DirectRasterEncoding::Sampled { pixels },
      interpolate: false,
      soft_mask_interpolate: false,
      matte,
    })
  }

  fn sampled(alpha: Option<Vec<u8>>, matte: Option<[f32; 3]>) -> PreparedRasterImage {
    sampled_with_icc(alpha, matte, None)
  }

  fn dct(
    color_space: DirectRasterColorSpace,
    invert_cmyk: bool,
    icc_profile: Option<Vec<u8>>,
    interpolate: bool,
  ) -> PreparedRasterImage {
    prepared(DirectRasterImage {
      width: 2,
      height: 1,
      color_space,
      bits_per_component: 8,
      encoding: DirectRasterEncoding::Dct {
        data: Arc::from([0xff, 0xd8, 0xff, 0xd9]),
        icc_profile: icc_profile.map(Into::into),
        invert_cmyk,
        alpha: None,
      },
      interpolate,
      soft_mask_interpolate: false,
      matte: None,
    })
  }

  fn dct_with_soft_mask(soft_mask_interpolate: bool) -> PreparedRasterImage {
    prepared(DirectRasterImage {
      width: 2,
      height: 1,
      color_space: DirectRasterColorSpace::Rgb,
      bits_per_component: 8,
      encoding: DirectRasterEncoding::Dct {
        data: Arc::from([0xff, 0xd8, 0xff, 0xd9]),
        icc_profile: None,
        invert_cmyk: false,
        alpha: Some(Arc::from([32, 224])),
      },
      interpolate: true,
      soft_mask_interpolate,
      matte: Some([0.0, 0.0, 0.0]),
    })
  }

  fn indexed(
    bits_per_component: u8,
    palette: Vec<u8>,
    icc_profile: Option<Vec<u8>>,
  ) -> PreparedRasterImage {
    prepared(DirectRasterImage {
      width: 8,
      height: 1,
      color_space: DirectRasterColorSpace::Rgb,
      bits_per_component,
      encoding: DirectRasterEncoding::IndexedPng {
        data: deflate(&[0, 0]).unwrap().into(),
        palette: palette.into(),
        icc_profile: icc_profile.map(Into::into),
      },
      interpolate: false,
      soft_mask_interpolate: false,
      matte: None,
    })
  }

  fn icc_profile(color_space: &[u8; 4]) -> Vec<u8> {
    let mut profile = vec![0; 132];
    profile[..4].copy_from_slice(&132_u32.to_be_bytes());
    profile[8] = 4;
    profile[16..20].copy_from_slice(color_space);
    profile[36..40].copy_from_slice(b"acsp");
    profile
  }

  fn register_error(image: PreparedRasterImage) -> PdfError {
    DirectImageSet::default()
      .register(image, || Ok(Ref::new(1)))
      .unwrap_err()
  }

  fn serialized(image: PreparedRasterImage) -> Vec<u8> {
    let mut next = 1;
    let mut images = DirectImageSet::default();
    images
      .register(image, || {
        let id = Ref::new(next);
        next += 1;
        Ok(id)
      })
      .unwrap();
    let mut pdf = Pdf::with_settings(Settings { pretty: false });
    images.write_objects(&mut pdf).unwrap();
    pdf.finish()
  }

  #[test]
  fn sampled_rgb_is_one_flate_image_without_a_mask() {
    let pdf = serialized(sampled(None, None));
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("/Subtype/Image"));
    assert!(text.contains("/Width 2"));
    assert!(text.contains("/Height 2"));
    assert!(text.contains("/ColorSpace/DeviceRGB"));
    assert!(text.contains("/BitsPerComponent 8"));
    assert!(text.contains("/Filter/FlateDecode"));
    assert!(!text.contains("/SMask"));
    assert!(!text.contains("/Matte"));
    assert!(!text.contains("/Interpolate"));
  }

  #[test]
  fn sampled_alpha_writes_a_gray_soft_mask_and_explicit_matte() {
    let pdf = serialized(sampled(Some(vec![0, 64, 128, 255]), Some([0.0, 0.0, 0.0])));
    let text = String::from_utf8_lossy(&pdf);
    assert_eq!(text.matches("/Subtype/Image").count(), 2);
    assert!(text.contains("/SMask 2 0 R"));
    assert!(text.contains("/ColorSpace/DeviceGray"));
    assert!(text.contains("/Matte[0 0 0]"));
  }

  #[test]
  fn dct_rgb_preserves_the_jpeg_stream_and_interpolation_contract() {
    let pdf = serialized(dct(DirectRasterColorSpace::Rgb, false, None, true));
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("/Filter/DCTDecode"));
    assert!(text.contains("/ColorSpace/DeviceRGB"));
    assert!(text.contains("/Interpolate true"));
    assert!(!text.contains("/Decode["));
    assert!(!text.contains("/SMask"));
    assert!(
      pdf
        .windows(4)
        .any(|window| window == [0xff, 0xd8, 0xff, 0xd9])
    );
  }

  #[test]
  fn dct_rgb_carries_an_independent_noninterpolated_soft_mask_and_matte() {
    let pdf = serialized(dct_with_soft_mask(false));
    let text = String::from_utf8_lossy(&pdf);
    assert_eq!(text.matches("/Subtype/Image").count(), 2);
    assert!(text.contains("/Filter/DCTDecode"));
    assert!(text.contains("/SMask 2 0 R"));
    assert!(text.contains("/ColorSpace/DeviceGray"));
    assert!(text.contains("/Matte[0 0 0]"));
    assert_eq!(text.matches("/Interpolate true").count(), 1);
  }

  #[test]
  fn dct_cmyk_inverts_exactly_four_components() {
    let pdf = serialized(dct(DirectRasterColorSpace::Cmyk, true, None, false));
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("/ColorSpace/DeviceCMYK"));
    assert!(text.contains("/Decode[1 0 1 0 1 0 1 0]"));
  }

  #[test]
  fn dct_rejects_cmyk_inversion_on_an_rgb_stream() {
    let error = register_error(dct(DirectRasterColorSpace::Rgb, true, None, false));
    assert!(matches!(error, PdfError::Writer(message) if message.contains("requires a CMYK")));
  }

  #[test]
  fn indexed_png_preserves_palette_and_png_predictor_parameters() {
    let pdf = serialized(indexed(1, vec![20, 120, 40, 240, 20, 10], None));
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("/ColorSpace[/Indexed/DeviceRGB 1"));
    assert!(text.contains("/BitsPerComponent 1"));
    assert!(text.contains("/Filter/FlateDecode"));
    assert!(text.contains("/DecodeParms<</Predictor 15/Colors 1/BitsPerComponent 1/Columns 8>>"));
  }

  #[test]
  fn indexed_png_rejects_a_palette_larger_than_its_component_depth() {
    let error = register_error(indexed(1, vec![0; 9], None));
    assert!(matches!(error, PdfError::Writer(message) if message.contains("invalid palette")));
  }

  #[test]
  fn one_valid_rgb_icc_profile_is_shared_by_distinct_images() {
    let profile = icc_profile(b"RGB ");
    let mut next = 1;
    let mut images = DirectImageSet::default();
    for image in [
      dct(
        DirectRasterColorSpace::Rgb,
        false,
        Some(profile.clone()),
        false,
      ),
      sampled_with_icc(None, None, Some(profile)),
    ] {
      images
        .register(image, || {
          let id = Ref::new(next);
          next += 1;
          Ok(id)
        })
        .unwrap();
    }
    assert_eq!(images.icc_profiles.len(), 1);
    let mut pdf = Pdf::with_settings(Settings { pretty: false });
    images.write_objects(&mut pdf).unwrap();
    let pdf = pdf.finish();
    let text = String::from_utf8_lossy(&pdf);
    assert_eq!(text.matches("/ICCBased 2 0 R").count(), 2);
    let icc = text.split("2 0 obj").nth(1).unwrap();
    let icc = &icc[..icc.find("endobj").unwrap()];
    assert!(icc.contains("/N 3"));
    assert!(icc.contains("/Alternate/DeviceRGB"));
    assert!(icc.contains("/Filter/FlateDecode"));
  }

  #[test]
  fn malformed_icc_signature_falls_back_to_device_rgb() {
    let mut profile = icc_profile(b"RGB ");
    profile[36..40].copy_from_slice(b"nope");
    let pdf = serialized(dct(
      DirectRasterColorSpace::Rgb,
      false,
      Some(profile),
      false,
    ));
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("/ColorSpace/DeviceRGB"));
    assert!(!text.contains("/ICCBased"));
  }

  #[test]
  fn icc_component_mismatch_falls_back_to_device_rgb() {
    let pdf = serialized(dct(
      DirectRasterColorSpace::Rgb,
      false,
      Some(icc_profile(b"CMYK")),
      false,
    ));
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("/ColorSpace/DeviceRGB"));
    assert!(!text.contains("/ICCBased"));
  }

  #[test]
  fn sampled_alpha_length_must_match_the_pixel_count() {
    let error = register_error(sampled(Some(vec![255; 3]), None));
    assert!(matches!(error, PdfError::Writer(message) if message.contains("planes do not match")));
  }

  #[test]
  fn dct_alpha_length_must_match_the_pixel_count() {
    let mut direct = dct_with_soft_mask(false).direct().clone();
    let DirectRasterEncoding::Dct { alpha, .. } = &mut direct.encoding else {
      unreachable!();
    };
    *alpha = Some(Arc::from([255]));
    let error = register_error(prepared(direct));
    assert!(
      matches!(error, PdfError::Writer(message) if message.contains("matching 8-bit RGB dimensions"))
    );
  }

  #[test]
  fn matte_requires_a_soft_mask() {
    let error = register_error(sampled(None, Some([0.0, 0.0, 0.0])));
    assert!(
      matches!(error, PdfError::Writer(message) if message.contains("requires an RGB image with a soft mask"))
    );
  }

  #[test]
  fn matte_components_must_be_finite_device_rgb_values() {
    let error = register_error(sampled(
      Some(vec![0, 64, 128, 255]),
      Some([0.0, f32::NAN, 0.0]),
    ));
    assert!(matches!(error, PdfError::Writer(message) if message.contains("finite DeviceRGB")));
  }

  #[test]
  fn one_prepared_image_is_reused_without_reallocation() {
    let image = sampled(None, None);
    let mut next = 1;
    let mut images = DirectImageSet::default();
    let first = images
      .register(image.clone(), || {
        let id = Ref::new(next);
        next += 1;
        Ok(id)
      })
      .unwrap();
    let second = images
      .register(image, || {
        let id = Ref::new(next);
        next += 1;
        Ok(id)
      })
      .unwrap();
    assert_eq!(first, second);
    assert_eq!(next, 2);
    assert_eq!(images.images.len(), 1);
  }
}
