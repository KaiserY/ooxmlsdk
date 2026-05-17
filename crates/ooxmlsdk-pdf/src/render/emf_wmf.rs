use image::codecs::jpeg::JpegEncoder;
use image::{ColorType, ImageEncoder};

// Source: LibreOffice vcl/source/filter/wmf/emfwr.cxx writes these Win32 EMF
// record ids. The byte offsets below are the EMR_STRETCHDIBITS /
// EMR_SETDIBITSTODEVICE record layout fields.
const EMF_HEADER_SIZE: usize = 108;
const EMF_RECORD_HEADER_SIZE: usize = 8;
const EMR_EOF: u32 = 14;
const EMR_POLYGON: u32 = 3;
const EMR_SET_WINDOW_EXT_EX: u32 = 9;
const EMR_SET_WINDOW_ORG_EX: u32 = 11;
const EMR_SELECT_OBJECT: u32 = 37;
const EMR_CREATE_BRUSH_INDIRECT: u32 = 39;
const EMR_SET_DIBITS_TO_DEVICE: u32 = 80;
const EMR_STRETCH_DIBITS: u32 = 81;
const EMR_BITMAP_INFO_OFFSET_OFFSET: usize = 48;
const EMR_BITMAP_INFO_SIZE_OFFSET: usize = 52;
const EMR_BITMAP_BITS_OFFSET_OFFSET: usize = 56;
const EMR_BITMAP_BITS_SIZE_OFFSET: usize = 60;

// Source: LibreOffice vcl/source/bitmap/dibtools.cxx parses BITMAPINFOHEADER
// values and keeps DIB scanlines aligned to four bytes.
const BITMAPINFOHEADER_SIZE: usize = 40;
const BITMAP_WIDTH_OFFSET: usize = 4;
const BITMAP_HEIGHT_OFFSET: usize = 8;
const BITMAP_PLANES_OFFSET: usize = 12;
const BITMAP_BIT_COUNT_OFFSET: usize = 14;
const BITMAP_COMPRESSION_OFFSET: usize = 16;
const DIB_PLANES: u16 = 1;
const DIB_BIT_COUNT_24: u16 = 24;
const DIB_BIT_COUNT_32: u16 = 32;
const RGB_BYTES_PER_PIXEL: usize = 3;
const BGRA_BYTES_PER_PIXEL: usize = 4;
const DIB_ROW_ALIGNMENT_BYTES: usize = 4;
const BI_RGB: u32 = 0;
const BI_JPEG: u32 = 4;
const BI_PNG: u32 = 5;
const JPEG_QUALITY: u8 = 75;

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

  while pos + EMF_RECORD_HEADER_SIZE <= data.len() {
    let record_type = read_u32(data, pos)?;
    let record_size = read_u32(data, pos + 4)? as usize;
    if record_size < EMF_RECORD_HEADER_SIZE || pos + record_size > data.len() {
      return Err(format!(
        "invalid EMF record at offset {pos}: type=0x{record_type:08x} size={record_size}"
      ));
    }

    if matches!(record_type, EMR_SET_DIBITS_TO_DEVICE | EMR_STRETCH_DIBITS)
      && bitmap_record
        .replace((record_type, pos, record_size))
        .is_some()
    {
      return Err("multiple EMF bitmap records are not supported yet".into());
    }

    pos += record_size;
    if record_type == EMR_EOF {
      break;
    }
  }

  let (record_type, record_offset, record_size) = match bitmap_record {
    Some(record) => record,
    None => return decode_vector_emf_as_jpeg(data).map(Some),
  };
  decode_bitmap_record_as_jpeg(data, record_type, record_offset, record_size).map(Some)
}

#[derive(Clone, Copy, Debug)]
struct EmfColor {
  r: u8,
  g: u8,
  b: u8,
}

#[derive(Clone, Copy, Debug)]
struct EmfPoint {
  x: i32,
  y: i32,
}

#[derive(Clone, Debug)]
struct EmfVectorState {
  width: usize,
  height: usize,
  window_org_x: i32,
  window_org_y: i32,
  window_ext_x: i32,
  window_ext_y: i32,
  brush_colors: std::collections::HashMap<u32, EmfColor>,
  current_brush: Option<EmfColor>,
  rgb: Vec<u8>,
}

impl EmfVectorState {
  fn new(data: &[u8]) -> Result<Self, String> {
    let left = read_i32(data, 8)?;
    let top = read_i32(data, 12)?;
    let right = read_i32(data, 16)?;
    let bottom = read_i32(data, 20)?;
    let width = (right - left + 1).max(1) as usize;
    let height = (bottom - top + 1).max(1) as usize;

    Ok(Self {
      width,
      height,
      window_org_x: 0,
      window_org_y: 0,
      window_ext_x: width as i32,
      window_ext_y: height as i32,
      brush_colors: std::collections::HashMap::new(),
      current_brush: None,
      rgb: vec![255; width * height * RGB_BYTES_PER_PIXEL],
    })
  }

  fn map_point(&self, point: EmfPoint) -> (f32, f32) {
    let scale_x = self.width as f32 / self.window_ext_x.max(1) as f32;
    let scale_y = self.height as f32 / self.window_ext_y.max(1) as f32;
    (
      (point.x - self.window_org_x) as f32 * scale_x,
      (point.y - self.window_org_y) as f32 * scale_y,
    )
  }

  fn fill_polygon(&mut self, points: &[EmfPoint]) {
    let Some(color) = self.current_brush else {
      return;
    };
    if points.len() < 3 {
      return;
    }

    let mapped = points
      .iter()
      .map(|point| self.map_point(*point))
      .collect::<Vec<_>>();
    for y in 0..self.height {
      let scan_y = y as f32 + 0.5;
      let mut intersections = Vec::new();
      for index in 0..mapped.len() {
        let (x1, y1) = mapped[index];
        let (x2, y2) = mapped[(index + 1) % mapped.len()];
        if (y1 <= scan_y && y2 > scan_y) || (y2 <= scan_y && y1 > scan_y) {
          let t = (scan_y - y1) / (y2 - y1);
          intersections.push(x1 + t * (x2 - x1));
        }
      }
      intersections.sort_by(|a, b| a.total_cmp(b));
      for pair in intersections.chunks_exact(2) {
        let start = pair[0].floor().max(0.0) as usize;
        let end = pair[1].ceil().min(self.width as f32) as usize;
        for x in start..end {
          let offset = (y * self.width + x) * RGB_BYTES_PER_PIXEL;
          self.rgb[offset] = color.r;
          self.rgb[offset + 1] = color.g;
          self.rgb[offset + 2] = color.b;
        }
      }
    }
  }
}

fn decode_vector_emf_as_jpeg(data: &[u8]) -> Result<Vec<u8>, String> {
  let mut state = EmfVectorState::new(data)?;
  let mut pos = EMF_HEADER_SIZE;

  while pos + EMF_RECORD_HEADER_SIZE <= data.len() {
    let record_type = read_u32(data, pos)?;
    let record_size = read_u32(data, pos + 4)? as usize;
    if record_size < EMF_RECORD_HEADER_SIZE || pos + record_size > data.len() {
      return Err(format!(
        "invalid EMF record at offset {pos}: type=0x{record_type:08x} size={record_size}"
      ));
    }

    match record_type {
      EMR_SET_WINDOW_ORG_EX if record_size >= 16 => {
        state.window_org_x = read_i32(data, pos + 8)?;
        state.window_org_y = read_i32(data, pos + 12)?;
      }
      EMR_SET_WINDOW_EXT_EX if record_size >= 16 => {
        state.window_ext_x = read_i32(data, pos + 8)?.abs().max(1);
        state.window_ext_y = read_i32(data, pos + 12)?.abs().max(1);
      }
      EMR_CREATE_BRUSH_INDIRECT if record_size >= 24 => {
        let object_id = read_u32(data, pos + 8)?;
        let color_ref = read_u32(data, pos + 16)?;
        state.brush_colors.insert(
          object_id,
          EmfColor {
            r: (color_ref & 0xff) as u8,
            g: ((color_ref >> 8) & 0xff) as u8,
            b: ((color_ref >> 16) & 0xff) as u8,
          },
        );
      }
      EMR_SELECT_OBJECT if record_size >= 12 => {
        let object_id = read_u32(data, pos + 8)?;
        if let Some(brush) = state.brush_colors.get(&object_id).copied() {
          state.current_brush = Some(brush);
        }
      }
      EMR_POLYGON if record_size >= 28 => {
        let count = read_u32(data, pos + 24)? as usize;
        let points_start = pos + 28;
        let points_end = points_start
          .checked_add(count * 8)
          .ok_or_else(|| "EMF polygon points overflow".to_string())?;
        if points_end <= pos + record_size {
          let mut points = Vec::with_capacity(count);
          for index in 0..count {
            let point_offset = points_start + index * 8;
            points.push(EmfPoint {
              x: read_i32(data, point_offset)?,
              y: read_i32(data, point_offset + 4)?,
            });
          }
          state.fill_polygon(&points);
        }
      }
      EMR_EOF => break,
      _ => {}
    }

    pos += record_size;
  }

  rgb_to_jpeg(&state.rgb, state.width as u32, state.height as u32)
}

fn decode_bitmap_record_as_jpeg(
  data: &[u8],
  record_type: u32,
  record_offset: usize,
  _record_size: usize,
) -> Result<Vec<u8>, String> {
  let (off_bmi_src, cb_bmi_src, off_bits_src, cb_bits_src) = match record_type {
    EMR_STRETCH_DIBITS => (
      read_u32(data, record_offset + EMR_BITMAP_INFO_OFFSET_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_INFO_SIZE_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_BITS_OFFSET_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_BITS_SIZE_OFFSET)? as usize,
    ),
    EMR_SET_DIBITS_TO_DEVICE => (
      read_u32(data, record_offset + EMR_BITMAP_INFO_OFFSET_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_INFO_SIZE_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_BITS_OFFSET_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_BITS_SIZE_OFFSET)? as usize,
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
  if cb_bmi_src < BITMAPINFOHEADER_SIZE {
    return Err("EMF bitmap info header is too small".into());
  }

  let header_size = read_u32(data, bmi_start)? as usize;
  if header_size < BITMAPINFOHEADER_SIZE {
    return Err(format!("unsupported BITMAPINFOHEADER size: {header_size}"));
  }

  let width = read_i32(data, bmi_start + BITMAP_WIDTH_OFFSET)?;
  let height = read_i32(data, bmi_start + BITMAP_HEIGHT_OFFSET)?;
  let planes = read_u16(data, bmi_start + BITMAP_PLANES_OFFSET)?;
  let bit_count = read_u16(data, bmi_start + BITMAP_BIT_COUNT_OFFSET)?;
  let compression = read_u32(data, bmi_start + BITMAP_COMPRESSION_OFFSET)?;

  if planes != DIB_PLANES {
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
    DIB_BIT_COUNT_24 => RGB_BYTES_PER_PIXEL,
    DIB_BIT_COUNT_32 => BGRA_BYTES_PER_PIXEL,
    other => return Err(format!("unsupported BI_RGB bit depth: {other}")),
  };
  let row_stride = (width * bytes_per_pixel).next_multiple_of(DIB_ROW_ALIGNMENT_BYTES);
  let required_size = row_stride
    .checked_mul(height_abs)
    .ok_or_else(|| "bitmap dimensions overflow".to_string())?;
  if bits.len() < required_size {
    return Err(format!(
      "bitmap payload is truncated: need {required_size} bytes, got {}",
      bits.len()
    ));
  }

  let mut rgb = vec![0u8; width * height_abs * RGB_BYTES_PER_PIXEL];
  for row in 0..height_abs {
    let src_row = if top_down { row } else { height_abs - 1 - row };
    let src_offset = src_row * row_stride;
    let dest_offset = row * width * RGB_BYTES_PER_PIXEL;
    let src = &bits[src_offset..src_offset + row_stride];
    let dest = &mut rgb[dest_offset..dest_offset + width * RGB_BYTES_PER_PIXEL];

    match bit_count {
      DIB_BIT_COUNT_24 => {
        for col in 0..width {
          let src_pixel =
            &src[col * RGB_BYTES_PER_PIXEL..col * RGB_BYTES_PER_PIXEL + RGB_BYTES_PER_PIXEL];
          let dest_pixel =
            &mut dest[col * RGB_BYTES_PER_PIXEL..col * RGB_BYTES_PER_PIXEL + RGB_BYTES_PER_PIXEL];
          dest_pixel[0] = src_pixel[2];
          dest_pixel[1] = src_pixel[1];
          dest_pixel[2] = src_pixel[0];
        }
      }
      DIB_BIT_COUNT_32 => {
        for col in 0..width {
          let src_pixel =
            &src[col * BGRA_BYTES_PER_PIXEL..col * BGRA_BYTES_PER_PIXEL + BGRA_BYTES_PER_PIXEL];
          let dest_pixel =
            &mut dest[col * RGB_BYTES_PER_PIXEL..col * RGB_BYTES_PER_PIXEL + RGB_BYTES_PER_PIXEL];
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
  let encoder = JpegEncoder::new_with_quality(&mut output, JPEG_QUALITY);
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
  use std::path::PathBuf;

  use ooxmlsdk::parts::PartRef;
  use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

  use super::decode_metafile_as_jpeg;

  #[test]
  fn emf_fixture_decodes_to_jpeg() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("../../test-data/ooxmlsdk-pdf-test/libreoffice/tdf129085.docx");
    let package = WordprocessingDocument::new_from_file(path).unwrap();
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

  #[test]
  fn emf_bitmap_record_fixture_decodes_to_jpeg() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("../../test-data/ooxmlsdk-pdf-test/libreoffice/tdf136841.docx");
    let package = WordprocessingDocument::new_from_file(path).unwrap();
    let image = package
      .get_all_parts()
      .find_map(|part| match part {
        PartRef::ImagePart(image) if image.path(&package) == Some("word/media/image1.emf") => {
          Some(image)
        }
        _ => None,
      })
      .unwrap();
    let emf = image.data(&package).unwrap();
    let content_type = image.content_type(&package);

    let jpeg = decode_metafile_as_jpeg(emf, content_type).unwrap().unwrap();
    let decoded = image::load_from_memory(&jpeg).unwrap();
    assert_eq!(decoded.width(), 76);
    assert_eq!(decoded.height(), 76);
  }
}
