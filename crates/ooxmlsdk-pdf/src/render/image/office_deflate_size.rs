//! Stock-zlib level-6 byte count for Office image representation selection.
//!
//! Office compares the JPEG cost with a classic zlib stream. A different valid
//! DEFLATE encoder can change that comparison and thereby introduce JPEG loss
//! and interpolation into an otherwise lossless image. Count the native cost
//! independently of flate2's feature-selected backend; actual PDF compression
//! remains the responsibility of the existing writer.
//!
//! This is an altered, Rust, count-only adaptation of zlib's deflate_slow,
//! longest_match, build_tree, scan_tree and _tr_flush_block. It implements only
//! level 6, windowBits 15, memLevel 8, default strategy, and a final flush.
//! It neither produces nor decodes a compressed stream. See:
//! https://github.com/madler/zlib/blob/v1.3.1/deflate.c
//! https://github.com/madler/zlib/blob/v1.3.1/trees.c
//!
//! The three-byte rolling hash, lazy matches, heap tie-breaking, 16,383-token
//! blocks, stored-block availability and bit alignment are all significant.
//! In particular, zlib-rs uses zlib-ng algorithms; its level 6 is not this model.

/*
  Copyright (C) 1995-2026 Jean-loup Gailly and Mark Adler

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

  Jean-loup Gailly        Mark Adler
  jloup@gzip.org          madler@alumni.caltech.edu
*/

const LENGTH_BASE: [usize; 29] = [
  3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
  163, 195, 227, 258,
];
const LENGTH_EXTRA: [usize; 29] = [
  0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [usize; 30] = [
  1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049,
  3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [usize; 30] = [
  0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];
const ORDER: [usize; 19] = [
  16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];
// Preserve zlib's heap ordering, including depth ties and dummy leaves. A
// generic Huffman heap can produce a different code-length header cost.
fn code_lengths(freq: &[usize], max_bits: usize) -> Vec<usize> {
  let n = freq.len();
  let mut f = vec![0; 2 * n + 1];
  f[..n].copy_from_slice(freq);
  let mut depth = vec![0; 2 * n + 1];
  let mut parent = vec![0; 2 * n + 1];
  let mut heap = vec![0];
  let mut max_code = None;
  for (i, &v) in freq.iter().enumerate() {
    if v != 0 {
      heap.push(i);
      max_code = Some(i);
    }
  }
  while heap.len() < 3 {
    let i = match max_code {
      None => 0,
      Some(v) if v < 2 => v + 1,
      _ => 0,
    };
    max_code = Some(max_code.map_or(i, |v| v.max(i)));
    f[i] = 1;
    heap.push(i);
  }
  fn down_heap(heap: &mut [usize], f: &[usize], d: &[usize], mut k: usize) {
    let v = heap[k];
    let mut j = 2 * k;
    while j < heap.len() {
      let smaller = |a: usize, b: usize| f[a] < f[b] || (f[a] == f[b] && d[a] <= d[b]);
      if j + 1 < heap.len() && smaller(heap[j + 1], heap[j]) {
        j += 1;
      }
      if smaller(v, heap[j]) {
        break;
      }
      heap[k] = heap[j];
      k = j;
      j *= 2;
    }
    heap[k] = v;
  }
  for k in (1..heap.len() / 2 + 1).rev() {
    down_heap(&mut heap, &f, &depth, k);
  }
  let mut order = Vec::new();
  let mut node = n;
  loop {
    let a = heap[1];
    let last = heap.pop().unwrap();
    heap[1] = last;
    down_heap(&mut heap, &f, &depth, 1);
    let b = heap[1];
    order.push(a);
    order.push(b);
    f[node] = f[a] + f[b];
    depth[node] = depth[a].max(depth[b]) + 1;
    parent[a] = node;
    parent[b] = node;
    heap[1] = node;
    node += 1;
    down_heap(&mut heap, &f, &depth, 1);
    if heap.len() == 2 {
      break;
    }
  }
  let mut lengths = vec![0; 2 * n + 1];
  let mut counts = vec![0; max_bits + 1];
  let mut overflow = 0i32;
  for &node in order.iter().rev() {
    let mut bits = lengths[parent[node]] + 1;
    if bits > max_bits {
      bits = max_bits;
      overflow += 1;
    }
    lengths[node] = bits;
    if node < n {
      counts[bits] += 1;
    }
  }
  if overflow > 0 {
    while overflow > 0 {
      let mut bits = max_bits - 1;
      while counts[bits] == 0 {
        bits -= 1;
      }
      counts[bits] -= 1;
      counts[bits + 1] += 2;
      counts[max_bits] -= 1;
      overflow -= 2;
    }
    let mut leaves = order.iter().copied().filter(|&x| x < n);
    for bits in (1..=max_bits).rev() {
      for _ in 0..counts[bits] {
        lengths[leaves.next().unwrap()] = bits;
      }
    }
  }
  lengths.truncate(max_code.unwrap() + 1);
  lengths
}
// RFC 1951 code-length runs, with zlib's run partitioning at tree boundaries.
fn count_length_symbols(lengths: &[usize], freq: &mut [usize; 19]) {
  let mut prev = usize::MAX;
  let mut count = 0;
  let (mut max_count, mut min_count) = if lengths[0] == 0 { (138, 3) } else { (7, 4) };
  for (i, &cur) in lengths.iter().enumerate() {
    let next = lengths.get(i + 1).copied().unwrap_or(usize::MAX);
    count += 1;
    if count < max_count && cur == next {
      continue;
    }
    if count < min_count {
      freq[cur] += count;
    } else if cur != 0 {
      if cur != prev {
        freq[cur] += 1;
      }
      freq[16] += 1;
    } else if count <= 10 {
      freq[17] += 1;
    } else {
      freq[18] += 1;
    }
    count = 0;
    prev = cur;
    (max_count, min_count) = if next == 0 {
      (138, 3)
    } else if cur == next {
      (6, 3)
    } else {
      (7, 4)
    };
  }
}
struct BlockSize {
  lit: [usize; 286],
  dist: [usize; 30],
  extra: usize,
  tokens: usize,
  start: usize,
  bits: usize,
}
impl BlockSize {
  fn new() -> Self {
    let mut b = Self {
      lit: [0; 286],
      dist: [0; 30],
      extra: 0,
      tokens: 0,
      start: 0,
      bits: 0,
    };
    b.lit[256] = 1;
    b
  }
  fn literal(&mut self, b: u8) {
    self.lit[b as usize] += 1;
    self.tokens += 1;
  }
  fn match_token(&mut self, len: usize, dist: usize) {
    let l = LENGTH_BASE.partition_point(|&v| v <= len) - 1;
    let d = DIST_BASE.partition_point(|&v| v <= dist) - 1;
    self.lit[257 + l] += 1;
    self.dist[d] += 1;
    self.extra += LENGTH_EXTRA[l] + DIST_EXTRA[d];
    self.tokens += 1;
  }
  fn flush(&mut self, end: usize, window_base: usize) {
    let lit = code_lengths(&self.lit, 15);
    let dist = code_lengths(&self.dist, 15);
    let mut header_freq = [0; 19];
    count_length_symbols(&lit, &mut header_freq);
    count_length_symbols(&dist, &mut header_freq);
    let header = code_lengths(&header_freq, 7);
    let hclen = (4..=19)
      .rev()
      .find(|&n| header.get(ORDER[n - 1]).copied().unwrap_or(0) != 0)
      .unwrap_or(4);
    let dyn_bits = self.extra
      + self.lit.iter().zip(&lit).map(|(f, l)| f * l).sum::<usize>()
      + self
        .dist
        .iter()
        .zip(&dist)
        .map(|(f, l)| f * l)
        .sum::<usize>()
      + header_freq
        .iter()
        .zip(&header)
        .map(|(f, l)| f * l)
        .sum::<usize>()
      + header_freq[16] * 2
      + header_freq[17] * 3
      + header_freq[18] * 7
      + 14
      + 3 * hclen;
    let fixed_bits = self.extra
      + self
        .lit
        .iter()
        .enumerate()
        .map(|(i, &f)| {
          f * match i {
            0..=143 => 8,
            144..=255 => 9,
            256..=279 => 7,
            _ => 8,
          }
        })
        .sum::<usize>()
      + self.dist.iter().sum::<usize>() * 5;
    // zlib compares rounded block costs, then emits unrounded bit lengths.
    let dyn_bytes = (dyn_bits + 3).div_ceil(8);
    let fixed_bytes = (fixed_bits + 3).div_ceil(8);
    let stored = end - self.start;
    // Stored bytes must still exist in the modeled sliding window.
    if self.start >= window_base && stored + 4 <= dyn_bytes.min(fixed_bytes) {
      self.bits = (self.bits + 3).div_ceil(8) * 8 + 32 + stored * 8;
    } else {
      self.bits += 3
        + if fixed_bytes <= dyn_bytes {
          fixed_bits
        } else {
          dyn_bits
        };
    }
    self.lit.fill(0);
    self.lit[256] = 1;
    self.dist.fill(0);
    self.extra = 0;
    self.tokens = 0;
    self.start = end;
  }
}
fn insert_string(input: &[u8], pos: usize, head: &mut [usize], prev: &mut [usize]) -> usize {
  let hash =
    ((input[pos] as usize) << 10 ^ (input[pos + 1] as usize) << 5 ^ input[pos + 2] as usize)
      & 32767;
  let h = head[hash];
  prev[pos & 32767] = h;
  head[hash] = pos;
  h
}
/// Count the complete zlib stream (two-byte header and four-byte Adler trailer).
/// Working memory is bounded independently of the RGB plane length.
pub(super) fn compressed_size(input: &[u8]) -> usize {
  // Absolute positions avoid copying a 64 KiB window. `base` still models
  // each slide: index zero is the NIL sentinel, and old block bytes expire.
  let mut head = vec![0; 32768];
  let mut prev = vec![0; 32768];
  let mut pos = 0;
  let mut base = 0;
  let mut loaded = input.len().min(65536);
  let mut match_len = 2;
  let mut match_start = 0;
  let mut available = false;
  let mut block = BlockSize::new();
  while pos < input.len() {
    if loaded - pos < 262 {
      if pos - base >= 65274 {
        base += 32768;
      }
      loaded = input.len().min(base + 65536);
    }
    let lookahead = loaded - pos;
    let h = if lookahead >= 3 {
      insert_string(input, pos, &mut head, &mut prev)
    } else {
      0
    };
    let prev_len = match_len;
    let prev_match = match_start;
    match_len = 2;
    if h > base && prev_len < 16 && pos - h <= 32506 {
      let limit = pos.saturating_sub(32506).max(base);
      let cap = lookahead.min(258);
      let nice = lookahead.min(128);
      let mut best = prev_len;
      // Level 6: good_length=8, max_lazy=16, nice_length=128, max_chain=128.
      let mut chain = if best >= 8 { 32 } else { 128 };
      let mut candidate = h;
      loop {
        if best < cap
          && input[candidate + best] == input[pos + best]
          && input[candidate + best - 1] == input[pos + best - 1]
          && input[candidate] == input[pos]
          && input[candidate + 1] == input[pos + 1]
        {
          let mut len = 2;
          while len < cap && input[candidate + len] == input[pos + len] {
            len += 1;
          }
          if len > best {
            best = len;
            match_start = candidate;
            if len >= nice {
              break;
            }
          }
        }
        chain -= 1;
        candidate = prev[candidate & 32767];
        if chain == 0 || candidate <= limit {
          break;
        }
      }
      match_len = best.min(lookahead);
      if match_len == 3 && pos - match_start > 4096 {
        match_len = 2;
      }
    }
    if prev_len >= 3 && match_len <= prev_len {
      block.match_token(prev_len, pos - 1 - prev_match);
      let end = pos + prev_len - 1;
      pos += 1;
      while pos < end {
        if pos + 3 <= loaded {
          insert_string(input, pos, &mut head, &mut prev);
        }
        pos += 1;
      }
      available = false;
      match_len = 2;
      // memLevel 8 flushes after 16,383 symbols, including literals/matches.
      if block.tokens == 16383 {
        block.flush(pos, base);
      }
    } else if available {
      block.literal(input[pos - 1]);
      if block.tokens == 16383 {
        block.flush(pos, base);
      }
      pos += 1;
    } else {
      available = true;
      pos += 1;
    }
  }
  if available {
    block.literal(input[pos - 1]);
  }
  block.flush(pos, base);
  6 + block.bits.div_ceil(8)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn stock_zlib_cost_covers_window_and_symbol_boundaries() {
    // Independent oracle: CPython zlib 1.3.1 compress(data, 6), default memory
    // and window sizes. These exercise stored, fixed and dynamic Huffman blocks,
    // hash-window slides, delayed literals and matches spanning block flushes.
    let lengths = [
      0, 1, 2, 3, 4, 257, 258, 259, 16382, 16383, 16384, 32506, 32768, 65274, 65535, 65536, 65537,
      100000, 400000,
    ];
    let expected = [
      [
        8, 9, 10, 11, 12, 12, 12, 12, 39, 39, 39, 53, 52, 85, 84, 84, 85, 120, 410,
      ],
      [
        8, 9, 10, 11, 12, 268, 269, 270, 16393, 16394, 16395, 32522, 32784, 65300, 65561, 65562,
        65563, 100041, 400131,
      ],
      [
        8, 9, 10, 11, 12, 133, 133, 134, 7559, 7559, 7559, 14835, 14953, 29223, 29332, 29331,
        29332, 43974, 171369,
      ],
    ];
    for (kind, expected) in expected.iter().enumerate() {
      for (&len, &cost) in lengths.iter().zip(expected) {
        let mut state = 0x1234_5678u32;
        let data: Vec<_> = (0..len)
          .map(|_| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            match kind {
              0 => 0,
              1 => state as u8,
              _ => (state & 7) as u8,
            }
          })
          .collect();
        assert_eq!(compressed_size(&data), cost, "kind={kind}, length={len}");
      }
    }
  }

  #[test]
  fn empty_distance_tree_has_the_stock_dummy_codes() {
    assert_eq!(code_lengths(&[0; 30], 15), [1, 1]);
    let mut single = [0; 30];
    single[5] = 7;
    assert_eq!(code_lengths(&single, 15), [1, 0, 0, 0, 0, 1]);
  }
}
