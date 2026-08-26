use std::io::Write as _;
use std::sync::Arc;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use pdf_writer::types::{CidFontType, FontFlags, SystemInfo, UnicodeCmap};
use pdf_writer::writers::WMode;
use pdf_writer::{Filter, Finish, Name, Pdf, Rect, Ref, Str};
use rustc_hash::FxHashMap as HashMap;
use skrifa::instance::{Location, Size};
use skrifa::raw::tables::cff::Cff;
use skrifa::raw::{TableProvider, TopLevelTable};
use skrifa::{FontRef, GlyphId, MetadataProvider};
use subsetter::GlyphRemapper;

use crate::error::{PdfError, Result};
use ooxmlsdk_fonts::FontBytes;
use ooxmlsdk_layout::fonts::FontFaceData;

const IDENTITY_H: &[u8] = b"Identity-H";
const SUBSET_TAG_LEN: usize = 6;
const PDF_UNITS_PER_EM: f32 = 1000.0;
const SYSTEM_INFO: SystemInfo = SystemInfo {
  registry: Str(b"Adobe"),
  ordering: Str(b"Identity"),
  supplement: 0,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FontHandle(usize);

#[derive(Clone, Copy, Debug)]
pub(super) struct RegisteredGlyph {
  pub(super) cid: u16,
  pub(super) natural_advance_pdf_units: f32,
  pub(super) semantic_conflict: bool,
}

#[derive(Debug)]
struct FontObjects {
  type0: Ref,
  cid: Ref,
  descriptor: Ref,
  cmap: Ref,
  data: Ref,
}

#[derive(Debug)]
struct DirectFont {
  face: DirectFontFace,
  resource_name: Vec<u8>,
  objects: FontObjects,
  metadata: FontMetadata,
  glyphs: GlyphRemapper,
  codepoints: HashMap<u16, String>,
  advance_widths: HashMap<u16, f32>,
}

#[derive(Clone, Debug)]
struct DirectFontFace {
  data: Arc<FontBytes>,
  index: u32,
  id: Arc<str>,
  variations: Arc<[DirectFontVariation]>,
}

impl DirectFontFace {
  fn from_layout(face: &FontFaceData) -> Self {
    Self {
      data: face.data.clone(),
      index: face.index,
      id: Arc::from(face.id()),
      variations: Arc::from([]),
    }
  }

  fn key(&self) -> DirectFontKey {
    DirectFontKey {
      id: self.id.clone(),
      index: self.index,
      variations: self
        .variations
        .iter()
        .map(|variation| DirectFontVariationKey {
          tag: variation.tag,
          value_bits: variation.value.to_bits(),
        })
        .collect(),
    }
  }

  fn id(&self) -> &str {
    &self.id
  }

  fn location(&self, face: &FontRef<'_>) -> Location {
    face.axes().location(
      self
        .variations
        .iter()
        .map(|variation| (skrifa::Tag::new(&variation.tag), variation.value)),
    )
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct DirectFontKey {
  id: Arc<str>,
  index: u32,
  variations: Vec<DirectFontVariationKey>,
}

#[derive(Clone, Copy, Debug)]
struct DirectFontVariation {
  tag: [u8; 4],
  value: f32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DirectFontVariationKey {
  tag: [u8; 4],
  value_bits: u32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DirectExternalFontCacheKey {
  database: usize,
  id: usvg::fontdb::ID,
}

#[derive(Clone, Debug)]
struct DirectExternalFontSource {
  data: Arc<FontBytes>,
  index: u32,
  id: Arc<str>,
  units_per_em: f32,
  num_glyphs: u32,
  supported_axes: Vec<[u8; 4]>,
}

#[derive(Clone, Debug)]
struct FontMetadata {
  postscript_name: String,
  is_cff: bool,
  bbox: [f32; 4],
  ascent: f32,
  descent: f32,
  cap_height: f32,
  italic_angle: f32,
  stem_v: f32,
  serif: bool,
  monospaced: bool,
}

#[derive(Debug, Default)]
pub(super) struct DirectFontSet {
  by_face: HashMap<DirectFontKey, usize>,
  fonts: Vec<DirectFont>,
  last_face: Option<(DirectFontKey, usize)>,
  external_sources: HashMap<DirectExternalFontCacheKey, DirectExternalFontSource>,
}

impl DirectFontSet {
  pub(super) fn register_face(
    &mut self,
    face: &FontFaceData,
    alloc: impl FnMut() -> Result<Ref>,
  ) -> Result<FontHandle> {
    self.register_direct_face(DirectFontFace::from_layout(face), alloc)
  }

  fn register_external_face(
    &mut self,
    data: Arc<FontBytes>,
    index: u32,
    id: Arc<str>,
    variations: Arc<[DirectFontVariation]>,
    alloc: impl FnMut() -> Result<Ref>,
  ) -> Result<FontHandle> {
    self.register_direct_face(
      DirectFontFace {
        data,
        index,
        id,
        variations,
      },
      alloc,
    )
  }

  pub(super) fn register_usvg_span_face(
    &mut self,
    database: &Arc<usvg::fontdb::Database>,
    span: &usvg::layout::Span,
    font_id: usvg::fontdb::ID,
    alloc: impl FnMut() -> Result<Ref>,
  ) -> Result<(FontHandle, f32, u32)> {
    self.register_usvg_face(
      database,
      font_id,
      &span.variations,
      span.font_optical_sizing,
      span.font_size.get(),
      alloc,
    )
  }

  pub(super) fn register_usvg_source_face(
    &mut self,
    database: &Arc<usvg::fontdb::Database>,
    font_id: usvg::fontdb::ID,
    font_size: f32,
    alloc: impl FnMut() -> Result<Ref>,
  ) -> Result<(FontHandle, f32, u32)> {
    self.register_usvg_face(
      database,
      font_id,
      &[],
      usvg::FontOpticalSizing::Auto,
      font_size,
      alloc,
    )
  }

  fn register_usvg_face(
    &mut self,
    database: &Arc<usvg::fontdb::Database>,
    font_id: usvg::fontdb::ID,
    declared_variations: &[usvg::FontVariation],
    font_optical_sizing: usvg::FontOpticalSizing,
    font_size: f32,
    alloc: impl FnMut() -> Result<Ref>,
  ) -> Result<(FontHandle, f32, u32)> {
    let cache_key = DirectExternalFontCacheKey {
      database: Arc::as_ptr(database) as usize,
      id: font_id,
    };
    if let std::collections::hash_map::Entry::Vacant(entry) = self.external_sources.entry(cache_key)
    {
      let (source, index) = database.face_source(font_id).ok_or_else(|| {
        PdfError::Writer(format!(
          "OfficeMath SVG font {font_id} is missing from its font database"
        ))
      })?;
      let data = match source {
        usvg::fontdb::Source::Binary(data) => Arc::new(FontBytes::from(data)),
        _ => Arc::new(FontBytes::from(
          database
            .with_face_data(font_id, |bytes, _| bytes.to_vec())
            .ok_or_else(|| {
              PdfError::Writer(format!(
                "OfficeMath SVG font {font_id} data could not be read"
              ))
            })?,
        )),
      };
      let face = FontRef::from_index(data.as_slice(), index).map_err(|error| {
        PdfError::Writer(format!(
          "OfficeMath SVG font {font_id} face {index} could not be parsed: {error}"
        ))
      })?;
      let units_per_em = f32::from(
        face
          .head()
          .map_err(|error| {
            PdfError::Writer(format!(
              "OfficeMath SVG font {font_id} face {index} has no head table: {error}"
            ))
          })?
          .units_per_em(),
      );
      if !units_per_em.is_finite() || units_per_em <= 0.0 {
        return Err(PdfError::Writer(format!(
          "OfficeMath SVG font {font_id} face {index} has an invalid units-per-em value"
        )));
      }
      let num_glyphs = face.glyph_names().num_glyphs();
      let supported_axes = face
        .axes()
        .iter()
        .map(|axis| axis.tag().to_be_bytes())
        .collect();
      let stable_id = Arc::from(format!(
        "office-math-svg-font:{}:{index}",
        uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, data.as_slice())
      ));
      entry.insert(DirectExternalFontSource {
        data,
        index,
        id: stable_id,
        units_per_em,
        num_glyphs,
        supported_axes,
      });
    }

    let source = self
      .external_sources
      .get(&cache_key)
      .expect("OfficeMath SVG font source was inserted above")
      .clone();
    let mut variations = Vec::<DirectFontVariation>::new();
    for variation in declared_variations {
      if !source.supported_axes.contains(&variation.tag) {
        continue;
      }
      if let Some(existing) = variations
        .iter_mut()
        .find(|existing| existing.tag == variation.tag)
      {
        existing.value = variation.value;
      } else {
        variations.push(DirectFontVariation {
          tag: variation.tag,
          value: variation.value,
        });
      }
    }
    const OPTICAL_SIZE_TAG: [u8; 4] = *b"opsz";
    if font_optical_sizing == usvg::FontOpticalSizing::Auto
      && source.supported_axes.contains(&OPTICAL_SIZE_TAG)
      && !variations
        .iter()
        .any(|variation| variation.tag == OPTICAL_SIZE_TAG)
    {
      variations.push(DirectFontVariation {
        tag: OPTICAL_SIZE_TAG,
        value: font_size,
      });
    }
    if variations
      .iter()
      .any(|variation| !variation.value.is_finite())
    {
      return Err(PdfError::Writer(
        "OfficeMath SVG font variation contains a non-finite value".to_string(),
      ));
    }
    variations.sort_by_key(|variation| variation.tag);
    let handle = self.register_external_face(
      source.data,
      source.index,
      source.id,
      variations.into(),
      alloc,
    )?;
    Ok((handle, source.units_per_em, source.num_glyphs))
  }

  fn register_direct_face(
    &mut self,
    face: DirectFontFace,
    mut alloc: impl FnMut() -> Result<Ref>,
  ) -> Result<FontHandle> {
    let key = face.key();
    if let Some((last_key, index)) = self.last_face.as_ref()
      && last_key == &key
    {
      return Ok(FontHandle(*index));
    }

    if let Some(&index) = self.by_face.get(&key) {
      self.last_face = Some((key, index));
      return Ok(FontHandle(index));
    }

    let index = self.fonts.len();
    let metadata = FontMetadata::from_face(&face)?;
    self.fonts.push(DirectFont {
      face: face.clone(),
      resource_name: format!("F{index}").into_bytes(),
      objects: FontObjects {
        type0: alloc()?,
        cid: alloc()?,
        descriptor: alloc()?,
        cmap: alloc()?,
        data: alloc()?,
      },
      metadata,
      glyphs: GlyphRemapper::new(),
      codepoints: HashMap::default(),
      advance_widths: HashMap::default(),
    });
    self.by_face.insert(key.clone(), index);
    self.last_face = Some((key, index));
    Ok(FontHandle(index))
  }

  pub(super) fn register_glyph(
    &mut self,
    handle: FontHandle,
    glyph_id: u32,
    semantic_text: Option<&str>,
  ) -> Result<RegisteredGlyph> {
    let font = self
      .fonts
      .get_mut(handle.0)
      .ok_or_else(|| PdfError::Writer("direct font handle is out of range".to_string()))?;
    let glyph_id = u16::try_from(glyph_id).map_err(|_| {
      PdfError::Writer(format!(
        "OpenType glyph ID {glyph_id} exceeds the CID font range"
      ))
    })?;
    let cid = font.glyphs.remap(glyph_id);
    let semantic_conflict = if let Some(semantic_text) = semantic_text {
      match font.codepoints.get(&cid) {
        Some(existing) => existing != semantic_text,
        None => {
          font.codepoints.insert(cid, semantic_text.to_string());
          false
        }
      }
    } else {
      false
    };

    let natural_advance_pdf_units = if let Some(width) = font.advance_widths.get(&glyph_id) {
      *width
    } else {
      let face =
        FontRef::from_index(font.face.data.as_slice(), font.face.index).map_err(|error| {
          PdfError::Writer(format!(
            "resolved PDF font binary could not be parsed: font_id={} face_index={} error={error}",
            font.face.id(),
            font.face.index
          ))
        })?;
      let location = font.face.location(&face);
      let metrics = face.glyph_metrics(Size::new(PDF_UNITS_PER_EM), &location);
      let width = metrics
        .advance_width(GlyphId::new(u32::from(glyph_id)))
        .unwrap_or(0.0);
      if !width.is_finite() {
        return Err(PdfError::Writer(format!(
          "font {} glyph {glyph_id} has a non-finite advance width",
          font.face.id()
        )));
      }
      font.advance_widths.insert(glyph_id, width);
      width
    };

    Ok(RegisteredGlyph {
      cid,
      natural_advance_pdf_units,
      semantic_conflict,
    })
  }

  pub(super) fn resource(&self, handle: FontHandle) -> Result<(&[u8], Ref)> {
    let font = self
      .fonts
      .get(handle.0)
      .ok_or_else(|| PdfError::Writer("direct font handle is out of range".to_string()))?;
    Ok((&font.resource_name, font.objects.type0))
  }

  pub(super) fn write_objects(&self, pdf: &mut Pdf) -> Result<()> {
    for font in &self.fonts {
      font.write(pdf)?;
    }
    Ok(())
  }
}

impl DirectFont {
  fn write(&self, pdf: &mut Pdf) -> Result<()> {
    let variation_coordinates = self
      .face
      .variations
      .iter()
      .map(|variation| (subsetter::Tag::new(&variation.tag), variation.value))
      .collect::<Vec<_>>();
    let subset = subsetter::subset_with_variations(
      self.face.data.as_slice(),
      self.face.index,
      &variation_coordinates,
      &self.glyphs,
    )
    .map_err(|error| {
      PdfError::Writer(format!(
        "failed to subset PDF font {} at face {}: {error}",
        self.face.id(),
        self.face.index
      ))
    })?;
    let embedded = if self.metadata.is_cff {
      let subset_face = FontRef::new(&subset)
        .map_err(|error| PdfError::Writer(format!("failed to parse CFF font subset: {error}")))?;
      subset_face
        .data_for_tag(Cff::TAG)
        .ok_or_else(|| PdfError::Writer("CFF subset has no CFF table".to_string()))?
        .as_bytes()
    } else {
      subset.as_slice()
    };

    let base_name = self.base_font_name();
    let type0_name = if self.metadata.is_cff {
      format!("{base_name}-Identity-H")
    } else {
      base_name.clone()
    };
    pdf
      .type0_font(self.objects.type0)
      .base_font(Name(type0_name.as_bytes()))
      .encoding_predefined(Name(IDENTITY_H))
      .descendant_font(self.objects.cid)
      .to_unicode(self.objects.cmap);

    let face =
      FontRef::from_index(self.face.data.as_slice(), self.face.index).map_err(|error| {
        PdfError::Writer(format!(
          "resolved PDF font binary could not be parsed while writing widths: {error}"
        ))
      })?;
    let location = self.face.location(&face);
    let metrics = face.glyph_metrics(Size::new(PDF_UNITS_PER_EM), &location);
    let widths = self
      .glyphs
      .remapped_gids()
      .map(|glyph_id| {
        metrics
          .advance_width(GlyphId::new(u32::from(glyph_id)))
          .unwrap_or(0.0)
      })
      .collect::<Vec<_>>();

    let mut cid = pdf.cid_font(self.objects.cid);
    cid
      .subtype(if self.metadata.is_cff {
        CidFontType::Type0
      } else {
        CidFontType::Type2
      })
      .base_font(Name(base_name.as_bytes()))
      .system_info(SYSTEM_INFO)
      .font_descriptor(self.objects.descriptor)
      .default_width(0.0);
    if !self.metadata.is_cff {
      cid.cid_to_gid_map_predefined(Name(b"Identity"));
    }
    cid.widths().consecutive(0, widths).finish();
    cid.finish();

    let mut flags = FontFlags::empty();
    flags.set(FontFlags::SERIF, self.metadata.serif);
    flags.set(FontFlags::FIXED_PITCH, self.metadata.monospaced);
    flags.set(FontFlags::ITALIC, self.metadata.italic_angle != 0.0);
    flags.insert(FontFlags::SYMBOLIC);
    flags.insert(FontFlags::SMALL_CAP);
    let mut descriptor = pdf.font_descriptor(self.objects.descriptor);
    descriptor
      .name(Name(base_name.as_bytes()))
      .flags(flags)
      .bbox(Rect::new(
        self.metadata.bbox[0],
        self.metadata.bbox[1],
        self.metadata.bbox[2],
        self.metadata.bbox[3],
      ))
      .italic_angle(self.metadata.italic_angle)
      .ascent(self.metadata.ascent)
      .descent(self.metadata.descent)
      .cap_height(self.metadata.cap_height)
      .stem_v(self.metadata.stem_v);
    if self.metadata.is_cff {
      descriptor.font_file3(self.objects.data);
    } else {
      descriptor.font_file2(self.objects.data);
    }
    descriptor.finish();

    let mut cmap = UnicodeCmap::new(Name(b"Custom"), SYSTEM_INFO);
    for cid in 0..self.glyphs.num_gids() {
      if let Some(text) = self.codepoints.get(&cid)
        && !text.is_empty()
      {
        cmap.pair_with_multiple(cid, text.chars());
      }
    }
    let cmap = deflate(&cmap.finish())?;
    pdf
      .cmap(self.objects.cmap, &cmap)
      .writing_mode(WMode::Horizontal)
      .filter(Filter::FlateDecode);

    let embedded = deflate(embedded)?;
    let mut stream = pdf.stream(self.objects.data, &embedded);
    stream.filter(Filter::FlateDecode);
    if self.metadata.is_cff {
      stream.pair(Name(b"Subtype"), Name(b"CIDFontType0C"));
    }
    stream.finish();
    Ok(())
  }

  fn base_font_name(&self) -> String {
    const REST_LEN: usize = SUBSET_TAG_LEN + 1 + 1 + IDENTITY_H.len();
    // ISO 19005 constrains PDF/A names to 127 bytes. Reserve the subset tag
    // and the possible Type0 `-Identity-H` suffix even for ordinary output so
    // the same embedded-font identity remains valid when conformance arrives.
    const MAX_POSTSCRIPT_NAME_LEN: usize = 127 - REST_LEN;
    let mut end = self
      .metadata
      .postscript_name
      .len()
      .min(MAX_POSTSCRIPT_NAME_LEN);
    while end > 0 && !self.metadata.postscript_name.is_char_boundary(end) {
      end -= 1;
    }
    format!(
      "{}+{}",
      subset_tag(&self.face, &self.glyphs),
      &self.metadata.postscript_name[..end]
    )
  }
}

impl FontMetadata {
  fn from_face(face_data: &DirectFontFace) -> Result<Self> {
    let face =
      FontRef::from_index(face_data.data.as_slice(), face_data.index).map_err(|error| {
        PdfError::Writer(format!(
          "resolved PDF font binary could not be parsed: font_id={} face_index={} error={error}",
          face_data.id(),
          face_data.index
        ))
      })?;
    let is_glyf = face.glyf().is_ok();
    let is_cff = face.cff().is_ok();
    let is_cff2 = face.cff2().is_ok();
    if !is_glyf && !is_cff && !is_cff2 {
      return Err(PdfError::DirectWriterUnsupported {
        feature: "fonts without TrueType or CFF outlines",
      });
    }

    let location = face_data.location(&face);
    let metrics = face.metrics(Size::new(PDF_UNITS_PER_EM), &location);
    let fallback_bounds = metrics.bounds.unwrap_or_default();
    let fallback_bbox = [
      fallback_bounds.x_min,
      fallback_bounds.y_min,
      fallback_bounds.x_max,
      fallback_bounds.y_max,
    ];
    let (bbox, ascent, descent, cap_height) = match (face.head(), face.hhea(), face.os2()) {
      (Ok(head), Ok(hhea), Ok(os2)) => {
        let units_per_em = f32::from(head.units_per_em());
        if units_per_em <= 0.0 {
          return Err(PdfError::Writer(format!(
            "font {} has a zero units-per-em value",
            face_data.id()
          )));
        }
        let scale = PDF_UNITS_PER_EM / units_per_em;
        let scaled = |value: i16| f32::from(value) * scale;
        let typo_ascent = scaled(os2.s_typo_ascender());
        let typo_descent = scaled(os2.s_typo_descender());
        (
          [
            scaled(head.x_min()),
            typo_descent,
            scaled(head.x_max()),
            typo_ascent,
          ],
          scaled(hhea.ascender().to_i16()),
          scaled(hhea.descender().to_i16()),
          // Word's fixed writer uses OS/2.sTypoAscender here; this is the
          // source-backed value written directly into the font descriptor.
          typo_ascent,
        )
      }
      _ => (
        fallback_bbox,
        metrics.ascent,
        metrics.descent,
        metrics.cap_height.unwrap_or(metrics.ascent),
      ),
    };
    if !bbox.into_iter().all(f32::is_finite)
      || !ascent.is_finite()
      || !descent.is_finite()
      || !cap_height.is_finite()
    {
      return Err(PdfError::Writer(format!(
        "font {} has non-finite PDF descriptor metrics",
        face_data.id()
      )));
    }

    let postscript_name = face
      .localized_strings(skrifa::string::StringId::POSTSCRIPT_NAME)
      .english_or_first()
      .map(|name| name.to_string())
      .filter(|name| !name.is_empty())
      .unwrap_or_else(|| "unknown".to_string());
    let weight = face.attributes().weight.value();
    Ok(Self {
      serif: postscript_name.contains("Serif"),
      postscript_name,
      is_cff,
      bbox,
      ascent,
      descent,
      cap_height,
      italic_angle: metrics.italic_angle,
      stem_v: 10.0 + 0.244 * (weight - 50.0),
      monospaced: metrics.is_monospace,
    })
  }
}

fn subset_tag(face: &DirectFontFace, glyphs: &GlyphRemapper) -> String {
  let mut identity = Vec::with_capacity(face.id().len() + 4 + glyphs.num_gids() as usize * 2);
  identity.extend_from_slice(face.id().as_bytes());
  identity.extend_from_slice(&face.index.to_be_bytes());
  for variation in face.variations.iter() {
    identity.extend_from_slice(&variation.tag);
    identity.extend_from_slice(&variation.value.to_bits().to_be_bytes());
  }
  for glyph in glyphs.remapped_gids() {
    identity.extend_from_slice(&glyph.to_be_bytes());
  }
  let mut hash = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, &identity).as_u128();
  let mut tag = [b'A'; SUBSET_TAG_LEN];
  for byte in &mut tag {
    *byte = b'A' + (hash % 26) as u8;
    hash /= 26;
  }
  String::from_utf8(tag.to_vec()).expect("subset tag contains only ASCII letters")
}

fn deflate(data: &[u8]) -> Result<Vec<u8>> {
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  encoder
    .write_all(data)
    .map_err(|error| PdfError::Writer(format!("font compression failed: {error}")))?;
  encoder
    .finish()
    .map_err(|error| PdfError::Writer(format!("font compression failed: {error}")))
}
