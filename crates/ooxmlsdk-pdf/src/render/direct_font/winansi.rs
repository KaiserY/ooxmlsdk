//! Word's missing C1 glyphs use simple TrueType WinAnsi resources.
//!
//! The source scalar and .notdef advance survive layout. PDF Reference 1.6,
//! Appendix D.1, selects the encoded character name independently: unused
//! WinAnsi codes name bullet. The shared font subset supplies that glyph only
//! if it was actually used elsewhere. Replacing source controls with Unicode
//! bullets would lose both the missing-box paint and this subset dependency.

use pdf_writer::types::{FontFlags, UnicodeCmap};
use pdf_writer::writers::WMode;
use pdf_writer::{Filter, Finish, Name, Pdf, Ref};
use skrifa::{FontRef, MetadataProvider};
use subsetter::GlyphRemapper;
use write_fonts::tables::cmap::Cmap;
use write_fonts::types::GlyphId;

use super::{DirectFontFace, FontMetadata, SYSTEM_INFO, deflate, font_descriptor_flags};
use crate::error::{PdfError, Result};

#[derive(Debug)]
pub(super) struct WordWinAnsiFont {
  name: Vec<u8>,
  font: Ref,
  descriptor: Ref,
  cmap: Ref,
  widths: [Option<f32>; 33],
}

impl WordWinAnsiFont {
  pub(super) fn new(name: Vec<u8>, mut alloc: impl FnMut() -> Result<Ref>) -> Result<Self> {
    Ok(Self {
      name,
      font: alloc()?,
      descriptor: alloc()?,
      cmap: alloc()?,
      widths: [None; 33],
    })
  }

  pub(super) fn resource(&self) -> (&[u8], Ref) {
    (&self.name, self.font)
  }

  pub(super) fn register_width(&mut self, code: u8, width: f32) -> Result<()> {
    let slot = code
      .checked_sub(0x7f)
      .and_then(|index| self.widths.get_mut(usize::from(index)))
      .ok_or_else(|| PdfError::Writer("Word control is outside WinAnsi's C1 range".into()))?;
    *slot = Some(width);
    Ok(())
  }

  pub(super) fn write(
    &self,
    pdf: &mut Pdf,
    name: &str,
    metadata: &FontMetadata,
    data: Ref,
  ) -> Result<()> {
    let first = self
      .widths
      .iter()
      .position(Option::is_some)
      .ok_or_else(|| PdfError::Writer("Word WinAnsi resource has no registered control".into()))?;
    let last = self
      .widths
      .iter()
      .rposition(Option::is_some)
      .unwrap_or(first);
    let mut font = pdf.indirect(self.font).dict();
    font
      .pair(Name(b"Type"), Name(b"Font"))
      .pair(Name(b"Subtype"), Name(b"TrueType"))
      .pair(Name(b"BaseFont"), Name(name.as_bytes()))
      .pair(Name(b"Encoding"), Name(b"WinAnsiEncoding"))
      .pair(Name(b"FontDescriptor"), self.descriptor)
      .pair(Name(b"ToUnicode"), self.cmap)
      .pair(Name(b"FirstChar"), (first + 0x7f) as i32)
      .pair(Name(b"LastChar"), (last + 0x7f) as i32);
    font
      .insert(Name(b"Widths"))
      .array()
      .items(self.widths[first..=last].iter().map(|w| w.unwrap_or(0.0)))
      .finish();
    font.finish();
    let mut flags = font_descriptor_flags(metadata);
    flags.remove(FontFlags::SYMBOLIC);
    flags.insert(FontFlags::NON_SYMBOLIC);
    metadata.write_descriptor(pdf, self.descriptor, data, name, flags);
    // Spell out the standard encoding's Unicode interpretation so every
    // emitted text resource has an explicit, nonempty mapping. This does
    // not change WinAnsi's independent glyph selection or normalize input.
    let mut cmap = UnicodeCmap::<u8>::new(Name(b"WordWinAnsi"), SYSTEM_INFO);
    for (index, width) in self.widths.iter().enumerate() {
      if width.is_some() {
        let code = index as u8 + 0x7f;
        cmap.pair(code, winansi_character(code));
      }
    }
    let cmap = deflate(&cmap.finish())?;
    pdf
      .cmap(self.cmap, &cmap)
      .writing_mode(WMode::Horizontal)
      .filter(Filter::FlateDecode);
    Ok(())
  }
}

pub(super) fn with_subset_cmap(
  subset: &[u8],
  source: &DirectFontFace,
  glyphs: &GlyphRemapper,
) -> Result<Vec<u8>> {
  let source = FontRef::from_index(source.data.as_slice(), source.index)
    .map_err(|e| PdfError::Writer(format!("Word WinAnsi source font is invalid: {e}")))?;
  let charmap = source.charmap();
  let mut mappings: Vec<_> = (0x20..=0xff)
    .filter_map(|code| {
      let character = winansi_character(code);
      let original = charmap.map(character)?.to_u32();
      if original == 0 {
        return None;
      }
      let subset = glyphs.get(u16::try_from(original).ok()?)?;
      Some((character, GlyphId::new(u32::from(subset))))
    })
    .collect();
  if mappings.is_empty() {
    // A font used only for a missing control still needs a Unicode encoding
    // subtable. An empty input would give it no encoding records, which some
    // readers reject in favor of a substitute font. Mapping NUL to .notdef
    // provides a valid format 4 table without adding character coverage or
    // changing the subset's glyphs (OpenType 'cmap', Table overview).
    mappings.push(('\0', GlyphId::NOTDEF));
  }
  let cmap = Cmap::from_mappings(mappings)
    .map_err(|e| PdfError::Writer(format!("Word WinAnsi subset cmap conflicts: {e}")))?;
  let subset = write_fonts::read::FontRef::new(subset)
    .map_err(|e| PdfError::Writer(format!("Word WinAnsi subset is invalid: {e}")))?;
  let mut builder = write_fonts::FontBuilder::new();
  builder
    .add_table(&cmap)
    .map_err(|e| PdfError::Writer(format!("Word WinAnsi cmap could not be written: {e}")))?;
  builder.copy_missing_tables(subset);
  Ok(builder.build())
}

fn winansi_character(code: u8) -> char {
  match code {
    0x7f | 0x81 | 0x8d | 0x8f | 0x90 | 0x95 | 0x9d => '\u{2022}',
    0x80 => '\u{20ac}',
    0x82 => '\u{201a}',
    0x83 => '\u{192}',
    0x84 => '\u{201e}',
    0x85 => '\u{2026}',
    0x86 => '\u{2020}',
    0x87 => '\u{2021}',
    0x88 => '\u{2c6}',
    0x89 => '\u{2030}',
    0x8a => '\u{160}',
    0x8b => '\u{2039}',
    0x8c => '\u{152}',
    0x8e => '\u{17d}',
    0x91 => '\u{2018}',
    0x92 => '\u{2019}',
    0x93 => '\u{201c}',
    0x94 => '\u{201d}',
    0x96 => '\u{2013}',
    0x97 => '\u{2014}',
    0x98 => '\u{2dc}',
    0x99 => '\u{2122}',
    0x9a => '\u{161}',
    0x9b => '\u{203a}',
    0x9c => '\u{153}',
    0x9e => '\u{17e}',
    0x9f => '\u{178}',
    0xa0 => ' ',
    0xad => '-',
    _ => char::from(code),
  }
}

#[cfg(test)]
mod tests {
  use skrifa::raw::TableProvider;

  use super::*;

  #[test]
  fn word_winansi_subset_preserves_glyphs_and_requires_actual_bullet_use() {
    let mut resolver = ooxmlsdk_layout::fonts::FontResolver::default();
    for family in ["Arial", "Calibri", "Times New Roman"] {
      let face = resolver
        .cached_text_face(&ooxmlsdk_layout::common::TextStyle {
          font_family: Some(family.into()),
          ..Default::default()
        })
        .unwrap();
      let original = FontRef::from_index(face.data.as_slice(), face.index).unwrap();
      let a = original.charmap().map('A').unwrap().to_u32() as u16;
      let bullet = original.charmap().map('\u{2022}').unwrap().to_u32() as u16;
      for (include_a, include_bullet) in [(false, false), (true, false), (true, true)] {
        let mut glyphs = GlyphRemapper::new();
        let subset_a = include_a.then(|| glyphs.remap(a));
        let subset_bullet = include_bullet.then(|| glyphs.remap(bullet));
        let subset = subsetter::subset(face.data.as_slice(), face.index, &glyphs).unwrap();
        let with_cmap =
          with_subset_cmap(&subset, &DirectFontFace::from_layout(&face), &glyphs).unwrap();
        let before = FontRef::new(&subset).unwrap();
        let after = FontRef::new(&with_cmap).unwrap();
        assert!(!after.cmap().unwrap().encoding_records().is_empty());
        assert_eq!(
          after.charmap().map('A').map(|gid| gid.to_u32()),
          subset_a.map(u32::from)
        );
        assert_eq!(
          after.charmap().map('\u{2022}').map(|gid| gid.to_u32()),
          subset_bullet.map(u32::from)
        );
        assert!(after.charmap().map('\u{81}').is_none());
        for tag in [b"glyf", b"loca", b"hmtx", b"maxp"] {
          let tag = skrifa::raw::types::Tag::new(tag);
          assert_eq!(
            before.data_for_tag(tag).unwrap().as_bytes(),
            after.data_for_tag(tag).unwrap().as_bytes(),
            "{family} {tag}"
          );
        }
      }
    }
  }
}
