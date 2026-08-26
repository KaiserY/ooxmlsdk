use png::{BitDepth, ColorType};

const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

/// The subset of a PNG which a PDF image XObject can consume directly.
///
/// The concatenated IDAT zlib stream is paired with PDF's PNG predictor, while
/// an indexed palette becomes an `/Indexed` color space. The direct writer
/// consumes this validated representation without decoding and recompressing
/// the source samples.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct NativeIndexedPng {
  pub width: u32,
  pub height: u32,
  pub bit_depth: BitDepth,
  pub palette: Vec<u8>,
  pub idat: Vec<u8>,
}

impl NativeIndexedPng {
  pub(super) fn parse(data: &[u8]) -> Option<Self> {
    let mut cursor = data.strip_prefix(PNG_SIGNATURE)?;
    let mut header = None;
    let mut palette = None;
    let mut idat = Vec::new();
    let mut reached_iend = false;

    while !cursor.is_empty() {
      let length = usize::try_from(read_u32(&mut cursor)?).ok()?;
      let kind: [u8; 4] = take(&mut cursor, 4)?.try_into().ok()?;
      let payload = take(&mut cursor, length)?;
      // The ordinary PNG decoder validates the complete stream before a
      // replacement is registered. The bridge only needs to skip the stored
      // CRC while retaining the already-validated IDAT bytes.
      take(&mut cursor, 4)?;

      match &kind {
        b"IHDR" => {
          if header.is_some() || payload.len() != 13 {
            return None;
          }
          let width = u32::from_be_bytes(payload[0..4].try_into().ok()?);
          let height = u32::from_be_bytes(payload[4..8].try_into().ok()?);
          let bit_depth = BitDepth::from_u8(payload[8])?;
          let color_type = ColorType::from_u8(payload[9])?;
          if width == 0
            || height == 0
            || color_type != ColorType::Indexed
            || !matches!(
              bit_depth,
              BitDepth::One | BitDepth::Two | BitDepth::Four | BitDepth::Eight
            )
            || payload[10] != 0
            || payload[11] != 0
            || payload[12] != 0
          {
            return None;
          }
          header = Some((width, height, bit_depth));
        }
        b"PLTE" => {
          let (_, _, bit_depth) = header?;
          let max_entries = 1_usize << bit_depth as usize;
          if palette.is_some()
            || !idat.is_empty()
            || payload.is_empty()
            || payload.len() % 3 != 0
            || payload.len() / 3 > max_entries
          {
            return None;
          }
          palette = Some(payload.to_vec());
        }
        b"tRNS" => return None,
        b"IDAT" => {
          header?;
          palette.as_ref()?;
          idat.extend_from_slice(payload);
        }
        b"IEND" => {
          if !payload.is_empty() {
            return None;
          }
          reached_iend = true;
          break;
        }
        _ if kind[0] & 0x20 == 0 => return None,
        _ => {}
      }
    }

    let (width, height, bit_depth) = header?;
    (reached_iend && !idat.is_empty()).then_some(Self {
      width,
      height,
      bit_depth,
      palette: palette?,
      idat,
    })
  }
}

fn read_u32(data: &mut &[u8]) -> Option<u32> {
  Some(u32::from_be_bytes(take(data, 4)?.try_into().ok()?))
}

fn take<'a>(data: &mut &'a [u8], length: usize) -> Option<&'a [u8]> {
  let (value, rest) = data.split_at_checked(length)?;
  *data = rest;
  Some(value)
}
