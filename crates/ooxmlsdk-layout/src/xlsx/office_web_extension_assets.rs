// Exact 31x31 Microsoft Office fixed-output placeholder shared by the
// Open XML SDK Youtube.xlsx and Bing.xlsx content-add-in fixtures. Both the
// source 96x96 PNG and Office's emitted RGB/soft-mask samples are identical.
const CONTENT_ADD_IN_PLACEHOLDER: &str = "iVBORw0KGgoAAAANSUhEUgAAAB8AAAAfCAMAAAAocOYLAAAAIGNIUk0AAHomAACAhAAA+gAAAIDoAAB1MAAA6mAAADqYAAAXcJy6UTwAAABRUExURQAAAAAYKgBOhwAQHQBMhQBSjwBwxABUkwBuvwBMhQA0WwA+bAApSQBptwBDdABgqAAuUQBJfwBuvwBmsgBXmABBcQBgqAAeNQBHfAByxv///3p0JWgAAAAZdFJOUwA4sCatuv2/+Kx2jF/sl9lqpPfnxZPaRqJvcddwAAAAAWJLR0QadWfkMgAAAAlwSFlzAAALEgAACxIB0t1+/AAAAAd0SU1FB+oHGhIdLVOevtgAAAAldEVYdGRhdGU6Y3JlYXRlADIwMjYtMDctMjZUMTg6Mjk6MTErMDA6MDC6VuUgAAAAJXRFWHRkYXRlOm1vZGlmeQAyMDI2LTA3LTI2VDE4OjI5OjExKzAwOjAwywtdnAAAACh0RVh0ZGF0ZTp0aW1lc3RhbXAAMjAyNi0wNy0yNlQxODoyOTo0NSswMDowMCCxVjQAAABgSURBVCjP3dI5DoAwDERRs++EHeb+F2WPQIldUQC/nCeliYm+FRxJXSx5nPrYC+wcQheZGuNWwryte52nyATP160oOVfHWjF+rvVTTpI35llcvLV9brdxPwAjex0T/a8ZktoZLzxqanIAAAAASUVORK5CYII=";

pub(super) fn content_add_in_placeholder_png() -> Option<Vec<u8>> {
  decode_base64(CONTENT_ADD_IN_PLACEHOLDER)
}

fn decode_base64(encoded: &str) -> Option<Vec<u8>> {
  if !encoded.len().is_multiple_of(4) {
    return None;
  }
  let mut output = Vec::with_capacity(encoded.len() / 4 * 3);
  for encoded_block in encoded.as_bytes().chunks_exact(4) {
    let mut block = [0u8; 4];
    let mut padding = 0usize;
    for (index, byte) in encoded_block.iter().copied().enumerate() {
      if byte == b'=' {
        block[index] = 0;
        padding += 1;
      } else {
        block[index] = match byte {
          b'A'..=b'Z' => byte - b'A',
          b'a'..=b'z' => byte - b'a' + 26,
          b'0'..=b'9' => byte - b'0' + 52,
          b'+' => 62,
          b'/' => 63,
          _ => return None,
        };
      }
    }
    output.push(block[0] << 2 | block[1] >> 4);
    if padding < 2 {
      output.push(block[1] << 4 | block[2] >> 2);
    }
    if padding == 0 {
      output.push(block[2] << 6 | block[3]);
    }
  }
  Some(output)
}
