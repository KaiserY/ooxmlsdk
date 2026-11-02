//! Optional application proofing data. Dictionary selection is independent of
//! document formatting and the availability of bundled language providers.

use std::{fmt, ops::Range, sync::Arc};

/// English pattern data from a Microsoft Word installation. This immutable
/// reader operates entirely in Rust; it does not load or call native code.
///
/// Callers supply their own `RESOURCE_PRM` resource from `MSHY7EN.LEX`.
/// No Office dictionary data is bundled with this crate. Unsupported resources
/// or word encodings retain the ordinary automatic-hyphenation provider.
#[derive(Clone, Eq, PartialEq)]
pub struct EnglishHyphenationDictionary {
  data: Arc<[u8]>,
  trie: Range<usize>,
  sequences: Range<usize>,
  character_ranges: Vec<CharacterRange>,
  character_data: usize,
  character_width: usize,
  ascii: [Option<u32>; 128],
  root: usize,
  sequence_width: usize,
  sequence_count: usize,
  sequence_data: usize,
}

impl fmt::Debug for EnglishHyphenationDictionary {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("EnglishHyphenationDictionary")
      .field("resource_bytes", &self.data.len())
      .field("sequence_count", &self.sequence_count)
      .finish_non_exhaustive()
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CharacterRange {
  first: u32,
  last: u32,
  offset: usize,
}

impl EnglishHyphenationDictionary {
  /// Read the English `RESOURCE_PRM` payload, rather than its enclosing PE
  /// file. The supported format uses case-folded context patterns, `^` word
  /// boundaries, a three-character minimum pattern, and two-letter edges.
  pub fn from_microsoft_resource(data: Arc<[u8]>) -> Option<Self> {
    let count = read(&data, 0, 4)? as usize;
    if !(3..=16).contains(&count) {
      return None;
    }
    let header = 4 + count * 4;
    let mut offsets = (0..count)
      .map(|index| read(&data, 4 + index * 4, 4).map(|offset| offset as usize))
      .collect::<Option<Vec<_>>>()?;
    offsets.push(data.len());
    if offsets[0] < header
      || offsets.windows(2).any(|pair| pair[0] > pair[1])
      || offsets.iter().any(|offset| *offset > data.len())
    {
      return None;
    }
    let trie = offsets[1]..offsets[2];
    let sequences = offsets[2]..offsets[3];
    let t = data.get(trie.clone())?;
    let alphabet_words = (read(t, 8, 4)? & 0x7fff_ffff) as usize;
    let map = 12_usize
      .checked_add(alphabet_words.checked_mul(4)?)?
      .checked_add(4)?;
    let character_width = read(t, map, 4)? as usize;
    if !(1..=4).contains(&character_width) {
      return None;
    }
    let ranges = read(t, map + 4, 4)? as usize;
    let at = map.checked_add(8)?;
    let character_data = at.checked_add(ranges.checked_mul(12)?)?;
    if ranges == 0 || character_data > t.len() {
      return None;
    }
    let mut character_ranges = Vec::with_capacity(ranges);
    let mut character_end = character_data;
    for index in 0..ranges {
      let first = read(t, at + index * 4, 4)?;
      let last = read(t, at + ranges * 4 + index * 8, 4)?;
      let offset = read(t, at + ranges * 4 + index * 8 + 4, 4)? as usize;
      if last < first
        || character_ranges
          .last()
          .is_some_and(|previous: &CharacterRange| previous.last >= first)
      {
        return None;
      }
      let length = usize::try_from(last - first).ok()?.checked_add(1)?;
      let end = character_data
        .checked_add(offset)?
        .checked_add(length.checked_mul(character_width)?)?;
      if end > t.len() {
        return None;
      }
      character_end = character_end.max(end);
      character_ranges.push(CharacterRange {
        first,
        last,
        offset,
      });
    }
    let root = character_end.checked_add(3)? & !3;
    let s = data.get(sequences.clone())?;
    let sequence_count = read(s, 0, 4)? as usize;
    let sequence_width = read(s, 4, 4)? as usize;
    if !(1..=4).contains(&sequence_width) {
      return None;
    }
    let sequence_data =
      8_usize.checked_add(sequence_count.checked_add(1)?.checked_mul(sequence_width)?)?;
    if sequence_data > s.len() {
      return None;
    }
    let mut dictionary = Self {
      data,
      trie,
      sequences,
      character_ranges,
      character_data,
      character_width,
      ascii: [None; 128],
      root,
      sequence_width,
      sequence_count,
      sequence_data,
    };
    dictionary.node(root)?;
    for character in 0..128 {
      dictionary.ascii[character] = dictionary.character(character as u32);
    }
    // Validate all value-table references once, outside the line-fit loop.
    for index in 0..=sequence_count {
      dictionary.values(index)?;
    }
    Some(dictionary)
  }

  fn character(&self, character: u32) -> Option<u32> {
    let index = self
      .character_ranges
      .partition_point(|range| range.first <= character)
      .checked_sub(1)?;
    let range = &self.character_ranges[index];
    if character > range.last {
      return None;
    }
    let at = self.character_data
      + range.offset
      + (character - range.first) as usize * self.character_width;
    read_be(&self.data[self.trie.clone()], at, self.character_width)?.checked_sub(1)
  }

  fn node(&self, state: usize) -> Option<Node<'_>> {
    Node::read(&self.data[self.trie.clone()], state)
  }

  fn values(&self, index: usize) -> Option<&[u8]> {
    if index > self.sequence_count {
      return None;
    }
    let data = &self.data[self.sequences.clone()];
    let offset = read_be(data, 8 + index * self.sequence_width, self.sequence_width)? as usize;
    let at = self.sequence_data.checked_add(offset.checked_sub(1)?)?;
    let length = read(data, at, 4)? as usize;
    if length > 300 {
      return None;
    }
    data.get(at.checked_add(4)?..at.checked_add(4)?.checked_add(length.checked_mul(4)?)?)
  }

  /// UTF-8 offsets for ordinary insertion breaks. Unsupported Unicode case
  /// tables or replacement/cut patterns return `None` for the caller's fallback.
  pub(crate) fn candidate_offsets(&self, word: &str) -> Option<Vec<usize>> {
    if word.len() > 596 {
      return None;
    }
    let mut characters = Vec::with_capacity(word.len() + 2);
    let mut byte_ends = Vec::with_capacity(word.len());
    characters.push(self.ascii[b'^' as usize]);
    for (at, character) in word.char_indices() {
      let value = u32::from(character);
      // These single-character folds are the native English reader's Latin-1
      // fast path. Other scripts retain the bundled provider until their
      // independent case tables and replacement rules are supported.
      if value > 255 || value == 0xb5 {
        return None;
      }
      let folded = if character.is_ascii_uppercase() || (0xc0..=0xde).contains(&value) {
        value + 32
      } else {
        value
      };
      characters.push(if folded < 128 {
        self.ascii[folded as usize]
      } else {
        self.character(folded)
      });
      byte_ends.push(at + character.len_utf8());
    }
    characters.push(self.ascii[b'^' as usize]);
    let length = byte_ends.len();
    if length == 0 || length > 298 {
      return None;
    }
    let mut scores = vec![-1_i8; length];
    for start in 0..characters.len().saturating_sub(2) {
      let mut state = self.root;
      for character in &characters[start..] {
        let Some(next) = character.and_then(|character| self.node(state)?.advance(character))
        else {
          break;
        };
        state = next;
        let node = self.node(state)?;
        if !node.terminal() {
          continue;
        }
        let values = self.values(usize::try_from(node.value()?).ok()?)?;
        for (index, bytes) in values
          .as_chunks::<4>()
          .0
          .iter()
          .enumerate()
          .take(length.saturating_sub(start).saturating_add(1))
          .skip(usize::from(start == 0))
        {
          let value = i32::from_le_bytes(*bytes);
          if value == 7 {
            continue;
          }
          if !matches!(value, 0 | 1) {
            return None;
          }
          let target = scores.get_mut(start + index - 1)?;
          *target = if *target == -1 || i32::from(*target) == value {
            value as i8
          } else {
            -2
          };
        }
      }
    }
    scores[0] = 0;
    if length > 2 {
      scores[length - 2] = 0;
    }
    Some(
      scores
        .into_iter()
        .zip(byte_ends)
        .filter_map(|(value, at)| (value == 1 && at < word.len()).then_some(at))
        .collect(),
    )
  }
}

struct Node<'a> {
  data: &'a [u8],
  state: usize,
  header: u8,
  width: usize,
  count: usize,
  characters: usize,
  highs: usize,
  edges: usize,
  value: usize,
}

impl<'a> Node<'a> {
  fn read(data: &'a [u8], state: usize) -> Option<Self> {
    let header = *data.get(state)?;
    let width = usize::from((header >> 3 & 3) + 1);
    if width == 3 {
      return None;
    }
    let mut at = state.checked_add(1)?;
    let kind = header & 7;
    let mut count = 0;
    let mut characters = at;
    let mut highs = at;
    let mut edges = at;
    match kind {
      1 | 3 | 4 | 7 => {
        count = (read(data, at, width)? as usize).checked_add(1)?;
        characters = at.checked_add(width)?;
        at = characters.checked_add(count.checked_mul(width)?)?;
        highs = if matches!(kind, 1 | 7) {
          let result = at;
          at = at.checked_add(count.checked_mul(width)?)?;
          result
        } else {
          characters
        };
        edges = at;
        at = at.checked_add(count.checked_mul(if matches!(kind, 1 | 4) { 3 } else { 4 })?)?;
      }
      5 | 6 => {
        let low = read(data, at, width)?;
        highs = at.checked_add(width)?;
        let high = read(data, highs, width)?;
        count = usize::try_from(high.checked_sub(low)?)
          .ok()?
          .checked_add(1)?;
        edges = highs.checked_add(width)?;
        at = edges.checked_add(count.checked_mul(if kind == 6 { 3 } else { 4 })?)?;
      }
      2 => {
        count = 1;
        at = at.checked_add(width)?;
      }
      0 => {}
      _ => return None,
    }
    let value_width = match header >> 5 & 3 {
      3 => 4,
      value => usize::from(value),
    };
    data.get(at..at.checked_add(value_width)?)?;
    Some(Self {
      data,
      state,
      header,
      width,
      count,
      characters,
      highs,
      edges,
      value: at,
    })
  }

  fn terminal(&self) -> bool {
    self.header & 128 != 0
  }

  fn value(&self) -> Option<i32> {
    match self.header >> 5 & 3 {
      1 => Some(i32::from(*self.data.get(self.value)? as i8)),
      2 => Some(i32::from(i16::from_le_bytes(
        self.data.get(self.value..self.value + 2)?.try_into().ok()?,
      ))),
      3 => Some(i32::from_le_bytes(
        self.data.get(self.value..self.value + 4)?.try_into().ok()?,
      )),
      _ => None,
    }
  }

  fn advance(&self, character: u32) -> Option<usize> {
    let kind = self.header & 7;
    if kind == 0 {
      return None;
    }
    if kind == 2 {
      return (read(self.data, self.characters, self.width)? == character).then(|| {
        self.value
          + match self.header >> 5 & 3 {
            3 => 4,
            value => usize::from(value),
          }
      });
    }
    let index = if matches!(kind, 5 | 6) {
      let low = read(self.data, self.characters, self.width)?;
      let high = read(self.data, self.highs, self.width)?;
      if character < low || character > high {
        return None;
      }
      (character - low) as usize
    } else {
      let mut low = 0;
      let mut high = self.count;
      while low < high {
        let mid = low + (high - low) / 2;
        if read(self.data, self.characters + mid * self.width, self.width)? <= character {
          low = mid + 1
        } else {
          high = mid
        }
      }
      let index = low.checked_sub(1)?;
      if read(self.data, self.highs + index * self.width, self.width)? < character {
        return None;
      }
      index
    };
    let width = if matches!(kind, 1 | 4 | 6) { 3 } else { 4 };
    let next = read_be(self.data, self.edges + index * width, width)?;
    if matches!(next, 0xffffff | 0x7fffffff) || (matches!(kind, 5 | 6) && next == 0) {
      return None;
    }
    let next = next as usize;
    (next < self.data.len() && next != self.state).then_some(next)
  }
}

fn read(data: &[u8], at: usize, width: usize) -> Option<u32> {
  let bytes = data.get(at..at.checked_add(width)?)?;
  Some(bytes.iter().enumerate().fold(0, |value, (index, byte)| {
    value | u32::from(*byte) << (index * 8)
  }))
}

fn read_be(data: &[u8], at: usize, width: usize) -> Option<u32> {
  let bytes = data.get(at..at.checked_add(width)?)?;
  Some(
    bytes
      .iter()
      .fold(0, |value, byte| value << 8 | u32::from(*byte)),
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BTreeMap;

  // Synthetic contexts exercise the format and conflict rule without copying
  // third-party dictionary contents into implementation fixtures.
  fn resource(patterns: &[(&str, &[i32])]) -> Vec<u8> {
    #[derive(Default)]
    struct Branch {
      children: BTreeMap<u8, usize>,
      value: Option<usize>,
    }
    let mut branches = vec![Branch::default()];
    for (index, (pattern, _)) in patterns.iter().enumerate() {
      let mut node = 0;
      for byte in pattern.bytes() {
        let child = if let Some(child) = branches[node].children.get(&byte) {
          *child
        } else {
          let child = branches.len();
          branches.push(Branch::default());
          branches[node].children.insert(byte, child);
          child
        };
        node = child;
      }
      branches[node].value = Some(index);
    }
    let mut trie = vec![0; 16];
    trie.extend(1_u32.to_le_bytes());
    trie.extend(1_u32.to_le_bytes());
    trie.extend(32_u32.to_le_bytes());
    trie.extend(127_u32.to_le_bytes());
    trie.extend(0_u32.to_le_bytes());
    trie.extend(33_u8..=128);
    while !trie.len().is_multiple_of(4) {
      trie.push(0)
    }
    let mut at = trie.len();
    let offsets = branches
      .iter()
      .map(|node| {
        let offset = at;
        at += 1 + usize::from(node.value.is_some());
        if !node.children.is_empty() {
          at += 1 + node.children.len() * 4
        }
        offset
      })
      .collect::<Vec<_>>();
    for node in &branches {
      let header =
        if node.children.is_empty() { 0 } else { 4 } | if node.value.is_some() { 0xa0 } else { 0 };
      trie.push(header);
      if !node.children.is_empty() {
        trie.push((node.children.len() - 1) as u8);
        trie.extend(node.children.keys());
        for child in node.children.values() {
          trie.extend(&u32::try_from(offsets[*child]).unwrap().to_be_bytes()[1..])
        }
      }
      if let Some(value) = node.value {
        trie.push(value as u8)
      }
    }
    let mut values = Vec::new();
    let mut table = Vec::new();
    for (_, pattern) in patterns {
      table.extend(u32::try_from(values.len() + 1).unwrap().to_be_bytes());
      values.extend(u32::try_from(pattern.len()).unwrap().to_le_bytes());
      for value in *pattern {
        values.extend(value.to_le_bytes())
      }
    }
    let mut sequences = u32::try_from(patterns.len() - 1)
      .unwrap()
      .to_le_bytes()
      .to_vec();
    sequences.extend(4_u32.to_le_bytes());
    sequences.extend(table);
    sequences.extend(values);
    let mut result = 4_u32.to_le_bytes().to_vec();
    let mut position = 20_u32;
    for length in [0, trie.len(), sequences.len(), 0] {
      result.extend(position.to_le_bytes());
      position += u32::try_from(length).unwrap();
    }
    result.extend(trie);
    result.extend(sequences);
    result
  }

  #[test]
  fn application_patterns_preserve_edges_case_and_source_byte_offsets() {
    let dictionary =
      EnglishHyphenationDictionary::from_microsoft_resource(resource(&[("ab", &[7, 1])]).into())
        .unwrap();
    assert_eq!(dictionary.candidate_offsets("abcd"), Some(vec![2]));
    assert_eq!(dictionary.candidate_offsets("ABCD"), Some(vec![2]));
    assert_eq!(dictionary.candidate_offsets("zabcd"), Some(vec![3]));
    assert_eq!(dictionary.candidate_offsets("\u{00e9}abcd"), Some(vec![4]));
    assert_eq!(dictionary.candidate_offsets("abc"), Some(vec![]));
    assert_eq!(dictionary.candidate_offsets("ab"), Some(vec![]));
    assert_eq!(dictionary.candidate_offsets("\u{0100}abcd"), None);
    assert_eq!(dictionary.candidate_offsets(&"a".repeat(1000)), None);
  }

  #[test]
  fn conflicting_contexts_forbid_a_break_instead_of_using_liang_maximum_weights() {
    let dictionary = EnglishHyphenationDictionary::from_microsoft_resource(
      resource(&[("ab", &[7, 1]), ("abc", &[7, 0, 7])]).into(),
    )
    .unwrap();
    assert_eq!(dictionary.candidate_offsets("abcd"), Some(vec![]));
    assert_eq!(dictionary.candidate_offsets("abzz"), Some(vec![2]));
  }

  #[test]
  fn resource_truncations_and_invalid_table_offsets_are_rejected() {
    let data = resource(&[("ab", &[7, 1])]);
    for length in 0..data.len() {
      assert!(
        EnglishHyphenationDictionary::from_microsoft_resource(Arc::from(&data[..length])).is_none(),
        "prefix {length}"
      );
    }
    for at in [0, 4, 8, 12, 16] {
      let mut corrupt = data.clone();
      corrupt[at..at + 4].fill(255);
      assert!(
        EnglishHyphenationDictionary::from_microsoft_resource(corrupt.into()).is_none(),
        "offset {at}"
      );
    }
  }
}
