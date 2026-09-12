use std::io::Cursor;

use image::{DynamicImage, RgbImage, metadata::Orientation};
use tiff::{
  decoder::{Decoder, DecodingResult, Limits},
  tags::Tag,
};

use super::image::DecodedRasterImage;
use crate::error::{PdfError, Result};

fn error(message: impl std::fmt::Display) -> PdfError {
  PdfError::Image(format!("palette TIFF: {message}"))
}

pub(super) fn decode(data: &[u8]) -> Result<Option<DecodedRasterImage>> {
  let Some((offset, little_endian)) = palette_tag(data) else {
    return Ok(None);
  };
  // TIFF 6.0 section 5: palette pixels have the same storage as grayscale
  // samples. The upstream decoder supports their compression and predictors,
  // but rejects PhotometricInterpretation=3 before reading the indices.
  // Change only that inline tag in a private copy; retain all image data,
  // directory offsets, sample depths and metadata.
  let mut encoded = data.to_vec();
  encoded[offset..offset + 2].copy_from_slice(if little_endian { &[1, 0] } else { &[0, 1] });
  let mut decoder = Decoder::new(Cursor::new(encoded)).map_err(error)?;
  let bits = decoder.get_tag_u32(Tag::BitsPerSample).map_err(error)?;
  if !matches!(bits, 1 | 2 | 4 | 8 | 16)
    || decoder.get_tag_u32(Tag::SamplesPerPixel).unwrap_or(1) != 1
    || decoder.get_tag_u32(Tag::SampleFormat).unwrap_or(1) != 1
  {
    return Err(error("unsupported palette sample layout"));
  }
  let colors = 1usize << bits;
  let map = decoder.get_tag_u16_vec(Tag::ColorMap).map_err(error)?;
  if map.len() != colors * 3 {
    return Err(error("ColorMap length does not match BitsPerSample"));
  }
  let (width, height) = decoder.dimensions().map_err(error)?;
  let len = (width as usize)
    .checked_mul(height as usize)
    .and_then(|n| n.checked_mul(3))
    .filter(|&n| n <= Limits::default().decoding_buffer_size)
    .ok_or_else(|| error("expanded RGB image exceeds decoder limits"))?;
  let orientation = decoder
    .get_tag_u32(Tag::Orientation)
    .ok()
    .and_then(|v| u8::try_from(v).ok())
    .and_then(Orientation::from_exif)
    .unwrap_or(Orientation::NoTransforms);
  let icc_profile = decoder.get_tag_u8_vec(Tag::IccProfile).ok();
  let indices = decoder.read_image().map_err(error)?;
  let mut rgb = vec![0; len];
  let row_bytes = (width as usize * bits as usize).div_ceil(8);
  for (pixel, color) in rgb.as_chunks_mut::<3>().0.iter_mut().enumerate() {
    let index = match &indices {
      DecodingResult::U8(bytes) if bits <= 8 => {
        let x = pixel % width as usize;
        let y = pixel / width as usize;
        let bit = x * bits as usize;
        let byte = *bytes
          .get(y * row_bytes + bit / 8)
          .ok_or_else(|| error("truncated palette indices"))?;
        usize::from(byte >> (8 - bits as usize - bit % 8)) & (colors - 1)
      }
      DecodingResult::U16(values) if bits == 16 => usize::from(
        *values
          .get(pixel)
          .ok_or_else(|| error("truncated palette indices"))?,
      ),
      _ => return Err(error("unexpected decoded palette sample type")),
    };
    for channel in 0..3 {
      color[channel] = ((u32::from(map[channel * colors + index]) * 255 + 32767) / 65535) as u8;
    }
  }
  let mut image = DynamicImage::ImageRgb8(
    RgbImage::from_raw(width, height, rgb).ok_or_else(|| error("invalid RGB dimensions"))?,
  );
  image.apply_orientation(orientation);
  Ok(Some(DecodedRasterImage {
    image,
    icc_profile,
    jpeg_has_real_physical_resolution: false,
  }))
}

// Locate a SHORT/count=1 PhotometricInterpretation in the first IFD only,
// matching the ordinary image decoder's first-frame behavior. Every offset is
// checked before dereferencing; all other TIFF parsing stays with the decoder.
fn palette_tag(data: &[u8]) -> Option<(usize, bool)> {
  let little = match data.get(..2)? {
    b"II" => true,
    b"MM" => false,
    _ => return None,
  };
  let read = |offset, length| unsigned(data, offset, length, little);
  let (ifd, count_size, entry_size, count_offset, value_offset, offset_size) = match read(2, 2)? {
    42 => (read(4, 4)?, 2, 12, 4, 8, 4),
    43 if read(4, 2)? == 8 && read(6, 2)? == 0 => (read(8, 8)?, 8, 20, 4, 12, 8),
    _ => return None,
  };
  let count = read(ifd, count_size)?;
  let start = ifd.checked_add(count_size)?;
  let end = start.checked_add(count.checked_mul(entry_size)?)?;
  data.get(start..end)?;
  for entry in (start..end).step_by(entry_size) {
    if read(entry, 2)? == 262 {
      return (read(entry + 2, 2)? == 3
        && read(entry + count_offset, offset_size)? == 1
        && read(entry + value_offset, 2)? == 3)
        .then_some((entry + value_offset, little));
    }
  }
  None
}

fn unsigned(data: &[u8], offset: usize, length: usize, little: bool) -> Option<usize> {
  let bytes = data.get(offset..offset.checked_add(length)?)?;
  let mut value = 0u64;
  for i in 0..length {
    value |= u64::from(bytes[if little { i } else { length - 1 - i }]) << (i * 8);
  }
  usize::try_from(value).ok()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn write(bytes: &mut [u8], at: usize, len: usize, value: usize, little: bool) {
    for i in 0..len {
      bytes[at + if little { i } else { len - 1 - i }] = ((value as u64) >> (8 * i)) as u8;
    }
  }

  fn fixture(bits: usize, little: bool, big: bool, bad_map: bool) -> Vec<u8> {
    let (header, count_size, entry_size, offset_size, value_offset) = if big {
      (16, 8, 20, 8, 12)
    } else {
      (8, 2, 12, 4, 8)
    };
    let colors = 1usize << bits;
    let row = (3 * bits).div_ceil(8);
    let palette_at = header + count_size + 12 * entry_size + offset_size;
    let pixels_at = palette_at + colors * 6;
    let profile_at = pixels_at + 2 * row;
    let profile = b"palette-test-icc!";
    let mut bytes = vec![0; profile_at + profile.len()];
    bytes[..2].copy_from_slice(if little { b"II" } else { b"MM" });
    write(&mut bytes, 2, 2, if big { 43 } else { 42 }, little);
    if big {
      write(&mut bytes, 4, 2, 8, little);
      write(&mut bytes, 8, 8, header, little);
    } else {
      write(&mut bytes, 4, 4, header, little);
    }
    write(&mut bytes, header, count_size, 12, little);
    let entries = [
      (256, 4, 1, 3),
      (257, 4, 1, 2),
      (258, 3, 1, bits),
      (259, 3, 1, 1),
      (262, 3, 1, 3),
      (273, 4, 1, pixels_at),
      (274, 3, 1, 6),
      (277, 3, 1, 1),
      (278, 4, 1, 2),
      (279, 4, 1, 2 * row),
      (320, 3, colors * 3 - usize::from(bad_map), palette_at),
      (34675, 7, profile.len(), profile_at),
    ];
    for (i, (tag, kind, count, value)) in entries.into_iter().enumerate() {
      let at = header + count_size + i * entry_size;
      write(&mut bytes, at, 2, tag, little);
      write(&mut bytes, at + 2, 2, kind, little);
      write(&mut bytes, at + 4, offset_size, count, little);
      let len = if count == 1 {
        if kind == 3 { 2 } else { 4 }
      } else {
        offset_size
      };
      write(&mut bytes, at + value_offset, len, value, little);
    }
    for i in 0..colors {
      for (channel, v) in expected_color(i).into_iter().enumerate() {
        write(
          &mut bytes,
          palette_at + 2 * (channel * colors + i),
          2,
          usize::from(v) * 257,
          little,
        );
      }
    }
    for (i, value) in [0, 1, colors - 1, colors / 2, 1, 0].into_iter().enumerate() {
      let at = pixels_at + (i / 3) * row;
      if bits == 16 {
        write(&mut bytes, at + (i % 3) * 2, 2, value, little);
      } else {
        let bit = (i % 3) * bits;
        bytes[at + bit / 8] |= (value << (8 - bits - bit % 8)) as u8;
      }
    }
    bytes[profile_at..].copy_from_slice(profile);
    bytes
  }

  fn expected_color(index: usize) -> [u8; 3] {
    [index as u8, 255 - index as u8, index.wrapping_mul(29) as u8]
  }

  #[test]
  fn palette_tiff_expands_packed_indices_with_endianness_orientation_and_icc() {
    for bits in [1, 2, 4, 8, 16] {
      for little in [false, true] {
        for big in [false, true] {
          let bytes = fixture(bits, little, big, false);
          let before = bytes.clone();
          let decoded = decode(&bytes).unwrap().unwrap();
          assert_eq!(bytes, before);
          assert_eq!((decoded.image.width(), decoded.image.height()), (2, 3));
          let colors = 1 << bits;
          let expected = [colors / 2, 0, 1, 1, 0, colors - 1]
            .into_iter()
            .flat_map(expected_color)
            .collect::<Vec<_>>();
          assert_eq!(
            decoded.image.to_rgb8().into_raw(),
            expected,
            "{bits} LE={little} big={big}"
          );
          assert_eq!(
            decoded.icc_profile.as_deref(),
            Some(b"palette-test-icc!".as_slice())
          );
        }
      }
    }
  }

  #[test]
  fn palette_tiff_rejects_incomplete_color_maps_and_invalid_offsets() {
    assert!(decode(&fixture(8, true, false, true)).is_err());
    let mut bytes = fixture(8, true, true, false);
    bytes[8..16].fill(255);
    assert!(palette_tag(&bytes).is_none());
    for len in 0..32 {
      assert!(palette_tag(&bytes[..len]).is_none());
    }
    assert!(decode(b"not a TIFF").unwrap().is_none());
  }
}
