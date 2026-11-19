//! A planning-only view for Word's interleaved Arabic calt/liga stage.
//!
//! HarfRust removes its historical calt/liga pause when rclt is available (see
//! harfbuzz/harfbuzz#1573). Word also interleaves these lookups in older fonts
//! without rclt. An empty rclt feature exposes that stage policy to the planner;
//! every real feature, lookup and glyph remains owned by the original font.
//! The temporary bytes are discarded after compiling the cached plan. They are
//! never used for glyph metrics, substitution, outlines or PDF embedding.

use harfrust::{FontRef, Tag};

fn u16_at(data: &[u8], offset: usize) -> Option<usize> {
  Some(u16::from_be_bytes(data.get(offset..offset.checked_add(2)?)?.try_into().ok()?) as usize)
}

fn u32_at(data: &[u8], offset: usize) -> Option<usize> {
  Some(u32::from_be_bytes(data.get(offset..offset.checked_add(4)?)?.try_into().ok()?) as usize)
}

fn put_u16(data: &mut [u8], offset: usize, value: usize) -> Option<()> {
  data
    .get_mut(offset..offset + 2)?
    .copy_from_slice(&u16::try_from(value).ok()?.to_be_bytes());
  Some(())
}

fn put_u32(data: &mut [u8], offset: usize, value: usize) -> Option<()> {
  data
    .get_mut(offset..offset + 4)?
    .copy_from_slice(&u32::try_from(value).ok()?.to_be_bytes());
  Some(())
}

fn language_features(data: &[u8], offset: usize) -> Option<(usize, Vec<usize>)> {
  // lookupOrder is reserved by OpenType.
  if u16_at(data, offset)? != 0 {
    return None;
  }
  let required = u16_at(data, offset + 2)?;
  let count = u16_at(data, offset + 4)?;
  data.get(offset + 6..offset + 6 + count * 2)?;
  let features = (0..count)
    .map(|index| u16_at(data, offset + 6 + index * 2))
    .collect::<Option<Vec<_>>>()?;
  Some((required, features))
}

fn planning_gsub(original: &[u8]) -> Option<Vec<u8>> {
  let header_size = match (u16_at(original, 0)?, u16_at(original, 2)?) {
    (1, 0) => 10,
    (1, 1) => 14,
    _ => return None,
  };
  let script_list = u16_at(original, 4)?;
  let feature_list = u16_at(original, 6)?;
  let lookup_list = u16_at(original, 8)?;
  let script_count = u16_at(original, script_list)?;
  original.get(script_list + 2..script_list + 2 + script_count * 6)?;
  let arabic_record = (0..script_count)
    .map(|index| script_list + 2 + index * 6)
    .find(|&offset| original.get(offset..offset + 4) == Some(b"arab"))?;
  let arabic = script_list + u16_at(original, arabic_record + 4)?;
  let default_language = u16_at(original, arabic)?;
  let language_count = u16_at(original, arabic + 2)?;
  // Every LangSys offset is 16-bit. Bound the projected metadata before
  // expanding repeated/shared language tables from a compact input font.
  let mut language_bytes = usize::from(u16::MAX).checked_sub(4 + language_count * 6)?;
  let mut read_language = |offset| {
    let count = u16_at(original, offset + 4)?;
    language_bytes = language_bytes.checked_sub(6 + count * 2)?;
    language_features(original, offset)
  };
  original.get(arabic + 4..arabic + 4 + language_count * 6)?;
  let mut languages = Vec::with_capacity(language_count + 1);
  if default_language != 0 {
    let (required, features) = read_language(arabic + default_language)?;
    languages.push((None, required, features));
  }
  for index in 0..language_count {
    let record = arabic + 4 + index * 6;
    let tag: [u8; 4] = original.get(record..record + 4)?.try_into().ok()?;
    let offset = arabic + u16_at(original, record + 4)?;
    let (required, features) = read_language(offset)?;
    languages.push((Some(tag), required, features));
  }
  let feature_count = u16_at(original, feature_list)?;
  original.get(feature_list + 2..feature_list + 2 + feature_count * 6)?;
  let mut records = Vec::with_capacity(feature_count + 1);
  for index in 0..feature_count {
    let record = feature_list + 2 + index * 6;
    let tag: [u8; 4] = original.get(record..record + 4)?.try_into().ok()?;
    records.push((
      tag,
      index,
      Some(feature_list + u16_at(original, record + 4)?),
    ));
  }
  let needs_marker = |features: &[usize]| -> Option<bool> {
    let mut calt = false;
    let mut rclt = false;
    for &index in features {
      match &records.get(index)?.0 {
        b"calt" => calt = true,
        b"rclt" => rclt = true,
        _ => {}
      }
    }
    Some(calt && !rclt)
  };
  let needs_marker = languages
    .iter()
    .map(|(_, _, features)| needs_marker(features))
    .collect::<Option<Vec<_>>>()?;
  if !needs_marker.iter().any(|&needed| needed) {
    return None;
  }

  records.push((*b"rclt", feature_count, None));
  records.sort_by_key(|record| record.0);
  let mut remap = vec![0; records.len()];
  for (index, record) in records.iter().enumerate() {
    remap[record.1] = index;
  }

  // The planner is only used for Arabic. Preserve its default and every named
  // language, including required features. Retain original feature tables and
  // the complete LookupList verbatim in the appended original GSUB bytes.
  let mut scripts = vec![0; 8 + 4 + language_count * 6];
  put_u16(&mut scripts, 0, 1)?;
  scripts[2..6].copy_from_slice(b"arab");
  put_u16(&mut scripts, 6, 8)?;
  put_u16(&mut scripts, 10, language_count)?;
  let mut named_index = 0;
  for ((tag, required, features), add_marker) in languages.iter().zip(needs_marker) {
    let offset = scripts.len() - 8;
    if let Some(tag) = tag {
      let record = 12 + named_index * 6;
      scripts[record..record + 4].copy_from_slice(tag);
      put_u16(&mut scripts, record + 4, offset)?;
      named_index += 1;
    } else {
      put_u16(&mut scripts, 8, offset)?;
    }
    let required = if *required == 0xFFFF {
      0xFFFF
    } else {
      *remap.get(*required)?
    };
    let mut features = features
      .iter()
      .map(|&index| remap.get(index).copied())
      .collect::<Option<Vec<_>>>()?;
    if add_marker {
      features.push(remap[feature_count]);
    }
    features.sort_unstable();
    let start = scripts.len();
    scripts.resize(start + 6 + features.len() * 2, 0);
    put_u16(&mut scripts, start + 2, required)?;
    put_u16(&mut scripts, start + 4, features.len())?;
    for (index, feature) in features.into_iter().enumerate() {
      put_u16(&mut scripts, start + 6 + index * 2, feature)?;
    }
  }
  let feature_offset = header_size + scripts.len();
  let empty_feature = feature_offset + 2 + records.len() * 6;
  let original_offset = empty_feature + 4;
  let mut result = vec![0; original_offset];
  result[..header_size].copy_from_slice(original.get(..header_size)?);
  put_u16(&mut result, 4, header_size)?;
  put_u16(&mut result, 6, feature_offset)?;
  put_u16(&mut result, 8, original_offset + lookup_list)?;
  result[header_size..feature_offset].copy_from_slice(&scripts);
  put_u16(&mut result, feature_offset, records.len())?;
  for (index, (tag, _, table)) in records.iter().enumerate() {
    let record = feature_offset + 2 + index * 6;
    result[record..record + 4].copy_from_slice(tag);
    let table = table.map_or(empty_feature, |offset| original_offset + offset);
    put_u16(&mut result, record + 4, table.checked_sub(feature_offset)?)?;
  }
  result.extend_from_slice(original);
  if header_size == 14 {
    let variations = u32_at(original, 10)?;
    if variations != 0 {
      put_u32(&mut result, 10, original_offset.checked_add(variations)?)?;
      let records_start = variations.checked_add(8)?;
      original.get(variations..records_start)?;
      let count = u32_at(original, variations + 4)?;
      original.get(records_start..records_start.checked_add(count.checked_mul(8)?)?)?;
      for index in 0..count {
        let substitution = u32_at(original, variations + 12 + index * 8)?;
        if substitution == 0 {
          continue;
        }
        let substitution = variations.checked_add(substitution)?;
        original.get(substitution..substitution.checked_add(6)?)?;
        let count = u16_at(original, substitution + 4)?;
        original.get(substitution + 6..substitution + 6 + count * 6)?;
        for index in 0..count {
          let field = substitution + 6 + index * 6;
          put_u16(
            &mut result,
            original_offset + field,
            *remap.get(u16_at(original, field)?)?,
          )?;
        }
      }
    }
  }
  Some(result)
}

/// Build a transient face for compiling the Arabic stage schedule. The returned
/// face has exactly the original lookup IDs; execute its plan on the real face.
pub(crate) fn font_for_plan(face: &FontRef<'_>) -> Option<Vec<u8>> {
  let table = face.table_data(Tag::new(b"GSUB"))?;
  let gsub = planning_gsub(table.as_bytes())?;
  let original = face.data().as_bytes();
  let directory = if let Some(index) = face.ttc_index() {
    u32_at(original, 12 + index as usize * 4)?
  } else {
    0
  };
  let index = face
    .table_directory()
    .table_records()
    .iter()
    .position(|record| record.tag() == Tag::new(b"GSUB"))?;
  let record = directory + 12 + index * 16;
  let mut result = original.to_vec();
  result.resize(result.len().next_multiple_of(4), 0);
  let offset = result.len();
  put_u32(&mut result, record + 8, offset)?;
  put_u32(&mut result, record + 12, gsub.len())?;
  let checksum = gsub.chunks(4).fold(0_u32, |sum, chunk| {
    let mut word = [0; 4];
    word[..chunk.len()].copy_from_slice(chunk);
    sum.wrapping_add(u32::from_be_bytes(word))
  });
  put_u32(&mut result, record + 4, checksum as usize)?;
  result.extend_from_slice(&gsub);
  Some(result)
}
