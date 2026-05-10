use image::codecs::jpeg::JpegEncoder;
use image::{ColorType, ImageEncoder};

const EMF_HEADER_SIZE: usize = 108;
const EMR_EOF: u32 = 14;
const EMR_SET_DIBITS_TO_DEVICE: u32 = 80;
const EMR_STRETCH_DIBITS: u32 = 81;

const BI_RGB: u32 = 0;
const BI_JPEG: u32 = 4;
const BI_PNG: u32 = 5;

pub(crate) fn decode_metafile_as_jpeg(
  data: &[u8],
  content_type: Option<&str>,
) -> Result<Option<Vec<u8>>, String> {
  if !looks_like_metafile(data, content_type) {
    return Ok(None);
  }

  if let Some(jpeg) = decode_emf_as_jpeg(data)? {
    return Ok(Some(jpeg));
  }

  Ok(None)
}

fn looks_like_metafile(data: &[u8], content_type: Option<&str>) -> bool {
  matches!(
    content_type,
    Some("image/x-wmf" | "image/wmf" | "image/x-emf" | "image/emf" | "application/x-msmetafile")
  ) || is_emf(data)
}

fn decode_emf_as_jpeg(data: &[u8]) -> Result<Option<Vec<u8>>, String> {
  if !is_emf(data) {
    return Ok(None);
  }
  if data.len() < EMF_HEADER_SIZE {
    return Err("EMF header is truncated".into());
  }

  let mut pos = EMF_HEADER_SIZE;
  let mut bitmap_record = None;

  while pos + 8 <= data.len() {
    let record_type = read_u32(data, pos)?;
    let record_size = read_u32(data, pos + 4)? as usize;
    if record_size < 8 || pos + record_size > data.len() {
      return Err(format!(
        "invalid EMF record at offset {pos}: type=0x{record_type:08x} size={record_size}"
      ));
    }

    if matches!(record_type, EMR_SET_DIBITS_TO_DEVICE | EMR_STRETCH_DIBITS) {
      if bitmap_record
        .replace((record_type, pos, record_size))
        .is_some()
      {
        return Err("multiple EMF bitmap records are not supported yet".into());
      }
    } else if record_type != EMR_EOF {
      return Err(format!(
        "unsupported EMF record type 0x{record_type:08x} at offset {pos}"
      ));
    }

    pos += record_size;
    if record_type == EMR_EOF {
      break;
    }
  }

  let (record_type, record_offset, record_size) =
    bitmap_record.ok_or_else(|| "EMF contains no bitmap record".to_string())?;
  decode_bitmap_record_as_jpeg(data, record_type, record_offset, record_size).map(Some)
}

fn decode_bitmap_record_as_jpeg(
  data: &[u8],
  record_type: u32,
  record_offset: usize,
  _record_size: usize,
) -> Result<Vec<u8>, String> {
  let (off_bmi_src, cb_bmi_src, off_bits_src, cb_bits_src) = match record_type {
    EMR_STRETCH_DIBITS => (
      read_u32(data, record_offset + 48)? as usize,
      read_u32(data, record_offset + 52)? as usize,
      read_u32(data, record_offset + 56)? as usize,
      read_u32(data, record_offset + 60)? as usize,
    ),
    EMR_SET_DIBITS_TO_DEVICE => (
      read_u32(data, record_offset + 48)? as usize,
      read_u32(data, record_offset + 52)? as usize,
      read_u32(data, record_offset + 56)? as usize,
      read_u32(data, record_offset + 60)? as usize,
    ),
    _ => {
      return Err(format!(
        "unsupported EMF bitmap record type 0x{record_type:08x}"
      ));
    }
  };

  let bmi_start = record_offset + off_bmi_src;
  let bits_start = record_offset + off_bits_src;
  let bmi_end = bmi_start
    .checked_add(cb_bmi_src)
    .ok_or_else(|| "bitmap info range overflows".to_string())?;
  let bits_end = bits_start
    .checked_add(cb_bits_src)
    .ok_or_else(|| "bitmap bits range overflows".to_string())?;
  if bmi_end > data.len() || bits_end > data.len() {
    return Err("EMF bitmap record points outside the file".into());
  }
  if cb_bmi_src < 40 {
    return Err("EMF bitmap info header is too small".into());
  }

  let header_size = read_u32(data, bmi_start)? as usize;
  if header_size < 40 {
    return Err(format!("unsupported BITMAPINFOHEADER size: {header_size}"));
  }

  let width = read_i32(data, bmi_start + 4)?;
  let height = read_i32(data, bmi_start + 8)?;
  let planes = read_u16(data, bmi_start + 12)?;
  let bit_count = read_u16(data, bmi_start + 14)?;
  let compression = read_u32(data, bmi_start + 16)?;

  if planes != 1 {
    return Err(format!("unsupported DIB planes value: {planes}"));
  }

  let bits = &data[bits_start..bits_end];
  match compression {
    BI_JPEG => Ok(bits.to_vec()),
    BI_PNG => png_to_jpeg(bits),
    BI_RGB => dib_to_jpeg(bits, width, height, bit_count),
    other => Err(format!("unsupported DIB compression: {other}")),
  }
}

fn dib_to_jpeg(bits: &[u8], width: i32, height: i32, bit_count: u16) -> Result<Vec<u8>, String> {
  if width <= 0 || height == 0 {
    return Err(format!("unsupported DIB size {width}x{height}"));
  }

  let width = width as usize;
  let top_down = height < 0;
  let height_abs = height.unsigned_abs() as usize;

  let bytes_per_pixel = match bit_count {
    24 => 3usize,
    32 => 4usize,
    other => return Err(format!("unsupported BI_RGB bit depth: {other}")),
  };
  let row_stride = (width * bytes_per_pixel).next_multiple_of(4);
  let required_size = row_stride
    .checked_mul(height_abs)
    .ok_or_else(|| "bitmap dimensions overflow".to_string())?;
  if bits.len() < required_size {
    return Err(format!(
      "bitmap payload is truncated: need {required_size} bytes, got {}",
      bits.len()
    ));
  }

  let mut rgb = vec![0u8; width * height_abs * 3];
  for row in 0..height_abs {
    let src_row = if top_down { row } else { height_abs - 1 - row };
    let src_offset = src_row * row_stride;
    let dest_offset = row * width * 3;
    let src = &bits[src_offset..src_offset + row_stride];
    let dest = &mut rgb[dest_offset..dest_offset + width * 3];

    match bit_count {
      24 => {
        for col in 0..width {
          let src_pixel = &src[col * 3..col * 3 + 3];
          let dest_pixel = &mut dest[col * 3..col * 3 + 3];
          dest_pixel[0] = src_pixel[2];
          dest_pixel[1] = src_pixel[1];
          dest_pixel[2] = src_pixel[0];
        }
      }
      32 => {
        for col in 0..width {
          let src_pixel = &src[col * 4..col * 4 + 4];
          let dest_pixel = &mut dest[col * 3..col * 3 + 3];
          dest_pixel[0] = src_pixel[2];
          dest_pixel[1] = src_pixel[1];
          dest_pixel[2] = src_pixel[0];
        }
      }
      _ => unreachable!(),
    }
  }

  rgb_to_jpeg(&rgb, width as u32, height_abs as u32)
}

fn png_to_jpeg(data: &[u8]) -> Result<Vec<u8>, String> {
  let image = image::load_from_memory(data).map_err(|err| err.to_string())?;
  let rgb = image.to_rgb8();
  rgb_to_jpeg(&rgb, rgb.width(), rgb.height())
}

fn rgb_to_jpeg(rgb: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
  let mut output = Vec::new();
  let encoder = JpegEncoder::new_with_quality(&mut output, 75);
  encoder
    .write_image(rgb, width, height, ColorType::Rgb8.into())
    .map_err(|err| err.to_string())?;
  Ok(output)
}

fn is_emf(data: &[u8]) -> bool {
  data.len() >= EMF_HEADER_SIZE
    && matches!(read_u32(data, 0), Ok(1))
    && matches!(read_u32(data, 4), Ok(size) if size as usize == EMF_HEADER_SIZE)
}

fn read_u16(data: &[u8], offset: usize) -> Result<u16, String> {
  let bytes = data
    .get(offset..offset + 2)
    .ok_or_else(|| format!("read past end of buffer at offset {offset}"))?;
  Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32, String> {
  let bytes = data
    .get(offset..offset + 4)
    .ok_or_else(|| format!("read past end of buffer at offset {offset}"))?;
  Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_i32(data: &[u8], offset: usize) -> Result<i32, String> {
  let bytes = data
    .get(offset..offset + 4)
    .ok_or_else(|| format!("read past end of buffer at offset {offset}"))?;
  Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

#[cfg(test)]
mod tests {
  use ooxmlsdk::parts::PartRef;
  use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

  use super::decode_metafile_as_jpeg;

  #[test]
  fn emf_fixture_decodes_to_jpeg() {
    let package =
      WordprocessingDocument::new_from_file("test-data/ooxmlsdk-pdf-test/tdf129085.docx").unwrap();
    let image = package
      .get_all_parts()
      .find_map(|part| match part {
        PartRef::ImagePart(image) if image.path(&package) == Some("word/media/image1.wmf") => {
          Some(image)
        }
        _ => None,
      })
      .unwrap();
    let emf = image.data(&package).unwrap();
    let content_type = image.content_type(&package);

    let jpeg = decode_metafile_as_jpeg(emf, content_type).unwrap().unwrap();
    let decoded = image::load_from_memory(&jpeg).unwrap();
    assert_eq!(decoded.width(), 884);
    assert_eq!(decoded.height(), 925);
  }
}
