//! Safe baseline JPEG encoding for Microsoft fixed-output raster images.
//!
//! Controlled Office matrices agree with the IJG/libjpeg baseline pipeline:
//! integer RGB-to-YCbCr, h2v2 box downsampling, accurate integer FDCT, exact
//! integer quantization, and IJG dummy edge blocks. `jpeg-encoder` deliberately
//! uses reciprocal quantization and materializes lower padding as pixels; both
//! choices can change Office-visible coefficients. This module keeps the
//! generic encoder for other profiles while reproducing the fixed-output path.

use image::RgbaImage;

const ZIGZAG: [usize; 64] = [
  0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
  13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52,
  45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

const LUMA_QUANTIZATION: [u8; 64] = [
  16, 11, 10, 16, 24, 40, 51, 61, 12, 12, 14, 19, 26, 58, 60, 55, 14, 13, 16, 24, 40, 57, 69, 56,
  14, 17, 22, 29, 51, 87, 80, 62, 18, 22, 37, 56, 68, 109, 103, 77, 24, 35, 55, 64, 81, 104, 113,
  92, 49, 64, 78, 87, 103, 121, 120, 101, 72, 92, 95, 98, 112, 100, 103, 99,
];
const CHROMA_QUANTIZATION: [u8; 64] = [
  17, 18, 24, 47, 99, 99, 99, 99, 18, 21, 26, 66, 99, 99, 99, 99, 24, 26, 56, 99, 99, 99, 99, 99,
  47, 66, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
  99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
];

const LUMA_DC_COUNTS: [u8; 16] = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
const CHROMA_DC_COUNTS: [u8; 16] = [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
const DC_VALUES: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const LUMA_AC_COUNTS: [u8; 16] = [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 125];
const CHROMA_AC_COUNTS: [u8; 16] = [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 119];
const LUMA_AC_VALUES: [u8; 162] = [
  0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07,
  0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08, 0x23, 0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0,
  0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28,
  0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
  0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
  0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
  0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
  0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5,
  0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2,
  0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
  0xf9, 0xfa,
];
const CHROMA_AC_VALUES: [u8; 162] = [
  0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71,
  0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0,
  0x15, 0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34, 0xe1, 0x25, 0xf1, 0x17, 0x18, 0x19, 0x1a, 0x26,
  0x27, 0x28, 0x29, 0x2a, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
  0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
  0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
  0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5,
  0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3,
  0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda,
  0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
  0xf9, 0xfa,
];

#[derive(Clone, Copy)]
struct HuffmanCode {
  bits: u16,
  length: u8,
}

struct HuffmanTable {
  codes: [HuffmanCode; 256],
}

impl HuffmanTable {
  fn new(counts: &[u8; 16], values: &[u8]) -> Option<Self> {
    if counts
      .iter()
      .map(|&value| usize::from(value))
      .sum::<usize>()
      != values.len()
    {
      return None;
    }
    let mut codes = [HuffmanCode { bits: 0, length: 0 }; 256];
    let mut code = 0_u32;
    let mut value_index = 0_usize;
    for (index, &count) in counts.iter().enumerate() {
      let length = u8::try_from(index + 1).ok()?;
      for _ in 0..count {
        let symbol = usize::from(*values.get(value_index)?);
        codes[symbol] = HuffmanCode {
          bits: u16::try_from(code).ok()?,
          length,
        };
        code = code.checked_add(1)?;
        value_index += 1;
      }
      if index + 1 < counts.len() {
        code = code.checked_mul(2)?;
      }
    }
    Some(Self { codes })
  }

  fn get(&self, symbol: u8) -> Option<HuffmanCode> {
    let code = self.codes[usize::from(symbol)];
    (code.length != 0).then_some(code)
  }
}

struct BitWriter<'a> {
  output: &'a mut Vec<u8>,
  bits: u32,
  length: u8,
}

impl<'a> BitWriter<'a> {
  fn new(output: &'a mut Vec<u8>) -> Self {
    Self {
      output,
      bits: 0,
      length: 0,
    }
  }

  fn write(&mut self, bits: u16, length: u8) {
    debug_assert!(length <= 16);
    let mask = if length == 16 {
      u32::from(u16::MAX)
    } else {
      (1_u32 << length) - 1
    };
    self.bits = (self.bits << length) | (u32::from(bits) & mask);
    self.length += length;
    while self.length >= 8 {
      self.length -= 8;
      let byte = ((self.bits >> self.length) & 0xff) as u8;
      self.output.push(byte);
      if byte == 0xff {
        self.output.push(0);
      }
      self.bits &= if self.length == 0 {
        0
      } else {
        (1_u32 << self.length) - 1
      };
    }
  }

  fn finish(mut self) {
    if self.length != 0 {
      let padding = 8 - self.length;
      let ones = (1_u16 << padding) - 1;
      self.write(ones, padding);
    }
  }
}

pub(super) fn encode_rgba_h2v2(image: &RgbaImage, quality: u8) -> Option<Vec<u8>> {
  let width = usize::try_from(image.width()).ok()?;
  let height = usize::try_from(image.height()).ok()?;
  if width == 0 || height == 0 || width > usize::from(u16::MAX) || height > usize::from(u16::MAX) {
    return None;
  }
  let ComponentPlanes {
    luma,
    cb,
    cr,
    chroma_width,
    chroma_height,
  } = component_planes(image)?;
  let luma_quantization = scaled_quantization(&LUMA_QUANTIZATION, quality);
  let chroma_quantization = scaled_quantization(&CHROMA_QUANTIZATION, quality);
  let luma_dc = HuffmanTable::new(&LUMA_DC_COUNTS, &DC_VALUES)?;
  let luma_ac = HuffmanTable::new(&LUMA_AC_COUNTS, &LUMA_AC_VALUES)?;
  let chroma_dc = HuffmanTable::new(&CHROMA_DC_COUNTS, &DC_VALUES)?;
  let chroma_ac = HuffmanTable::new(&CHROMA_AC_COUNTS, &CHROMA_AC_VALUES)?;

  let mut output = Vec::new();
  write_headers(
    &mut output,
    image.width() as u16,
    image.height() as u16,
    &luma_quantization,
    &chroma_quantization,
  )?;
  let mut bits = BitWriter::new(&mut output);
  let mut predictors = [0_i16; 3];
  let mcu_columns = width.div_ceil(16);
  let mcu_rows = height.div_ceil(16);
  let luma_block_columns = width.div_ceil(8);
  let luma_block_rows = height.div_ceil(8);

  for mcu_y in 0..mcu_rows {
    for mcu_x in 0..mcu_columns {
      let luma_blocks = luma_mcu_blocks(
        LumaMcuSource {
          plane: &luma,
          width,
          height,
          block_columns: luma_block_columns,
          block_rows: luma_block_rows,
        },
        mcu_x,
        mcu_y,
        &luma_quantization,
      );
      for block in &luma_blocks {
        write_block(&mut bits, block, &mut predictors[0], &luma_dc, &luma_ac)?;
      }
      let cb_block = quantized_block(
        &cb,
        chroma_width,
        chroma_height,
        mcu_x,
        mcu_y,
        &chroma_quantization,
      );
      write_block(
        &mut bits,
        &cb_block,
        &mut predictors[1],
        &chroma_dc,
        &chroma_ac,
      )?;
      let cr_block = quantized_block(
        &cr,
        chroma_width,
        chroma_height,
        mcu_x,
        mcu_y,
        &chroma_quantization,
      );
      write_block(
        &mut bits,
        &cr_block,
        &mut predictors[2],
        &chroma_dc,
        &chroma_ac,
      )?;
    }
  }
  bits.finish();
  output.extend_from_slice(&[0xff, 0xd9]);
  Some(output)
}

struct ComponentPlanes {
  luma: Vec<u8>,
  cb: Vec<u8>,
  cr: Vec<u8>,
  chroma_width: usize,
  chroma_height: usize,
}

fn component_planes(image: &RgbaImage) -> Option<ComponentPlanes> {
  let width = usize::try_from(image.width()).ok()?;
  let height = usize::try_from(image.height()).ok()?;
  let pixels = width.checked_mul(height)?;
  let mut y_plane = Vec::with_capacity(pixels);
  let mut cb_plane = Vec::with_capacity(pixels);
  let mut cr_plane = Vec::with_capacity(pixels);
  for pixel in image.pixels() {
    let (y, cb, cr) = rgb_to_ycbcr(pixel[0], pixel[1], pixel[2]);
    y_plane.push(y);
    cb_plane.push(cb);
    cr_plane.push(cr);
  }

  let chroma_width = width.div_ceil(16).checked_mul(8)?;
  let chroma_height = height.div_ceil(16).checked_mul(8)?;
  let chroma_len = chroma_width.checked_mul(chroma_height)?;
  let mut downsampled_cb = Vec::with_capacity(chroma_len);
  let mut downsampled_cr = Vec::with_capacity(chroma_len);
  for output_y in 0..chroma_height {
    let y0 = (output_y * 2).min(height - 1);
    let y1 = (output_y * 2 + 1).min(height - 1);
    for output_x in 0..chroma_width {
      let x0 = (output_x * 2).min(width - 1);
      let x1 = (output_x * 2 + 1).min(width - 1);
      let bias = if output_x.is_multiple_of(2) { 1 } else { 2 };
      let average = |plane: &[u8]| {
        let sum = u16::from(plane[y0 * width + x0])
          + u16::from(plane[y0 * width + x1])
          + u16::from(plane[y1 * width + x0])
          + u16::from(plane[y1 * width + x1])
          + bias;
        (sum >> 2) as u8
      };
      downsampled_cb.push(average(&cb_plane));
      downsampled_cr.push(average(&cr_plane));
    }
  }
  Some(ComponentPlanes {
    luma: y_plane,
    cb: downsampled_cb,
    cr: downsampled_cr,
    chroma_width,
    chroma_height,
  })
}

fn rgb_to_ycbcr(red: u8, green: u8, blue: u8) -> (u8, u8, u8) {
  let red = i32::from(red);
  let green = i32::from(green);
  let blue = i32::from(blue);
  let y = (19_595 * red + 38_470 * green + 7_471 * blue + 32_768) >> 16;
  let cb = (-11_059 * red - 21_709 * green + 32_768 * blue + (128 << 16) + 32_767) >> 16;
  let cr = (32_768 * red - 27_439 * green - 5_329 * blue + (128 << 16) + 32_767) >> 16;
  (y as u8, cb as u8, cr as u8)
}

fn scaled_quantization(base: &[u8; 64], quality: u8) -> [u8; 64] {
  let quality = u32::from(quality.clamp(1, 100));
  let scale = if quality < 50 {
    5_000 / quality
  } else {
    200 - quality * 2
  };
  base.map(|value| ((u32::from(value) * scale + 50) / 100).clamp(1, 255) as u8)
}

#[derive(Clone, Copy)]
struct LumaMcuSource<'a> {
  plane: &'a [u8],
  width: usize,
  height: usize,
  block_columns: usize,
  block_rows: usize,
}

fn luma_mcu_blocks(
  source: LumaMcuSource<'_>,
  mcu_x: usize,
  mcu_y: usize,
  quantization: &[u8; 64],
) -> [[i16; 64]; 4] {
  let LumaMcuSource {
    plane,
    width,
    height,
    block_columns,
    block_rows,
  } = source;
  let mut blocks = [[0_i16; 64]; 4];
  for vertical in 0..2 {
    for horizontal in 0..2 {
      let index = vertical * 2 + horizontal;
      let block_x = mcu_x * 2 + horizontal;
      let block_y = mcu_y * 2 + vertical;
      if block_x < block_columns && block_y < block_rows {
        blocks[index] = quantized_block(plane, width, height, block_x, block_y, quantization);
      } else if horizontal > 0 && block_y < block_rows {
        blocks[index][0] = blocks[index - 1][0];
      } else {
        // IJG dummy rows repeat the last real block's DC value within each
        // MCU, not a DCT of repeated edge pixels.
        blocks[index][0] = blocks[1][0];
      }
    }
  }
  blocks
}

fn quantized_block(
  plane: &[u8],
  width: usize,
  height: usize,
  block_x: usize,
  block_y: usize,
  quantization: &[u8; 64],
) -> [i16; 64] {
  let mut block = [0_i32; 64];
  for row in 0..8 {
    let y = (block_y * 8 + row).min(height - 1);
    for column in 0..8 {
      let x = (block_x * 8 + column).min(width - 1);
      block[row * 8 + column] = i32::from(plane[y * width + x]) - 128;
    }
  }
  fdct_islow(&mut block);
  let mut output = [0_i16; 64];
  for (index, (&value, &divisor)) in block.iter().zip(quantization).enumerate() {
    output[index] = quantize_exact(value, divisor);
  }
  output
}

fn quantize_exact(value: i32, divisor: u8) -> i16 {
  let divisor = i32::from(divisor) * 8;
  let magnitude = (value.abs() + divisor / 2) / divisor;
  if value < 0 {
    -magnitude as i16
  } else {
    magnitude as i16
  }
}

fn fdct_islow(data: &mut [i32; 64]) {
  const CONST_BITS: u32 = 13;
  const PASS1_BITS: u32 = 2;
  const FIX_0_298631336: i32 = 2_446;
  const FIX_0_390180644: i32 = 3_196;
  const FIX_0_541196100: i32 = 4_433;
  const FIX_0_765366865: i32 = 6_270;
  const FIX_0_899976223: i32 = 7_373;
  const FIX_1_175875602: i32 = 9_633;
  const FIX_1_501321110: i32 = 12_299;
  const FIX_1_847759065: i32 = 15_137;
  const FIX_1_961570560: i32 = 16_069;
  const FIX_2_053119869: i32 = 16_819;
  const FIX_2_562915447: i32 = 20_995;
  const FIX_3_072711026: i32 = 25_172;
  let descale = |value: i32, shift: u32| (value + (1 << (shift - 1))) >> shift;

  let mut workspace = [0_i32; 64];
  for row in 0..8 {
    let base = row * 8;
    let tmp0 = data[base] + data[base + 7];
    let tmp7 = data[base] - data[base + 7];
    let tmp1 = data[base + 1] + data[base + 6];
    let tmp6 = data[base + 1] - data[base + 6];
    let tmp2 = data[base + 2] + data[base + 5];
    let tmp5 = data[base + 2] - data[base + 5];
    let tmp3 = data[base + 3] + data[base + 4];
    let tmp4 = data[base + 3] - data[base + 4];
    let tmp10 = tmp0 + tmp3;
    let tmp13 = tmp0 - tmp3;
    let tmp11 = tmp1 + tmp2;
    let tmp12 = tmp1 - tmp2;
    workspace[base] = (tmp10 + tmp11) << PASS1_BITS;
    workspace[base + 4] = (tmp10 - tmp11) << PASS1_BITS;
    let z1 = (tmp12 + tmp13) * FIX_0_541196100;
    workspace[base + 2] = descale(z1 + tmp13 * FIX_0_765366865, CONST_BITS - PASS1_BITS);
    workspace[base + 6] = descale(z1 - tmp12 * FIX_1_847759065, CONST_BITS - PASS1_BITS);
    let z1 = tmp4 + tmp7;
    let z2 = tmp5 + tmp6;
    let z3 = tmp4 + tmp6;
    let z4 = tmp5 + tmp7;
    let z5 = (z3 + z4) * FIX_1_175875602;
    let tmp4 = tmp4 * FIX_0_298631336;
    let tmp5 = tmp5 * FIX_2_053119869;
    let tmp6 = tmp6 * FIX_3_072711026;
    let tmp7 = tmp7 * FIX_1_501321110;
    let z1 = -z1 * FIX_0_899976223;
    let z2 = -z2 * FIX_2_562915447;
    let z3 = -z3 * FIX_1_961570560 + z5;
    let z4 = -z4 * FIX_0_390180644 + z5;
    workspace[base + 7] = descale(tmp4 + z1 + z3, CONST_BITS - PASS1_BITS);
    workspace[base + 5] = descale(tmp5 + z2 + z4, CONST_BITS - PASS1_BITS);
    workspace[base + 3] = descale(tmp6 + z2 + z3, CONST_BITS - PASS1_BITS);
    workspace[base + 1] = descale(tmp7 + z1 + z4, CONST_BITS - PASS1_BITS);
  }

  for column in 0..8 {
    let tmp0 = workspace[column] + workspace[7 * 8 + column];
    let tmp7 = workspace[column] - workspace[7 * 8 + column];
    let tmp1 = workspace[8 + column] + workspace[6 * 8 + column];
    let tmp6 = workspace[8 + column] - workspace[6 * 8 + column];
    let tmp2 = workspace[2 * 8 + column] + workspace[5 * 8 + column];
    let tmp5 = workspace[2 * 8 + column] - workspace[5 * 8 + column];
    let tmp3 = workspace[3 * 8 + column] + workspace[4 * 8 + column];
    let tmp4 = workspace[3 * 8 + column] - workspace[4 * 8 + column];
    let tmp10 = tmp0 + tmp3;
    let tmp13 = tmp0 - tmp3;
    let tmp11 = tmp1 + tmp2;
    let tmp12 = tmp1 - tmp2;
    data[column] = descale(tmp10 + tmp11, PASS1_BITS);
    data[4 * 8 + column] = descale(tmp10 - tmp11, PASS1_BITS);
    let z1 = (tmp12 + tmp13) * FIX_0_541196100;
    data[2 * 8 + column] = descale(z1 + tmp13 * FIX_0_765366865, CONST_BITS + PASS1_BITS);
    data[6 * 8 + column] = descale(z1 - tmp12 * FIX_1_847759065, CONST_BITS + PASS1_BITS);
    let z1 = tmp4 + tmp7;
    let z2 = tmp5 + tmp6;
    let z3 = tmp4 + tmp6;
    let z4 = tmp5 + tmp7;
    let z5 = (z3 + z4) * FIX_1_175875602;
    let tmp4 = tmp4 * FIX_0_298631336;
    let tmp5 = tmp5 * FIX_2_053119869;
    let tmp6 = tmp6 * FIX_3_072711026;
    let tmp7 = tmp7 * FIX_1_501321110;
    let z1 = -z1 * FIX_0_899976223;
    let z2 = -z2 * FIX_2_562915447;
    let z3 = -z3 * FIX_1_961570560 + z5;
    let z4 = -z4 * FIX_0_390180644 + z5;
    data[7 * 8 + column] = descale(tmp4 + z1 + z3, CONST_BITS + PASS1_BITS);
    data[5 * 8 + column] = descale(tmp5 + z2 + z4, CONST_BITS + PASS1_BITS);
    data[3 * 8 + column] = descale(tmp6 + z2 + z3, CONST_BITS + PASS1_BITS);
    data[8 + column] = descale(tmp7 + z1 + z4, CONST_BITS + PASS1_BITS);
  }
}

fn write_block(
  writer: &mut BitWriter<'_>,
  block: &[i16; 64],
  predictor: &mut i16,
  dc_table: &HuffmanTable,
  ac_table: &HuffmanTable,
) -> Option<()> {
  let difference = block[0].checked_sub(*predictor)?;
  *predictor = block[0];
  let dc_length = coefficient_length(difference);
  let code = dc_table.get(dc_length)?;
  writer.write(code.bits, code.length);
  if dc_length != 0 {
    writer.write(coefficient_bits(difference, dc_length), dc_length);
  }

  let mut zero_run = 0_u8;
  for &natural in &ZIGZAG[1..] {
    let value = block[natural];
    if value == 0 {
      zero_run += 1;
      continue;
    }
    while zero_run >= 16 {
      let code = ac_table.get(0xf0)?;
      writer.write(code.bits, code.length);
      zero_run -= 16;
    }
    let length = coefficient_length(value);
    let symbol = (zero_run << 4) | length;
    let code = ac_table.get(symbol)?;
    writer.write(code.bits, code.length);
    writer.write(coefficient_bits(value, length), length);
    zero_run = 0;
  }
  if zero_run != 0 {
    let code = ac_table.get(0)?;
    writer.write(code.bits, code.length);
  }
  Some(())
}

fn coefficient_length(value: i16) -> u8 {
  let magnitude = value.unsigned_abs();
  (u16::BITS - magnitude.leading_zeros()) as u8
}

fn coefficient_bits(value: i16, length: u8) -> u16 {
  if value >= 0 {
    value as u16
  } else {
    ((1_i32 << length) - 1 + i32::from(value)) as u16
  }
}

fn write_headers(
  output: &mut Vec<u8>,
  width: u16,
  height: u16,
  luma_quantization: &[u8; 64],
  chroma_quantization: &[u8; 64],
) -> Option<()> {
  output.extend_from_slice(&[0xff, 0xd8]);
  // JFIF 1.01 and square-pixel density are the neutral baseline contract.
  write_segment(output, 0xe0, b"JFIF\0\x01\x01\0\0\x01\0\x01\0\0")?;
  write_quantization(output, 0, luma_quantization)?;
  write_quantization(output, 1, chroma_quantization)?;
  let mut frame = vec![8];
  frame.extend_from_slice(&height.to_be_bytes());
  frame.extend_from_slice(&width.to_be_bytes());
  frame.extend_from_slice(&[3, 1, 0x22, 0, 2, 0x11, 1, 3, 0x11, 1]);
  write_segment(output, 0xc0, &frame)?;
  write_huffman(output, 0x00, &LUMA_DC_COUNTS, &DC_VALUES)?;
  write_huffman(output, 0x10, &LUMA_AC_COUNTS, &LUMA_AC_VALUES)?;
  write_huffman(output, 0x01, &CHROMA_DC_COUNTS, &DC_VALUES)?;
  write_huffman(output, 0x11, &CHROMA_AC_COUNTS, &CHROMA_AC_VALUES)?;
  write_segment(output, 0xda, &[3, 1, 0x00, 2, 0x11, 3, 0x11, 0, 63, 0])?;
  Some(())
}

fn write_quantization(output: &mut Vec<u8>, index: u8, table: &[u8; 64]) -> Option<()> {
  let mut payload = Vec::with_capacity(65);
  payload.push(index);
  payload.extend(ZIGZAG.iter().map(|&natural| table[natural]));
  write_segment(output, 0xdb, &payload)
}

fn write_huffman(
  output: &mut Vec<u8>,
  descriptor: u8,
  counts: &[u8; 16],
  values: &[u8],
) -> Option<()> {
  let mut payload = Vec::with_capacity(17 + values.len());
  payload.push(descriptor);
  payload.extend_from_slice(counts);
  payload.extend_from_slice(values);
  write_segment(output, 0xc4, &payload)
}

fn write_segment(output: &mut Vec<u8>, marker: u8, payload: &[u8]) -> Option<()> {
  let length = u16::try_from(payload.len().checked_add(2)?).ok()?;
  output.extend_from_slice(&[0xff, marker]);
  output.extend_from_slice(&length.to_be_bytes());
  output.extend_from_slice(payload);
  Some(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exact_quantization_matches_ijg_division_at_reciprocal_boundaries() {
    assert_eq!(quantize_exact(-3_239, 6), -67);
    assert_eq!(quantize_exact(-409, 34), -2);
    assert_eq!(quantize_exact(1_452, 11), 17);
    assert_eq!(quantize_exact(-1_628, 11), -19);
  }

  #[test]
  fn lower_dummy_blocks_copy_the_last_real_dc() {
    let plane = (0..16 * 8)
      .map(|index| (index % 251) as u8)
      .collect::<Vec<_>>();
    let quantization = scaled_quantization(&LUMA_QUANTIZATION, 75);
    let blocks = luma_mcu_blocks(
      LumaMcuSource {
        plane: &plane,
        width: 16,
        height: 8,
        block_columns: 2,
        block_rows: 1,
      },
      0,
      0,
      &quantization,
    );
    assert_eq!(blocks[2][0], blocks[1][0]);
    assert_eq!(blocks[3][0], blocks[1][0]);
    assert!(blocks[2][1..].iter().all(|&value| value == 0));
    assert!(blocks[3][1..].iter().all(|&value| value == 0));
  }

  #[test]
  fn encoded_baseline_round_trips_through_islow_decoder() {
    let image = RgbaImage::from_fn(19, 17, |x, y| {
      image::Rgba([
        (x * 11 + y * 3) as u8,
        (x * 5 + y * 13) as u8,
        (x * 7 + y * 17) as u8,
        255,
      ])
    });
    let jpeg = encode_rgba_h2v2(&image, 75).unwrap();
    let decoded = super::super::jpeg_islow::decode_rgb(&jpeg).unwrap();
    assert_eq!(decoded.dimensions(), image.dimensions());
  }
}
