use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fmt;
use std::fs;
use std::ops::{Deref, Range};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::Instant;

use fontdb::{Database as FontDatabase, Source as FontDbSource};
use fontique::{
  Attributes as PlatformFontAttributes, Collection as PlatformFontCollection,
  CollectionOptions as PlatformFontCollectionOptions, FontStyle as PlatformFontStyle,
  FontWeight as PlatformFontWeight, FontWidth as PlatformFontWidth,
  GenericFamily as PlatformGenericFamily, QueryFamily as PlatformQueryFamily,
  QueryStatus as PlatformQueryStatus, SourceCache as PlatformSourceCache,
};
use harfrust::{
  Direction as HarfDirection, Feature as HarfFeature, FontRef as HarfFontRef,
  Language as HarfLanguage, Script as HarfScript, ShapeOptions as HarfShapeOptions, ShapePlan,
  ShaperData, Tag as HarfTag, UnicodeBuffer, script,
};
use icu_segmenter::GraphemeClusterSegmenter;
use smallvec::SmallVec;
use ttf_parser::{Face as TtfFace, GlyphId, Rect as TtfRect, Tag};
use unicode_bidi::{Direction as BidiDirection, get_base_direction};
use unicode_script::{Script as UnicodeScriptValue, UnicodeScript};
use yoke::{Yoke, Yokeable};

use crate::{FontError, Result};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct FontId(pub Arc<str>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontBytes(Arc<[u8]>);

impl FontBytes {
  pub fn as_slice(&self) -> &[u8] {
    &self.0
  }
}

impl AsRef<[u8]> for FontBytes {
  fn as_ref(&self) -> &[u8] {
    self.as_slice()
  }
}

impl Deref for FontBytes {
  type Target = [u8];

  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl From<Vec<u8>> for FontBytes {
  fn from(data: Vec<u8>) -> Self {
    Self(Arc::from(data))
  }
}

impl From<Arc<[u8]>> for FontBytes {
  fn from(data: Arc<[u8]>) -> Self {
    Self(data)
  }
}

impl From<&'static [u8]> for FontBytes {
  fn from(data: &'static [u8]) -> Self {
    Self(Arc::from(data))
  }
}

impl<'a> From<Cow<'a, [u8]>> for FontBytes {
  fn from(data: Cow<'a, [u8]>) -> Self {
    match data {
      Cow::Borrowed(data) => Self(Arc::from(data)),
      Cow::Owned(data) => Self::from(data),
    }
  }
}

struct RuntimeFace {
  faces: Yoke<RuntimeFaces<'static>, Arc<[u8]>>,
  shaper_data: ShaperData,
  glyph_bounds: RwLock<HashMap<u16, Option<GlyphBounds>>>,
  shape_plans: RwLock<HashMap<ShapePlanKey, Arc<ShapePlan>>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ShapePlanKey {
  direction: HarfDirection,
  script: HarfScript,
  language: Option<HarfLanguage>,
  features: Vec<ShapeFeatureKey>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ShapeFeatureKey {
  tag: [u8; 4],
  value: u32,
  start: u32,
  end: u32,
}

impl ShapePlanKey {
  fn new(buffer: &UnicodeBuffer, features: &[HarfFeature]) -> Self {
    Self {
      direction: buffer.direction(),
      script: buffer.script(),
      language: buffer.language(),
      features: features
        .iter()
        .map(|feature| ShapeFeatureKey {
          tag: feature.tag.to_be_bytes(),
          value: feature.value,
          start: feature.start,
          end: feature.end,
        })
        .collect(),
    }
  }
}

#[derive(Yokeable)]
struct RuntimeFaces<'a> {
  harf: HarfFontRef<'a>,
  ttf: TtfFace<'a>,
}

impl RuntimeFace {
  fn new(data: FontBytes, face_index: u32) -> Result<Self> {
    let faces = Yoke::<RuntimeFaces<'static>, Arc<[u8]>>::try_attach_to_cart(data.0, |slice| {
      let harf = HarfFontRef::from_index(slice, face_index).map_err(|_| FontError::InvalidFace)?;
      let ttf = TtfFace::parse(slice, face_index).map_err(|_| FontError::InvalidFace)?;
      Ok(RuntimeFaces { harf, ttf })
    })?;
    let shaper_data = ShaperData::new(&faces.get().harf);
    Ok(Self {
      faces,
      shaper_data,
      glyph_bounds: RwLock::new(HashMap::new()),
      shape_plans: RwLock::new(HashMap::new()),
    })
  }

  fn harf(&self) -> &HarfFontRef<'_> {
    &self.faces.get().harf
  }

  fn ttf(&self) -> &TtfFace<'_> {
    &self.faces.get().ttf
  }

  fn shape(&self, buffer: UnicodeBuffer, features: &[HarfFeature]) -> harfrust::GlyphBuffer {
    let shaper = self.shaper_data.shaper(self.harf()).build();
    let key = ShapePlanKey::new(&buffer, features);
    let cached = self
      .shape_plans
      .read()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .get(&key)
      .cloned();
    let plan = cached.unwrap_or_else(|| {
      let language = buffer.language();
      let candidate = Arc::new(ShapePlan::new(
        &shaper,
        buffer.direction(),
        Some(buffer.script()),
        language.as_ref(),
        features,
      ));
      self
        .shape_plans
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .entry(key)
        .or_insert(candidate)
        .clone()
    });
    shaper.shape(
      buffer,
      HarfShapeOptions::new().plan(Some(&plan)).features(features),
    )
  }

  fn glyph_bounds(&self, glyph_id: u16) -> Option<GlyphBounds> {
    if let Some(bounds) = self
      .glyph_bounds
      .read()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .get(&glyph_id)
    {
      return *bounds;
    }

    let face = self.ttf();
    let bounds = face
      .glyph_bounding_box(GlyphId(glyph_id))
      .map(|bounds| glyph_bounds_from_ttf_rect(bounds, f32::from(face.units_per_em())));
    self
      .glyph_bounds
      .write()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .entry(glyph_id)
      .or_insert(bounds);
    bounds
  }
}

fn font_timing<T>(label: &str, work: impl FnOnce() -> T) -> T {
  static ENABLED: OnceLock<bool> = OnceLock::new();
  if !ENABLED.get_or_init(|| std::env::var_os("OOXMLSDK_FONT_TIMING").is_some()) {
    return work();
  }
  let start = Instant::now();
  let output = work();
  eprintln!("[ooxmlsdk-fonts] {label}: {:?}", start.elapsed());
  output
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontRegistry<'a> {
  pub sources: Vec<FontSource<'a>>,
  pub faces: Vec<RegisteredFontFace<'a>>,
  pub book: FontBook<'a>,
}

impl<'a> FontRegistry<'a> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_default_policy() -> Self {
    let mut registry = Self::new();
    registry.add_default_office_policy();
    registry
  }

  pub fn add_default_office_policy(&mut self) {
    for (from, to) in DEFAULT_OFFICE_ALIASES {
      self.book.family_aliases.push(FontFamilyAlias {
        from: Cow::Borrowed(from),
        to: Cow::Borrowed(to),
      });
    }
    for chain in default_fallback_chains() {
      self.book.fallback_chains.push(chain);
    }
  }

  pub fn register_face(&mut self, source: FontSource<'a>, face: FontFaceInfo<'a>) {
    self.sources.push(source.clone());
    self.faces.push(RegisteredFontFace {
      source,
      family_names: face.family_names.clone(),
      style_name: face.style_name.clone(),
      weight: face.weight,
      slant: face.slant,
      stretch: face.stretch,
      pitch: face.pitch,
      charset: None,
      face_index: face.face_index,
      origin_priority: 0,
      runtime_face: OnceLock::new(),
    });
    self.book.faces.push(face);
  }

  pub fn register_system_fonts(&mut self) -> Result<usize> {
    let mut database = FontDatabase::new();
    database.load_system_fonts();
    let mut registered = 0usize;
    for info in database.faces() {
      let Some((data, face_index)) =
        database.with_face_data(info.id, |data, face_index| (data.to_vec(), face_index))
      else {
        continue;
      };
      let id = format!("system:{}:{}", info.post_script_name, face_index);
      let face = FontFaceInfo::from_fontdb_face_info(&id, info);
      let source = match &info.source {
        FontDbSource::File(path) | FontDbSource::SharedFile(path, _) => FontSource::Path {
          id: Cow::Owned(id),
          path: path.clone(),
          data: Some(data.into()),
        },
        FontDbSource::Binary(_) => FontSource::Memory {
          id: Cow::Owned(id),
          data: data.into(),
        },
      };
      self.register_face(source, face);
      registered += 1;
    }
    Ok(registered)
  }

  pub fn register_system_query_fonts(&mut self, request: &FontRequest<'_>) -> Result<usize> {
    let database = font_timing("system font database", system_font_database);
    let mut registered = 0usize;
    let mut queries = SmallVec::<[FontDbQueryFamily; 8]>::new();
    if let Some(family) = request
      .family
      .as_deref()
      .filter(|family| !family.trim().is_empty())
    {
      for family in family
        .split(';')
        .map(str::trim)
        .filter(|family| !family.is_empty())
      {
        queries.push(FontDbQueryFamily::Name(family.to_string()));
        let aliased = resolve_family_alias(&self.book, Cow::Borrowed(family));
        if aliased.as_ref() != family {
          queries.push(FontDbQueryFamily::Name(aliased.into_owned()));
        }
      }
      for family in self.fallback_families(request) {
        queries.push(FontDbQueryFamily::Name(family.to_owned()));
      }
      queries.push(FontDbQueryFamily::SansSerif);
      queries.push(FontDbQueryFamily::Serif);
    } else {
      // `fontdb` scans platform font sources but does not guarantee that its
      // CSS generic-family names are configured on every backend. Resolve the
      // Office/script fallback chain by family name before asking for the
      // generic aliases, so an unspecified OOXML typeface still produces a
      // portable, shapeable system face.
      for family in self.fallback_families(request) {
        queries.push(FontDbQueryFamily::Name(family.to_owned()));
      }
      queries.push(FontDbQueryFamily::SansSerif);
      queries.push(FontDbQueryFamily::Serif);
    }

    for query_family in queries {
      let platform_fonts = font_timing("platform font query", || {
        platform_system_query_fonts(&query_family, request)
      });
      let mut platform_matched = false;
      for platform_font in platform_fonts {
        let Ok(mut face) = FontFaceInfo::from_ttf_bytes(
          "platform-system-query",
          &platform_font.data,
          platform_font.face_index,
        ) else {
          continue;
        };
        let matched_legacy_postscript_name = if let FontDbQueryFamily::Name(family) = &query_family
        {
          let normalized = normalize_family(family);
          if !family_matches_names(&face, std::slice::from_ref(&normalized))
            && !face
              .postscript_name
              .as_deref()
              .is_some_and(|name| normalized_family_eq_normalized(name, &normalized))
          {
            continue;
          }
          face
            .postscript_name
            .as_deref()
            .is_some_and(|name| normalized_family_eq_normalized(name, &normalized))
        } else {
          false
        };
        platform_matched = true;
        if matched_legacy_postscript_name && let FontDbQueryFamily::Name(family) = &query_family {
          // Fontique matched this platform family name even when it is a
          // legacy name that is not the face's preferred OpenType family.
          // Preserve that evidence so the Office resolver can still select
          // the requested family instead of falling through to a later
          // Office/script fallback.
          push_unique_string(&mut face.family_names, family.clone());
        }
        let postscript_name = face
          .postscript_name
          .as_deref()
          .or_else(|| face.family_names.first().map(Cow::as_ref))
          .unwrap_or("unknown");
        let font_id = format!(
          "system-query:{}:{}",
          postscript_name, platform_font.face_index
        );
        if self
          .sources
          .iter()
          .any(|source| source.id() == Some(font_id.as_str()))
        {
          if let Some(existing) = self
            .book
            .faces
            .iter_mut()
            .find(|existing| existing.font_id.0.as_ref() == font_id)
            && matched_legacy_postscript_name
            && let FontDbQueryFamily::Name(family) = &query_family
          {
            push_unique_string(&mut existing.family_names, family.clone());
          }
          continue;
        }
        face.font_id = FontId(Arc::from(font_id.as_str()));
        self.register_face(
          FontSource::Memory {
            id: Cow::Owned(font_id),
            data: platform_font.data.into(),
          },
          face,
        );
        registered += 1;
      }
      if platform_matched {
        continue;
      }

      let Some(id) = font_timing("fontdb query", || query_family.query(database, request)) else {
        continue;
      };
      let Some(info) = database.face(id) else {
        continue;
      };
      let font_id = format!("system-query:{}:{}", info.post_script_name, info.index);
      if self
        .sources
        .iter()
        .any(|source| source.id() == Some(font_id.as_str()))
      {
        continue;
      }
      let Some(system_font) = font_timing("system query font", || {
        cached_system_query_font(database, id, info, &font_id)
      }) else {
        continue;
      };
      let Some(data) = font_timing("fontdb face data", || cached_system_font_data(database, id))
      else {
        continue;
      };
      let source = match &info.source {
        FontDbSource::File(path) | FontDbSource::SharedFile(path, _) => FontSource::Path {
          id: Cow::Owned(font_id),
          path: path.clone(),
          data: Some(data),
        },
        FontDbSource::Binary(_) => FontSource::Memory {
          id: Cow::Owned(font_id),
          data,
        },
      };
      self.register_face(source, system_font.face.clone());
      registered += 1;
    }
    Ok(registered)
  }

  pub fn register_memory_font(
    &mut self,
    id: impl Into<Cow<'a, str>>,
    data: impl Into<FontBytes>,
  ) -> Result<FontId> {
    self.register_ttf_source(FontSource::Memory {
      id: id.into(),
      data: data.into(),
    })
  }

  pub fn register_embedded_font(
    &mut self,
    id: impl Into<Cow<'a, str>>,
    data: impl Into<FontBytes>,
  ) -> Result<FontId> {
    self.register_ttf_source(FontSource::EmbeddedOoxml {
      id: id.into(),
      data: data.into(),
    })
  }

  pub fn register_test_fixture_font(
    &mut self,
    id: impl Into<Cow<'a, str>>,
    data: impl Into<FontBytes>,
  ) -> Result<FontId> {
    self.register_ttf_source(FontSource::TestFixture {
      id: id.into(),
      data: data.into(),
    })
  }

  pub fn register_path_font(
    &mut self,
    id: impl Into<Cow<'a, str>>,
    path: impl AsRef<Path>,
  ) -> Result<FontId> {
    let id = id.into();
    if self
      .sources
      .iter()
      .any(|source| source.id() == Some(id.as_ref()))
    {
      return Ok(FontId(Arc::from(id.as_ref())));
    }
    let path = path.as_ref().to_path_buf();
    let data = fs::read(&path).map_err(|error| FontError::SourceUnavailable(error.to_string()))?;
    let face = FontFaceInfo::from_ttf_bytes(id.as_ref(), &data, 0)?;
    let font_id = face.font_id.clone();
    self.register_face(
      FontSource::Path {
        id,
        path,
        data: Some(data.into()),
      },
      face,
    );
    Ok(font_id)
  }

  fn register_ttf_source(&mut self, source: FontSource<'a>) -> Result<FontId> {
    let Some(id) = source.id() else {
      return Err(FontError::InvalidFace);
    };
    let Some(data) = source.data() else {
      return Err(FontError::InvalidFace);
    };
    let face = FontFaceInfo::from_ttf_bytes(id, data, 0)?;
    let font_id = face.font_id.clone();
    self.register_face(source, face);
    Ok(font_id)
  }

  pub fn resolve(&self, request: &FontRequest<'_>) -> Result<ResolvedFont<'a>> {
    match self.book.resolve(request, &self.faces) {
      Ok(resolved) => Ok(resolved),
      Err(FontError::NoMatch) => {
        for family in self.fallback_families(request) {
          if let Ok(mut resolved) =
            self
              .book
              .resolve_matching_family(request, &self.faces, family, false)
          {
            resolved.match_diagnostics.fallback_level = Some(0);
            return Ok(resolved);
          }
        }
        Err(FontError::NoMatch)
      }
      Err(error) => Err(error),
    }
  }

  pub fn resolve_with_diagnostics(&self, request: &FontRequest<'_>) -> Result<ResolvedFont<'a>> {
    match self.book.resolve_with_diagnostics(request, &self.faces) {
      Ok(resolved) => Ok(resolved),
      Err(FontError::NoMatch) => {
        for family in self.fallback_families(request) {
          if let Ok(mut resolved) =
            self
              .book
              .resolve_matching_family(request, &self.faces, family, true)
          {
            resolved.match_diagnostics.fallback_level = Some(0);
            return Ok(resolved);
          }
        }
        Err(FontError::NoMatch)
      }
      Err(error) => Err(error),
    }
  }

  pub fn face(&self, font_id: &FontId) -> Option<&FontFaceInfo<'a>> {
    self.book.faces.iter().find(|face| &face.font_id == font_id)
  }

  pub fn resolved_face_data(&self, resolved: &ResolvedFont<'_>) -> Option<FontFaceData<'_>> {
    self.font_face_data(&resolved.font_id)
  }

  pub fn font_face_data(&self, font_id: &FontId) -> Option<FontFaceData<'_>> {
    let registered = self
      .faces
      .iter()
      .find(|registered| registered.font_id().as_ref() == Some(font_id))?;
    Some(FontFaceData {
      font_id: font_id.clone(),
      source: registered.source.clone(),
      face_index: registered.face_index,
      family_names: registered.family_names.clone(),
      style_name: registered.style_name.clone(),
      data: registered.source.data_handle(),
    })
  }

  pub fn font_face_binary(&self, font_id: &FontId) -> Option<(FontBytes, u32)> {
    let registered = self
      .faces
      .iter()
      .find(|registered| registered.font_id().as_ref() == Some(font_id))?;
    Some((registered.source.data_handle()?, registered.face_index))
  }

  pub fn shape_text<'text, 'request>(
    &self,
    request: &'request FontRequest<'request>,
    text: &'text str,
    direction: TextDirection,
  ) -> Result<ShapedRun<'text, 'a>>
  where
    'a: 'request,
  {
    let resolved = self.resolve(request)?;
    self.shape_resolved_font(
      &resolved,
      text,
      &ShapeOptions::from_request(request, direction),
    )
  }

  pub fn shape_font_face<'text, 'request>(
    &self,
    resolved: &ResolvedFont<'a>,
    text: &'text str,
    options: &ShapeOptions<'request>,
  ) -> Result<ShapedRun<'text, 'a>>
  where
    'a: 'request,
  {
    self.shape_resolved_font(resolved, text, options)
  }

  fn shape_resolved_font<'text, 'request>(
    &self,
    resolved: &ResolvedFont<'a>,
    text: &'text str,
    options: &ShapeOptions<'request>,
  ) -> Result<ShapedRun<'text, 'a>>
  where
    'a: 'request,
  {
    match &resolved.source {
      FontSource::Memory { data, .. }
      | FontSource::EmbeddedOoxml { data, .. }
      | FontSource::TestFixture { data, .. }
      | FontSource::Path {
        data: Some(data), ..
      } => {
        let runtime_face = self
          .runtime_face_for_font(&resolved.font_id)
          .or_else(|| runtime_face_for_data(data.clone(), resolved.face_index))
          .ok_or(FontError::InvalidFace)?;
        resolved.shape_with_runtime_face(text, &runtime_face, options)
      }
      FontSource::System | FontSource::Path { data: None, .. } => {
        Ok(resolved.shape_approximate(text, options.size_pt, options.direction, options.script))
      }
    }
  }

  pub fn measure_text<'text, 'request>(
    &self,
    request: &'request FontRequest<'request>,
    text: &'text str,
    direction: TextDirection,
  ) -> Result<f32>
  where
    'a: 'request,
  {
    Ok(
      self
        .shape_text_runs(request, text, direction)?
        .iter()
        .map(|run| run.advance_pt)
        .sum(),
    )
  }

  pub fn shape_text_runs<'text, 'request>(
    &self,
    request: &'request FontRequest<'request>,
    text: &'text str,
    direction: TextDirection,
  ) -> Result<Vec<ShapedRun<'text, 'a>>>
  where
    'a: 'request,
  {
    self.shape_text_runs_with_options(
      request,
      text,
      &ShapeOptions::from_request(request, direction),
    )
  }

  pub fn shape_text_runs_with_options<'text, 'request>(
    &self,
    request: &'request FontRequest<'request>,
    text: &'text str,
    options: &ShapeOptions<'request>,
  ) -> Result<Vec<ShapedRun<'text, 'a>>>
  where
    'a: 'request,
  {
    self.shape_text_runs_inner(request, text, options)
  }

  /// Resolves the primary font and the configured fallback families once.
  ///
  /// The resulting chain is independent of font size and text content. Glyph
  /// coverage is still checked for every shaped text cluster.
  pub fn resolve_font_chain(&self, request: &FontRequest<'_>) -> Result<ResolvedFontChain<'a>> {
    let primary = self.resolve(request)?;
    let mut fonts = vec![(primary, None)];
    for family in self.fallback_families(request) {
      if let Ok(resolved) = self
        .book
        .resolve_matching_family(request, &self.faces, family, false)
        && !fonts
          .iter()
          .any(|(font, _)| font.font_id == resolved.font_id)
      {
        let fallback_level = fonts.len().try_into().ok();
        fonts.push((resolved, fallback_level));
      }
    }
    Ok(ResolvedFontChain { fonts })
  }

  /// Shapes text with a previously resolved primary/fallback font chain.
  pub fn shape_text_runs_with_font_chain<'text, 'request>(
    &self,
    chain: &ResolvedFontChain<'a>,
    text: &'text str,
    options: &ShapeOptions<'request>,
  ) -> Result<Vec<ShapedRun<'text, 'a>>>
  where
    'a: 'request,
  {
    let fonts = self.resolved_fonts_from_chain(chain);
    self.shape_text_runs_with_fonts(fonts, text, options)
  }

  fn shape_text_runs_inner<'text, 'request>(
    &self,
    request: &'request FontRequest<'request>,
    text: &'text str,
    options: &ShapeOptions<'request>,
  ) -> Result<Vec<ShapedRun<'text, 'a>>>
  where
    'a: 'request,
  {
    let fonts = font_timing("resolve fallback fonts", || {
      self.resolve_fallback_fonts(request, text, options)
    })?;
    self.shape_text_runs_with_fonts(fonts, text, options)
  }

  fn shape_text_runs_with_fonts<'text, 'request>(
    &self,
    fonts: Vec<ResolvedFontWithFace<'_, 'a>>,
    text: &'text str,
    options: &ShapeOptions<'request>,
  ) -> Result<Vec<ShapedRun<'text, 'a>>>
  where
    'a: 'request,
  {
    let runtime_faces = font_timing("prepare runtime faces", || {
      fonts
        .iter()
        .map(|font| self.runtime_face_for_font(&font.resolved.font_id))
        .collect::<Vec<_>>()
    });

    let mut runs = Vec::new();
    let mut start = 0usize;
    let mut active = None::<usize>;
    for cluster in grapheme_clusters(text) {
      let font_index = fonts
        .iter()
        .enumerate()
        .position(|(index, font)| {
          font_supports_text_cluster(
            font,
            runtime_faces[index].as_deref().map(RuntimeFace::ttf),
            &text[cluster.clone()],
          )
        })
        .unwrap_or(0);
      if active.is_some_and(|active| active != font_index) {
        runs.push(self.shape_resolved_segment(
          &fonts[active.unwrap_or(0)],
          text,
          start..cluster.start,
          options,
        )?);
        start = cluster.start;
      }
      active = Some(font_index);
    }
    if start < text.len() || text.is_empty() {
      runs.push(self.shape_resolved_segment(
        &fonts[active.unwrap_or(0)],
        text,
        start..text.len(),
        options,
      )?);
    }
    Ok(runs)
  }

  fn resolve_fallback_fonts<'request>(
    &self,
    request: &'request FontRequest<'request>,
    text: &str,
    options: &ShapeOptions<'request>,
  ) -> Result<Vec<ResolvedFontWithFace<'_, 'a>>>
  where
    'a: 'request,
  {
    let primary = self.resolve(request)?;
    let Some(primary_face) = self
      .book
      .faces
      .iter()
      .find(|face| face.font_id == primary.font_id)
    else {
      return Ok(vec![ResolvedFontWithFace {
        resolved: primary,
        face: None,
        fallback_level: None,
      }]);
    };

    let mut fonts = vec![ResolvedFontWithFace {
      resolved: primary,
      face: Some(primary_face),
      fallback_level: None,
    }];

    if !options.scan_registered_fallbacks {
      return self
        .resolve_font_chain(request)
        .map(|chain| self.resolved_fonts_from_chain(&chain));
    }

    let mut missing_chars = self.missing_chars_for_fonts(&fonts, text);

    for family in self.fallback_families(request) {
      if missing_chars.is_empty() {
        break;
      }
      if let Ok(resolved) = self
        .book
        .resolve_matching_family(request, &self.faces, family, false)
        && !fonts
          .iter()
          .any(|font| font.resolved.font_id == resolved.font_id)
        && let Some(face) = self
          .book
          .faces
          .iter()
          .find(|face| face.font_id == resolved.font_id)
      {
        let fallback_level = fonts.len().try_into().ok();
        fonts.push(ResolvedFontWithFace {
          resolved,
          face: Some(face),
          fallback_level,
        });
        missing_chars = self.missing_chars_for_fonts(&fonts, text);
      }
    }

    if missing_chars.is_empty() {
      return Ok(fonts);
    }

    for face in &self.book.faces {
      if fonts
        .iter()
        .any(|font| font.resolved.font_id == face.font_id)
        || !missing_chars
          .iter()
          .any(|ch| self.face_info_supports_char(face, *ch))
      {
        continue;
      }
      let fallback_level = fonts.len().try_into().ok();
      fonts.push(ResolvedFontWithFace {
        resolved: self.resolved_from_face(request, face, fallback_level),
        face: Some(face),
        fallback_level,
      });
      missing_chars = self.missing_chars_for_fonts(&fonts, text);
      if missing_chars.is_empty() {
        break;
      }
    }

    Ok(fonts)
  }

  fn resolved_fonts_from_chain(
    &self,
    chain: &ResolvedFontChain<'a>,
  ) -> Vec<ResolvedFontWithFace<'_, 'a>> {
    chain
      .fonts
      .iter()
      .map(|(resolved, fallback_level)| ResolvedFontWithFace {
        face: self
          .book
          .faces
          .iter()
          .find(|face| face.font_id == resolved.font_id),
        resolved: resolved.clone(),
        fallback_level: *fallback_level,
      })
      .collect()
  }

  fn missing_chars_for_fonts(
    &self,
    fonts: &[ResolvedFontWithFace<'_, 'a>],
    text: &str,
  ) -> SmallVec<[char; 8]> {
    let mut missing = SmallVec::<[char; 8]>::new();
    for ch in text.chars() {
      if is_private_use_char(ch) || missing.contains(&ch) {
        continue;
      }
      if !fonts.iter().any(|font| {
        font
          .face
          .is_some_and(|face| self.face_info_supports_char(face, ch))
      }) {
        missing.push(ch);
      }
    }
    missing
  }

  fn fallback_families<'book>(&'book self, request: &FontRequest<'_>) -> Vec<&'book str> {
    let mut families: Vec<&'book str> = Vec::new();
    let mut normalized_families: Vec<String> = Vec::new();
    let requested_family = request.family.as_deref().map(normalize_family);
    for chain in &self.book.fallback_chains {
      if chain.requested_family.as_deref().is_some_and(|family| {
        let chain_family = normalize_family(family);
        requested_family
          .as_ref()
          .is_none_or(|requested| requested != &chain_family)
      }) {
        continue;
      }
      if chain
        .script
        .is_some_and(|script| request.script.is_some_and(|requested| requested != script))
      {
        continue;
      }
      if chain.language.as_deref().is_some_and(|language| {
        request
          .language
          .as_deref()
          .is_some_and(|requested| !requested.eq_ignore_ascii_case(language))
      }) {
        continue;
      }
      for family in &chain.families {
        let normalized = normalize_family(family.as_ref());
        if !normalized_families
          .iter()
          .any(|existing| existing == &normalized)
        {
          normalized_families.push(normalized);
          families.push(family.as_ref());
        }
      }
    }
    families
  }

  fn shape_resolved_segment<'text, 'request>(
    &self,
    font: &ResolvedFontWithFace<'_, 'a>,
    text: &'text str,
    range: Range<usize>,
    options: &ShapeOptions<'request>,
  ) -> Result<ShapedRun<'text, 'a>>
  where
    'a: 'request,
  {
    let mut run = match &font.resolved.source {
      FontSource::Memory { data, .. }
      | FontSource::EmbeddedOoxml { data, .. }
      | FontSource::TestFixture { data, .. }
      | FontSource::Path {
        data: Some(data), ..
      } => {
        let runtime_face = self
          .runtime_face_for_font(&font.resolved.font_id)
          .or_else(|| runtime_face_for_data(data.clone(), font.resolved.face_index))
          .ok_or(FontError::InvalidFace)?;
        font
          .resolved
          .shape_with_runtime_face(&text[range.clone()], &runtime_face, options)?
      }
      FontSource::System | FontSource::Path { data: None, .. } => font.resolved.shape_approximate(
        &text[range.clone()],
        options.size_pt,
        options.direction,
        options.script,
      ),
    };
    run.offset_text_range(range.start);
    if let Some(fallback_level) = font.fallback_level {
      run.diagnostics.fallback_runs.push(FallbackRun {
        text_range: run.text_range.clone(),
        font_id: Some(font.resolved.font_id.clone()),
        fallback_level,
        reason: FontSubstitutionReason::MissingGlyph,
        family: Some(font.resolved.resolved_family.clone()),
      });
    }
    Ok(run)
  }

  fn face_info_supports_char(&self, face: &FontFaceInfo<'a>, ch: char) -> bool {
    if let Some(parsed) = self.runtime_face_for_font(&face.font_id) {
      return ttf_face_supports_char(parsed.ttf(), ch);
    }
    face.coverage.contains_char(ch)
  }

  fn runtime_face_for_font(&self, font_id: &FontId) -> Option<Arc<RuntimeFace>> {
    self
      .faces
      .iter()
      .find(|registered| registered.font_id().as_ref() == Some(font_id))?
      .runtime_face()
  }

  fn resolved_from_face(
    &self,
    _request: &FontRequest<'_>,
    face: &FontFaceInfo<'a>,
    fallback_level: Option<u8>,
  ) -> ResolvedFont<'a> {
    let registered = registered_face(face, &self.faces);
    ResolvedFont {
      font_id: face.font_id.clone(),
      resolved_family: primary_family(face),
      source: registered
        .map(|face| face.source.clone())
        .unwrap_or(FontSource::System),
      face_index: face.face_index,
      synthetic_bold: false,
      synthetic_italic: false,
      metrics: face.metrics.clone(),
      match_diagnostics: FontMatchDiagnostics {
        candidates: Vec::new(),
        fallback_level,
      },
    }
  }
}

struct ResolvedFontWithFace<'faces, 'book> {
  resolved: ResolvedFont<'book>,
  face: Option<&'faces FontFaceInfo<'book>>,
  fallback_level: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
struct ScoredFontMatch {
  rank: FontMatchRank,
  face_index: usize,
  rejected: bool,
  reason: Option<FontMatchReason>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontBook<'a> {
  pub faces: Vec<FontFaceInfo<'a>>,
  pub family_aliases: Vec<FontFamilyAlias<'a>>,
  pub substitutions: Vec<FontSubstitutionRule<'a>>,
  pub fallback_chains: Vec<FontFallbackChain<'a>>,
  pub fallback_cache: Vec<GlyphFallbackCacheEntry<'a>>,
}

impl<'a> FontBook<'a> {
  pub fn resolve(
    &self,
    request: &FontRequest<'_>,
    registered_faces: &[RegisteredFontFace<'a>],
  ) -> Result<ResolvedFont<'a>> {
    self.resolve_impl(request, registered_faces, None, false)
  }

  pub fn resolve_with_diagnostics(
    &self,
    request: &FontRequest<'_>,
    registered_faces: &[RegisteredFontFace<'a>],
  ) -> Result<ResolvedFont<'a>> {
    self.resolve_impl(request, registered_faces, None, true)
  }

  fn resolve_matching_family(
    &self,
    request: &FontRequest<'_>,
    registered_faces: &[RegisteredFontFace<'a>],
    family: &str,
    include_diagnostics: bool,
  ) -> Result<ResolvedFont<'a>> {
    self.resolve_impl(request, registered_faces, Some(family), include_diagnostics)
  }

  fn resolve_impl(
    &self,
    request: &FontRequest<'_>,
    registered_faces: &[RegisteredFontFace<'a>],
    family_override: Option<&str>,
    include_diagnostics: bool,
  ) -> Result<ResolvedFont<'a>> {
    let target_family_names = family_override.or(request.family.as_deref()).map(|family| {
      let aliased = resolve_family_alias(self, Cow::Borrowed(family));
      let substituted =
        find_substitution_rule(self, aliased.as_ref()).map(|rule| rule.substitute_family.as_ref());
      normalized_family_names(substituted.unwrap_or_else(|| aliased.as_ref()))
    });
    let requested_weight = requested_weight(request);
    let requested_slant = requested_slant(request);
    let requested_stretch = request.stretch.unwrap_or(FontStretch::Normal);

    let mut winner = None::<ScoredFontMatch>;
    let mut diagnostics = include_diagnostics.then(Vec::new);
    for (face_index, face) in self.faces.iter().enumerate() {
      let scored = score_font_match(
        face_index,
        face,
        registered_faces,
        request,
        target_family_names.as_deref(),
        requested_slant,
        requested_stretch,
        requested_weight,
      );
      if !scored.rejected
        && winner.is_none_or(|current| scored_font_match_cmp(scored, current, &self.faces).is_lt())
      {
        winner = Some(scored);
      }
      if let Some(diagnostics) = &mut diagnostics {
        diagnostics.push(scored);
      }
    }

    let Some(winner) = winner else {
      return Err(FontError::NoMatch);
    };
    let face: &FontFaceInfo<'a> = &self.faces[winner.face_index];
    let registered = registered_face_for_book_index(winner.face_index, face, registered_faces);

    let synthetic_bold =
      request.bold && font_weight_number(face.weight) < font_weight_number(FontWeight::Bold);
    let synthetic_italic = request.italic && face.slant == FontSlant::Upright;
    Ok(ResolvedFont {
      font_id: face.font_id.clone(),
      resolved_family: primary_family(face),
      source: registered
        .map(|face| face.source.clone())
        .unwrap_or(FontSource::System),
      face_index: face.face_index,
      synthetic_bold,
      synthetic_italic,
      metrics: face.metrics.clone(),
      match_diagnostics: FontMatchDiagnostics {
        candidates: diagnostics
          .map(|mut diagnostics| {
            diagnostics.sort_by(|left, right| scored_font_match_cmp(*left, *right, &self.faces));
            diagnostics
              .into_iter()
              .map(|scored| {
                let face: &FontFaceInfo<'a> = &self.faces[scored.face_index];
                FontMatchCandidate {
                  font_id: face.font_id.clone(),
                  family: primary_family(face),
                  score: -scored.rank.distance(),
                  rejected: scored.rejected,
                  reason: scored.reason,
                }
              })
              .collect()
          })
          .unwrap_or_default(),
        fallback_level: None,
      },
    })
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontFaceInfo<'a> {
  pub font_id: FontId,
  pub family_names: Vec<Cow<'a, str>>,
  pub postscript_name: Option<Cow<'a, str>>,
  pub style_name: Option<Cow<'a, str>>,
  pub family_class: Option<FontFamilyClass>,
  pub weight: FontWeight,
  pub slant: FontSlant,
  pub stretch: FontStretch,
  pub pitch: FontPitch,
  pub coverage: FontCoverage,
  pub flags: FontFlags,
  pub axes: Vec<VariationAxis<'a>>,
  pub features: Vec<OpenTypeFeature<'a>>,
  pub metrics: FontMetrics,
  pub embedding: FontEmbeddingPolicy,
  pub embedding_plan: FontEmbeddingPlan<'a>,
  pub bounds: FontBounds,
  pub face_index: u32,
}

impl<'a> FontFaceInfo<'a> {
  pub fn synthetic(id: impl Into<Arc<str>>, family: impl Into<Cow<'a, str>>) -> Self {
    Self {
      font_id: FontId(id.into()),
      family_names: vec![family.into()],
      postscript_name: None,
      style_name: None,
      family_class: None,
      weight: FontWeight::Normal,
      slant: FontSlant::Upright,
      stretch: FontStretch::Normal,
      pitch: FontPitch::Variable,
      coverage: FontCoverage::default(),
      flags: FontFlags::default(),
      axes: Vec::new(),
      features: Vec::new(),
      metrics: FontMetrics::default(),
      embedding: FontEmbeddingPolicy::default(),
      embedding_plan: FontEmbeddingPlan::default(),
      bounds: FontBounds::default(),
      face_index: 0,
    }
  }

  pub fn from_ttf_bytes(id: &str, data: &[u8], face_index: u32) -> Result<Self> {
    let face = TtfFace::parse(data, face_index).map_err(|_| FontError::InvalidFace)?;
    let mut family_names = Vec::new();
    let mut postscript_name = None;
    let mut style_name = None;
    for name in face.names() {
      let Some(value) = name.to_string() else {
        continue;
      };
      match name.name_id {
        ttf_parser::name_id::FAMILY => push_unique_string(&mut family_names, value),
        ttf_parser::name_id::TYPOGRAPHIC_FAMILY => push_unique_string(&mut family_names, value),
        ttf_parser::name_id::POST_SCRIPT_NAME => postscript_name = Some(Cow::Owned(value)),
        ttf_parser::name_id::SUBFAMILY => style_name = Some(Cow::Owned(value)),
        _ => {}
      }
    }
    if family_names.is_empty() {
      family_names.push(Cow::Owned(id.to_string()));
    }

    let pitch = if face.is_monospaced() {
      FontPitch::Fixed
    } else {
      FontPitch::Variable
    };
    let flags = FontFlags {
      monospace: face.is_monospaced(),
      color_glyphs: face.tables().colr.is_some() || face.tables().sbix.is_some(),
      vertical: face.tables().vhea.is_some() || face.tables().vmtx.is_some(),
      graphite: has_table(&face, b"Silf") || has_table(&face, b"Feat") || has_table(&face, b"Sill"),
      aat: face.tables().feat.is_some() || face.tables().morx.is_some(),
      cff2: face.tables().cff2.is_some(),
      variable: !face.variation_axes().is_empty(),
      kashida_positions: face.tables().morx.is_none(),
      ..FontFlags::default()
    };
    let metrics = font_metrics_from_ttf(&face, 1.0);

    Ok(Self {
      font_id: FontId(Arc::from(id)),
      family_names,
      postscript_name,
      style_name,
      family_class: None,
      weight: font_weight_from_ttf(face.weight().to_number()),
      slant: font_slant_from_ttf(face.style()),
      stretch: font_stretch_from_ttf(face.width().to_number()),
      pitch,
      coverage: font_coverage_from_ttf(&face),
      flags,
      axes: variation_axes_from_ttf(&face),
      features: opentype_features_from_ttf(&face),
      metrics,
      embedding: font_embedding_policy_from_ttf(&face),
      embedding_plan: font_embedding_plan_from_ttf(&face),
      bounds: font_bounds_from_ttf(&face),
      face_index,
    })
  }

  fn from_fontdb_face_info(id: &str, info: &fontdb::FaceInfo) -> Self {
    let mut face = Self::synthetic(id.to_string(), id.to_string());
    face.family_names = info
      .families
      .iter()
      .map(|(family, _)| Cow::Owned(family.clone()))
      .collect();
    if face.family_names.is_empty() {
      face.family_names.push(Cow::Owned(id.to_string()));
    }
    face.postscript_name =
      (!info.post_script_name.is_empty()).then(|| Cow::Owned(info.post_script_name.clone()));
    face.style_name = Some(Cow::Owned(format!("{:?}", info.style)));
    face.weight = font_weight_from_ttf(info.weight.0);
    face.slant = match info.style {
      fontdb::Style::Italic => FontSlant::Italic,
      fontdb::Style::Oblique => FontSlant::Oblique,
      fontdb::Style::Normal => FontSlant::Upright,
    };
    face.stretch = font_stretch_from_fontdb(info.stretch);
    face.pitch = if info.monospaced {
      FontPitch::Fixed
    } else {
      FontPitch::Variable
    };
    face.flags.monospace = info.monospaced;
    face.face_index = info.index;
    face
  }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FontCoverage {
  pub unicode_ranges: Vec<Range<u32>>,
  pub scripts: BTreeSet<TextScript>,
}

impl FontCoverage {
  pub fn contains_codepoint(&self, codepoint: u32) -> bool {
    self
      .unicode_ranges
      .iter()
      .any(|range| range.start <= codepoint && codepoint < range.end)
  }

  pub fn contains_char(&self, ch: char) -> bool {
    self.contains_codepoint(u32::from(ch))
  }

  pub fn missing_glyphs(&self, text: &str) -> Vec<MissingGlyph> {
    text
      .char_indices()
      .filter(|(_, ch)| !self.contains_char(*ch))
      .map(|(start, ch)| MissingGlyph {
        codepoint: u32::from(ch),
        text_range: start..start + ch.len_utf8(),
      })
      .collect()
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontFlags {
  pub symbolic: bool,
  pub serif: bool,
  pub monospace: bool,
  pub color_glyphs: bool,
  pub vertical: bool,
  pub graphite: bool,
  pub aat: bool,
  pub cff2: bool,
  pub variable: bool,
  pub kashida_positions: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontFamilyAlias<'a> {
  pub from: Cow<'a, str>,
  pub to: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontSubstitutionRule<'a> {
  pub requested_family: Cow<'a, str>,
  pub substitute_family: Cow<'a, str>,
  pub reason: FontSubstitutionReason,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FontFallbackChain<'a> {
  pub requested_family: Option<Cow<'a, str>>,
  pub script: Option<TextScript>,
  pub language: Option<Cow<'a, str>>,
  pub families: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphFallbackCacheEntry<'a> {
  pub codepoint: u32,
  pub request: FontRequestKey<'a>,
  pub fallback_font_id: Option<FontId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontRequestKey<'a> {
  pub family: Option<Cow<'a, str>>,
  pub weight: Option<FontWeight>,
  pub slant: Option<FontSlant>,
  pub stretch: Option<FontStretch>,
  pub script: Option<TextScript>,
  pub language: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariationAxis<'a> {
  pub tag: Cow<'a, str>,
  pub name: Option<Cow<'a, str>>,
  pub min: f32,
  pub default: f32,
  pub max: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenTypeFeature<'a> {
  pub tag: Cow<'a, str>,
  pub enabled_by_default: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FontSource<'a> {
  System,
  Path {
    id: Cow<'a, str>,
    path: PathBuf,
    data: Option<FontBytes>,
  },
  Memory {
    id: Cow<'a, str>,
    data: FontBytes,
  },
  EmbeddedOoxml {
    id: Cow<'a, str>,
    data: FontBytes,
  },
  TestFixture {
    id: Cow<'a, str>,
    data: FontBytes,
  },
}

impl<'a> FontSource<'a> {
  pub fn id(&self) -> Option<&str> {
    match self {
      Self::Memory { id, .. } | Self::EmbeddedOoxml { id, .. } | Self::TestFixture { id, .. } => {
        Some(id.as_ref())
      }
      Self::Path { id, .. } => Some(id.as_ref()),
      Self::System => None,
    }
  }

  pub fn data(&self) -> Option<&[u8]> {
    match self {
      Self::Memory { data, .. }
      | Self::EmbeddedOoxml { data, .. }
      | Self::TestFixture { data, .. } => Some(data.as_slice()),
      Self::Path {
        data: Some(data), ..
      } => Some(data.as_slice()),
      Self::System | Self::Path { data: None, .. } => None,
    }
  }

  fn data_handle(&self) -> Option<FontBytes> {
    match self {
      Self::Memory { data, .. }
      | Self::EmbeddedOoxml { data, .. }
      | Self::TestFixture { data, .. } => Some(data.clone()),
      Self::Path {
        data: Some(data), ..
      } => Some(data.clone()),
      Self::System | Self::Path { data: None, .. } => None,
    }
  }
}

pub struct RegisteredFontFace<'a> {
  pub source: FontSource<'a>,
  pub family_names: Vec<Cow<'a, str>>,
  pub style_name: Option<Cow<'a, str>>,
  pub weight: FontWeight,
  pub slant: FontSlant,
  pub stretch: FontStretch,
  pub pitch: FontPitch,
  pub charset: Option<FontCharset>,
  pub face_index: u32,
  pub origin_priority: u16,
  runtime_face: OnceLock<Option<Arc<RuntimeFace>>>,
}

impl RegisteredFontFace<'_> {
  fn font_id(&self) -> Option<FontId> {
    self.source.id().map(|id| FontId(Arc::from(id)))
  }

  fn runtime_face(&self) -> Option<Arc<RuntimeFace>> {
    self
      .runtime_face
      .get_or_init(|| {
        self
          .source
          .data_handle()
          .and_then(|data| RuntimeFace::new(data, self.face_index).ok())
          .map(Arc::new)
      })
      .clone()
  }
}

impl<'a> Clone for RegisteredFontFace<'a> {
  fn clone(&self) -> Self {
    Self {
      source: self.source.clone(),
      family_names: self.family_names.clone(),
      style_name: self.style_name.clone(),
      weight: self.weight,
      slant: self.slant,
      stretch: self.stretch,
      pitch: self.pitch,
      charset: self.charset,
      face_index: self.face_index,
      origin_priority: self.origin_priority,
      runtime_face: OnceLock::new(),
    }
  }
}

impl<'a> fmt::Debug for RegisteredFontFace<'a> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("RegisteredFontFace")
      .field("source", &self.source)
      .field("family_names", &self.family_names)
      .field("style_name", &self.style_name)
      .field("weight", &self.weight)
      .field("slant", &self.slant)
      .field("stretch", &self.stretch)
      .field("pitch", &self.pitch)
      .field("charset", &self.charset)
      .field("face_index", &self.face_index)
      .field("origin_priority", &self.origin_priority)
      .finish()
  }
}

impl<'a> PartialEq for RegisteredFontFace<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.source == other.source
      && self.family_names == other.family_names
      && self.style_name == other.style_name
      && self.weight == other.weight
      && self.slant == other.slant
      && self.stretch == other.stretch
      && self.pitch == other.pitch
      && self.charset == other.charset
      && self.face_index == other.face_index
      && self.origin_priority == other.origin_priority
  }
}

impl<'a> Eq for RegisteredFontFace<'a> {}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ThemeFontMap<'a> {
  pub major_latin: Option<Cow<'a, str>>,
  pub minor_latin: Option<Cow<'a, str>>,
  pub major_east_asian: Option<Cow<'a, str>>,
  pub minor_east_asian: Option<Cow<'a, str>>,
  pub major_complex_script: Option<Cow<'a, str>>,
  pub minor_complex_script: Option<Cow<'a, str>>,
}

impl<'a> ThemeFontMap<'a> {
  pub fn resolve(&self, kind: ThemeFontKind) -> Option<Cow<'a, str>> {
    match kind {
      ThemeFontKind::MajorLatin => self.major_latin.clone(),
      ThemeFontKind::MinorLatin => self.minor_latin.clone(),
      ThemeFontKind::MajorEastAsian => self.major_east_asian.clone(),
      ThemeFontKind::MinorEastAsian => self.minor_east_asian.clone(),
      ThemeFontKind::MajorComplexScript => self.major_complex_script.clone(),
      ThemeFontKind::MinorComplexScript => self.minor_complex_script.clone(),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontRequest<'a> {
  pub family: Option<Cow<'a, str>>,
  pub theme_family: Option<ThemeFontKind>,
  pub family_class: Option<FontFamilyClass>,
  pub bold: bool,
  pub italic: bool,
  pub weight: Option<FontWeight>,
  pub slant: Option<FontSlant>,
  pub stretch: Option<FontStretch>,
  pub size_pt: FontSize,
  pub script: Option<TextScript>,
  pub language: Option<Cow<'a, str>>,
  pub region: Option<Cow<'a, str>>,
  pub charset: Option<FontCharset>,
  pub pitch: Option<FontPitch>,
  pub variations: Vec<VariationValue<'a>>,
  pub features: Vec<FeatureValue<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFont<'book> {
  pub font_id: FontId,
  pub resolved_family: Cow<'book, str>,
  pub source: FontSource<'book>,
  pub face_index: u32,
  pub synthetic_bold: bool,
  pub synthetic_italic: bool,
  pub metrics: FontMetrics,
  pub match_diagnostics: FontMatchDiagnostics<'book>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFontChain<'book> {
  fonts: Vec<(ResolvedFont<'book>, Option<u8>)>,
}

impl<'book> ResolvedFontChain<'book> {
  pub fn resolved_fonts(&self) -> impl Iterator<Item = &ResolvedFont<'book>> {
    self.fonts.iter().map(|(font, _)| font)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontFaceData<'a> {
  pub font_id: FontId,
  pub source: FontSource<'a>,
  pub face_index: u32,
  pub family_names: Vec<Cow<'a, str>>,
  pub style_name: Option<Cow<'a, str>>,
  pub data: Option<FontBytes>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FontBounds {
  pub global: Option<GlyphBounds>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlyphBounds {
  pub x_min_pt: f32,
  pub y_min_pt: f32,
  pub x_max_pt: f32,
  pub y_max_pt: f32,
}

impl GlyphBounds {
  pub fn scaled(self, size_pt: f32) -> Self {
    Self {
      x_min_pt: self.x_min_pt * size_pt,
      y_min_pt: self.y_min_pt * size_pt,
      x_max_pt: self.x_max_pt * size_pt,
      y_max_pt: self.y_max_pt * size_pt,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontEmbeddingPolicy {
  pub subset_policy: FontSubsetPolicy,
  pub installable: bool,
  pub restricted: bool,
}

impl Default for FontEmbeddingPolicy {
  fn default() -> Self {
    Self {
      subset_policy: FontSubsetPolicy::Subset,
      installable: true,
      restricted: false,
    }
  }
}

impl FontEmbeddingPolicy {
  pub fn viewing_allowed(self) -> bool {
    !self.restricted
  }

  pub fn editing_allowed(self) -> bool {
    self.installable || self.subset_policy == FontSubsetPolicy::EmbedFull
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontEmbeddingPlan<'a> {
  pub keep_tables: Vec<Cow<'a, str>>,
  pub downgrade_cff2: bool,
  pub desubroutinize_cff: bool,
  pub pin_variation_axes: bool,
}

impl<'a> Default for FontEmbeddingPlan<'a> {
  fn default() -> Self {
    Self {
      keep_tables: DEFAULT_PDF_EMBED_TABLES
        .iter()
        .map(|table| Cow::Borrowed(*table))
        .collect(),
      downgrade_cff2: false,
      desubroutinize_cff: false,
      pin_variation_axes: false,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeOptions<'a> {
  pub size_pt: FontSize,
  pub direction: TextDirection,
  pub script: Option<TextScript>,
  pub language: Option<Cow<'a, str>>,
  pub character_spacing_pt: f32,
  /// Horizontal glyph scale. Character spacing is added after this scale,
  /// matching WordprocessingML's distinction between `w:w` and `w:spacing`.
  pub horizontal_scale: f32,
  pub small_caps: bool,
  pub scan_registered_fallbacks: bool,
  pub features: Vec<FeatureValue<'a>>,
  pub variations: Vec<VariationValue<'a>>,
}

impl Default for ShapeOptions<'_> {
  fn default() -> Self {
    Self {
      size_pt: FontSize::default(),
      direction: TextDirection::default(),
      script: None,
      language: None,
      character_spacing_pt: 0.0,
      horizontal_scale: 1.0,
      small_caps: false,
      scan_registered_fallbacks: true,
      features: Vec::new(),
      variations: Vec::new(),
    }
  }
}

impl<'a> ShapeOptions<'a> {
  pub fn from_request(request: &FontRequest<'a>, direction: TextDirection) -> Self {
    ShapeOptions {
      size_pt: request.size_pt,
      direction,
      script: request.script,
      language: request.language.clone(),
      character_spacing_pt: 0.0,
      horizontal_scale: 1.0,
      small_caps: false,
      scan_registered_fallbacks: true,
      features: request.features.clone(),
      variations: request.variations.clone(),
    }
  }
}

impl<'book> ResolvedFont<'book> {
  pub fn metrics_at_size(&self, size: FontSize) -> FontMetrics {
    self.metrics.scaled(size.0)
  }

  pub fn shape_approximate<'text>(
    &self,
    text: &'text str,
    size: FontSize,
    direction: TextDirection,
    script: Option<TextScript>,
  ) -> ShapedRun<'text, 'book> {
    let safe_breaks = text_safe_breaks(text);
    let glyphs = approximate_glyphs(text, size);
    let advance_pt = glyphs.iter().map(|glyph| glyph.x_advance_pt).sum();
    ShapedRun {
      font_id: self.font_id.clone(),
      font_size_pt: size,
      text_range: 0..text.len(),
      text,
      glyphs: Cow::Owned(glyphs),
      advance_pt,
      direction,
      script,
      safe_breaks,
      approximate: true,
      decorations: Vec::new(),
      diagnostics: ShapingDiagnostics::default(),
    }
  }

  pub fn shape_with_ttf_bytes<'text>(
    &self,
    text: &'text str,
    data: &[u8],
    options: &ShapeOptions<'_>,
  ) -> Result<ShapedRun<'text, 'book>> {
    self.shape_with_font_bytes(text, FontBytes::from(data.to_vec()), options)
  }

  fn shape_with_font_bytes<'text>(
    &self,
    text: &'text str,
    data: impl Into<FontBytes>,
    options: &ShapeOptions<'_>,
  ) -> Result<ShapedRun<'text, 'book>> {
    let runtime_face =
      runtime_face_for_data(data.into(), self.face_index).ok_or(FontError::InvalidFace)?;
    self.shape_with_runtime_face(text, &runtime_face, options)
  }

  fn shape_with_runtime_face<'text>(
    &self,
    text: &'text str,
    runtime_face: &RuntimeFace,
    options: &ShapeOptions<'_>,
  ) -> Result<ShapedRun<'text, 'book>> {
    let small_caps =
      options.script.is_none_or(small_caps_supported_for_script) && options.small_caps;
    let has_lowercase = small_caps && text.chars().any(char::is_lowercase);
    let (shaped_text, small_caps_ranges) = if has_lowercase {
      small_caps_shaped_text(text)
    } else {
      (Cow::Borrowed(text), Vec::new())
    };
    let shape_size = if has_lowercase {
      FontSize(options.size_pt.0 * 0.8)
    } else {
      options.size_pt
    };
    let units_per_em = f32::from(runtime_face.ttf().units_per_em());
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(shaped_text.as_ref());
    buffer.guess_segment_properties();
    if let Some(direction) = harf_direction(options.direction) {
      buffer.set_direction(direction);
    }
    if let Some(script) = options.script.and_then(harf_script) {
      buffer.set_script(script);
    }
    if let Some(language) = options
      .language
      .as_deref()
      .and_then(|language| HarfLanguage::from_str(language).ok())
    {
      buffer.set_language(language);
    }
    let features = harf_features(&options.features);
    let output = font_timing("harfrust shape", || runtime_face.shape(buffer, &features));
    let infos = output.glyph_infos();
    let positions = output.glyph_positions();
    let safe_breaks = text_safe_breaks(text);
    let tracking = options.character_spacing_pt;
    let horizontal_scale = options.horizontal_scale.max(f32::EPSILON);
    let glyphs = infos
      .iter()
      .zip(positions.iter())
      .enumerate()
      .map(|(index, (info, position))| {
        let shaped_text_range = glyph_text_range(shaped_text.as_ref(), infos, index);
        let source_char = shaped_text
          .get(shaped_text_range.clone())
          .and_then(|cluster| cluster.chars().next());
        let text_range = if small_caps_ranges.is_empty() {
          shaped_text_range
        } else {
          source_range_for_shaped_range(&small_caps_ranges, shaped_text_range, text.len())
        };
        let mut x_advance_pt =
          position.x_advance as f32 / units_per_em * shape_size.0 * horizontal_scale;
        if tracking.abs() > f32::EPSILON && index + 1 < infos.len() {
          x_advance_pt += tracking;
        }
        let justification = source_char.map(glyph_justification).unwrap_or_default();
        ShapedGlyph {
          glyph_id: info.glyph_id,
          cluster: text_range.start as u32,
          text_range,
          x_advance_pt,
          y_advance_pt: position.y_advance as f32 / units_per_em * shape_size.0,
          x_offset_pt: position.x_offset as f32 / units_per_em * shape_size.0 * horizontal_scale,
          y_offset_pt: position.y_offset as f32 / units_per_em * shape_size.0,
          safe_to_break: !info.unsafe_to_break(),
          source_char,
          justifiable: justification.space
            || justification.cjk
            || justification.cjk_punctuation
            || justification.kashida,
          justification,
          bounds: runtime_face
            .glyph_bounds(info.glyph_id as u16)
            .map(|bounds| {
              let mut bounds = bounds.scaled(shape_size.0);
              bounds.x_min_pt *= horizontal_scale;
              bounds.x_max_pt *= horizontal_scale;
              bounds
            }),
        }
      })
      .collect::<Vec<_>>();
    let advance_pt = glyphs.iter().map(|glyph| glyph.x_advance_pt).sum();
    let diagnostics = ShapingDiagnostics {
      missing_glyphs: missing_glyphs_from_shaped_glyphs(&glyphs),
      fallback_runs: Vec::new(),
    };

    Ok(ShapedRun {
      font_id: self.font_id.clone(),
      font_size_pt: shape_size,
      text_range: 0..text.len(),
      text,
      glyphs: Cow::Owned(glyphs),
      advance_pt,
      direction: options.direction,
      script: options.script,
      safe_breaks,
      approximate: false,
      decorations: Vec::new(),
      diagnostics,
    })
  }

  pub fn glyph_bounds(
    &self,
    data: &[u8],
    glyph_id: u32,
    size: FontSize,
  ) -> Result<Option<GlyphBounds>> {
    let face = TtfFace::parse(data, self.face_index).map_err(|_| FontError::InvalidFace)?;
    let units_per_em = f32::from(face.units_per_em());
    Ok(
      face
        .glyph_bounding_box(GlyphId(glyph_id as u16))
        .map(|bounds| glyph_bounds_from_ttf_rect(bounds, units_per_em).scaled(size.0)),
    )
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontScriptRun {
  pub text_range: Range<usize>,
  pub script: TextScript,
  pub direction: TextDirection,
  pub size_pt: FontSize,
  pub small_caps: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScriptScanOptions {
  pub app_script: TextScript,
  pub small_caps: bool,
  /// Apply the explicit WordprocessingML rFonts slot classification before
  /// falling back to Unicode Script for Common characters.
  pub wordprocessingml_font_slots: bool,
}

impl Default for ScriptScanOptions {
  fn default() -> Self {
    Self {
      app_script: TextScript::Common,
      small_caps: false,
      wordprocessingml_font_slots: false,
    }
  }
}

pub fn script_direction_runs(
  text: &str,
  size_pt: FontSize,
  small_caps: bool,
) -> Vec<FontScriptRun> {
  script_direction_runs_with_options(
    text,
    size_pt,
    ScriptScanOptions {
      small_caps,
      ..ScriptScanOptions::default()
    },
  )
}

pub fn script_direction_runs_with_options(
  text: &str,
  size_pt: FontSize,
  options: ScriptScanOptions,
) -> Vec<FontScriptRun> {
  if options.small_caps {
    small_caps_script_direction_runs(
      text,
      size_pt,
      options.app_script,
      options.wordprocessingml_font_slots,
    )
  } else {
    script_direction_runs_for_segment(
      text,
      0,
      size_pt,
      false,
      options.app_script,
      options.wordprocessingml_font_slots,
    )
  }
}

fn small_caps_script_direction_runs(
  text: &str,
  size_pt: FontSize,
  app_script: TextScript,
  wordprocessingml_font_slots: bool,
) -> Vec<FontScriptRun> {
  let mut runs = Vec::new();
  let mut start = 0usize;
  let mut active_upper = None::<bool>;
  for (index, ch) in text.char_indices() {
    let is_upper = ch.is_uppercase() && !ch.is_lowercase();
    if let Some(active) = active_upper
      && active != is_upper
    {
      push_small_caps_case_run(
        text,
        start..index,
        active,
        size_pt,
        app_script,
        wordprocessingml_font_slots,
        &mut runs,
      );
      start = index;
    }
    active_upper = Some(is_upper);
  }
  if start < text.len() {
    push_small_caps_case_run(
      text,
      start..text.len(),
      active_upper.unwrap_or(false),
      size_pt,
      app_script,
      wordprocessingml_font_slots,
      &mut runs,
    );
  }
  runs
}

fn push_small_caps_case_run(
  source: &str,
  range: Range<usize>,
  upper_run: bool,
  size_pt: FontSize,
  app_script: TextScript,
  wordprocessingml_font_slots: bool,
  runs: &mut Vec<FontScriptRun>,
) {
  let mut segment_runs = script_direction_runs_for_segment(
    &source[range.clone()],
    range.start,
    size_pt,
    !upper_run,
    app_script,
    wordprocessingml_font_slots,
  );
  runs.append(&mut segment_runs);
}

fn script_direction_runs_for_segment(
  text: &str,
  range_offset: usize,
  size_pt: FontSize,
  small_caps: bool,
  app_script: TextScript,
  wordprocessingml_font_slots: bool,
) -> Vec<FontScriptRun> {
  let mut runs = Vec::new();
  script_direction_runs_for_segment_into(
    text,
    range_offset,
    size_pt,
    small_caps,
    app_script,
    wordprocessingml_font_slots,
    &mut runs,
  );
  runs
}

fn script_direction_runs_for_segment_into(
  text: &str,
  range_offset: usize,
  size_pt: FontSize,
  small_caps: bool,
  app_script: TextScript,
  wordprocessingml_font_slots: bool,
  runs: &mut Vec<FontScriptRun>,
) {
  if text.is_empty() {
    return;
  }
  let leading_script =
    first_strong_text_script(text, wordprocessingml_font_slots).unwrap_or(app_script);
  let mut start = 0usize;
  let mut active = None::<TextScript>;
  let mut pending_weak_start = None::<usize>;
  let mut pending_weak_has_inherited = false;
  for (index, ch) in text.char_indices() {
    let unicode_script = ch.script();
    if is_nonspacing_mark(ch) {
      active.get_or_insert(leading_script);
      pending_weak_start.get_or_insert(index);
      pending_weak_has_inherited = true;
      continue;
    }
    let Some(script) = strong_text_script(ch, wordprocessingml_font_slots) else {
      active.get_or_insert(leading_script);
      pending_weak_start.get_or_insert(index);
      pending_weak_has_inherited |= unicode_script == UnicodeScriptValue::Inherited;
      continue;
    };

    match active {
      None => {
        active = Some(script);
      }
      Some(active_script) if script != active_script => {
        let split = if pending_weak_has_inherited {
          pending_weak_start.unwrap_or(index)
        } else {
          index
        };
        if start < split {
          push_script_direction_run(
            text,
            start..split,
            active_script,
            range_offset,
            size_pt,
            small_caps,
            runs,
          );
        }
        start = split;
        active = Some(script);
      }
      Some(_) => {}
    }
    pending_weak_start = None;
    pending_weak_has_inherited = false;
  }
  if start < text.len() {
    push_script_direction_run(
      text,
      start..text.len(),
      active.unwrap_or(leading_script),
      range_offset,
      size_pt,
      small_caps,
      runs,
    );
  }
}

fn push_script_direction_run(
  source: &str,
  range: Range<usize>,
  script: TextScript,
  range_offset: usize,
  size_pt: FontSize,
  small_caps: bool,
  runs: &mut Vec<FontScriptRun>,
) {
  let value = &source[range.clone()];
  runs.push(FontScriptRun {
    text_range: (range.start + range_offset)..(range.end + range_offset),
    script,
    direction: text_direction_from_bidi(get_base_direction(value)),
    size_pt,
    small_caps: small_caps && small_caps_supported_for_script(script),
  });
}

fn small_caps_supported_for_script(script: TextScript) -> bool {
  !matches!(
    script,
    TextScript::Arabic | TextScript::Hebrew | TextScript::Devanagari | TextScript::Thai
  )
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontMatchDiagnostics<'a> {
  pub candidates: Vec<FontMatchCandidate<'a>>,
  pub fallback_level: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontMatchCandidate<'a> {
  pub font_id: FontId,
  pub family: Cow<'a, str>,
  pub score: i32,
  pub rejected: bool,
  pub reason: Option<FontMatchReason>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FontMatchRank {
  rejected: bool,
  family_class_mismatch: bool,
  charset_mismatch: bool,
  slant_mismatch: bool,
  stretch_distance: i32,
  weight_distance: i32,
  pitch_mismatch: bool,
}

impl FontMatchRank {
  fn distance(self) -> i32 {
    i32::from(self.rejected)
      + i32::from(self.family_class_mismatch)
      + i32::from(self.charset_mismatch)
      + i32::from(self.slant_mismatch)
      + i32::from(self.stretch_distance != 0)
      + i32::from(self.weight_distance != 0)
      + i32::from(self.pitch_mismatch)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontMatchReason {
  Family,
  FamilyClass,
  StyleName,
  Charset,
  Pitch,
  Weight,
  Slant,
  Stretch,
  Coverage,
  SourcePriority,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariationValue<'a> {
  pub tag: Cow<'a, str>,
  pub value: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureValue<'a> {
  pub tag: Cow<'a, str>,
  pub value: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureSetting<'a> {
  pub tag: Cow<'a, str>,
  pub value: u32,
  pub start: u32,
  pub end: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontSubstitution<'a> {
  pub requested_family: Cow<'a, str>,
  pub substituted_family: Cow<'a, str>,
  pub reason: FontSubstitutionReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontSubstitutionReason {
  ThemeResolved,
  Alias,
  MissingFamily,
  MissingStyle,
  MissingGlyph,
  LastResort,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontMetrics {
  pub vertical: VerticalMetrics,
  pub decoration: DecorationMetrics,
  pub script: ScriptMetrics,
  pub em_size: f32,
}

impl Default for FontMetrics {
  fn default() -> Self {
    Self {
      vertical: VerticalMetrics::default(),
      decoration: DecorationMetrics::default(),
      script: ScriptMetrics::default(),
      em_size: 1.0,
    }
  }
}

impl FontMetrics {
  pub fn scaled(&self, size_pt: f32) -> Self {
    let scale = if self.em_size > 0.0 {
      size_pt / self.em_size
    } else {
      size_pt
    };
    Self {
      vertical: self.vertical.scaled(scale),
      decoration: self.decoration.scaled(scale),
      script: self.script.scaled(scale),
      em_size: size_pt,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VerticalMetrics {
  pub ascent_pt: f32,
  pub descent_pt: f32,
  pub internal_leading_pt: f32,
  pub external_leading_pt: f32,
  pub line_gap_pt: f32,
  pub ink_height_pt: f32,
  pub baseline_offset_pt: f32,
  pub hanging_baseline_pt: f32,
  pub cjk_horizontal_advance_pt: f32,
  pub cjk_vertical_advance_pt: f32,
}

impl VerticalMetrics {
  fn scaled(self, scale: f32) -> Self {
    Self {
      ascent_pt: self.ascent_pt * scale,
      descent_pt: self.descent_pt * scale,
      internal_leading_pt: self.internal_leading_pt * scale,
      external_leading_pt: self.external_leading_pt * scale,
      line_gap_pt: self.line_gap_pt * scale,
      ink_height_pt: self.ink_height_pt * scale,
      baseline_offset_pt: self.baseline_offset_pt * scale,
      hanging_baseline_pt: self.hanging_baseline_pt * scale,
      cjk_horizontal_advance_pt: self.cjk_horizontal_advance_pt * scale,
      cjk_vertical_advance_pt: self.cjk_vertical_advance_pt * scale,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DecorationMetrics {
  pub underline_offset_pt: f32,
  pub underline_thickness_pt: f32,
  pub strikeout_offset_pt: f32,
  pub strikeout_thickness_pt: f32,
}

impl DecorationMetrics {
  fn scaled(self, scale: f32) -> Self {
    Self {
      underline_offset_pt: self.underline_offset_pt * scale,
      underline_thickness_pt: self.underline_thickness_pt * scale,
      strikeout_offset_pt: self.strikeout_offset_pt * scale,
      strikeout_thickness_pt: self.strikeout_thickness_pt * scale,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScriptMetrics {
  pub superscript_scale: f32,
  pub subscript_scale: f32,
  pub superscript_offset_pt: f32,
  pub subscript_offset_pt: f32,
  pub small_caps_scale: f32,
}

impl ScriptMetrics {
  fn scaled(self, scale: f32) -> Self {
    Self {
      superscript_scale: self.superscript_scale,
      subscript_scale: self.subscript_scale,
      superscript_offset_pt: self.superscript_offset_pt * scale,
      subscript_offset_pt: self.subscript_offset_pt * scale,
      small_caps_scale: self.small_caps_scale,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapedRun<'text, 'meta> {
  pub font_id: FontId,
  /// Point size used to shape and render this run. Synthesized small-caps
  /// runs can be smaller than the requested text style.
  pub font_size_pt: FontSize,
  pub text: &'text str,
  pub text_range: Range<usize>,
  pub glyphs: Cow<'text, [ShapedGlyph]>,
  pub advance_pt: f32,
  pub direction: TextDirection,
  pub script: Option<TextScript>,
  pub safe_breaks: Vec<usize>,
  pub approximate: bool,
  pub decorations: Vec<TextDecoration>,
  pub diagnostics: ShapingDiagnostics<'meta>,
}

impl ShapedRun<'_, '_> {
  pub fn offset_text_range(&mut self, offset: usize) {
    if offset == 0 {
      return;
    }
    self.text_range = self.text_range.start + offset..self.text_range.end + offset;
    for glyph in self.glyphs.to_mut() {
      glyph.cluster = glyph.cluster.saturating_add(offset as u32);
      glyph.text_range = glyph.text_range.start + offset..glyph.text_range.end + offset;
    }
    for boundary in &mut self.safe_breaks {
      *boundary += offset;
    }
    for missing in &mut self.diagnostics.missing_glyphs {
      missing.text_range = missing.text_range.start + offset..missing.text_range.end + offset;
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShapingDiagnostics<'a> {
  pub missing_glyphs: Vec<MissingGlyph>,
  pub fallback_runs: Vec<FallbackRun<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingGlyph {
  pub codepoint: u32,
  pub text_range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FallbackRun<'a> {
  pub text_range: Range<usize>,
  pub font_id: Option<FontId>,
  pub fallback_level: u8,
  pub reason: FontSubstitutionReason,
  pub family: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShapedGlyph {
  pub glyph_id: u32,
  pub cluster: u32,
  pub text_range: Range<usize>,
  pub x_advance_pt: f32,
  pub y_advance_pt: f32,
  pub x_offset_pt: f32,
  pub y_offset_pt: f32,
  pub safe_to_break: bool,
  pub source_char: Option<char>,
  pub justifiable: bool,
  pub justification: GlyphJustification,
  pub bounds: Option<GlyphBounds>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GlyphJustification {
  pub space: bool,
  pub cjk: bool,
  pub cjk_punctuation: bool,
  pub kashida: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDecoration {
  Underline,
  DoubleUnderline,
  Strikeout,
  Overline,
  WaveUnderline,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FontUsage {
  pub font_id: FontId,
  pub glyph_ids: BTreeSet<u32>,
  pub unicode_ranges: Vec<Range<u32>>,
  pub needs_embedding: bool,
  pub subset_policy: FontSubsetPolicy,
  pub color_glyph_usage: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FontUsageCollector {
  pub usages: Vec<FontUsage>,
}

impl FontUsageCollector {
  pub fn record_run(&mut self, run: &ShapedRun<'_, '_>) {
    self.record_run_with_policy(run, FontEmbeddingPolicy::default());
  }

  pub fn record_run_with_policy(&mut self, run: &ShapedRun<'_, '_>, policy: FontEmbeddingPolicy) {
    let usage = match self
      .usages
      .iter_mut()
      .find(|usage| usage.font_id == run.font_id)
    {
      Some(usage) => usage,
      None => {
        self.usages.push(FontUsage {
          font_id: run.font_id.clone(),
          needs_embedding: !run.approximate && policy.subset_policy != FontSubsetPolicy::DoNotEmbed,
          subset_policy: policy.subset_policy,
          ..FontUsage::default()
        });
        self.usages.last_mut().expect("usage was just pushed")
      }
    };
    usage.needs_embedding |=
      !run.approximate && policy.subset_policy != FontSubsetPolicy::DoNotEmbed;
    if policy.subset_policy == FontSubsetPolicy::DoNotEmbed {
      usage.subset_policy = FontSubsetPolicy::DoNotEmbed;
    } else if policy.subset_policy == FontSubsetPolicy::EmbedFull {
      usage.subset_policy = FontSubsetPolicy::EmbedFull;
    }
    for glyph in run.glyphs.iter() {
      usage.glyph_ids.insert(glyph.glyph_id);
      if let Some(ch) = glyph.source_char {
        push_unicode_range(&mut usage.unicode_ranges, u32::from(ch));
      }
    }
  }

  pub fn record_runs<'run, 'text, 'meta>(
    &mut self,
    runs: impl IntoIterator<Item = &'run ShapedRun<'text, 'meta>>,
  ) where
    'text: 'run,
    'meta: 'run,
  {
    for run in runs {
      self.record_run(run);
    }
  }

  pub fn record_runs_with_policy<'run, 'text, 'meta>(
    &mut self,
    runs: impl IntoIterator<Item = &'run ShapedRun<'text, 'meta>>,
    policy: FontEmbeddingPolicy,
  ) where
    'text: 'run,
    'meta: 'run,
  {
    for run in runs {
      self.record_run_with_policy(run, policy);
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontSubsetPolicy {
  #[default]
  Subset,
  EmbedFull,
  DoNotEmbed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FontSize(pub f32);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ThemeFontKind {
  MajorLatin,
  #[default]
  MinorLatin,
  MajorEastAsian,
  MinorEastAsian,
  MajorComplexScript,
  MinorComplexScript,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontWeight {
  Thin,
  ExtraLight,
  Light,
  #[default]
  Normal,
  Medium,
  SemiBold,
  Bold,
  ExtraBold,
  Black,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontSlant {
  #[default]
  Upright,
  Italic,
  Oblique,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontStretch {
  UltraCondensed,
  ExtraCondensed,
  Condensed,
  SemiCondensed,
  #[default]
  Normal,
  SemiExpanded,
  Expanded,
  ExtraExpanded,
  UltraExpanded,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FontPitch {
  Fixed,
  #[default]
  Variable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FontFamilyClass {
  Serif,
  SansSerif,
  Fixed,
  Decorative,
  BrushScript,
  Titling,
  Capitals,
  OldStyle,
  Schoolbook,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontCharset {
  Ansi,
  Symbol,
  ShiftJis,
  Hangul,
  Gb2312,
  ChineseBig5,
  Greek,
  Turkish,
  Vietnamese,
  Hebrew,
  Arabic,
  Baltic,
  Russian,
  Thai,
  EastEurope,
  Oem,
  Other(u8),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextDirection {
  #[default]
  LeftToRight,
  RightToLeft,
  TopToBottom,
  BottomToTop,
  Mixed,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TextScript {
  #[default]
  Common,
  Latin,
  Cyrillic,
  Greek,
  Han,
  Hiragana,
  Katakana,
  Hangul,
  Arabic,
  Hebrew,
  Devanagari,
  Thai,
  Other,
}

const DEFAULT_OFFICE_ALIASES: &[(&str, &str)] = &[
  ("Courier", "Courier New"),
  ("TimesNewRomanPSMT", "Times New Roman"),
  // Office documents can store the Simplified Chinese localized family name,
  // while the same installed face is commonly exposed to fontdb under its
  // English family name.
  ("等线", "DengXian"),
  // Legacy Office workbooks use the old Windows localized family name,
  // while current Excel substitutes the installed Arial face.
  ("Arial Cyr", "Arial"),
  // Office's fixed-format writer uses Arial for Latin text when the legacy
  // Arial Unicode MS face is unavailable; script fallback still owns glyphs
  // outside Arial's coverage.
  ("Arial Unicode MS", "Arial"),
  // Office fixed-format output maps this unavailable Swiss-family face to
  // Arial for legacy SpreadsheetML workbooks.
  ("Frutiger 45 Light", "Arial"),
  // Apache POI 47862.xlsx requests this unavailable legacy Helvetica Neue
  // face; Office fixed-format output substitutes the installed Arial face.
  ("HelveticaNeue LightExt", "Arial"),
  ("Helvetica Neue", "Arial"),
  ("Helvetica Neue Light", "Arial"),
  ("Helvetica Neue Medium", "Arial"),
  // Apache POI bug65228.pptx carries a macOS Graphik theme, while Office's
  // fixed PDF substitutes Calibri for the unavailable family.
  ("Graphik", "Calibri"),
  ("DINPro-Medium", "DINPro"),
  ("Univers 45 Light", "Univers Light"),
];

const DEFAULT_PDF_EMBED_TABLES: &[&str] = &[
  "head", "hhea", "hmtx", "loca", "maxp", "glyf", "CFF ", "post", "name", "OS/2", "cvt ", "fpgm",
  "prep", "CFF2",
];

fn default_fallback_chains<'a>() -> Vec<FontFallbackChain<'a>> {
  vec![
    office_family_fallback("Calibri", &["Carlito", "Liberation Sans"]),
    office_family_fallback("Calibri Light", &["Carlito", "Liberation Sans"]),
    office_family_fallback("Cambria", &["Caladea", "Liberation Serif"]),
    office_family_fallback("Times New Roman", &["Liberation Serif", "Nimbus Roman"]),
    office_family_fallback(
      "TimesNewRomanPSMT",
      &["Times New Roman", "Liberation Serif", "Nimbus Roman"],
    ),
    office_family_fallback(
      "Courier",
      &["Courier New", "Liberation Mono", "DejaVu Sans Mono"],
    ),
    office_family_fallback("Arial", &["Liberation Sans", "Arimo"]),
    office_family_fallback("Arial Black", &["Arial", "Liberation Sans"]),
    office_family_fallback("Yu Gothic", &["Noto Sans CJK JP"]),
    office_family_fallback("游ゴシック", &["Yu Gothic", "Noto Sans CJK JP"]),
    office_family_fallback("BIZ UD明朝", &["BIZ UDMincho", "Noto Serif CJK JP"]),
    office_family_fallback(
      "BIZ UD明朝 Medium",
      &["BIZ UDMincho Medium", "Noto Serif CJK JP"],
    ),
    office_family_fallback(
      "BIZ UDMincho",
      &["BIZ UDMincho Medium", "Noto Serif CJK JP"],
    ),
    office_family_fallback(
      "BIZ UDMincho Medium",
      &["BIZ UDMincho", "Noto Serif CJK JP"],
    ),
    FontFallbackChain {
      requested_family: None,
      script: Some(TextScript::Han),
      language: None,
      families: vec![
        Cow::Borrowed("Noto Sans CJK JP"),
        Cow::Borrowed("Noto Serif CJK JP"),
      ],
    },
    FontFallbackChain {
      requested_family: None,
      script: Some(TextScript::Hangul),
      language: None,
      // Current Windows Office uses Malgun Gothic as its Korean UI/body
      // fallback. Noto Sans CJK KR is the metrically stable open fallback in
      // the checked-in Linux golden environment.
      families: vec![
        Cow::Borrowed("Malgun Gothic"),
        Cow::Borrowed("Noto Sans CJK KR"),
      ],
    },
    FontFallbackChain {
      requested_family: None,
      script: Some(TextScript::Arabic),
      language: None,
      families: vec![Cow::Borrowed("Amiri"), Cow::Borrowed("Noto Naskh Arabic")],
    },
    FontFallbackChain {
      requested_family: None,
      script: Some(TextScript::Greek),
      language: None,
      // Office uses Cambria Math for the Mathematical Alphanumeric Symbols
      // Greek block when the requested worksheet face has no glyphs. Keep it
      // ahead of generic sans-serif fallbacks so coverage, not .notdef, owns
      // these code points.
      families: vec![Cow::Borrowed("Cambria Math")],
    },
    FontFallbackChain {
      requested_family: None,
      script: Some(TextScript::Other),
      language: None,
      // Windows routes historic scripts such as Gothic through Segoe UI
      // Historic. Noto Sans Gothic preserves that coverage on systems where
      // the Office face is unavailable; coverage checks reject either face
      // for unrelated `Other` scripts.
      families: vec![
        Cow::Borrowed("Segoe UI Historic"),
        Cow::Borrowed("Noto Sans Gothic"),
      ],
    },
    FontFallbackChain {
      requested_family: None,
      script: None,
      language: None,
      families: vec![
        // Windows Office routes unsupported Common-script symbols (including
        // Word 2010 checkbox characters U+2610/U+2612) through Segoe UI
        // Symbol before generic text faces.
        Cow::Borrowed("Segoe UI Symbol"),
        Cow::Borrowed("DejaVu Sans"),
        Cow::Borrowed("Liberation Sans"),
        Cow::Borrowed("Noto Sans"),
        // Keep a pan-CJK face in the cached chain for Japanese kana and
        // fullwidth Latin forms. Those runs are not classified as Han, so
        // the Han-specific chain above cannot supply their glyphs.
        Cow::Borrowed("Noto Sans CJK JP"),
      ],
    },
  ]
}

fn office_family_fallback<'a>(
  family: &'static str,
  fallbacks: &[&'static str],
) -> FontFallbackChain<'a> {
  FontFallbackChain {
    requested_family: Some(Cow::Borrowed(family)),
    script: None,
    language: None,
    families: fallbacks
      .iter()
      .map(|family| Cow::Borrowed(*family))
      .collect(),
  }
}

fn push_unique_string<'a>(values: &mut Vec<Cow<'a, str>>, value: String) {
  if !values.iter().any(|existing| existing.as_ref() == value) {
    values.push(Cow::Owned(value));
  }
}

fn primary_family<'a>(face: &FontFaceInfo<'a>) -> Cow<'a, str> {
  face
    .family_names
    .first()
    .cloned()
    .unwrap_or_else(|| Cow::Owned(face.font_id.0.to_string()))
}

fn primary_family_sort_key<'a>(face: &'a FontFaceInfo<'_>) -> &'a str {
  face
    .family_names
    .first()
    .map(Cow::as_ref)
    .unwrap_or(face.font_id.0.as_ref())
}

#[allow(clippy::too_many_arguments)]
fn score_font_match(
  face_index: usize,
  face: &FontFaceInfo<'_>,
  registered_faces: &[RegisteredFontFace<'_>],
  request: &FontRequest<'_>,
  target_family_names: Option<&[String]>,
  requested_slant: FontSlant,
  requested_stretch: FontStretch,
  requested_weight: FontWeight,
) -> ScoredFontMatch {
  let mut rejected = false;
  let mut reason = None;

  if let Some(target_names) = target_family_names {
    if family_matches_names(face, target_names) {
      reason = Some(FontMatchReason::Family);
    } else {
      rejected = true;
      reason = Some(FontMatchReason::Family);
    }
  }

  let registered = registered_face_for_book_index(face_index, face, registered_faces);
  let family_class_mismatch = target_family_names.is_none()
    && request
      .family_class
      .is_some_and(|class| !font_family_class_matches(class, face));
  let charset_mismatch = request
    .charset
    .is_some_and(|charset| !font_charset_matches(charset, face, registered));
  let slant_mismatch = face.slant != requested_slant;
  let stretch_distance = stretch_distance(face.stretch, requested_stretch);
  let weight_distance = weight_distance(face.weight, requested_weight);
  let pitch_mismatch = request.pitch.is_some_and(|pitch| pitch != face.pitch);
  if charset_mismatch && !rejected && reason == Some(FontMatchReason::Family) {
    reason = Some(FontMatchReason::Charset);
  }
  if target_family_names.is_none() && charset_mismatch {
    rejected = true;
    reason = Some(FontMatchReason::Charset);
  }
  if family_class_mismatch {
    rejected = true;
    reason = Some(FontMatchReason::FamilyClass);
  }
  if slant_mismatch && !rejected {
    reason = Some(FontMatchReason::Slant);
  }
  if stretch_distance != 0 && !rejected && reason == Some(FontMatchReason::Family) {
    reason = Some(FontMatchReason::Stretch);
  }
  if pitch_mismatch && !rejected && reason == Some(FontMatchReason::Family) {
    reason = Some(FontMatchReason::Pitch);
  }

  ScoredFontMatch {
    rank: FontMatchRank {
      rejected,
      family_class_mismatch,
      charset_mismatch,
      slant_mismatch,
      stretch_distance,
      weight_distance,
      pitch_mismatch,
    },
    face_index,
    rejected,
    reason,
  }
}

fn scored_font_match_cmp(
  left: ScoredFontMatch,
  right: ScoredFontMatch,
  faces: &[FontFaceInfo<'_>],
) -> std::cmp::Ordering {
  left.rank.cmp(&right.rank).then_with(|| {
    primary_family_sort_key(&faces[left.face_index])
      .cmp(primary_family_sort_key(&faces[right.face_index]))
  })
}

fn normalize_family(value: &str) -> String {
  value
    .chars()
    .filter(|ch| !ch.is_ascii_whitespace() && *ch != '-' && *ch != '_')
    .flat_map(char::to_lowercase)
    .collect()
}

fn family_matches_names(face: &FontFaceInfo<'_>, target_names: &[String]) -> bool {
  face.family_names.iter().any(|candidate| {
    candidate.split(';').map(str::trim).any(|candidate| {
      target_names
        .iter()
        .any(|target| normalized_family_eq_normalized(candidate, target))
    })
  })
}

fn normalized_family_eq_normalized(candidate: &str, normalized: &str) -> bool {
  normalized_family_chars(candidate).eq(normalized.chars())
}

fn normalized_family_chars(value: &str) -> impl Iterator<Item = char> + '_ {
  value
    .chars()
    .filter(|ch| !ch.is_ascii_whitespace() && *ch != '-' && *ch != '_')
    .flat_map(char::to_lowercase)
}

fn registered_face<'faces, 'book>(
  face: &FontFaceInfo<'book>,
  registered_faces: &'faces [RegisteredFontFace<'book>],
) -> Option<&'faces RegisteredFontFace<'book>> {
  registered_faces.iter().find(|registered| {
    registered.face_index == face.face_index
      && registered.source.id() == Some(face.font_id.0.as_ref())
  })
}

fn registered_face_for_book_index<'faces, 'book>(
  book_index: usize,
  face: &FontFaceInfo<'book>,
  registered_faces: &'faces [RegisteredFontFace<'book>],
) -> Option<&'faces RegisteredFontFace<'book>> {
  registered_faces
    .get(book_index)
    .filter(|registered| {
      registered.face_index == face.face_index
        && registered.source.id() == Some(face.font_id.0.as_ref())
    })
    .or_else(|| registered_face(face, registered_faces))
}

fn font_family_class_matches(requested: FontFamilyClass, face: &FontFaceInfo<'_>) -> bool {
  face.family_class == Some(requested)
    || (requested == FontFamilyClass::Fixed
      && (face.pitch == FontPitch::Fixed || face.flags.monospace))
    || (requested == FontFamilyClass::Serif && face.flags.serif)
    || face
      .family_names
      .iter()
      .any(|family| font_family_name_matches_class(family, requested))
}

fn font_family_name_matches_class(family: &str, requested: FontFamilyClass) -> bool {
  let normalized = normalize_family(family);
  match requested {
    FontFamilyClass::Serif => {
      !normalized.contains("sans") && (normalized.contains("serif") || normalized.contains("roman"))
    }
    FontFamilyClass::SansSerif => normalized.contains("sans"),
    FontFamilyClass::Fixed => false,
    FontFamilyClass::Decorative => normalized.contains("decorative"),
    FontFamilyClass::BrushScript => normalized.contains("script"),
    FontFamilyClass::Titling => normalized.contains("titling"),
    FontFamilyClass::Capitals => normalized.contains("caps") || normalized.contains("capitals"),
    FontFamilyClass::OldStyle => normalized.contains("oldstyle"),
    FontFamilyClass::Schoolbook => normalized.contains("schoolbook"),
  }
}

fn normalized_family_names(value: &str) -> Vec<String> {
  value
    .split(';')
    .map(str::trim)
    .filter(|token| !token.is_empty())
    .map(normalize_family)
    .collect()
}

fn font_charset_matches(
  charset: FontCharset,
  face: &FontFaceInfo<'_>,
  registered: Option<&RegisteredFontFace<'_>>,
) -> bool {
  registered.and_then(|face| face.charset) == Some(charset)
    || (charset == FontCharset::Symbol && face.flags.symbolic)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum FontDbQueryFamily {
  Name(String),
  SansSerif,
  Serif,
}

struct PlatformFontSystem {
  collection: PlatformFontCollection,
  source_cache: PlatformSourceCache,
}

impl PlatformFontSystem {
  fn new() -> Self {
    Self {
      collection: PlatformFontCollection::new(PlatformFontCollectionOptions {
        shared: false,
        system_fonts: true,
      }),
      source_cache: PlatformSourceCache::default(),
    }
  }
}

struct PlatformFontCandidate {
  data: Vec<u8>,
  face_index: u32,
}

fn platform_system_query_fonts(
  query_family: &FontDbQueryFamily,
  request: &FontRequest<'_>,
) -> Vec<PlatformFontCandidate> {
  static SYSTEM: OnceLock<Mutex<PlatformFontSystem>> = OnceLock::new();

  let mut system = SYSTEM
    .get_or_init(|| Mutex::new(PlatformFontSystem::new()))
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
  let PlatformFontSystem {
    collection,
    source_cache,
  } = &mut *system;
  let family = match query_family {
    FontDbQueryFamily::Name(name) => PlatformQueryFamily::Named(name),
    FontDbQueryFamily::SansSerif => PlatformQueryFamily::Generic(PlatformGenericFamily::SansSerif),
    FontDbQueryFamily::Serif => PlatformQueryFamily::Generic(PlatformGenericFamily::Serif),
  };
  let mut query = collection.query(source_cache);
  query.set_families([family]);
  query.set_attributes(PlatformFontAttributes::new(
    platform_font_width(request),
    platform_font_style(request),
    platform_font_weight(request),
  ));

  let mut candidates = Vec::new();
  query.matches_with(|font| {
    candidates.push(PlatformFontCandidate {
      data: font.blob.as_ref().to_vec(),
      face_index: font.index,
    });
    if candidates.len() >= 32 {
      PlatformQueryStatus::Stop
    } else {
      PlatformQueryStatus::Continue
    }
  });
  candidates
}

fn platform_font_weight(request: &FontRequest<'_>) -> PlatformFontWeight {
  match requested_weight(request) {
    FontWeight::Thin => PlatformFontWeight::THIN,
    FontWeight::ExtraLight => PlatformFontWeight::EXTRA_LIGHT,
    FontWeight::Light => PlatformFontWeight::LIGHT,
    FontWeight::Normal => PlatformFontWeight::NORMAL,
    FontWeight::Medium => PlatformFontWeight::MEDIUM,
    FontWeight::SemiBold => PlatformFontWeight::SEMI_BOLD,
    FontWeight::Bold => PlatformFontWeight::BOLD,
    FontWeight::ExtraBold => PlatformFontWeight::EXTRA_BOLD,
    FontWeight::Black => PlatformFontWeight::BLACK,
  }
}

fn platform_font_style(request: &FontRequest<'_>) -> PlatformFontStyle {
  match requested_slant(request) {
    FontSlant::Italic => PlatformFontStyle::Italic,
    FontSlant::Oblique => PlatformFontStyle::Oblique(None),
    FontSlant::Upright => PlatformFontStyle::Normal,
  }
}

fn platform_font_width(request: &FontRequest<'_>) -> PlatformFontWidth {
  match request.stretch.unwrap_or_default() {
    FontStretch::UltraCondensed => PlatformFontWidth::ULTRA_CONDENSED,
    FontStretch::ExtraCondensed => PlatformFontWidth::EXTRA_CONDENSED,
    FontStretch::Condensed => PlatformFontWidth::CONDENSED,
    FontStretch::SemiCondensed => PlatformFontWidth::SEMI_CONDENSED,
    FontStretch::Normal => PlatformFontWidth::NORMAL,
    FontStretch::SemiExpanded => PlatformFontWidth::SEMI_EXPANDED,
    FontStretch::Expanded => PlatformFontWidth::EXPANDED,
    FontStretch::ExtraExpanded => PlatformFontWidth::EXTRA_EXPANDED,
    FontStretch::UltraExpanded => PlatformFontWidth::ULTRA_EXPANDED,
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct SystemFontQueryKey {
  family: FontDbQueryFamily,
  weight: fontdb::Weight,
  style: fontdb::Style,
}

type SystemFontQuerySlot = OnceLock<Option<fontdb::ID>>;

const SYSTEM_FONT_QUERY_CACHE_ENTRIES: usize = 4_096;

#[derive(Default)]
struct SystemFontQueryCache {
  slots: HashMap<SystemFontQueryKey, Arc<SystemFontQuerySlot>>,
  insertion_order: VecDeque<SystemFontQueryKey>,
}

impl SystemFontQueryCache {
  fn slot(&mut self, key: SystemFontQueryKey) -> Arc<SystemFontQuerySlot> {
    if let Some(slot) = self.slots.get(&key) {
      return slot.clone();
    }
    while self.slots.len() >= SYSTEM_FONT_QUERY_CACHE_ENTRIES {
      let Some(evicted) = self.insertion_order.pop_front() else {
        break;
      };
      self.slots.remove(&evicted);
    }
    let slot = Arc::new(SystemFontQuerySlot::new());
    self.insertion_order.push_back(key.clone());
    self.slots.insert(key, slot.clone());
    slot
  }
}

impl FontDbQueryFamily {
  fn as_fontdb_family(&self) -> fontdb::Family<'_> {
    match self {
      Self::Name(family) => fontdb::Family::Name(family),
      Self::SansSerif => fontdb::Family::SansSerif,
      Self::Serif => fontdb::Family::Serif,
    }
  }

  fn query(&self, database: &FontDatabase, request: &FontRequest<'_>) -> Option<fontdb::ID> {
    static CACHE: OnceLock<Mutex<SystemFontQueryCache>> = OnceLock::new();

    let key = SystemFontQueryKey {
      family: self.clone(),
      weight: fontdb_weight(request),
      style: fontdb_style(request),
    };
    let slot = CACHE
      .get_or_init(|| Mutex::new(SystemFontQueryCache::default()))
      .lock()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .slot(key);
    *slot.get_or_init(|| {
      let family = self.as_fontdb_family();
      let query = fontdb::Query {
        families: &[family],
        weight: fontdb_weight(request),
        style: fontdb_style(request),
        ..fontdb::Query::default()
      };
      match self {
        // A typographic family alias may also appear on a distinct Office face:
        // Aptos Display, for example, advertises Aptos as an alias. Resolve the
        // requested concrete/legacy typeface before accepting fontdb's broader
        // family match so an Aptos request cannot become Aptos Display merely
        // because that face was indexed first.
        Self::Name(typeface) => query_legacy_system_typeface(database, typeface, request)
          .or_else(|| database.query(&query)),
        Self::SansSerif | Self::Serif => database.query(&query),
      }
    })
  }
}

/// Resolves OpenType legacy family names (name ID 1) that contain a face
/// style, such as `Poppins Medium` and `Calibri Light`.
///
/// `fontdb` indexes the typographic family (name ID 16) when it exists, so a
/// CSS-like family query cannot find these Office typeface names. LibreOffice
/// handles the same distinction in
/// `PhysicalFontCollection::FindFontFaceByLegacyName`: first narrow by the
/// typographic-family prefix, then inspect the matching faces' legacy names.
fn query_legacy_system_typeface(
  database: &FontDatabase,
  requested_typeface: &str,
  request: &FontRequest<'_>,
) -> Option<fontdb::ID> {
  let requested = normalize_family(requested_typeface);
  let requested_weight = fontdb_weight(request).0;
  let requested_style = fontdb_style(request);

  database
    .faces()
    .filter(|info| {
      normalized_family_eq_normalized(&info.post_script_name, &requested)
        || info.families.iter().any(|(family, _)| {
          let family = normalize_family(family);
          !family.is_empty() && requested.starts_with(&family)
        })
    })
    .filter_map(|info| {
      database
        .with_face_data(info.id, |data, face_index| {
          let face =
            FontFaceInfo::from_ttf_bytes("system-typeface-probe", data, face_index).ok()?;
          (normalized_family_eq_normalized(&info.post_script_name, &requested)
            || family_matches_names(&face, std::slice::from_ref(&requested)))
          .then(|| {
            let primary_family_mismatch = face
              .family_names
              .first()
              .is_none_or(|family| !normalized_family_eq_normalized(family, &requested));
            (info, primary_family_mismatch)
          })
        })
        .flatten()
    })
    .min_by(
      |(left, left_primary_mismatch), (right, right_primary_mismatch)| {
        let left_rank = (
          left_primary_mismatch,
          left.style != requested_style,
          left.weight.0.abs_diff(requested_weight),
          left.post_script_name.as_str(),
        );
        let right_rank = (
          right_primary_mismatch,
          right.style != requested_style,
          right.weight.0.abs_diff(requested_weight),
          right.post_script_name.as_str(),
        );
        left_rank.cmp(&right_rank)
      },
    )
    .map(|(info, _)| info.id)
}

fn fontdb_weight(request: &FontRequest<'_>) -> fontdb::Weight {
  match requested_weight(request) {
    FontWeight::Thin => fontdb::Weight::THIN,
    FontWeight::ExtraLight => fontdb::Weight::EXTRA_LIGHT,
    FontWeight::Light => fontdb::Weight::LIGHT,
    FontWeight::Normal => fontdb::Weight::NORMAL,
    FontWeight::Medium => fontdb::Weight::MEDIUM,
    FontWeight::SemiBold => fontdb::Weight::SEMIBOLD,
    FontWeight::Bold => fontdb::Weight::BOLD,
    FontWeight::ExtraBold => fontdb::Weight::EXTRA_BOLD,
    FontWeight::Black => fontdb::Weight::BLACK,
  }
}

fn fontdb_style(request: &FontRequest<'_>) -> fontdb::Style {
  match requested_slant(request) {
    FontSlant::Italic => fontdb::Style::Italic,
    FontSlant::Oblique => fontdb::Style::Oblique,
    FontSlant::Upright => fontdb::Style::Normal,
  }
}

fn system_font_database() -> &'static FontDatabase {
  static DATABASE: OnceLock<FontDatabase> = OnceLock::new();
  DATABASE.get_or_init(|| {
    let mut database = FontDatabase::new();
    database.load_system_fonts();
    database
  })
}

struct CachedSystemQueryFont {
  face: FontFaceInfo<'static>,
}

const SYSTEM_FONT_DATA_CACHE_BYTES: usize = 64 * 1024 * 1024;

#[derive(Default)]
struct SystemFontDataCache {
  data: HashMap<fontdb::ID, FontBytes>,
  least_to_most_recent: VecDeque<fontdb::ID>,
  bytes: usize,
}

impl SystemFontDataCache {
  fn get(&mut self, id: fontdb::ID) -> Option<FontBytes> {
    let data = self.data.get(&id)?.clone();
    if let Some(index) = self
      .least_to_most_recent
      .iter()
      .position(|cached| *cached == id)
    {
      self.least_to_most_recent.remove(index);
    }
    self.least_to_most_recent.push_back(id);
    Some(data)
  }

  fn insert(&mut self, id: fontdb::ID, data: FontBytes) -> FontBytes {
    if let Some(cached) = self.get(id) {
      return cached;
    }
    if data.len() > SYSTEM_FONT_DATA_CACHE_BYTES {
      return data;
    }
    while self.bytes + data.len() > SYSTEM_FONT_DATA_CACHE_BYTES {
      let Some(evicted_id) = self.least_to_most_recent.pop_front() else {
        break;
      };
      if let Some(evicted) = self.data.remove(&evicted_id) {
        self.bytes -= evicted.len();
      }
    }
    self.bytes += data.len();
    self.least_to_most_recent.push_back(id);
    self.data.insert(id, data.clone());
    data
  }
}

fn cached_system_font_data(database: &FontDatabase, id: fontdb::ID) -> Option<FontBytes> {
  static CACHE: OnceLock<Mutex<SystemFontDataCache>> = OnceLock::new();
  let cache = CACHE.get_or_init(|| Mutex::new(SystemFontDataCache::default()));
  if let Some(data) = cache
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner)
    .get(id)
  {
    return Some(data);
  }

  let data = database.with_face_data(id, |data, _| FontBytes::from(Arc::<[u8]>::from(data)))?;
  Some(
    cache
      .lock()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .insert(id, data),
  )
}

fn cached_system_query_font(
  database: &FontDatabase,
  id: fontdb::ID,
  info: &fontdb::FaceInfo,
  font_id: &str,
) -> Option<Arc<CachedSystemQueryFont>> {
  type FontSlot = OnceLock<Option<Arc<CachedSystemQueryFont>>>;
  static CACHE: OnceLock<RwLock<HashMap<fontdb::ID, Arc<FontSlot>>>> = OnceLock::new();

  let cache = CACHE.get_or_init(|| RwLock::new(HashMap::new()));
  let cached = {
    cache
      .read()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .get(&id)
      .cloned()
  };
  let slot = cached.unwrap_or_else(|| {
    cache
      .write()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .entry(id)
      .or_insert_with(|| Arc::new(FontSlot::new()))
      .clone()
  });
  slot
    .get_or_init(|| {
      let face = database.with_face_data(id, |data, face_index| {
        debug_assert_eq!(face_index, info.index);
        let mut face = FontFaceInfo::from_ttf_bytes(font_id, data, face_index)
          .unwrap_or_else(|_| FontFaceInfo::from_fontdb_face_info(font_id, info));
        // fontdb exposes the family aliases produced by the platform font
        // matcher. Preserve them alongside the raw name-table families: some
        // Office faces (notably Calibri Light) use a WWS family alias that is
        // required to address the face by the name stored in the theme.
        for (family, _) in &info.families {
          push_unique_string(&mut face.family_names, family.clone());
        }
        face
      })?;
      Some(Arc::new(CachedSystemQueryFont { face }))
    })
    .clone()
}

fn runtime_face_for_data(data: FontBytes, face_index: u32) -> Option<Arc<RuntimeFace>> {
  RuntimeFace::new(data, face_index).ok().map(Arc::new)
}

fn font_supports_char(
  font: &ResolvedFontWithFace<'_, '_>,
  parsed_face: Option<&TtfFace<'_>>,
  ch: char,
) -> bool {
  if let Some(face) = parsed_face {
    return ttf_face_supports_char(face, ch);
  }
  font
    .face
    .is_some_and(|face| face.coverage.contains_char(ch))
}

fn ttf_face_supports_char(face: &TtfFace<'_>, ch: char) -> bool {
  face.glyph_index(ch).is_some_and(|glyph_id| glyph_id.0 != 0)
}

fn font_supports_text_cluster(
  font: &ResolvedFontWithFace<'_, '_>,
  parsed_face: Option<&TtfFace<'_>>,
  text: &str,
) -> bool {
  !text.chars().any(is_private_use_char)
    && text
      .chars()
      .all(|ch| font_supports_char(font, parsed_face, ch))
}

fn is_private_use_char(ch: char) -> bool {
  matches!(
    u32::from(ch),
    0xE000..=0xF8FF | 0xF0000..=0xFFFFD | 0x100000..=0x10FFFD
  )
}

fn grapheme_clusters(text: &str) -> Vec<Range<usize>> {
  let mut breaks = GraphemeClusterSegmenter::new()
    .segment_str(text)
    .collect::<Vec<_>>();
  if breaks.first().copied() != Some(0) {
    breaks.insert(0, 0);
  }
  if breaks.last().copied() != Some(text.len()) {
    breaks.push(text.len());
  }
  let mut clusters = Vec::new();
  let mut index = 0usize;
  while index + 1 < breaks.len() {
    let start = breaks[index];
    let mut end = breaks[index + 1];
    if text.get(start..end) == Some("\u{202f}")
      && index + 2 < breaks.len()
      && text[breaks[index + 1]..breaks[index + 2]]
        .chars()
        .next()
        .is_some_and(is_mongolian_char)
    {
      end = breaks[index + 2];
      index += 1;
    }
    clusters.push(start..end);
    index += 1;
  }
  clusters
}

fn is_mongolian_char(ch: char) -> bool {
  ch.script() == UnicodeScriptValue::Mongolian
}

pub fn trim_font_name_features(font_name: &str) -> &str {
  font_name
    .split_once(':')
    .map_or(font_name, |(name, _)| name)
}

pub fn parse_font_feature_settings<'a>(
  font_name: &str,
) -> (Vec<FeatureSetting<'a>>, Option<Cow<'a, str>>) {
  let Some((_, raw_features)) = font_name.split_once(':') else {
    return (Vec::new(), None);
  };

  let mut features = Vec::new();
  let mut language = None;
  for token in raw_features.split('&').filter(|token| !token.is_empty()) {
    if let Some(value) = token.strip_prefix("lang=") {
      language = Some(Cow::Owned(value.to_string()));
      continue;
    }
    if let Ok(feature) = HarfFeature::from_str(token) {
      let tag = feature.tag.to_be_bytes();
      features.push(FeatureSetting {
        tag: Cow::Owned(String::from_utf8_lossy(&tag).trim_end().to_string()),
        value: feature.value,
        start: feature.start,
        end: feature.end,
      });
    }
  }
  (features, language)
}

pub fn parse_font_variations<'a>(value: &str) -> Vec<VariationValue<'a>> {
  value
    .split(',')
    .filter_map(|token| parse_font_variation(token.trim()))
    .collect()
}

pub fn format_font_variations(variations: &[VariationValue<'_>]) -> String {
  variations
    .iter()
    .map(|variation| {
      format!(
        "\"{}\" {}",
        variation.tag,
        format_variation_value(variation.value)
      )
    })
    .collect::<Vec<_>>()
    .join(", ")
}

fn parse_font_variation<'a>(token: &str) -> Option<VariationValue<'a>> {
  let mut chars = token.char_indices();
  let (_, quote) = chars.next()?;
  if quote != '"' && quote != '\'' {
    return None;
  }
  let close = token[1..].find(quote)? + 1;
  let tag = &token[1..close];
  if tag.len() != 4
    || !tag
      .chars()
      .all(|ch| ch.is_ascii_alphanumeric() || ch == ' ')
  {
    return None;
  }
  let value = token[close + quote.len_utf8()..]
    .trim()
    .parse::<f32>()
    .ok()?;
  Some(VariationValue {
    tag: Cow::Owned(tag.to_string()),
    value,
  })
}

fn format_variation_value(value: f32) -> String {
  if value.fract().abs() <= f32::EPSILON {
    format!("{value:.0}")
  } else {
    value.to_string()
  }
}

fn resolve_family_alias<'book, 'request>(
  book: &FontBook<'book>,
  family: Cow<'request, str>,
) -> Cow<'request, str>
where
  'book: 'request,
{
  let normalized_family = normalize_family(family.as_ref());
  book
    .family_aliases
    .iter()
    .find(|alias| normalize_family(&alias.from) == normalized_family)
    .map(|alias| {
      let family: Cow<'request, str> = alias.to.clone();
      family
    })
    .unwrap_or(family)
}

fn find_substitution_rule<'a, 'b>(
  book: &'b FontBook<'a>,
  family: &str,
) -> Option<&'b FontSubstitutionRule<'a>> {
  let normalized_family = normalize_family(family);
  book
    .substitutions
    .iter()
    .find(|rule| normalize_family(&rule.requested_family) == normalized_family)
}

fn requested_weight(request: &FontRequest<'_>) -> FontWeight {
  request.weight.unwrap_or_else(|| {
    if request
      .family
      .as_deref()
      .is_some_and(|family| family.eq_ignore_ascii_case("Arial Black"))
    {
      FontWeight::Black
    } else if !request.bold
      && request
        .family
        .as_deref()
        .is_some_and(|family| family.eq_ignore_ascii_case("Calibri Light"))
    {
      FontWeight::Light
    } else if request.bold {
      FontWeight::Bold
    } else {
      FontWeight::Normal
    }
  })
}

fn requested_slant(request: &FontRequest<'_>) -> FontSlant {
  request.slant.unwrap_or(if request.italic {
    FontSlant::Italic
  } else {
    FontSlant::Upright
  })
}

fn weight_distance(left: FontWeight, right: FontWeight) -> i32 {
  (font_weight_number(left) - font_weight_number(right)).abs()
}

fn stretch_distance(left: FontStretch, right: FontStretch) -> i32 {
  (font_stretch_number(left) - font_stretch_number(right)).abs()
}

fn font_metrics_from_ttf(face: &TtfFace<'_>, em_size: f32) -> FontMetrics {
  let units_per_em = f32::from(face.units_per_em());
  let to_em = |value: i32| value as f32 / units_per_em * em_size;
  let ascent_units = face.ascender().max(0);
  let descent_units = (-face.descender()).max(0);
  let line_gap_units = face.line_gap().max(0);
  let fallback_gap_units =
    (i32::from(face.units_per_em()) - i32::from(ascent_units) - i32::from(descent_units)).max(0);
  let ascender = to_em(i32::from(ascent_units));
  let descender = to_em(i32::from(descent_units));
  // Windows Office lays out the baseline from OS/2 Windows metrics unless
  // the face explicitly opts into typographic metrics. Keep that baseline
  // separate from the natural line box: usWinAscent was designed as a
  // clipping extent and can be larger than the typographic ascender.
  let baseline_offset = face.tables().os2.map_or(ascender, |os2| {
    let units = if os2.use_typographic_metrics() {
      os2.typographic_ascender()
    } else {
      os2.windows_ascender()
    };
    let units = units.max(0);
    if units == 0 {
      ascender
    } else {
      to_em(i32::from(units))
    }
  });
  let line_gap = if line_gap_units > 0 {
    to_em(i32::from(line_gap_units))
  } else {
    to_em(fallback_gap_units)
  };
  let underline = face.underline_metrics();
  let strikeout = face.strikeout_metrics();
  FontMetrics {
    vertical: VerticalMetrics {
      ascent_pt: ascender,
      descent_pt: descender,
      baseline_offset_pt: baseline_offset,
      line_gap_pt: line_gap,
      ink_height_pt: ascender + descender,
      ..VerticalMetrics::default()
    },
    decoration: DecorationMetrics {
      underline_offset_pt: underline
        .map(|metrics| -to_em(i32::from(metrics.position)))
        .unwrap_or_default(),
      underline_thickness_pt: underline
        .map(|metrics| to_em(i32::from(metrics.thickness)).abs())
        .unwrap_or_default(),
      strikeout_offset_pt: strikeout
        .map(|metrics| to_em(i32::from(metrics.position)))
        .unwrap_or_default(),
      strikeout_thickness_pt: strikeout
        .map(|metrics| to_em(i32::from(metrics.thickness)).abs())
        .unwrap_or_default(),
    },
    script: ScriptMetrics {
      superscript_scale: 1.0,
      subscript_scale: 1.0,
      small_caps_scale: 1.0,
      ..ScriptMetrics::default()
    },
    em_size,
  }
}

fn font_coverage_from_ttf(face: &TtfFace<'_>) -> FontCoverage {
  let mut ranges = Vec::new();
  for subtable in face
    .tables()
    .cmap
    .into_iter()
    .flat_map(|table| table.subtables)
  {
    if subtable.is_unicode() {
      let mut subtable_ranges = Vec::new();
      let mut previous = None;
      subtable.codepoints(|codepoint| {
        debug_assert!(previous.is_none_or(|previous| previous <= codepoint));
        previous = Some(codepoint);
        if char::from_u32(codepoint).is_some_and(|ch| ttf_face_supports_char(face, ch)) {
          push_coverage_codepoint(&mut subtable_ranges, codepoint);
        }
      });
      merge_coverage_ranges(&mut ranges, subtable_ranges);
    }
  }
  FontCoverage {
    unicode_ranges: ranges,
    scripts: BTreeSet::new(),
  }
}

fn push_coverage_codepoint(ranges: &mut Vec<Range<u32>>, codepoint: u32) {
  if let Some(range) = ranges.last_mut()
    && codepoint <= range.end
  {
    range.end = range.end.max(codepoint.saturating_add(1));
    return;
  }
  ranges.push(codepoint..codepoint.saturating_add(1));
}

fn merge_coverage_ranges(ranges: &mut Vec<Range<u32>>, additions: Vec<Range<u32>>) {
  if additions.is_empty() {
    return;
  }
  if ranges.is_empty() {
    *ranges = additions;
    return;
  }

  let current = std::mem::take(ranges);
  let mut current = current.into_iter().peekable();
  let mut additions = additions.into_iter().peekable();
  ranges.reserve(current.len() + additions.len());
  while current.peek().is_some() || additions.peek().is_some() {
    let next = match (current.peek(), additions.peek()) {
      (Some(left), Some(right)) if left.start <= right.start => current.next(),
      (Some(_), Some(_)) => additions.next(),
      (Some(_), None) => current.next(),
      (None, Some(_)) => additions.next(),
      (None, None) => None,
    }
    .expect("a non-empty range iterator has a next item");
    if let Some(previous) = ranges.last_mut()
      && next.start <= previous.end
    {
      previous.end = previous.end.max(next.end);
    } else {
      ranges.push(next);
    }
  }
}

fn font_embedding_policy_from_ttf(face: &TtfFace<'_>) -> FontEmbeddingPolicy {
  match face.permissions() {
    Some(ttf_parser::Permissions::Restricted) => FontEmbeddingPolicy {
      subset_policy: FontSubsetPolicy::DoNotEmbed,
      installable: false,
      restricted: true,
    },
    Some(ttf_parser::Permissions::Installable) | None => FontEmbeddingPolicy::default(),
    Some(ttf_parser::Permissions::PreviewAndPrint) => FontEmbeddingPolicy {
      subset_policy: FontSubsetPolicy::Subset,
      installable: false,
      restricted: false,
    },
    Some(ttf_parser::Permissions::Editable) => FontEmbeddingPolicy {
      subset_policy: FontSubsetPolicy::EmbedFull,
      installable: false,
      restricted: false,
    },
  }
}

fn font_embedding_plan_from_ttf<'a>(face: &TtfFace<'_>) -> FontEmbeddingPlan<'a> {
  FontEmbeddingPlan {
    keep_tables: DEFAULT_PDF_EMBED_TABLES
      .iter()
      .map(|table| Cow::Borrowed(*table))
      .collect(),
    downgrade_cff2: face.tables().cff2.is_some(),
    desubroutinize_cff: face.tables().cff.is_some() || face.tables().cff2.is_some(),
    pin_variation_axes: !face.variation_axes().is_empty(),
  }
}

fn has_table(face: &TtfFace<'_>, tag: &[u8; 4]) -> bool {
  face.raw_face().table(Tag::from_bytes(tag)).is_some()
}

fn font_bounds_from_ttf(face: &TtfFace<'_>) -> FontBounds {
  let units_per_em = f32::from(face.units_per_em());
  FontBounds {
    global: Some(glyph_bounds_from_ttf_rect(
      face.global_bounding_box(),
      units_per_em,
    )),
  }
}

fn glyph_bounds_from_ttf_rect(bounds: TtfRect, units_per_em: f32) -> GlyphBounds {
  GlyphBounds {
    x_min_pt: f32::from(bounds.x_min) / units_per_em,
    y_min_pt: f32::from(bounds.y_min) / units_per_em,
    x_max_pt: f32::from(bounds.x_max) / units_per_em,
    y_max_pt: f32::from(bounds.y_max) / units_per_em,
  }
}

fn variation_axes_from_ttf<'a>(face: &TtfFace<'_>) -> Vec<VariationAxis<'a>> {
  face
    .variation_axes()
    .into_iter()
    .filter(|axis| !axis.hidden)
    .map(|axis| VariationAxis {
      tag: Cow::Owned(tag_to_string(axis.tag)),
      name: ttf_name_by_id(face, axis.name_id).map(Cow::Owned),
      min: axis.min_value,
      default: axis.def_value,
      max: axis.max_value,
    })
    .collect()
}

fn opentype_features_from_ttf<'a>(face: &TtfFace<'_>) -> Vec<OpenTypeFeature<'a>> {
  let mut features = Vec::new();
  if let Some(gsub) = face.tables().gsub {
    for feature in gsub.features {
      push_opentype_feature(&mut features, feature.tag);
    }
  }
  if let Some(gpos) = face.tables().gpos {
    for feature in gpos.features {
      push_opentype_feature(&mut features, feature.tag);
    }
  }
  if let Some(feat) = face.tables().feat {
    for feature in feat.names {
      push_named_feature(&mut features, format!("aat:{}", feature.feature));
    }
  }
  if face.tables().morx.is_some() {
    push_named_feature(&mut features, "aat:morx".to_string());
  }
  if has_table(face, b"Silf") {
    push_named_feature(&mut features, "graphite:Silf".to_string());
  }
  if has_table(face, b"Feat") {
    push_named_feature(&mut features, "graphite:Feat".to_string());
  }
  if has_table(face, b"Sill") {
    push_named_feature(&mut features, "graphite:Sill".to_string());
  }
  features
}

fn push_opentype_feature<'a>(features: &mut Vec<OpenTypeFeature<'a>>, tag: Tag) {
  let tag = tag_to_string(tag);
  if !features.iter().any(|feature| feature.tag.as_ref() == tag) {
    features.push(OpenTypeFeature {
      tag: Cow::Owned(tag),
      enabled_by_default: true,
    });
  }
}

fn push_named_feature<'a>(features: &mut Vec<OpenTypeFeature<'a>>, tag: String) {
  if !features.iter().any(|feature| feature.tag.as_ref() == tag) {
    features.push(OpenTypeFeature {
      tag: Cow::Owned(tag),
      enabled_by_default: true,
    });
  }
}

fn ttf_name_by_id(face: &TtfFace<'_>, name_id: u16) -> Option<String> {
  face
    .names()
    .into_iter()
    .filter(|name| name.name_id == name_id)
    .find_map(|name| name.to_string())
}

fn tag_to_string(tag: Tag) -> String {
  tag.to_string().trim_end().to_string()
}

fn first_strong_text_script(text: &str, wordprocessingml_font_slots: bool) -> Option<TextScript> {
  text.chars().find_map(|ch| {
    (!is_nonspacing_mark(ch))
      .then(|| strong_text_script(ch, wordprocessingml_font_slots))
      .flatten()
  })
}

fn strong_text_script(ch: char, wordprocessingml_font_slots: bool) -> Option<TextScript> {
  // ECMA-376 Part 1 §17.3.2.26 assigns Basic Latin text to the ASCII font
  // slot. Unicode Script marks digits and punctuation as Common; leaving
  // printable characters weak would incorrectly attach a trailing
  // chart/shape number to a preceding East Asian run instead of selecting the
  // Latin face. Layout controls and spaces remain weak so they stay with an
  // adjacent painted run and do not create empty PDF font subsets.
  if wordprocessingml_font_slots && matches!(ch as u32, 0x0021..=0x007E) {
    return Some(TextScript::Latin);
  }
  // ECMA-376 Part 1 §17.3.2.26 assigns the Latin-1 Supplement to the High
  // ANSI font slot unless w:hint=eastAsia activates one of its enumerated
  // exceptions. Treat its Common punctuation as Latin for the default case;
  // Unicode Script alone would incorrectly attach a trailing guillemet to a
  // preceding Han run.
  if wordprocessingml_font_slots && matches!(ch as u32, 0x00A0..=0x00FF) {
    return Some(TextScript::Latin);
  }
  // Unicode assigns Mathematical Alphanumeric Symbols to Common, but the
  // styled Greek letters retain their Greek semantic identity through their
  // compatibility decompositions. Office consequently falls them back to
  // Cambria Math rather than the application-script face.
  if matches!(u32::from(ch), 0x1D6A8..=0x1D7CB) {
    return Some(TextScript::Greek);
  }
  strong_text_script_from_unicode(ch.script())
}

fn strong_text_script_from_unicode(script: UnicodeScriptValue) -> Option<TextScript> {
  match script {
    UnicodeScriptValue::Latin => Some(TextScript::Latin),
    UnicodeScriptValue::Cyrillic => Some(TextScript::Cyrillic),
    UnicodeScriptValue::Greek => Some(TextScript::Greek),
    UnicodeScriptValue::Han => Some(TextScript::Han),
    UnicodeScriptValue::Hiragana => Some(TextScript::Hiragana),
    UnicodeScriptValue::Katakana => Some(TextScript::Katakana),
    UnicodeScriptValue::Hangul => Some(TextScript::Hangul),
    UnicodeScriptValue::Arabic => Some(TextScript::Arabic),
    UnicodeScriptValue::Hebrew => Some(TextScript::Hebrew),
    UnicodeScriptValue::Devanagari => Some(TextScript::Devanagari),
    UnicodeScriptValue::Thai => Some(TextScript::Thai),
    UnicodeScriptValue::Common | UnicodeScriptValue::Inherited => None,
    _ => Some(TextScript::Other),
  }
}

fn text_direction_from_bidi(direction: BidiDirection) -> TextDirection {
  match direction {
    BidiDirection::Ltr => TextDirection::LeftToRight,
    BidiDirection::Rtl => TextDirection::RightToLeft,
    BidiDirection::Mixed => TextDirection::Mixed,
  }
}

fn is_nonspacing_mark(ch: char) -> bool {
  matches!(
    u32::from(ch),
    0x0300..=0x036F
      | 0x0591..=0x05BD
      | 0x05BF
      | 0x05C1..=0x05C2
      | 0x05C4..=0x05C5
      | 0x05C7
      | 0x0610..=0x061A
      | 0x064B..=0x065F
      | 0x0670
      | 0x06D6..=0x06DC
      | 0x06DF..=0x06E4
      | 0x06E7..=0x06E8
      | 0x06EA..=0x06ED
      | 0x0711
      | 0x0730..=0x074A
      | 0x07A6..=0x07B0
      | 0x0816..=0x0819
      | 0x081B..=0x0823
      | 0x0825..=0x0827
      | 0x0829..=0x082D
      | 0x0859..=0x085B
      | 0x08D3..=0x08E1
      | 0x08E3..=0x0902
      | 0x093A
      | 0x093C
      | 0x0941..=0x0948
      | 0x094D
      | 0x0951..=0x0957
      | 0x0962..=0x0963
  )
}

fn is_justifiable_char(ch: char) -> bool {
  let justification = glyph_justification(ch);
  justification.space || justification.cjk || justification.cjk_punctuation || justification.kashida
}

fn glyph_justification(ch: char) -> GlyphJustification {
  let script = ch.script();
  GlyphJustification {
    space: ch.is_whitespace(),
    cjk: matches!(
      script,
      UnicodeScriptValue::Han
        | UnicodeScriptValue::Hiragana
        | UnicodeScriptValue::Katakana
        | UnicodeScriptValue::Hangul
    ),
    cjk_punctuation: is_cjk_punctuation(ch),
    kashida: script == UnicodeScriptValue::Arabic || ch == '\u{0640}',
  }
}

fn is_cjk_punctuation(ch: char) -> bool {
  matches!(
    u32::from(ch),
    0x3000..=0x303F | 0xFE10..=0xFE1F | 0xFE30..=0xFE4F | 0xFF00..=0xFFEF
  )
}

fn glyph_text_range(text: &str, infos: &[harfrust::GlyphInfo], index: usize) -> Range<usize> {
  let start = infos[index].cluster as usize;
  let end = infos
    .iter()
    .map(|info| info.cluster as usize)
    .filter(|cluster| *cluster > start)
    .min()
    .unwrap_or(text.len());
  start.min(text.len())..end.min(text.len())
}

#[derive(Clone, Debug)]
struct SourceTextRange {
  shaped: Range<usize>,
  source: Range<usize>,
}

fn small_caps_shaped_text(text: &str) -> (Cow<'_, str>, Vec<SourceTextRange>) {
  let mut shaped = String::with_capacity(text.len());
  let mut ranges = Vec::with_capacity(text.chars().count());
  for (source_start, ch) in text.char_indices() {
    let source_end = source_start + ch.len_utf8();
    let shaped_start = shaped.len();
    shaped.extend(ch.to_uppercase());
    let shaped_end = shaped.len();
    ranges.push(SourceTextRange {
      shaped: shaped_start..shaped_end,
      source: source_start..source_end,
    });
  }
  (Cow::Owned(shaped), ranges)
}

fn source_range_for_shaped_range(
  ranges: &[SourceTextRange],
  shaped: Range<usize>,
  source_len: usize,
) -> Range<usize> {
  let start_index = ranges.partition_point(|entry| entry.shaped.end <= shaped.start);
  let end_index = ranges.partition_point(|entry| entry.shaped.start < shaped.end);
  if start_index >= end_index || end_index == 0 {
    return shaped.start.min(source_len)..shaped.end.min(source_len);
  }
  let source_start = ranges[start_index].source.start;
  let source_end = ranges[end_index - 1].source.end;
  source_start.min(source_len)..source_end.min(source_len)
}

fn missing_glyphs_from_shaped_glyphs(glyphs: &[ShapedGlyph]) -> Vec<MissingGlyph> {
  glyphs
    .iter()
    .filter(|glyph| glyph.glyph_id == 0)
    .map(|glyph| MissingGlyph {
      codepoint: glyph.source_char.map(u32::from).unwrap_or_default(),
      text_range: glyph.text_range.clone(),
    })
    .collect()
}

fn harf_features(features: &[FeatureValue<'_>]) -> Vec<HarfFeature> {
  features
    .iter()
    .filter_map(|feature| {
      let tag = feature.tag.as_ref().as_bytes();
      (tag.len() == 4).then(|| {
        HarfFeature::new(
          HarfTag::new(&[tag[0], tag[1], tag[2], tag[3]]),
          feature.value,
          ..,
        )
      })
    })
    .collect()
}

fn push_unicode_range(ranges: &mut Vec<Range<u32>>, codepoint: u32) {
  if let Some(range) = ranges.iter_mut().find(|range| {
    range.start <= codepoint.saturating_add(1) && codepoint <= range.end.saturating_add(1)
  }) {
    range.start = range.start.min(codepoint);
    range.end = range.end.max(codepoint + 1);
    return;
  }
  ranges.push(codepoint..codepoint + 1);
  ranges.sort_by_key(|range| range.start);
}

fn font_weight_number(weight: FontWeight) -> i32 {
  match weight {
    FontWeight::Thin => 100,
    FontWeight::ExtraLight => 200,
    FontWeight::Light => 300,
    FontWeight::Normal => 400,
    FontWeight::Medium => 500,
    FontWeight::SemiBold => 600,
    FontWeight::Bold => 700,
    FontWeight::ExtraBold => 800,
    FontWeight::Black => 900,
  }
}

fn approximate_glyphs(text: &str, _size: FontSize) -> Vec<ShapedGlyph> {
  text
    .char_indices()
    .map(|(start, ch)| {
      let end = start + ch.len_utf8();
      ShapedGlyph {
        glyph_id: 0,
        cluster: start as u32,
        text_range: start..end,
        x_advance_pt: 0.0,
        y_advance_pt: 0.0,
        x_offset_pt: 0.0,
        y_offset_pt: 0.0,
        safe_to_break: ch.is_whitespace(),
        source_char: Some(ch),
        justifiable: is_justifiable_char(ch),
        justification: glyph_justification(ch),
        bounds: None,
      }
    })
    .collect()
}

fn text_safe_breaks(text: &str) -> Vec<usize> {
  text
    .char_indices()
    .filter_map(|(index, ch)| ch.is_whitespace().then_some(index + ch.len_utf8()))
    .collect()
}

fn font_stretch_number(stretch: FontStretch) -> i32 {
  match stretch {
    FontStretch::UltraCondensed => 1,
    FontStretch::ExtraCondensed => 2,
    FontStretch::Condensed => 3,
    FontStretch::SemiCondensed => 4,
    FontStretch::Normal => 5,
    FontStretch::SemiExpanded => 6,
    FontStretch::Expanded => 7,
    FontStretch::ExtraExpanded => 8,
    FontStretch::UltraExpanded => 9,
  }
}

fn font_weight_from_ttf(weight: u16) -> FontWeight {
  match weight {
    0..=149 => FontWeight::Thin,
    150..=249 => FontWeight::ExtraLight,
    250..=349 => FontWeight::Light,
    350..=449 => FontWeight::Normal,
    450..=549 => FontWeight::Medium,
    550..=649 => FontWeight::SemiBold,
    650..=749 => FontWeight::Bold,
    750..=849 => FontWeight::ExtraBold,
    _ => FontWeight::Black,
  }
}

fn font_slant_from_ttf(style: ttf_parser::Style) -> FontSlant {
  match style {
    ttf_parser::Style::Italic => FontSlant::Italic,
    ttf_parser::Style::Oblique => FontSlant::Oblique,
    ttf_parser::Style::Normal => FontSlant::Upright,
  }
}

fn font_stretch_from_ttf(width: u16) -> FontStretch {
  match width {
    1 => FontStretch::UltraCondensed,
    2 => FontStretch::ExtraCondensed,
    3 => FontStretch::Condensed,
    4 => FontStretch::SemiCondensed,
    5 => FontStretch::Normal,
    6 => FontStretch::SemiExpanded,
    7 => FontStretch::Expanded,
    8 => FontStretch::ExtraExpanded,
    _ => FontStretch::UltraExpanded,
  }
}

fn font_stretch_from_fontdb(stretch: fontdb::Stretch) -> FontStretch {
  font_stretch_from_ttf(stretch.to_number())
}

fn harf_direction(direction: TextDirection) -> Option<HarfDirection> {
  match direction {
    TextDirection::LeftToRight => Some(HarfDirection::LeftToRight),
    TextDirection::RightToLeft => Some(HarfDirection::RightToLeft),
    TextDirection::TopToBottom => Some(HarfDirection::TopToBottom),
    TextDirection::BottomToTop => Some(HarfDirection::BottomToTop),
    TextDirection::Mixed => None,
  }
}

fn harf_script(script: TextScript) -> Option<HarfScript> {
  match script {
    TextScript::Common => Some(script::COMMON),
    TextScript::Latin => Some(script::LATIN),
    TextScript::Cyrillic => Some(script::CYRILLIC),
    TextScript::Greek => Some(script::GREEK),
    TextScript::Han => Some(script::HAN),
    TextScript::Hiragana => Some(script::HIRAGANA),
    TextScript::Katakana => Some(script::KATAKANA),
    TextScript::Hangul => Some(script::HANGUL),
    TextScript::Arabic => Some(script::ARABIC),
    TextScript::Hebrew => Some(script::HEBREW),
    TextScript::Devanagari => Some(script::DEVANAGARI),
    TextScript::Thai => Some(script::THAI),
    TextScript::Other => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_office_policy_maps_localized_dengxian_family() {
    let mut registry = FontRegistry::with_default_policy();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("dengxian", "DengXian"),
    );

    let request = FontRequest {
      family: Some(Cow::Borrowed("等线")),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("dengxian")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("DengXian"));
  }

  #[test]
  fn unspecified_family_registers_a_shapeable_database_fallback() {
    let mut registry = FontRegistry::with_default_policy();
    let request = FontRequest::default();

    let registered = registry.register_system_query_fonts(&request).unwrap();
    let resolved = registry.resolve(&request).unwrap();

    assert!(registered > 0);
    assert!(registry.sources.iter().all(|source| {
      !source
        .id()
        .is_some_and(|id| id.starts_with("fallback-path:"))
    }));
    assert!(registry.font_face_binary(&resolved.font_id).is_some());
  }

  #[test]
  fn system_font_query_cache_stays_bounded() {
    let mut cache = SystemFontQueryCache::default();
    for index in 0..=SYSTEM_FONT_QUERY_CACHE_ENTRIES {
      cache.slot(SystemFontQueryKey {
        family: FontDbQueryFamily::Name(format!("Fixture {index}")),
        weight: fontdb::Weight::NORMAL,
        style: fontdb::Style::Normal,
      });
    }

    assert_eq!(cache.slots.len(), SYSTEM_FONT_QUERY_CACHE_ENTRIES);
    assert!(!cache.slots.contains_key(&SystemFontQueryKey {
      family: FontDbQueryFamily::Name("Fixture 0".to_string()),
      weight: fontdb::Weight::NORMAL,
      style: fontdb::Style::Normal,
    }));
  }

  #[test]
  fn resolves_exact_family_and_records_candidates() {
    let mut registry = FontRegistry::new();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("regular", "Example"),
    );
    let mut bold = FontFaceInfo::synthetic("bold", "Example");
    bold.weight = FontWeight::Bold;
    registry.register_face(FontSource::System, bold);

    let request = FontRequest {
      family: Some(Cow::Borrowed("Example")),
      bold: true,
      ..FontRequest::default()
    };
    let resolved = registry.resolve_with_diagnostics(&request).unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("bold")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("Example"));
    assert!(!resolved.synthetic_bold);
    assert_eq!(resolved.match_diagnostics.candidates.len(), 2);
  }

  #[test]
  fn applies_alias_before_matching() {
    let mut registry = FontRegistry::new();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("liberation", "Liberation Serif"),
    );
    registry.book.family_aliases.push(FontFamilyAlias {
      from: Cow::Borrowed("Times New Roman"),
      to: Cow::Borrowed("Liberation Serif"),
    });

    let request = FontRequest {
      family: Some(Cow::Borrowed("Times New Roman")),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("liberation")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("Liberation Serif"));
  }

  #[test]
  fn default_office_policy_maps_legacy_arial_cyr_family() {
    let mut registry = FontRegistry::with_default_policy();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("arial", "Arial"),
    );

    let request = FontRequest {
      family: Some(Cow::Borrowed("Arial Cyr")),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("arial")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("Arial"));
  }

  #[test]
  fn default_office_policy_maps_unavailable_frutiger_spreadsheet_face() {
    let mut registry = FontRegistry::with_default_policy();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("arial", "Arial"),
    );

    let request = FontRequest {
      family: Some(Cow::Borrowed("Frutiger 45 Light")),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("arial")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("Arial"));
  }

  #[test]
  fn default_office_policy_maps_unavailable_helvetica_neue_spreadsheet_face() {
    let mut registry = FontRegistry::with_default_policy();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("arial", "Arial"),
    );

    let request = FontRequest {
      family: Some(Cow::Borrowed("HelveticaNeue LightExt")),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("arial")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("Arial"));
  }

  #[test]
  fn default_office_policy_maps_unavailable_legacy_office_families() {
    let mut registry = FontRegistry::with_default_policy();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("arial", "Arial"),
    );
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("calibri", "Calibri"),
    );

    for family in [
      "Arial Unicode MS",
      "Helvetica Neue",
      "Helvetica Neue Light",
      "Helvetica Neue Medium",
    ] {
      let resolved = registry
        .resolve(&FontRequest {
          family: Some(Cow::Borrowed(family)),
          ..FontRequest::default()
        })
        .unwrap();
      assert_eq!(resolved.resolved_family, Cow::Borrowed("Arial"));
    }

    let graphik = registry
      .resolve(&FontRequest {
        family: Some(Cow::Borrowed("Graphik")),
        ..FontRequest::default()
      })
      .unwrap();
    assert_eq!(graphik.resolved_family, Cow::Borrowed("Calibri"));
  }

  #[test]
  fn family_matching_does_not_cross_match_shared_tokens() {
    let mut registry = FontRegistry::new();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("sans", "Liberation Sans"),
    );
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("serif", "Liberation Serif"),
    );

    let request = FontRequest {
      family: Some(Cow::Borrowed("Liberation Serif")),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("serif")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("Liberation Serif"));
  }

  #[test]
  fn system_query_prefers_installed_calibri_light_face() {
    if !system_font_database()
      .faces()
      .any(|face| face.post_script_name.contains("Calibri-Light"))
    {
      return;
    }
    let mut registry = FontRegistry::with_default_policy();
    let request = FontRequest {
      family: Some(Cow::Borrowed("Calibri Light")),
      ..FontRequest::default()
    };

    registry.register_system_query_fonts(&request).unwrap();
    let resolved = registry.resolve_with_diagnostics(&request).unwrap();
    let families = registry
      .book
      .faces
      .iter()
      .map(|face| {
        (
          face.font_id.0.to_string(),
          face
            .family_names
            .iter()
            .map(|family| family.to_string())
            .collect::<Vec<_>>(),
          face.weight,
        )
      })
      .collect::<Vec<_>>();

    assert!(
      resolved.font_id.0.contains("Calibri-Light"),
      "resolved={}; faces={families:?}",
      resolved.font_id.0
    );
  }

  #[test]
  fn system_query_prefers_installed_aptos_face_over_display_alias() {
    if !system_font_database()
      .faces()
      .any(|face| face.post_script_name == "Aptos")
    {
      return;
    }
    let mut registry = FontRegistry::with_default_policy();
    let request = FontRequest {
      family: Some(Cow::Borrowed("Aptos")),
      ..FontRequest::default()
    };

    registry.register_system_query_fonts(&request).unwrap();
    let resolved = registry.resolve_with_diagnostics(&request).unwrap();

    assert!(
      resolved.font_id.0.contains("system-query:Aptos:"),
      "resolved={}",
      resolved.font_id.0
    );
  }

  #[test]
  fn system_query_prefers_installed_noto_sans_face_over_condensed_alias() {
    if !system_font_database()
      .faces()
      .any(|face| face.post_script_name == "NotoSans-Regular")
    {
      return;
    }
    let mut registry = FontRegistry::with_default_policy();
    let request = FontRequest {
      family: Some(Cow::Borrowed("Noto Sans")),
      ..FontRequest::default()
    };

    registry.register_system_query_fonts(&request).unwrap();
    let resolved = registry.resolve_with_diagnostics(&request).unwrap();

    assert!(
      resolved
        .font_id
        .0
        .contains("system-query:NotoSans-Regular:"),
      "resolved={}",
      resolved.font_id.0
    );
  }

  #[test]
  fn resolved_font_scales_face_metrics() {
    let mut registry = FontRegistry::new();
    let mut face = FontFaceInfo::synthetic("example", "Example");
    face.metrics = FontMetrics {
      vertical: VerticalMetrics {
        ascent_pt: 1.0,
        descent_pt: 0.25,
        baseline_offset_pt: 1.125,
        ..VerticalMetrics::default()
      },
      em_size: 1.0,
      ..FontMetrics::default()
    };
    registry.register_face(FontSource::System, face);

    let request = FontRequest {
      family: Some(Cow::Borrowed("Example")),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();
    let metrics = resolved.metrics_at_size(FontSize(12.0));

    assert_eq!(metrics.vertical.ascent_pt, 12.0);
    assert_eq!(metrics.vertical.descent_pt, 3.0);
    assert_eq!(metrics.vertical.baseline_offset_pt, 13.5);
  }

  #[test]
  fn approximate_shaping_preserves_text_ranges_without_fake_advances() {
    let resolved = ResolvedFont {
      font_id: FontId(Arc::from("example")),
      resolved_family: Cow::Borrowed("Example"),
      source: FontSource::System,
      face_index: 0,
      synthetic_bold: false,
      synthetic_italic: false,
      metrics: FontMetrics::default(),
      match_diagnostics: FontMatchDiagnostics::default(),
    };

    let shaped = resolved.shape_approximate(
      "A B",
      FontSize(12.0),
      TextDirection::LeftToRight,
      Some(TextScript::Latin),
    );

    assert!(shaped.approximate);
    assert_eq!(shaped.glyphs.len(), 3);
    assert_eq!(shaped.glyphs[0].text_range, 0..1);
    assert_eq!(shaped.glyphs[0].x_advance_pt, 0.0);
    assert_eq!(shaped.safe_breaks, vec![2]);
  }

  #[test]
  fn small_caps_range_mapping_preserves_original_text_ranges() {
    let source = "ßa";
    let (shaped, ranges) = small_caps_shaped_text(source);

    assert_eq!(shaped.as_ref(), "SSA");
    assert_eq!(
      source_range_for_shaped_range(&ranges, 0..1, source.len()),
      0..2
    );
    assert_eq!(
      source_range_for_shaped_range(&ranges, 1..2, source.len()),
      0..2
    );
    assert_eq!(
      source_range_for_shaped_range(&ranges, 2..3, source.len()),
      2..3
    );
  }

  #[test]
  fn font_coverage_tracks_non_bmp_codepoints() {
    let coverage = FontCoverage {
      unicode_ranges: vec![
        u32::from('A')..u32::from('B'),
        u32::from('😀')..u32::from('😀') + 1,
      ],
      scripts: BTreeSet::new(),
    };

    assert!(coverage.contains_char('A'));
    assert!(coverage.contains_char('😀'));
    assert!(!coverage.contains_char('B'));
    assert_eq!(
      coverage.missing_glyphs("A😀B"),
      vec![MissingGlyph {
        codepoint: u32::from('B'),
        text_range: 5..6,
      }]
    );
  }

  #[test]
  fn coverage_range_merge_coalesces_overlapping_unicode_subtables() {
    let mut ranges = vec![1..4, 10..12, 20..21];
    merge_coverage_ranges(&mut ranges, vec![3..6, 8..11, 21..23]);

    assert_eq!(ranges, vec![1..6, 8..12, 20..23]);
    let coverage = FontCoverage {
      unicode_ranges: ranges,
      scripts: BTreeSet::new(),
    };
    assert!(coverage.contains_codepoint(1));
    assert!(coverage.contains_codepoint(11));
    assert!(coverage.contains_codepoint(22));
    assert!(!coverage.contains_codepoint(6));
    assert!(!coverage.contains_codepoint(19));
  }

  #[test]
  fn font_source_exposes_registered_bytes_for_renderers() {
    let source = FontSource::EmbeddedOoxml {
      id: Cow::Borrowed("embedded"),
      data: [1, 2, 3].as_slice().into(),
    };

    assert_eq!(source.id(), Some("embedded"));
    assert_eq!(source.data(), Some([1, 2, 3].as_slice()));
    assert_eq!(FontSource::System.id(), None);
    assert_eq!(FontSource::System.data(), None);
  }

  #[test]
  fn registry_exposes_face_data_for_registered_memory_fonts() {
    let mut registry = FontRegistry::new();
    registry.register_face(
      FontSource::Memory {
        id: Cow::Borrowed("memory"),
        data: [1, 2, 3].as_slice().into(),
      },
      FontFaceInfo::synthetic("memory", "Memory"),
    );

    let data = registry
      .font_face_data(&FontId(Arc::from("memory")))
      .expect("registered memory font data");

    assert_eq!(data.face_index, 0);
    assert_eq!(data.data.as_deref(), Some([1, 2, 3].as_slice()));
    assert_eq!(data.family_names, vec![Cow::Borrowed("Memory")]);
  }

  #[test]
  fn font_usage_collector_records_shaped_runs_for_embedding() {
    let run = ShapedRun {
      font_id: FontId(Arc::from("example")),
      font_size_pt: FontSize(12.0),
      text: "AB",
      text_range: 0..2,
      glyphs: Cow::Owned(vec![
        ShapedGlyph {
          glyph_id: 7,
          text_range: 0..1,
          source_char: Some('A'),
          ..ShapedGlyph::default()
        },
        ShapedGlyph {
          glyph_id: 9,
          text_range: 1..2,
          source_char: Some('B'),
          ..ShapedGlyph::default()
        },
      ]),
      advance_pt: 0.0,
      direction: TextDirection::LeftToRight,
      script: Some(TextScript::Latin),
      safe_breaks: Vec::new(),
      approximate: false,
      decorations: Vec::new(),
      diagnostics: ShapingDiagnostics::default(),
    };
    let mut collector = FontUsageCollector::default();

    collector.record_run(&run);

    assert_eq!(collector.usages.len(), 1);
    assert!(collector.usages[0].needs_embedding);
    assert!(collector.usages[0].glyph_ids.contains(&7));
    assert!(collector.usages[0].glyph_ids.contains(&9));
    assert_eq!(collector.usages[0].unicode_ranges, vec![65..67]);
  }

  #[test]
  fn theme_font_map_resolves_requested_kind() {
    let map = ThemeFontMap {
      major_latin: Some(Cow::Borrowed("Major Latin")),
      minor_east_asian: Some(Cow::Borrowed("Minor EA")),
      ..ThemeFontMap::default()
    };

    assert_eq!(
      map.resolve(ThemeFontKind::MajorLatin),
      Some(Cow::Borrowed("Major Latin"))
    );
    assert_eq!(
      map.resolve(ThemeFontKind::MinorEastAsian),
      Some(Cow::Borrowed("Minor EA"))
    );
    assert_eq!(map.resolve(ThemeFontKind::MajorComplexScript), None);
  }

  #[test]
  fn shape_text_runs_uses_registered_fallback_coverage() {
    let mut registry = FontRegistry::new();
    let mut primary = FontFaceInfo::synthetic("primary", "Primary");
    primary.coverage.unicode_ranges = std::iter::once(u32::from('A')..u32::from('A') + 1).collect();
    registry.register_face(FontSource::System, primary);

    let mut fallback = FontFaceInfo::synthetic("fallback", "Fallback");
    fallback.coverage.unicode_ranges =
      std::iter::once(u32::from('B')..u32::from('B') + 1).collect();
    registry.register_face(FontSource::System, fallback);
    registry.book.fallback_chains.push(FontFallbackChain {
      requested_family: None,
      script: Some(TextScript::Latin),
      language: None,
      families: vec![Cow::Borrowed("Fallback")],
    });

    let request = FontRequest {
      family: Some(Cow::Borrowed("Primary")),
      script: Some(TextScript::Latin),
      size_pt: FontSize(12.0),
      ..FontRequest::default()
    };
    let runs = registry
      .shape_text_runs(&request, "AB", TextDirection::LeftToRight)
      .unwrap();

    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].font_id, FontId(Arc::from("primary")));
    assert_eq!(runs[0].text_range, 0..1);
    assert_eq!(runs[1].font_id, FontId(Arc::from("fallback")));
    assert_eq!(runs[1].text_range, 1..2);
    assert_eq!(
      runs[1].diagnostics.fallback_runs[0].reason,
      FontSubstitutionReason::MissingGlyph
    );

    let chain = registry.resolve_font_chain(&request).unwrap();
    let cached_runs = registry
      .shape_text_runs_with_font_chain(
        &chain,
        "AB",
        &ShapeOptions {
          scan_registered_fallbacks: false,
          ..ShapeOptions::from_request(&request, TextDirection::LeftToRight)
        },
      )
      .unwrap();
    assert_eq!(cached_runs, runs);
  }

  #[test]
  fn office_greek_fallback_prefers_cambria_math_before_generic_faces() {
    let registry = FontRegistry::with_default_policy();
    let request = FontRequest {
      family: Some(Cow::Borrowed("Calibri")),
      script: Some(TextScript::Greek),
      ..FontRequest::default()
    };

    let families = registry.fallback_families(&request);
    let math = families
      .iter()
      .position(|family| *family == "Cambria Math")
      .expect("Cambria Math fallback");
    let generic = families
      .iter()
      .position(|family| *family == "DejaVu Sans")
      .expect("generic fallback");
    assert!(math < generic);
  }

  #[test]
  fn office_hangul_fallback_prefers_korean_faces_before_generic_faces() {
    let registry = FontRegistry::with_default_policy();
    let request = FontRequest {
      family: Some(Cow::Borrowed("Calibri")),
      script: Some(TextScript::Hangul),
      ..FontRequest::default()
    };

    let families = registry.fallback_families(&request);
    let malgun = families
      .iter()
      .position(|family| *family == "Malgun Gothic")
      .expect("Office Korean fallback");
    let noto_kr = families
      .iter()
      .position(|family| *family == "Noto Sans CJK KR")
      .expect("open Korean fallback");
    let generic = families
      .iter()
      .position(|family| *family == "DejaVu Sans")
      .expect("generic fallback");
    assert!(malgun < noto_kr);
    assert!(noto_kr < generic);
  }

  #[test]
  fn office_historic_script_fallback_precedes_generic_faces() {
    let registry = FontRegistry::with_default_policy();
    let request = FontRequest {
      family: Some(Cow::Borrowed("Arial")),
      script: Some(TextScript::Other),
      ..FontRequest::default()
    };

    let families = registry.fallback_families(&request);
    let historic = families
      .iter()
      .position(|family| *family == "Segoe UI Historic")
      .expect("Office historic-script fallback");
    let noto_gothic = families
      .iter()
      .position(|family| *family == "Noto Sans Gothic")
      .expect("open Gothic fallback");
    let generic = families
      .iter()
      .position(|family| *family == "DejaVu Sans")
      .expect("generic fallback");
    assert!(historic < noto_gothic);
    assert!(noto_gothic < generic);
  }

  #[test]
  fn office_common_symbol_fallback_precedes_generic_sans() {
    let registry = FontRegistry::with_default_policy();
    let request = FontRequest {
      family: Some(Cow::Borrowed("Liberation Serif")),
      script: Some(TextScript::Common),
      ..FontRequest::default()
    };

    let families = registry.fallback_families(&request);
    let symbol = families
      .iter()
      .position(|family| *family == "Segoe UI Symbol")
      .expect("Office symbol fallback");
    let generic = families
      .iter()
      .position(|family| *family == "DejaVu Sans")
      .expect("generic fallback");
    let cjk = families
      .iter()
      .position(|family| *family == "Noto Sans CJK JP")
      .expect("pan-CJK fallback");
    assert!(symbol < generic);
    assert!(generic < cjk);
  }

  #[test]
  fn mathematical_greek_compatibility_letters_keep_greek_script() {
    let runs = script_direction_runs("𝝊𝝋", FontSize(11.0), false);

    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].script, TextScript::Greek);
  }

  #[test]
  fn wordprocessingml_latin1_punctuation_uses_high_ansi_slot() {
    let runs = script_direction_runs_with_options(
      "Junzha«问候语»",
      FontSize(11.0),
      ScriptScanOptions {
        wordprocessingml_font_slots: true,
        ..ScriptScanOptions::default()
      },
    );

    assert_eq!(runs.len(), 3);
    assert_eq!(runs[0].script, TextScript::Latin);
    assert_eq!(&"Junzha«问候语»"[runs[0].text_range.clone()], "Junzha«");
    assert_eq!(runs[1].script, TextScript::Han);
    assert_eq!(&"Junzha«问候语»"[runs[1].text_range.clone()], "问候语");
    assert_eq!(runs[2].script, TextScript::Latin);
    assert_eq!(&"Junzha«问候语»"[runs[2].text_range.clone()], "»");
  }

  #[test]
  fn office_font_slots_keep_ascii_digits_in_the_latin_run() {
    let runs = script_direction_runs_with_options(
      "系列1",
      FontSize(11.0),
      ScriptScanOptions {
        wordprocessingml_font_slots: true,
        ..ScriptScanOptions::default()
      },
    );

    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].script, TextScript::Han);
    assert_eq!(&"系列1"[runs[0].text_range.clone()], "系列");
    assert_eq!(runs[1].script, TextScript::Latin);
    assert_eq!(&"系列1"[runs[1].text_range.clone()], "1");
  }

  #[test]
  fn maps_ooxml_text_context_to_harfrust_context() {
    assert_eq!(
      harf_direction(TextDirection::RightToLeft),
      Some(HarfDirection::RightToLeft)
    );
    assert_eq!(harf_script(TextScript::Arabic), Some(script::ARABIC));
    assert_eq!(harf_script(TextScript::Other), None);
  }
}
