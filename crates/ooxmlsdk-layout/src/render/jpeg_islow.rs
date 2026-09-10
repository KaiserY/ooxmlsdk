//! Safe, bounded baseline-JPEG decoding with libjpeg's accurate integer IDCT.
//!
//! Microsoft fixed output decodes source JPEGs through GDI+. Controlled host
//! matrices show that its 8-bit samples agree with libjpeg-turbo. The ordinary
//! Rust decoder intentionally uses stb's faster IDCT, which differs by a few
//! samples. This module translates the baseline sequential path from the local
//! Wine/libjpeg sources (`jidctint.c`, `jdcolor.c`, and `jdsample.c`). Inputs
//! outside that proven path return `None` and retain the general decoder.

use image::{GrayImage, RgbImage};

const MAX_DECODE_BYTES: usize = 512 * 1024 * 1024;
const ZIGZAG_TO_NATURAL: [usize; 64] = [
  0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
  13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52,
  45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

#[derive(Clone, Debug)]
struct HuffmanTable {
  min_code: [i32; 17],
  max_code: [i32; 17],
  value_offset: [usize; 17],
  values: Vec<u8>,
}

impl HuffmanTable {
  fn new(counts: [u8; 16], values: &[u8]) -> Option<Self> {
    let value_count = counts
      .iter()
      .map(|&count| usize::from(count))
      .sum::<usize>();
    if value_count != values.len() || value_count > 256 {
      return None;
    }

    let mut table = Self {
      min_code: [-1; 17],
      max_code: [-1; 17],
      value_offset: [0; 17],
      values: values.to_vec(),
    };
    let mut code = 0_i32;
    let mut offset = 0_usize;
    for (index, &count) in counts.iter().enumerate() {
      let length = index + 1;
      let count = i32::from(count);
      if count > 0 {
        table.min_code[length] = code;
        table.max_code[length] = code.checked_add(count - 1)?;
        table.value_offset[length] = offset;
        offset = offset.checked_add(usize::try_from(count).ok()?)?;
        code = code.checked_add(count)?;
      }
      if code > (1_i32 << length) {
        return None;
      }
      code = code.checked_mul(2)?;
    }
    Some(table)
  }

  fn decode(&self, bits: &mut EntropyBits<'_>) -> Option<u8> {
    let mut code = 0_i32;
    for length in 1..=16 {
      code = code
        .checked_mul(2)?
        .checked_add(i32::from(bits.read_bit()?))?;
      let maximum = self.max_code[length];
      if maximum >= 0 && code >= self.min_code[length] && code <= maximum {
        let index = self.value_offset[length]
          .checked_add(usize::try_from(code - self.min_code[length]).ok()?)?;
        return self.values.get(index).copied();
      }
    }
    None
  }
}

#[derive(Clone, Copy, Debug)]
struct ScanComponent {
  component: usize,
  dc_table: usize,
  ac_table: usize,
}

#[derive(Debug)]
struct Component {
  id: u8,
  horizontal_sampling: usize,
  vertical_sampling: usize,
  quantization_table: usize,
  stride: usize,
  sample_width: usize,
  sample_height: usize,
  data: Vec<u8>,
}

#[derive(Debug)]
struct Frame {
  width: usize,
  height: usize,
  max_horizontal_sampling: usize,
  max_vertical_sampling: usize,
  mcu_columns: usize,
  mcu_rows: usize,
  components: Vec<Component>,
}

#[derive(Debug)]
struct ParsedJpeg<'a> {
  frame: Frame,
  quantization_tables: [Option<[u16; 64]>; 4],
  dc_tables: [Option<HuffmanTable>; 4],
  ac_tables: [Option<HuffmanTable>; 4],
  scan: Vec<ScanComponent>,
  restart_interval: usize,
  entropy: &'a [u8],
  ycbcr: bool,
}

/// Decode a supported baseline YCbCr JPEG using accurate integer IDCT.
/// Unsupported JPEG processes return `None` so callers retain their fallback.
pub fn decode_rgb(data: &[u8]) -> Option<RgbImage> {
  let mut jpeg = parse(data)?;
  if jpeg.frame.components.len() != 3 {
    return None;
  }
  decode_coefficients(&mut jpeg)?;
  interleave_rgb(jpeg.frame)
}

/// Decode a supported baseline grayscale JPEG without introducing RGB samples.
pub fn decode_gray(data: &[u8]) -> Option<GrayImage> {
  let mut jpeg = parse(data)?;
  if jpeg.frame.components.len() != 1 {
    return None;
  }
  decode_coefficients(&mut jpeg)?;
  let frame = jpeg.frame;
  let component = frame.components.first()?;
  let mut samples = Vec::with_capacity(frame.width.checked_mul(frame.height)?);
  for row in 0..frame.height {
    let start = row.checked_mul(component.stride)?;
    samples.extend_from_slice(component.data.get(start..start.checked_add(frame.width)?)?);
  }
  GrayImage::from_raw(
    u32::try_from(frame.width).ok()?,
    u32::try_from(frame.height).ok()?,
    samples,
  )
}

fn parse(data: &[u8]) -> Option<ParsedJpeg<'_>> {
  if !data.starts_with(&[0xff, 0xd8]) {
    return None;
  }

  let mut offset = 2_usize;
  let mut frame = None;
  let mut quantization_tables = [None; 4];
  let mut dc_tables: [Option<HuffmanTable>; 4] = [None, None, None, None];
  let mut ac_tables: [Option<HuffmanTable>; 4] = [None, None, None, None];
  let mut restart_interval = 0_usize;
  let mut has_jfif = false;
  let mut adobe_transform = None;

  loop {
    let marker = next_marker(data, &mut offset)?;
    if marker == 0xd9 {
      return None;
    }
    if marker == 0xda {
      let payload = marker_payload(data, &mut offset)?;
      let frame = frame?;
      let scan = parse_scan(payload, &frame)?;
      let ycbcr = match adobe_transform {
        Some(1) => true,
        Some(_) => false,
        None => {
          has_jfif
            || frame
              .components
              .iter()
              .map(|component| component.id)
              .eq([1, 2, 3])
        }
      };
      if !ycbcr && frame.components.len() != 1 {
        return None;
      }
      return Some(ParsedJpeg {
        frame,
        quantization_tables,
        dc_tables,
        ac_tables,
        scan,
        restart_interval,
        entropy: data.get(offset..)?,
        ycbcr,
      });
    }
    if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
      continue;
    }
    let payload = marker_payload(data, &mut offset)?;
    match marker {
      0xc0 => frame = Some(parse_frame(payload)?),
      // Extended/progressive/lossless/arithmetic processes stay with the
      // established decoder rather than being approximated here.
      0xc1..=0xcf if !matches!(marker, 0xc4 | 0xc8 | 0xcc) => return None,
      0xc4 => parse_huffman_tables(payload, &mut dc_tables, &mut ac_tables)?,
      0xdb => parse_quantization_tables(payload, &mut quantization_tables)?,
      0xdd => {
        restart_interval = usize::from(u16::from_be_bytes(payload.try_into().ok()?));
      }
      0xe0 if payload.starts_with(b"JFIF\0") => has_jfif = true,
      0xee if payload.starts_with(b"Adobe") && payload.len() >= 12 => {
        adobe_transform = Some(payload[11]);
      }
      _ => {}
    }
  }
}

fn next_marker(data: &[u8], offset: &mut usize) -> Option<u8> {
  while data.get(*offset) != Some(&0xff) {
    *offset = offset.checked_add(1)?;
  }
  while data.get(*offset) == Some(&0xff) {
    *offset = offset.checked_add(1)?;
  }
  let marker = *data.get(*offset)?;
  *offset = offset.checked_add(1)?;
  (marker != 0).then_some(marker)
}

fn marker_payload<'a>(data: &'a [u8], offset: &mut usize) -> Option<&'a [u8]> {
  let bytes = data.get(*offset..offset.checked_add(2)?)?;
  let length = usize::from(u16::from_be_bytes([bytes[0], bytes[1]]));
  if length < 2 {
    return None;
  }
  let end = offset.checked_add(length)?;
  let payload = data.get(offset.checked_add(2)?..end)?;
  *offset = end;
  Some(payload)
}

fn parse_frame(payload: &[u8]) -> Option<Frame> {
  if payload.len() < 6 || payload[0] != 8 {
    return None;
  }
  let height = usize::from(u16::from_be_bytes([payload[1], payload[2]]));
  let width = usize::from(u16::from_be_bytes([payload[3], payload[4]]));
  let component_count = usize::from(payload[5]);
  if width == 0
    || height == 0
    || !matches!(component_count, 1 | 3)
    || payload.len() != 6 + component_count * 3
  {
    return None;
  }

  let mut specifications = Vec::with_capacity(component_count);
  for bytes in payload[6..].as_chunks::<3>().0.iter() {
    let horizontal_sampling = usize::from(bytes[1] >> 4);
    let vertical_sampling = usize::from(bytes[1] & 0x0f);
    let quantization_table = usize::from(bytes[2]);
    if horizontal_sampling == 0
      || vertical_sampling == 0
      || horizontal_sampling > 4
      || vertical_sampling > 4
      || quantization_table >= 4
      || specifications
        .iter()
        .any(|specification: &(u8, usize, usize, usize)| specification.0 == bytes[0])
    {
      return None;
    }
    // JPEG noninterleaved scans contain exactly one block per MCU regardless
    // of the frame's sampling factors (IJG jdinput/jcmaster per_scan_setup).
    let (horizontal_sampling, vertical_sampling) = if component_count == 1 {
      (1, 1)
    } else {
      (horizontal_sampling, vertical_sampling)
    };
    specifications.push((
      bytes[0],
      horizontal_sampling,
      vertical_sampling,
      quantization_table,
    ));
  }
  let max_horizontal_sampling = specifications.iter().map(|value| value.1).max()?;
  let max_vertical_sampling = specifications.iter().map(|value| value.2).max()?;
  let mcu_width = max_horizontal_sampling.checked_mul(8)?;
  let mcu_height = max_vertical_sampling.checked_mul(8)?;
  let mcu_columns = width.checked_add(mcu_width - 1)?.checked_div(mcu_width)?;
  let mcu_rows = height
    .checked_add(mcu_height - 1)?
    .checked_div(mcu_height)?;

  let mut components = Vec::with_capacity(component_count);
  let mut allocated = 0_usize;
  for (id, horizontal_sampling, vertical_sampling, quantization_table) in specifications {
    let stride = mcu_columns
      .checked_mul(horizontal_sampling)?
      .checked_mul(8)?;
    let padded_height = mcu_rows.checked_mul(vertical_sampling)?.checked_mul(8)?;
    let length = stride.checked_mul(padded_height)?;
    allocated = allocated.checked_add(length)?;
    if allocated > MAX_DECODE_BYTES {
      return None;
    }
    let sample_width = width
      .checked_mul(horizontal_sampling)?
      .checked_add(max_horizontal_sampling - 1)?
      .checked_div(max_horizontal_sampling)?;
    let sample_height = height
      .checked_mul(vertical_sampling)?
      .checked_add(max_vertical_sampling - 1)?
      .checked_div(max_vertical_sampling)?;
    components.push(Component {
      id,
      horizontal_sampling,
      vertical_sampling,
      quantization_table,
      stride,
      sample_width,
      sample_height,
      data: vec![0; length],
    });
  }

  Some(Frame {
    width,
    height,
    max_horizontal_sampling,
    max_vertical_sampling,
    mcu_columns,
    mcu_rows,
    components,
  })
}

fn parse_scan(payload: &[u8], frame: &Frame) -> Option<Vec<ScanComponent>> {
  let count = usize::from(*payload.first()?);
  if count != frame.components.len() || payload.len() != 1 + count * 2 + 3 {
    return None;
  }
  let tail = payload.get(1 + count * 2..)?;
  if tail != [0, 63, 0] {
    return None;
  }

  let mut scan = Vec::with_capacity(count);
  for bytes in payload[1..1 + count * 2].as_chunks::<2>().0.iter() {
    let component = frame
      .components
      .iter()
      .position(|component| component.id == bytes[0])?;
    let dc_table = usize::from(bytes[1] >> 4);
    let ac_table = usize::from(bytes[1] & 0x0f);
    if dc_table >= 4
      || ac_table >= 4
      || scan
        .iter()
        .any(|entry: &ScanComponent| entry.component == component)
    {
      return None;
    }
    scan.push(ScanComponent {
      component,
      dc_table,
      ac_table,
    });
  }
  Some(scan)
}

fn parse_quantization_tables(
  mut payload: &[u8],
  tables: &mut [Option<[u16; 64]>; 4],
) -> Option<()> {
  while !payload.is_empty() {
    let descriptor = *payload.first()?;
    payload = payload.get(1..)?;
    let precision = descriptor >> 4;
    let index = usize::from(descriptor & 0x0f);
    if index >= tables.len() || precision > 1 {
      return None;
    }
    let bytes_per_value = usize::from(precision) + 1;
    let byte_count = 64_usize.checked_mul(bytes_per_value)?;
    let values = payload.get(..byte_count)?;
    payload = payload.get(byte_count..)?;
    let mut table = [0_u16; 64];
    for (zigzag, &natural) in ZIGZAG_TO_NATURAL.iter().enumerate() {
      table[natural] = if precision == 0 {
        u16::from(values[zigzag])
      } else {
        let offset = zigzag * 2;
        u16::from_be_bytes([values[offset], values[offset + 1]])
      };
    }
    tables[index] = Some(table);
  }
  Some(())
}

fn parse_huffman_tables(
  mut payload: &[u8],
  dc_tables: &mut [Option<HuffmanTable>; 4],
  ac_tables: &mut [Option<HuffmanTable>; 4],
) -> Option<()> {
  while !payload.is_empty() {
    let descriptor = *payload.first()?;
    payload = payload.get(1..)?;
    let class = descriptor >> 4;
    let index = usize::from(descriptor & 0x0f);
    if class > 1 || index >= 4 {
      return None;
    }
    let counts: [u8; 16] = payload.get(..16)?.try_into().ok()?;
    payload = payload.get(16..)?;
    let value_count = counts
      .iter()
      .map(|&count| usize::from(count))
      .sum::<usize>();
    let values = payload.get(..value_count)?;
    payload = payload.get(value_count..)?;
    let table = HuffmanTable::new(counts, values)?;
    if class == 0 {
      dc_tables[index] = Some(table);
    } else {
      ac_tables[index] = Some(table);
    }
  }
  Some(())
}

struct EntropyBits<'a> {
  data: &'a [u8],
  offset: usize,
  byte: u8,
  bits_left: u8,
  pending_marker: Option<u8>,
}

impl<'a> EntropyBits<'a> {
  fn new(data: &'a [u8]) -> Self {
    Self {
      data,
      offset: 0,
      byte: 0,
      bits_left: 0,
      pending_marker: None,
    }
  }

  fn read_bit(&mut self) -> Option<u8> {
    if self.bits_left == 0 {
      self.byte = self.read_byte()?;
      self.bits_left = 8;
    }
    self.bits_left -= 1;
    Some((self.byte >> self.bits_left) & 1)
  }

  fn read_bits(&mut self, count: u8) -> Option<i32> {
    let mut value = 0_i32;
    for _ in 0..count {
      value = value
        .checked_mul(2)?
        .checked_add(i32::from(self.read_bit()?))?;
    }
    Some(value)
  }

  fn read_byte(&mut self) -> Option<u8> {
    let byte = *self.data.get(self.offset)?;
    self.offset += 1;
    if byte != 0xff {
      return Some(byte);
    }
    while self.data.get(self.offset) == Some(&0xff) {
      self.offset += 1;
    }
    let marker = *self.data.get(self.offset)?;
    self.offset += 1;
    if marker == 0 {
      Some(0xff)
    } else {
      self.pending_marker = Some(marker);
      None
    }
  }

  fn consume_restart(&mut self, expected: u8) -> Option<()> {
    self.bits_left = 0;
    let marker = if let Some(marker) = self.pending_marker.take() {
      marker
    } else {
      if *self.data.get(self.offset)? != 0xff {
        return None;
      }
      while self.data.get(self.offset) == Some(&0xff) {
        self.offset += 1;
      }
      let marker = *self.data.get(self.offset)?;
      self.offset += 1;
      marker
    };
    (marker == expected).then_some(())
  }
}

fn decode_coefficients(jpeg: &mut ParsedJpeg<'_>) -> Option<()> {
  if !jpeg.ycbcr && jpeg.frame.components.len() != 1 {
    return None;
  }
  for entry in &jpeg.scan {
    jpeg.quantization_tables[jpeg.frame.components[entry.component].quantization_table]?;
    jpeg.dc_tables[entry.dc_table].as_ref()?;
    jpeg.ac_tables[entry.ac_table].as_ref()?;
  }

  let mut bits = EntropyBits::new(jpeg.entropy);
  let mut dc_predictors = vec![0_i32; jpeg.frame.components.len()];
  let total_mcus = jpeg.frame.mcu_columns.checked_mul(jpeg.frame.mcu_rows)?;
  let mut restart = 0_u8;

  for mcu_index in 0..total_mcus {
    let mcu_x = mcu_index % jpeg.frame.mcu_columns;
    let mcu_y = mcu_index / jpeg.frame.mcu_columns;
    for entry in &jpeg.scan {
      let horizontal_sampling = jpeg.frame.components[entry.component].horizontal_sampling;
      let vertical_sampling = jpeg.frame.components[entry.component].vertical_sampling;
      for vertical in 0..vertical_sampling {
        for horizontal in 0..horizontal_sampling {
          let coefficients = decode_block(
            &mut bits,
            jpeg.dc_tables[entry.dc_table].as_ref()?,
            jpeg.ac_tables[entry.ac_table].as_ref()?,
            &mut dc_predictors[entry.component],
          )?;
          let component = &mut jpeg.frame.components[entry.component];
          let quantization = jpeg.quantization_tables[component.quantization_table].as_ref()?;
          let samples = idct_islow(&coefficients, quantization);
          let block_x = mcu_x
            .checked_mul(horizontal_sampling)?
            .checked_add(horizontal)?;
          let block_y = mcu_y
            .checked_mul(vertical_sampling)?
            .checked_add(vertical)?;
          write_block(component, block_x, block_y, &samples)?;
        }
      }
    }

    if jpeg.restart_interval > 0
      && (mcu_index + 1) % jpeg.restart_interval == 0
      && mcu_index + 1 < total_mcus
    {
      bits.consume_restart(0xd0 + restart)?;
      restart = (restart + 1) & 7;
      dc_predictors.fill(0);
    }
  }
  Some(())
}

fn decode_block(
  bits: &mut EntropyBits<'_>,
  dc_table: &HuffmanTable,
  ac_table: &HuffmanTable,
  dc_predictor: &mut i32,
) -> Option<[i16; 64]> {
  let mut coefficients = [0_i16; 64];
  let dc_length = dc_table.decode(bits)?;
  if dc_length > 16 {
    return None;
  }
  let difference = receive_extend(bits, dc_length)?;
  *dc_predictor = dc_predictor.checked_add(difference)?;
  coefficients[0] = i16::try_from(*dc_predictor).ok()?;

  let mut zigzag = 1_usize;
  while zigzag < 64 {
    let symbol = ac_table.decode(bits)?;
    let run = usize::from(symbol >> 4);
    let length = symbol & 0x0f;
    if length == 0 {
      if run == 0 {
        break;
      }
      if run != 15 {
        return None;
      }
      zigzag = zigzag.checked_add(16)?;
      continue;
    }
    zigzag = zigzag.checked_add(run)?;
    if zigzag >= 64 || length > 16 {
      return None;
    }
    coefficients[ZIGZAG_TO_NATURAL[zigzag]] = i16::try_from(receive_extend(bits, length)?).ok()?;
    zigzag += 1;
  }
  Some(coefficients)
}

fn receive_extend(bits: &mut EntropyBits<'_>, length: u8) -> Option<i32> {
  if length == 0 {
    return Some(0);
  }
  let value = bits.read_bits(length)?;
  let threshold = 1_i32.checked_shl(u32::from(length - 1))?;
  if value < threshold {
    value
      .checked_add(1)?
      .checked_sub(1_i32.checked_shl(u32::from(length))?)
  } else {
    Some(value)
  }
}

fn write_block(
  component: &mut Component,
  block_x: usize,
  block_y: usize,
  samples: &[u8; 64],
) -> Option<()> {
  let start_x = block_x.checked_mul(8)?;
  let start_y = block_y.checked_mul(8)?;
  for row in 0..8 {
    let destination = start_y
      .checked_add(row)?
      .checked_mul(component.stride)?
      .checked_add(start_x)?;
    component
      .data
      .get_mut(destination..destination.checked_add(8)?)?
      .copy_from_slice(&samples[row * 8..row * 8 + 8]);
  }
  Some(())
}

fn idct_islow(coefficients: &[i16; 64], quantization: &[u16; 64]) -> [u8; 64] {
  const CONST_BITS: u32 = 13;
  const PASS1_BITS: u32 = 2;
  const PASS2_BITS: u32 = 5;
  const FIX_0_298631336: i64 = 2_446;
  const FIX_0_390180644: i64 = 3_196;
  const FIX_0_541196100: i64 = 4_433;
  const FIX_0_765366865: i64 = 6_270;
  const FIX_0_899976223: i64 = 7_373;
  const FIX_1_175875602: i64 = 9_633;
  const FIX_1_501321110: i64 = 12_299;
  const FIX_1_847759065: i64 = 15_137;
  const FIX_1_961570560: i64 = 16_069;
  const FIX_2_053119869: i64 = 16_819;
  const FIX_2_562915447: i64 = 20_995;
  const FIX_3_072711026: i64 = 25_172;

  let dequantize = |index: usize| i64::from(coefficients[index]) * i64::from(quantization[index]);
  let mut workspace = [0_i64; 64];

  // Column pass. This follows `jpeg_idct_islow` without algebraic
  // reassociation so every fixed-point rounding boundary stays visible.
  for column in 0..8 {
    if (1..8).all(|row| coefficients[row * 8 + column] == 0) {
      let dc = dequantize(column) << PASS1_BITS;
      for row in 0..8 {
        workspace[row * 8 + column] = dc;
      }
      continue;
    }

    let mut z2 = dequantize(column);
    let mut z3 = dequantize(4 * 8 + column);
    z2 <<= CONST_BITS;
    z3 <<= CONST_BITS;
    z2 += 1_i64 << (CONST_BITS - PASS1_BITS - 1);
    let mut tmp0 = z2 + z3;
    let mut tmp1 = z2 - z3;

    z2 = dequantize(2 * 8 + column);
    z3 = dequantize(6 * 8 + column);
    let mut z1 = (z2 + z3) * FIX_0_541196100;
    let tmp2_even = z1 + z2 * FIX_0_765366865;
    let tmp3_even = z1 - z3 * FIX_1_847759065;
    let tmp10 = tmp0 + tmp2_even;
    let tmp13 = tmp0 - tmp2_even;
    let tmp11 = tmp1 + tmp3_even;
    let tmp12 = tmp1 - tmp3_even;

    tmp0 = dequantize(7 * 8 + column);
    tmp1 = dequantize(5 * 8 + column);
    let mut tmp2 = dequantize(3 * 8 + column);
    let mut tmp3 = dequantize(8 + column);
    z2 = tmp0 + tmp2;
    z3 = tmp1 + tmp3;
    z1 = (z2 + z3) * FIX_1_175875602;
    z2 = z2 * -FIX_1_961570560 + z1;
    z3 = z3 * -FIX_0_390180644 + z1;
    z1 = (tmp0 + tmp3) * -FIX_0_899976223;
    tmp0 = tmp0 * FIX_0_298631336 + z1 + z2;
    tmp3 = tmp3 * FIX_1_501321110 + z1 + z3;
    z1 = (tmp1 + tmp2) * -FIX_2_562915447;
    tmp1 = tmp1 * FIX_2_053119869 + z1 + z3;
    tmp2 = tmp2 * FIX_3_072711026 + z1 + z2;

    let shift = CONST_BITS - PASS1_BITS;
    workspace[column] = (tmp10 + tmp3) >> shift;
    workspace[7 * 8 + column] = (tmp10 - tmp3) >> shift;
    workspace[8 + column] = (tmp11 + tmp2) >> shift;
    workspace[6 * 8 + column] = (tmp11 - tmp2) >> shift;
    workspace[2 * 8 + column] = (tmp12 + tmp1) >> shift;
    workspace[5 * 8 + column] = (tmp12 - tmp1) >> shift;
    workspace[3 * 8 + column] = (tmp13 + tmp0) >> shift;
    workspace[4 * 8 + column] = (tmp13 - tmp0) >> shift;
  }

  let mut output = [0_u8; 64];
  let pass2_offset = (128_i64 << PASS2_BITS) + (1_i64 << (PASS2_BITS - 1));
  for row in 0..8 {
    let base = row * 8;
    let mut z2 = workspace[base] + pass2_offset;
    if workspace[base + 1..base + 8]
      .iter()
      .all(|&value| value == 0)
    {
      let sample = z2 >> PASS2_BITS;
      output[base..base + 8].fill(sample.clamp(0, 255) as u8);
      continue;
    }

    let mut z3 = workspace[base + 4];
    z2 <<= CONST_BITS;
    z3 <<= CONST_BITS;
    let mut tmp0 = z2 + z3;
    let mut tmp1 = z2 - z3;
    z2 = workspace[base + 2];
    z3 = workspace[base + 6];
    let mut z1 = (z2 + z3) * FIX_0_541196100;
    let tmp2_even = z1 + z2 * FIX_0_765366865;
    let tmp3_even = z1 - z3 * FIX_1_847759065;
    let tmp10 = tmp0 + tmp2_even;
    let tmp13 = tmp0 - tmp2_even;
    let tmp11 = tmp1 + tmp3_even;
    let tmp12 = tmp1 - tmp3_even;

    tmp0 = workspace[base + 7];
    tmp1 = workspace[base + 5];
    let mut tmp2 = workspace[base + 3];
    let mut tmp3 = workspace[base + 1];
    z2 = tmp0 + tmp2;
    z3 = tmp1 + tmp3;
    z1 = (z2 + z3) * FIX_1_175875602;
    z2 = z2 * -FIX_1_961570560 + z1;
    z3 = z3 * -FIX_0_390180644 + z1;
    z1 = (tmp0 + tmp3) * -FIX_0_899976223;
    tmp0 = tmp0 * FIX_0_298631336 + z1 + z2;
    tmp3 = tmp3 * FIX_1_501321110 + z1 + z3;
    z1 = (tmp1 + tmp2) * -FIX_2_562915447;
    tmp1 = tmp1 * FIX_2_053119869 + z1 + z3;
    tmp2 = tmp2 * FIX_3_072711026 + z1 + z2;

    let shift = CONST_BITS + PASS2_BITS;
    let samples = [
      (tmp10 + tmp3) >> shift,
      (tmp11 + tmp2) >> shift,
      (tmp12 + tmp1) >> shift,
      (tmp13 + tmp0) >> shift,
      (tmp13 - tmp0) >> shift,
      (tmp12 - tmp1) >> shift,
      (tmp11 - tmp2) >> shift,
      (tmp10 - tmp3) >> shift,
    ];
    for (destination, sample) in output[base..base + 8].iter_mut().zip(samples) {
      *destination = sample.clamp(0, 255) as u8;
    }
  }
  output
}

fn interleave_rgb(frame: Frame) -> Option<RgbImage> {
  let [y_component, cb_component, cr_component]: [Component; 3] =
    frame.components.try_into().ok()?;
  let y = upsample_component(
    &y_component,
    frame.width,
    frame.height,
    frame.max_horizontal_sampling,
    frame.max_vertical_sampling,
  )?;
  let cb = upsample_component(
    &cb_component,
    frame.width,
    frame.height,
    frame.max_horizontal_sampling,
    frame.max_vertical_sampling,
  )?;
  let cr = upsample_component(
    &cr_component,
    frame.width,
    frame.height,
    frame.max_horizontal_sampling,
    frame.max_vertical_sampling,
  )?;
  let pixel_count = frame.width.checked_mul(frame.height)?;
  if y.len() != pixel_count || cb.len() != pixel_count || cr.len() != pixel_count {
    return None;
  }
  let output_len = pixel_count.checked_mul(3)?;
  if output_len > MAX_DECODE_BYTES {
    return None;
  }

  let mut rgb = Vec::with_capacity(output_len);
  for ((y, cb), cr) in y.into_iter().zip(cb).zip(cr) {
    rgb.extend_from_slice(&ycbcr_to_rgb(y, cb, cr));
  }
  RgbImage::from_raw(
    u32::try_from(frame.width).ok()?,
    u32::try_from(frame.height).ok()?,
    rgb,
  )
}

fn upsample_component(
  component: &Component,
  output_width: usize,
  output_height: usize,
  max_horizontal_sampling: usize,
  max_vertical_sampling: usize,
) -> Option<Vec<u8>> {
  if !max_horizontal_sampling.is_multiple_of(component.horizontal_sampling)
    || !max_vertical_sampling.is_multiple_of(component.vertical_sampling)
  {
    return None;
  }
  let horizontal_scale = max_horizontal_sampling / component.horizontal_sampling;
  let vertical_scale = max_vertical_sampling / component.vertical_sampling;
  let length = output_width.checked_mul(output_height)?;
  let mut output = vec![0_u8; length];

  for output_y in 0..output_height {
    let destination = output.get_mut(
      output_y.checked_mul(output_width)?..output_y.checked_add(1)?.checked_mul(output_width)?,
    )?;
    match (horizontal_scale, vertical_scale) {
      (1, 1) => {
        let source = component_row(component, output_y)?;
        destination.copy_from_slice(source.get(..output_width)?);
      }
      (2, 1) => {
        upsample_h2v1(
          component_row(component, output_y)?,
          component.sample_width,
          destination,
        )?;
      }
      (1, 2) => {
        let (near, far) = vertical_rows(component, output_y)?;
        for (destination, (&near, &far)) in destination.iter_mut().zip(near.iter().zip(far)) {
          *destination = ((3 * u32::from(near) + u32::from(far) + 2) >> 2) as u8;
        }
      }
      (2, 2) => {
        let (near, far) = vertical_rows(component, output_y)?;
        upsample_h2v2(near, far, component.sample_width, destination)?;
      }
      _ => {
        let source_y = (output_y / vertical_scale).min(component.sample_height.checked_sub(1)?);
        let source = component_row(component, source_y)?;
        for (x, destination) in destination.iter_mut().enumerate() {
          *destination = *source.get((x / horizontal_scale).min(component.sample_width - 1))?;
        }
      }
    }
  }
  Some(output)
}

fn component_row(component: &Component, row: usize) -> Option<&[u8]> {
  if row >= component.sample_height {
    return None;
  }
  let start = row.checked_mul(component.stride)?;
  component
    .data
    .get(start..start.checked_add(component.sample_width)?)
}

fn vertical_rows(component: &Component, output_row: usize) -> Option<(&[u8], &[u8])> {
  let near_row = (output_row / 2).min(component.sample_height.checked_sub(1)?);
  let far_row = if output_row.is_multiple_of(2) {
    near_row.saturating_sub(1)
  } else {
    near_row
      .checked_add(1)?
      .min(component.sample_height.checked_sub(1)?)
  };
  Some((
    component_row(component, near_row)?,
    component_row(component, far_row)?,
  ))
}

fn upsample_h2v1(input: &[u8], input_width: usize, output: &mut [u8]) -> Option<()> {
  if input_width == 0 || input.len() < input_width {
    return None;
  }
  let mut expanded = vec![0_u8; input_width.checked_mul(2)?];
  if input_width == 1 {
    expanded.fill(input[0]);
  } else {
    expanded[0] = input[0];
    expanded[1] = ((3 * u32::from(input[0]) + u32::from(input[1]) + 2) >> 2) as u8;
    for index in 1..input_width - 1 {
      let sample = 3 * u32::from(input[index]) + 2;
      expanded[index * 2] = ((sample + u32::from(input[index - 1])) >> 2) as u8;
      expanded[index * 2 + 1] = ((sample + u32::from(input[index + 1])) >> 2) as u8;
    }
    expanded[(input_width - 1) * 2] =
      ((3 * u32::from(input[input_width - 1]) + u32::from(input[input_width - 2]) + 2) >> 2) as u8;
    expanded[(input_width - 1) * 2 + 1] = input[input_width - 1];
  }
  output.copy_from_slice(expanded.get(..output.len())?);
  Some(())
}

fn upsample_h2v2(near: &[u8], far: &[u8], input_width: usize, output: &mut [u8]) -> Option<()> {
  if input_width == 0 || near.len() < input_width || far.len() < input_width {
    return None;
  }
  let mut expanded = vec![0_u8; input_width.checked_mul(2)?];
  if input_width == 1 {
    let value = ((3 * u32::from(near[0]) + u32::from(far[0]) + 2) >> 2) as u8;
    expanded.fill(value);
  } else {
    let mut current = 3 * u32::from(near[0]) + u32::from(far[0]);
    expanded[0] = ((current + 2) >> 2) as u8;
    for index in 1..input_width {
      let previous = current;
      current = 3 * u32::from(near[index]) + u32::from(far[index]);
      expanded[index * 2 - 1] = ((3 * previous + current + 8) >> 4) as u8;
      expanded[index * 2] = ((3 * current + previous + 8) >> 4) as u8;
    }
    expanded[input_width * 2 - 1] = ((current + 2) >> 2) as u8;
  }
  output.copy_from_slice(expanded.get(..output.len())?);
  Some(())
}

fn ycbcr_to_rgb(y: u8, cb: u8, cr: u8) -> [u8; 3] {
  const SCALE_BITS: u32 = 16;
  const ONE_HALF: i32 = 1 << (SCALE_BITS - 1);
  let y = i32::from(y);
  let cb = i32::from(cb) - 128;
  let cr = i32::from(cr) - 128;
  let red = y + ((91_881 * cr + ONE_HALF) >> SCALE_BITS);
  let green = y + ((-22_554 * cb - 46_802 * cr + ONE_HALF) >> SCALE_BITS);
  let blue = y + ((116_130 * cb + ONE_HALF) >> SCALE_BITS);
  [
    red.clamp(0, 255) as u8,
    green.clamp(0, 255) as u8,
    blue.clamp(0, 255) as u8,
  ]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn libjpeg_islow_dc_scaling_keeps_the_level_shift_exact() {
    let quantization = [1_u16; 64];
    assert_eq!(idct_islow(&[0; 64], &quantization), [128; 64]);

    let mut coefficients = [0_i16; 64];
    coefficients[0] = 8;
    assert_eq!(idct_islow(&coefficients, &quantization), [129; 64]);
    coefficients[0] = -8;
    assert_eq!(idct_islow(&coefficients, &quantization), [127; 64]);
  }

  #[test]
  fn canonical_huffman_table_rejects_oversubscribed_codes() {
    let mut valid_counts = [0_u8; 16];
    valid_counts[0] = 2;
    assert!(HuffmanTable::new(valid_counts, &[10, 20]).is_some());

    let mut invalid_counts = [0_u8; 16];
    invalid_counts[0] = 3;
    assert!(HuffmanTable::new(invalid_counts, &[1, 2, 3]).is_none());
  }

  #[test]
  fn libjpeg_fancy_h2v1_edges_and_interior_are_independent() {
    let input = [10, 30, 90];
    let mut output = [0_u8; 6];
    upsample_h2v1(&input, input.len(), &mut output).unwrap();
    assert_eq!(output, [10, 15, 25, 45, 75, 90]);
  }
}
