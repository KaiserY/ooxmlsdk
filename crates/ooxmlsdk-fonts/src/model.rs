use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fmt;
use std::fs;
use std::ops::{Deref, Range};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::Instant;

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
use skrifa::{
  FontRef as SkrifaFontRef, GlyphId as SkrifaGlyphId, MetadataProvider,
  attribute::Style as SkrifaStyle,
  instance::{LocationRef as SkrifaLocationRef, Size as SkrifaSize},
  metrics::BoundingBox as SkrifaBoundingBox,
  raw::TableProvider,
  string::StringId,
};
use smallvec::SmallVec;
use unicode_bidi::{Direction as BidiDirection, get_base_direction};
use unicode_script::{Script as UnicodeScriptValue, UnicodeScript};
use yoke::{Yoke, Yokeable};

use crate::{FontError, Result};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct FontId(pub Arc<str>);

#[derive(Clone)]
pub struct FontBytes(Arc<dyn AsRef<[u8]> + Send + Sync>);

impl FontBytes {
  pub fn as_slice(&self) -> &[u8] {
    self.0.as_ref().as_ref()
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

impl fmt::Debug for FontBytes {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("FontBytes")
      .field("len", &self.len())
      .finish_non_exhaustive()
  }
}

impl PartialEq for FontBytes {
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}

impl Eq for FontBytes {}

impl From<Vec<u8>> for FontBytes {
  fn from(data: Vec<u8>) -> Self {
    Self(Arc::new(data))
  }
}

impl From<Arc<[u8]>> for FontBytes {
  fn from(data: Arc<[u8]>) -> Self {
    Self(Arc::new(data))
  }
}

impl From<Arc<dyn AsRef<[u8]> + Send + Sync>> for FontBytes {
  fn from(data: Arc<dyn AsRef<[u8]> + Send + Sync>) -> Self {
    Self(data)
  }
}

impl From<&'static [u8]> for FontBytes {
  fn from(data: &'static [u8]) -> Self {
    Self(Arc::new(data))
  }
}

impl<'a> From<Cow<'a, [u8]>> for FontBytes {
  fn from(data: Cow<'a, [u8]>) -> Self {
    match data {
      Cow::Borrowed(data) => Self::from(data.to_vec()),
      Cow::Owned(data) => Self::from(data),
    }
  }
}

struct RuntimeFace {
  faces: Yoke<RuntimeFaces<'static>, Box<FontBytes>>,
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
  skrifa: SkrifaFontRef<'a>,
}

impl RuntimeFace {
  fn new(data: FontBytes, face_index: u32) -> Result<Self> {
    let faces =
      Yoke::<RuntimeFaces<'static>, Box<FontBytes>>::try_attach_to_cart(Box::new(data), |data| {
        let harf = HarfFontRef::from_index(data.as_slice(), face_index)
          .map_err(|_| FontError::InvalidFace)?;
        let skrifa = SkrifaFontRef::from_index(data.as_slice(), face_index)
          .map_err(|_| FontError::InvalidFace)?;
        Ok(RuntimeFaces { harf, skrifa })
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

  fn skrifa(&self) -> &SkrifaFontRef<'_> {
    &self.faces.get().skrifa
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

    let face = self.skrifa();
    let bounds = face
      .glyph_metrics(SkrifaSize::new(1.0), SkrifaLocationRef::default())
      .bounds(SkrifaGlyphId::new(u32::from(glyph_id)))
      .map(glyph_bounds_from_skrifa);
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
    for chain in default_family_substitution_chains() {
      self.book.family_substitution_chains.push(chain);
    }
    for chain in default_glyph_fallback_chains() {
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
    let mut registered = 0usize;
    for platform_font in platform_system_fonts() {
      let id = format!(
        "system:{}:{}",
        platform_font
          .face
          .postscript_name
          .as_deref()
          .unwrap_or("unknown"),
        platform_font.face_index
      );
      if self
        .sources
        .iter()
        .any(|source| source.id() == Some(id.as_str()))
      {
        continue;
      }
      let mut face = (*platform_font.face).clone();
      face.font_id = FontId(Arc::from(id.as_str()));
      self.register_face(
        FontSource::Memory {
          id: Cow::Owned(id),
          data: platform_font.data,
        },
        face,
      );
      registered += 1;
    }
    Ok(registered)
  }

  pub fn register_system_query_fonts(&mut self, request: &FontRequest<'_>) -> Result<usize> {
    let mut registered = 0usize;
    let mut queries = SmallVec::<[PlatformFontQueryFamily; 8]>::new();
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
        queries.push(PlatformFontQueryFamily::Name(family.to_string()));
        let aliased = resolve_family_alias(&self.book, Cow::Borrowed(family));
        if aliased.as_ref() != family {
          queries.push(PlatformFontQueryFamily::Name(aliased.into_owned()));
        }
      }
      for (family, _) in self.family_substitution_families(request) {
        queries.push(PlatformFontQueryFamily::Name(family.to_owned()));
      }
      for family in self.fallback_families(request) {
        queries.push(PlatformFontQueryFamily::Name(family.to_owned()));
      }
      push_platform_generic_queries(&mut queries, request.family_class);
    } else {
      // Resolve the
      // Office/script fallback chain by family name before asking for the
      // generic aliases, so an unspecified OOXML typeface still produces a
      // portable, shapeable system face.
      for family in self.fallback_families(request) {
        queries.push(PlatformFontQueryFamily::Name(family.to_owned()));
      }
      push_platform_generic_queries(&mut queries, request.family_class);
    }

    for query_family in queries {
      let platform_fonts = font_timing("platform font query", || {
        platform_system_query_fonts(&query_family, request)
      });
      for platform_font in platform_fonts {
        let mut face = (*platform_font.face).clone();
        let matched_legacy_postscript_name =
          if let PlatformFontQueryFamily::Name(family) = &query_family {
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
        if matched_legacy_postscript_name
          && let PlatformFontQueryFamily::Name(family) = &query_family
        {
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
            && let PlatformFontQueryFamily::Name(family) = &query_family
          {
            push_unique_string(&mut existing.family_names, family.clone());
          }
          continue;
        }
        face.font_id = FontId(Arc::from(font_id.as_str()));
        self.register_face(
          FontSource::Memory {
            id: Cow::Owned(font_id),
            data: platform_font.data,
          },
          face,
        );
        registered += 1;
      }
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
        for (family, reason) in self.family_substitution_families(request) {
          if let Ok(mut resolved) =
            self
              .book
              .resolve_matching_family(request, &self.faces, family, false)
          {
            resolved.substitution = font_substitution(request, &resolved, reason);
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
        for (family, reason) in self.family_substitution_families(request) {
          if let Ok(mut resolved) =
            self
              .book
              .resolve_matching_family(request, &self.faces, family, true)
          {
            resolved.substitution = font_substitution(request, &resolved, reason);
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
            runtime_faces[index].as_deref().map(RuntimeFace::skrifa),
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

  /// Returns deterministic family-substitution candidates for an unavailable
  /// requested family.
  ///
  /// Family substitution is deliberately separate from glyph fallback. This
  /// follows LibreOffice's `PhysicalFontCollection::FindFontFamily()` versus
  /// `GetGlyphFallbackFont()` split, Skia DirectWrite's family match versus
  /// `onFallback()` split, and ReactOS's font mapper versus FontLink split.
  fn family_substitution_families<'book>(
    &'book self,
    request: &FontRequest<'_>,
  ) -> SmallVec<[(&'book str, FontSubstitutionReason); 8]> {
    let Some(requested_family) = request.family.as_deref() else {
      return SmallVec::new();
    };
    let mut families = SmallVec::<[(&'book str, FontSubstitutionReason); 8]>::new();
    for chain in &self.book.family_substitution_chains {
      let reason = if let Some(family) = chain.requested_family.as_deref() {
        if !normalized_family_eq(family, requested_family) {
          continue;
        }
        FontSubstitutionReason::MissingFamily
      } else {
        FontSubstitutionReason::LastResort
      };
      if chain
        .script
        .is_some_and(|script| request.script != Some(script))
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
        if !families
          .iter()
          .any(|(existing, _)| normalized_family_eq(existing, family.as_ref()))
        {
          families.push((family.as_ref(), reason));
        }
      }
    }
    families
  }

  /// Returns fonts that may cover glyphs missing from an already selected
  /// primary face. These chains must never select the primary face for a
  /// missing requested family.
  fn fallback_families<'book>(
    &'book self,
    request: &FontRequest<'_>,
  ) -> SmallVec<[&'book str; 16]> {
    let mut families = SmallVec::<[&'book str; 16]>::new();
    let requested_family = request.family.as_deref();
    for chain in &self.book.fallback_chains {
      if chain.requested_family.as_deref().is_some_and(|family| {
        requested_family.is_none_or(|requested| !normalized_family_eq(requested, family))
      }) {
        continue;
      }
      if chain
        .script
        .is_some_and(|script| request.script != Some(script))
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
        if !families
          .iter()
          .any(|existing| normalized_family_eq(existing, family.as_ref()))
        {
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
      return skrifa_face_supports_char(parsed.skrifa(), ch);
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
      substitution: None,
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
  /// Ordered pure-Rust policy used only when the requested family is absent.
  pub family_substitution_chains: Vec<FontFallbackChain<'a>>,
  /// Ordered glyph-coverage policy used only after a primary face is chosen.
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
    let mut substitution = None;
    let target_family_names = family_override.or(request.family.as_deref()).map(|family| {
      let aliased = resolve_family_alias(self, Cow::Borrowed(family));
      let rule = find_substitution_rule(self, aliased.as_ref());
      let target = rule
        .map(|rule| rule.substitute_family.as_ref())
        .unwrap_or_else(|| aliased.as_ref());
      if family_override.is_none() {
        let reason = rule
          .map(|rule| rule.reason)
          .or_else(|| (aliased.as_ref() != family).then_some(FontSubstitutionReason::Alias));
        if let Some(reason) = reason {
          substitution = Some(FontSubstitution {
            requested_family: Cow::Owned(family.to_string()),
            substituted_family: Cow::Owned(target.to_string()),
            reason,
          });
        }
      }
      normalized_family_names(target)
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
      substitution,
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
    let face = SkrifaFontRef::from_index(data, face_index).map_err(|_| FontError::InvalidFace)?;
    let mut family_names = Vec::new();
    for name_id in [StringId::FAMILY_NAME, StringId::TYPOGRAPHIC_FAMILY_NAME] {
      for name in face.localized_strings(name_id) {
        push_unique_string(&mut family_names, name.to_string());
      }
    }
    let postscript_name = skrifa_name_by_id(&face, StringId::POSTSCRIPT_NAME).map(Cow::Owned);
    let style_name = skrifa_name_by_id(&face, StringId::SUBFAMILY_NAME).map(Cow::Owned);
    if family_names.is_empty() {
      family_names.push(Cow::Owned(id.to_string()));
    }

    let metrics = face.metrics(SkrifaSize::new(1.0), SkrifaLocationRef::default());
    let axes = face.axes();
    let pitch = if metrics.is_monospace {
      FontPitch::Fixed
    } else {
      FontPitch::Variable
    };
    let flags = FontFlags {
      // OpenType `cmap` platform 3 / encoding 0 identifies a Windows symbol
      // font.  Keep the parser's selected cmap classification on the face so
      // a charset-only request can distinguish it from an ordinary text face.
      symbolic: face.charmap().is_symbol(),
      monospace: metrics.is_monospace,
      color_glyphs: face.colr().is_ok() || face.sbix().is_ok(),
      vertical: face.vhea().is_ok() || face.vmtx().is_ok(),
      graphite: has_table(&face, b"Silf") || has_table(&face, b"Feat") || has_table(&face, b"Sill"),
      aat: face.feat().is_ok() || face.morx().is_ok(),
      cff2: face.cff2().is_ok(),
      variable: !axes.is_empty(),
      kashida_positions: face.morx().is_err(),
      ..FontFlags::default()
    };
    let metrics = font_metrics_from_skrifa(&face, 1.0);
    let attributes = face.attributes();
    let (weight, stretch) = face
      .os2()
      .map_or((FontWeight::Normal, FontStretch::Normal), |os2| {
        (
          font_weight_from_opentype(os2.us_weight_class()),
          font_stretch_from_opentype(os2.us_width_class()),
        )
      });

    Ok(Self {
      font_id: FontId(Arc::from(id)),
      family_names,
      postscript_name,
      style_name,
      family_class: None,
      weight,
      slant: font_slant_from_skrifa(attributes.style),
      stretch,
      pitch,
      coverage: font_coverage_from_skrifa(&face),
      flags,
      axes: variation_axes_from_skrifa(&face),
      features: opentype_features_from_skrifa(&face),
      metrics,
      embedding: font_embedding_policy_from_skrifa(&face),
      embedding_plan: font_embedding_plan_from_skrifa(&face),
      bounds: font_bounds_from_skrifa(&face),
      face_index,
    })
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
  /// The family-selection decision, distinct from later missing-glyph runs.
  pub substitution: Option<FontSubstitution<'book>>,
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
    let reduced_small_caps_segment =
      small_caps && (has_lowercase || text.chars().all(char::is_whitespace));
    let (shaped_text, small_caps_ranges) = if has_lowercase {
      small_caps_shaped_text(text)
    } else {
      (Cow::Borrowed(text), Vec::new())
    };
    let shape_size = if reduced_small_caps_segment {
      FontSize(options.size_pt.0 * 0.8)
    } else {
      options.size_pt
    };
    let units_per_em = runtime_face
      .skrifa()
      .head()
      .map(|head| f32::from(head.units_per_em()))
      .unwrap_or(1.0);
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(shaped_text.as_ref());
    buffer.guess_segment_properties();
    if let Some(direction) = harf_direction(options.direction) {
      buffer.set_direction(direction);
    }
    if let Some(script) = harf_script_for_shape_options(options) {
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
        // ECMA-376 Part 1 §17.3.2.35 adds the authored pitch after each
        // character. Retain it on the final glyph as well: it participates in
        // centered/right-aligned run measurement and separates this run from
        // a following differently shaped run without moving the final glyph's
        // own paint origin.
        if tracking.abs() > f32::EPSILON {
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
    let face =
      SkrifaFontRef::from_index(data, self.face_index).map_err(|_| FontError::InvalidFace)?;
    Ok(
      face
        .glyph_metrics(SkrifaSize::new(size.0), SkrifaLocationRef::default())
        .bounds(SkrifaGlyphId::new(glyph_id))
        .map(glyph_bounds_from_skrifa),
    )
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontScriptRun {
  pub text_range: Range<usize>,
  pub script: TextScript,
  /// WordprocessingML font slot selected independently from the Unicode
  /// shaping script. A Greek character can, for example, keep Greek shaping
  /// while `w:hint="eastAsia"` selects the East Asian font face.
  pub wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  pub direction: TextDirection,
  pub size_pt: FontSize,
  pub small_caps: bool,
}

/// Effective WordprocessingML run-font slot from [MS-OI29500] section 2.1.88
/// (Part 1 section 17.3.2.26).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WordprocessingFontSlot {
  Ascii,
  HighAnsi,
  EastAsia,
  ComplexScript,
}

/// WordprocessingML `w:rFonts/@w:hint` selection for characters whose font
/// slot is otherwise ambiguous.
///
/// The names follow ECMA-376 Part 1 §17.18.41. `Ascii` is the transitional
/// extension accepted by Office and selects the Latin/ASCII slot explicitly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WordprocessingFontTypeHint {
  Default,
  Ascii,
  EastAsia,
  ComplexScript,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScriptScanOptions {
  pub app_script: TextScript,
  pub small_caps: bool,
  /// Apply the explicit WordprocessingML rFonts slot classification before
  /// falling back to Unicode Script for Common characters.
  pub wordprocessingml_font_slots: bool,
  /// Resolve weak and ECMA-376 conditionally classified characters through
  /// the effective `w:rFonts/@w:hint` slot.
  pub wordprocessingml_font_hint: Option<WordprocessingFontTypeHint>,
  /// Whether effective `w:lang/@w:eastAsia` has a `zh` language component.
  pub wordprocessingml_east_asia_language_is_chinese: bool,
  /// Character set declared for the effective East Asian font-table entry.
  pub wordprocessingml_east_asia_font_charset: Option<FontCharset>,
  /// A run-level `w:cs` or `w:rtl` selects Word's complex-script font for the
  /// run, independently from its Unicode scripts. Word fixed output retains
  /// the ASCII family for Basic Latin decimal digits; complex-script run
  /// properties such as size remain a separate layout decision.
  pub wordprocessingml_complex_font_override: bool,
  /// Word uses the ASCII face for East Asian-classified characters when the
  /// effective East Asian face is Times New Roman and ASCII and High ANSI
  /// resolve to the same face.
  pub wordprocessingml_east_asia_uses_ascii: bool,
}

impl Default for ScriptScanOptions {
  fn default() -> Self {
    Self {
      app_script: TextScript::Common,
      small_caps: false,
      wordprocessingml_font_slots: false,
      wordprocessingml_font_hint: None,
      wordprocessingml_east_asia_language_is_chinese: false,
      wordprocessingml_east_asia_font_charset: None,
      wordprocessingml_complex_font_override: false,
      wordprocessingml_east_asia_uses_ascii: false,
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
    small_caps_script_direction_runs(text, size_pt, options)
  } else {
    script_direction_runs_for_segment(text, 0, size_pt, false, options)
  }
}

fn small_caps_script_direction_runs(
  text: &str,
  size_pt: FontSize,
  options: ScriptScanOptions,
) -> Vec<FontScriptRun> {
  let mut runs = Vec::new();
  let mut start = 0usize;
  let mut active_reduced = None::<bool>;
  for range in grapheme_clusters(text) {
    let cluster = &text[range.clone()];
    // ISO/IEC 29500-1 §17.3.2.33 limits the synthesized form to lowercase
    // letters and leaves non-alphabetic characters unchanged. Word's fixed
    // output also keeps digits and punctuation at the authored size, while
    // whitespace uses the reduced advance of the surrounding small-cap text.
    // Classify a complete grapheme so a lowercase base and its combining
    // marks are never shaped at different sizes.
    let reduced =
      cluster.chars().any(char::is_lowercase) || cluster.chars().all(char::is_whitespace);
    if let Some(active) = active_reduced
      && active != reduced
    {
      push_small_caps_case_run(
        text,
        start..range.start,
        active,
        size_pt,
        options,
        &mut runs,
      );
      start = range.start;
    }
    active_reduced = Some(reduced);
  }
  if start < text.len() {
    push_small_caps_case_run(
      text,
      start..text.len(),
      active_reduced.unwrap_or(false),
      size_pt,
      options,
      &mut runs,
    );
  }
  runs
}

fn push_small_caps_case_run(
  source: &str,
  range: Range<usize>,
  reduced_run: bool,
  size_pt: FontSize,
  options: ScriptScanOptions,
  runs: &mut Vec<FontScriptRun>,
) {
  let mut segment_runs = script_direction_runs_for_segment(
    &source[range.clone()],
    range.start,
    size_pt,
    reduced_run,
    options,
  );
  runs.append(&mut segment_runs);
}

fn script_direction_runs_for_segment(
  text: &str,
  range_offset: usize,
  size_pt: FontSize,
  small_caps: bool,
  options: ScriptScanOptions,
) -> Vec<FontScriptRun> {
  let mut runs = Vec::new();
  if !options.wordprocessingml_font_slots {
    script_direction_runs_for_slot_segment_into(
      text,
      range_offset,
      size_pt,
      small_caps,
      options,
      None,
      &mut runs,
    );
    return runs;
  }

  let mut start = 0usize;
  let mut active_slot = None;
  for (index, ch) in text.char_indices() {
    let slot = wordprocessing_font_slot(ch, options);
    if let Some(active) = active_slot
      && slot != active
    {
      script_direction_runs_for_slot_segment_into(
        &text[start..index],
        range_offset + start,
        size_pt,
        small_caps,
        options,
        Some(active),
        &mut runs,
      );
      start = index;
    }
    active_slot = Some(slot);
  }
  if start < text.len() {
    script_direction_runs_for_slot_segment_into(
      &text[start..],
      range_offset + start,
      size_pt,
      small_caps,
      options,
      active_slot,
      &mut runs,
    );
  }
  runs
}

fn script_direction_runs_for_slot_segment_into(
  text: &str,
  range_offset: usize,
  size_pt: FontSize,
  small_caps: bool,
  options: ScriptScanOptions,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  runs: &mut Vec<FontScriptRun>,
) {
  if text.is_empty() {
    return;
  }
  let mut push_run = |range: Range<usize>, script: TextScript| {
    let value = &text[range.clone()];
    runs.push(FontScriptRun {
      text_range: (range.start + range_offset)..(range.end + range_offset),
      script,
      wordprocessingml_font_slot,
      direction: text_direction_from_bidi(get_base_direction(value)),
      size_pt,
      small_caps: small_caps && small_caps_supported_for_script(script),
    });
  };
  let leading_script = first_strong_text_script(text, options).unwrap_or(options.app_script);
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
    let Some(script) = strong_text_script(ch, options) else {
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
          push_run(start..split, active_script);
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
    push_run(start..text.len(), active.unwrap_or(leading_script));
  }
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
  pub directwrite_baseline_offset_pt: f32,
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
      directwrite_baseline_offset_pt: self.directwrite_baseline_offset_pt * scale,
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FontSlant {
  #[default]
  Upright,
  Italic,
  Oblique,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
  // while the same installed face is commonly exposed by platform APIs under its
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
  // Nyala is an optional Windows Ethiopic supplemental font. Office fixed
  // output without that feature uses Calibri for its Latin portions and the
  // system African-script face from the requested-family fallback below.
  ("Nyala", "Calibri"),
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

fn default_family_specific_chains<'a>() -> Vec<FontFallbackChain<'a>> {
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
    // Microsoft's SimSun family inventory lists SimSun-ExtB and SimSun as
    // members of the same family. Prefer the installed base face for covered
    // BMP glyphs when the Ext-B member is unavailable; true Extension-B
    // characters that it cannot cover continue through script fallback.
    office_family_fallback("SimSun-ExtB", &["SimSun"]),
    // Ebrima is Windows' default African-language face and supplies real
    // regular/bold Ethiopic fonts when optional Nyala is unavailable.
    office_family_fallback("Nyala", &["Ebrima"]),
    // AR PL SungtiL GB is the legacy Simplified Chinese Song face written by
    // older LibreOffice documents. Current LibreOffice's zh-CN CJK_TEXT list
    // starts with Source Han/Noto Serif CJK SC before SimSun-style fallbacks;
    // keep the same serif/script identity instead of letting the generic Han
    // chain substitute a sans face.
    office_family_fallback(
      "AR PL SungtiL GB",
      &["Noto Serif CJK SC", "Source Han Serif SC", "SimSun"],
    ),
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
  ]
}

fn default_family_substitution_chains<'a>() -> Vec<FontFallbackChain<'a>> {
  let mut chains = default_family_specific_chains();
  // A family with no authored or known metric-compatible replacement still
  // needs a text face before glyph coverage can be evaluated. Keep this
  // deterministic and separate from the Common-script symbol chain below.
  // LibreOffice ends `FindFontFamily()` at an attribute/default family and
  // only then calls `GetGlyphFallbackFont()` for missing code points.
  chains.push(FontFallbackChain {
    requested_family: None,
    script: None,
    language: None,
    families: vec![
      Cow::Borrowed("DejaVu Sans"),
      Cow::Borrowed("Liberation Sans"),
      Cow::Borrowed("Noto Sans"),
      Cow::Borrowed("Noto Sans CJK JP"),
    ],
  });
  chains
}

fn default_glyph_fallback_chains<'a>() -> Vec<FontFallbackChain<'a>> {
  let mut chains = default_family_specific_chains();
  chains.extend([
    // Word keeps Liberation Sans for its covered Latin glyphs, but fixed
    // output links missing Han glyphs from that requested face through SimSun.
    // Scope this to Han so explicit Chinese faces and other script fallbacks
    // continue to use their own chains.
    FontFallbackChain {
      requested_family: Some(Cow::Borrowed("Liberation Sans")),
      script: Some(TextScript::Han),
      language: None,
      families: vec![Cow::Borrowed("SimSun")],
    },
    FontFallbackChain {
      requested_family: None,
      script: Some(TextScript::Han),
      language: None,
      families: vec![
        // Windows fixed output uses its installed Simplified Chinese font
        // linking before generic pan-CJK fallbacks. Keep family discovery in
        // Fontique so platforms without these Office fonts continue
        // through the portable SC/JP chain.
        Cow::Borrowed("Microsoft YaHei"),
        Cow::Borrowed("Microsoft YaHei UI"),
        Cow::Borrowed("DengXian"),
        Cow::Borrowed("Noto Sans CJK SC"),
        Cow::Borrowed("Noto Sans CJK JP"),
        Cow::Borrowed("Noto Serif CJK SC"),
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
  ]);
  chains
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

fn font_substitution<'book>(
  request: &FontRequest<'_>,
  resolved: &ResolvedFont<'book>,
  reason: FontSubstitutionReason,
) -> Option<FontSubstitution<'book>> {
  Some(FontSubstitution {
    requested_family: Cow::Owned(request.family.as_deref()?.to_string()),
    substituted_family: resolved.resolved_family.clone(),
    reason,
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

fn normalized_family_eq(left: &str, right: &str) -> bool {
  normalized_family_chars(left).eq(normalized_family_chars(right))
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
enum PlatformFontQueryFamily {
  Name(String),
  SansSerif,
  Serif,
  Monospace,
}

fn push_platform_generic_queries(
  queries: &mut SmallVec<[PlatformFontQueryFamily; 8]>,
  family_class: Option<FontFamilyClass>,
) {
  match family_class {
    Some(FontFamilyClass::Serif | FontFamilyClass::OldStyle | FontFamilyClass::Schoolbook) => {
      // ECMA-376 Part 1 §17.18.30 defines `roman` as a proportional serif
      // family and uses Times New Roman as its representative. Query the
      // Windows representative before the host's generic serif so Linux
      // fixed output follows the same document font-table substitution.
      queries.push(PlatformFontQueryFamily::Name("Times New Roman".to_string()));
      queries.push(PlatformFontQueryFamily::Serif);
      queries.push(PlatformFontQueryFamily::SansSerif);
    }
    Some(FontFamilyClass::Fixed) => {
      queries.push(PlatformFontQueryFamily::Name("Courier New".to_string()));
      queries.push(PlatformFontQueryFamily::Monospace);
      queries.push(PlatformFontQueryFamily::SansSerif);
      queries.push(PlatformFontQueryFamily::Serif);
    }
    Some(FontFamilyClass::SansSerif) => {
      queries.push(PlatformFontQueryFamily::Name("Arial".to_string()));
      queries.push(PlatformFontQueryFamily::SansSerif);
      queries.push(PlatformFontQueryFamily::Serif);
    }
    _ => {
      queries.push(PlatformFontQueryFamily::SansSerif);
      queries.push(PlatformFontQueryFamily::Serif);
    }
  }
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

#[derive(Clone)]
struct PlatformFontCandidate {
  data: FontBytes,
  face_index: u32,
  face: Arc<FontFaceInfo<'static>>,
}

fn platform_font_system() -> &'static Mutex<PlatformFontSystem> {
  static SYSTEM: OnceLock<Mutex<PlatformFontSystem>> = OnceLock::new();
  SYSTEM.get_or_init(|| Mutex::new(PlatformFontSystem::new()))
}

fn platform_system_fonts() -> Vec<PlatformFontCandidate> {
  let mut system = platform_font_system()
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
  let PlatformFontSystem {
    collection,
    source_cache,
  } = &mut *system;
  let family_names = collection
    .family_names()
    .map(str::to_owned)
    .collect::<Vec<_>>();
  let family_ids = family_names
    .iter()
    .filter_map(|name| collection.family_id(name))
    .collect::<BTreeSet<_>>();
  let mut seen = BTreeSet::new();
  let mut candidates = Vec::new();
  for family_id in family_ids {
    let Some(family) = collection.family(family_id) else {
      continue;
    };
    for (font_index, font) in family.fonts().iter().enumerate() {
      let Some(blob) = font.load(Some(source_cache)) else {
        continue;
      };
      if !seen.insert((blob.id(), font.index())) {
        continue;
      }
      let data = FontBytes::from(blob.into_raw_parts().0);
      let Some(face) = cached_platform_font_face(
        PlatformFontFaceKey {
          family_id: family_id.to_u64(),
          font_index,
          face_index: font.index(),
        },
        data.clone(),
      ) else {
        continue;
      };
      candidates.push(PlatformFontCandidate {
        data,
        face_index: font.index(),
        face,
      });
    }
  }
  candidates
}

fn platform_system_query_fonts(
  query_family: &PlatformFontQueryFamily,
  request: &FontRequest<'_>,
) -> Vec<PlatformFontCandidate> {
  static CACHE: OnceLock<Mutex<PlatformFontQueryCache>> = OnceLock::new();

  let key = PlatformFontQueryKey {
    family: query_family.clone(),
    weight: requested_weight(request),
    slant: requested_slant(request),
    stretch: request.stretch.unwrap_or_default(),
  };
  let slot = CACHE
    .get_or_init(|| Mutex::new(PlatformFontQueryCache::default()))
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner)
    .slot(key);

  slot
    .get_or_init(|| {
      let mut system = platform_font_system()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
      let PlatformFontSystem {
        collection,
        source_cache,
      } = &mut *system;
      if let PlatformFontQueryFamily::Name(name) = query_family {
        let normalized = normalize_family(name);
        let direct_family_id = collection.family_id(name);
        let prefix_family_name = if direct_family_id.is_none() {
          collection
            .family_names()
            .filter_map(|family| {
              let normalized_family = normalize_family(family);
              (!normalized_family.is_empty() && normalized.starts_with(&normalized_family))
                .then_some((normalized_family.len(), family.to_owned()))
            })
            .max_by_key(|(length, _)| *length)
            .map(|(_, family)| family)
        } else {
          None
        };
        let family_id = direct_family_id.or_else(|| {
          prefix_family_name
            .as_deref()
            .and_then(|family| collection.family_id(family))
        });
        if let Some(family_id) = family_id
          && let Some(family) = collection.family(family_id)
        {
          let mut family_candidates = Vec::new();
          for (font_index, font) in family.fonts().iter().enumerate() {
            let Some(blob) = font.load(Some(source_cache)) else {
              continue;
            };
            let data = FontBytes::from(blob.into_raw_parts().0);
            let Some(face) = cached_platform_font_face(
              PlatformFontFaceKey {
                family_id: family_id.to_u64(),
                font_index,
                face_index: font.index(),
              },
              data.clone(),
            ) else {
              continue;
            };
            if !family_matches_names(face.as_ref(), std::slice::from_ref(&normalized))
              && !face
                .postscript_name
                .as_deref()
                .is_some_and(|candidate| normalized_family_eq_normalized(candidate, &normalized))
            {
              continue;
            }
            family_candidates.push(PlatformFontCandidate {
              data,
              face_index: font.index(),
              face,
            });
          }

          // DirectWrite and CoreText expose legacy subfamilies such as
          // "Calibri Light" and "Segoe UI Light" as aliases of a larger
          // typographic family. Fontique correctly resolves that family, but
          // attribute matching alone can then turn a bold request into
          // Calibri Bold. Office instead keeps the explicitly named legacy
          // face and synthesizes bold. Prefer faces whose primary family or
          // PostScript name exactly represents the requested legacy name,
          // then apply the ordinary attribute ranking within that subset.
          let has_exact_legacy_face = family_candidates.iter().any(|candidate| {
            candidate
              .face
              .family_names
              .first()
              .is_some_and(|family| normalized_family_eq_normalized(family, &normalized))
              || candidate
                .face
                .postscript_name
                .as_deref()
                .is_some_and(|postscript| normalized_family_eq_normalized(postscript, &normalized))
          });
          if has_exact_legacy_face {
            family_candidates.retain(|candidate| {
              candidate
                .face
                .family_names
                .first()
                .is_some_and(|family| normalized_family_eq_normalized(family, &normalized))
                || candidate
                  .face
                  .postscript_name
                  .as_deref()
                  .is_some_and(|postscript| {
                    normalized_family_eq_normalized(postscript, &normalized)
                  })
            });
          }
          if !family_candidates.is_empty() {
            return family_candidates;
          }
        }
      }
      let family = match query_family {
        PlatformFontQueryFamily::Name(name) => PlatformQueryFamily::Named(name),
        PlatformFontQueryFamily::SansSerif => {
          PlatformQueryFamily::Generic(PlatformGenericFamily::SansSerif)
        }
        PlatformFontQueryFamily::Serif => {
          PlatformQueryFamily::Generic(PlatformGenericFamily::Serif)
        }
        PlatformFontQueryFamily::Monospace => {
          PlatformQueryFamily::Generic(PlatformGenericFamily::Monospace)
        }
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
        let data = FontBytes::from(font.blob.clone().into_raw_parts().0);
        let Some(face) = cached_platform_font_face(
          PlatformFontFaceKey {
            family_id: font.family.0.to_u64(),
            font_index: font.family.1,
            face_index: font.index,
          },
          data.clone(),
        ) else {
          return PlatformQueryStatus::Continue;
        };
        let matches_requested_family = match query_family {
          PlatformFontQueryFamily::Name(family) => {
            let normalized = normalize_family(family);
            family_matches_names(face.as_ref(), std::slice::from_ref(&normalized))
              || face
                .postscript_name
                .as_deref()
                .is_some_and(|name| normalized_family_eq_normalized(name, &normalized))
          }
          PlatformFontQueryFamily::SansSerif
          | PlatformFontQueryFamily::Serif
          | PlatformFontQueryFamily::Monospace => true,
        };
        if !matches_requested_family {
          return PlatformQueryStatus::Continue;
        }
        candidates.push(PlatformFontCandidate {
          data,
          face_index: font.index,
          face,
        });
        // Fontique orders matches by the requested family and attributes. One
        // accepted face is sufficient for this query: later families are fallback
        // candidates that the OOXML registry adds through their own ordered
        // queries. Copying all of them here duplicated tens of complete font files
        // for every text style and made a small DOCX consume gigabytes.
        PlatformQueryStatus::Stop
      });
      candidates
    })
    .clone()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PlatformFontFaceKey {
  family_id: u64,
  font_index: usize,
  face_index: u32,
}

fn cached_platform_font_face(
  key: PlatformFontFaceKey,
  data: FontBytes,
) -> Option<Arc<FontFaceInfo<'static>>> {
  type FaceSlot = OnceLock<Option<Arc<FontFaceInfo<'static>>>>;
  static CACHE: OnceLock<RwLock<HashMap<PlatformFontFaceKey, Arc<FaceSlot>>>> = OnceLock::new();

  let cache = CACHE.get_or_init(|| RwLock::new(HashMap::new()));
  let cached = {
    cache
      .read()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .get(&key)
      .cloned()
  };
  let slot = cached.unwrap_or_else(|| {
    cache
      .write()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .entry(key)
      .or_insert_with(|| Arc::new(FaceSlot::new()))
      .clone()
  });
  slot
    .get_or_init(|| {
      FontFaceInfo::from_ttf_bytes("platform-system-font", &data, key.face_index)
        .ok()
        .map(Arc::new)
    })
    .clone()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct PlatformFontQueryKey {
  family: PlatformFontQueryFamily,
  weight: FontWeight,
  slant: FontSlant,
  stretch: FontStretch,
}

type PlatformFontSlot = OnceLock<Vec<PlatformFontCandidate>>;

const PLATFORM_FONT_QUERY_CACHE_ENTRIES: usize = 4_096;

#[derive(Default)]
struct PlatformFontQueryCache {
  slots: HashMap<PlatformFontQueryKey, Arc<PlatformFontSlot>>,
  insertion_order: VecDeque<PlatformFontQueryKey>,
}

impl PlatformFontQueryCache {
  fn slot(&mut self, key: PlatformFontQueryKey) -> Arc<PlatformFontSlot> {
    if let Some(slot) = self.slots.get(&key) {
      return slot.clone();
    }
    while self.slots.len() >= PLATFORM_FONT_QUERY_CACHE_ENTRIES {
      let Some(evicted) = self.insertion_order.pop_front() else {
        break;
      };
      self.slots.remove(&evicted);
    }
    let slot = Arc::new(PlatformFontSlot::new());
    self.insertion_order.push_back(key.clone());
    self.slots.insert(key, slot.clone());
    slot
  }
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

fn runtime_face_for_data(data: FontBytes, face_index: u32) -> Option<Arc<RuntimeFace>> {
  RuntimeFace::new(data, face_index).ok().map(Arc::new)
}

fn font_supports_char(
  font: &ResolvedFontWithFace<'_, '_>,
  parsed_face: Option<&SkrifaFontRef<'_>>,
  ch: char,
) -> bool {
  if let Some(face) = parsed_face {
    return skrifa_face_supports_char(face, ch);
  }
  font
    .face
    .is_some_and(|face| face.coverage.contains_char(ch))
}

fn skrifa_face_supports_char(face: &SkrifaFontRef<'_>, ch: char) -> bool {
  face
    .charmap()
    .map(ch)
    .is_some_and(|glyph_id| glyph_id != SkrifaGlyphId::NOTDEF)
}

fn font_supports_text_cluster(
  font: &ResolvedFontWithFace<'_, '_>,
  parsed_face: Option<&SkrifaFontRef<'_>>,
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
  book
    .family_aliases
    .iter()
    .find(|alias| normalized_family_eq(&alias.from, family.as_ref()))
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
  book
    .substitutions
    .iter()
    .find(|rule| normalized_family_eq(&rule.requested_family, family))
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

fn font_metrics_from_skrifa(face: &SkrifaFontRef<'_>, em_size: f32) -> FontMetrics {
  let metrics = face.metrics(SkrifaSize::new(em_size), SkrifaLocationRef::default());
  let units_per_em = f32::from(metrics.units_per_em.max(1));
  let to_em = |value: i32| value as f32 / units_per_em * em_size;
  let ascender = metrics.ascent.max(0.0);
  let descender = (-metrics.descent).max(0.0);
  let os2 = face.os2().ok();
  let uses_typographic_metrics = os2.as_ref().is_some_and(|os2| {
    use skrifa::raw::tables::os2::SelectionFlags;
    os2.version() >= 4
      && os2
        .fs_selection()
        .contains(SelectionFlags::USE_TYPO_METRICS)
  });
  // Windows Office lays out the baseline from OS/2 Windows metrics unless
  // the face explicitly opts into typographic metrics. Keep that baseline
  // separate from the natural line box: usWinAscent was designed as a
  // clipping extent and can be larger than the typographic ascender.
  let baseline_offset = os2.as_ref().map_or(ascender, |os2| {
    let units = if uses_typographic_metrics {
      i32::from(os2.s_typo_ascender())
    } else {
      i32::from(os2.us_win_ascent())
    };
    let units = units.max(0);
    if units == 0 { ascender } else { to_em(units) }
  });
  // IDWriteTextLayout's default DWRITE_LINE_METRICS baseline is not the
  // typographic baseline used by paragraph layout. DirectWrite starts with
  // its alignment-box ascent and adds its derived line gap. Wine's
  // Windows-conformance implementation derives that gap by preserving the
  // hhea total while substituting OS/2 Windows ascent/descent; fonts opting
  // into USE_TYPO_METRICS use their typographic ascent and line gap directly.
  let directwrite_baseline_offset = os2.as_ref().map_or(ascender, |os2| {
    if uses_typographic_metrics {
      ascender + metrics.leading
    } else {
      let windows_ascent = to_em(i32::from(os2.us_win_ascent()));
      let windows_descent = to_em(i32::from(os2.us_win_descent()));
      directwrite_default_baseline_offset(
        ascender,
        descender,
        metrics.leading,
        windows_ascent,
        windows_descent,
      )
    }
  });
  let line_gap = if metrics.leading > 0.0 {
    metrics.leading
  } else {
    (em_size - ascender - descender).max(0.0)
  };
  let script = face.os2().ok().map_or_else(
    || ScriptMetrics {
      superscript_scale: 1.0,
      subscript_scale: 1.0,
      small_caps_scale: 1.0,
      ..ScriptMetrics::default()
    },
    |os2| {
      let scale = |units: i16| {
        if units > 0 {
          f32::from(units) / units_per_em
        } else {
          1.0
        }
      };
      ScriptMetrics {
        superscript_scale: scale(os2.y_superscript_y_size()),
        subscript_scale: scale(os2.y_subscript_y_size()),
        superscript_offset_pt: to_em(i32::from(os2.y_superscript_y_offset().max(0))),
        subscript_offset_pt: to_em(i32::from(os2.y_subscript_y_offset().max(0))),
        small_caps_scale: 1.0,
      }
    },
  );
  FontMetrics {
    vertical: VerticalMetrics {
      ascent_pt: ascender,
      descent_pt: descender,
      baseline_offset_pt: baseline_offset,
      directwrite_baseline_offset_pt: directwrite_baseline_offset,
      line_gap_pt: line_gap,
      ink_height_pt: ascender + descender,
      ..VerticalMetrics::default()
    },
    decoration: DecorationMetrics {
      underline_offset_pt: metrics
        .underline
        .map(|metrics| -metrics.offset)
        .unwrap_or_default(),
      underline_thickness_pt: metrics
        .underline
        .map(|metrics| metrics.thickness.abs())
        .unwrap_or_default(),
      strikeout_offset_pt: metrics
        .strikeout
        .map(|metrics| metrics.offset)
        .unwrap_or_default(),
      strikeout_thickness_pt: metrics
        .strikeout
        .map(|metrics| metrics.thickness.abs())
        .unwrap_or_default(),
    },
    script,
    em_size,
  }
}

fn directwrite_default_baseline_offset(
  hhea_ascent: f32,
  hhea_descent: f32,
  hhea_line_gap: f32,
  windows_ascent: f32,
  windows_descent: f32,
) -> f32 {
  let hhea_height = hhea_ascent + hhea_descent + hhea_line_gap;
  let windows_height = windows_ascent + windows_descent;
  windows_ascent + (hhea_height - windows_height).max(0.0)
}

fn font_coverage_from_skrifa(face: &SkrifaFontRef<'_>) -> FontCoverage {
  let mut ranges = Vec::new();
  let mut previous = None;
  for (codepoint, glyph_id) in face.charmap().mappings() {
    debug_assert!(previous.is_none_or(|previous| previous <= codepoint));
    previous = Some(codepoint);
    if glyph_id != SkrifaGlyphId::NOTDEF && char::from_u32(codepoint).is_some() {
      push_coverage_codepoint(&mut ranges, codepoint);
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

fn font_embedding_policy_from_skrifa(face: &SkrifaFontRef<'_>) -> FontEmbeddingPolicy {
  enum Permission {
    Installable,
    Restricted,
    PreviewAndPrint,
    Editable,
  }

  let permission = face.os2().ok().and_then(|os2| {
    let bits = os2.fs_type() & 0x000F;
    if os2.version() <= 2 {
      Some(if bits == 0 {
        Permission::Installable
      } else if bits & 0x0008 != 0 {
        Permission::Editable
      } else if bits & 0x0004 != 0 {
        Permission::PreviewAndPrint
      } else {
        Permission::Restricted
      })
    } else {
      match bits {
        0 => Some(Permission::Installable),
        2 => Some(Permission::Restricted),
        4 => Some(Permission::PreviewAndPrint),
        8 => Some(Permission::Editable),
        _ => None,
      }
    }
  });
  match permission {
    Some(Permission::Restricted) => FontEmbeddingPolicy {
      subset_policy: FontSubsetPolicy::DoNotEmbed,
      installable: false,
      restricted: true,
    },
    Some(Permission::Installable) | None => FontEmbeddingPolicy::default(),
    Some(Permission::PreviewAndPrint) => FontEmbeddingPolicy {
      subset_policy: FontSubsetPolicy::Subset,
      installable: false,
      restricted: false,
    },
    Some(Permission::Editable) => FontEmbeddingPolicy {
      subset_policy: FontSubsetPolicy::EmbedFull,
      installable: false,
      restricted: false,
    },
  }
}

fn font_embedding_plan_from_skrifa<'a>(face: &SkrifaFontRef<'_>) -> FontEmbeddingPlan<'a> {
  FontEmbeddingPlan {
    keep_tables: DEFAULT_PDF_EMBED_TABLES
      .iter()
      .map(|table| Cow::Borrowed(*table))
      .collect(),
    downgrade_cff2: face.cff2().is_ok(),
    desubroutinize_cff: face.cff().is_ok() || face.cff2().is_ok(),
    pin_variation_axes: !face.axes().is_empty(),
  }
}

fn has_table(face: &SkrifaFontRef<'_>, tag: &[u8; 4]) -> bool {
  face.table_data(skrifa::Tag::new(tag)).is_some()
}

fn font_bounds_from_skrifa(face: &SkrifaFontRef<'_>) -> FontBounds {
  FontBounds {
    global: face
      .metrics(SkrifaSize::new(1.0), SkrifaLocationRef::default())
      .bounds
      .map(glyph_bounds_from_skrifa),
  }
}

fn glyph_bounds_from_skrifa(bounds: SkrifaBoundingBox) -> GlyphBounds {
  GlyphBounds {
    x_min_pt: bounds.x_min,
    y_min_pt: bounds.y_min,
    x_max_pt: bounds.x_max,
    y_max_pt: bounds.y_max,
  }
}

fn variation_axes_from_skrifa<'a>(face: &SkrifaFontRef<'_>) -> Vec<VariationAxis<'a>> {
  face
    .axes()
    .iter()
    .filter(|axis| !axis.is_hidden())
    .map(|axis| VariationAxis {
      tag: Cow::Owned(tag_to_string(axis.tag())),
      name: skrifa_name_by_id(face, axis.name_id()).map(Cow::Owned),
      min: axis.min_value(),
      default: axis.default_value(),
      max: axis.max_value(),
    })
    .collect()
}

fn opentype_features_from_skrifa<'a>(face: &SkrifaFontRef<'_>) -> Vec<OpenTypeFeature<'a>> {
  let mut features = Vec::new();
  if let Ok(gsub) = face.gsub()
    && let Ok(feature_list) = gsub.feature_list()
  {
    for feature in feature_list.feature_records() {
      push_opentype_feature(&mut features, feature.feature_tag());
    }
  }
  if let Ok(gpos) = face.gpos()
    && let Ok(feature_list) = gpos.feature_list()
  {
    for feature in feature_list.feature_records() {
      push_opentype_feature(&mut features, feature.feature_tag());
    }
  }
  if let Ok(feat) = face.feat() {
    for feature in feat.names() {
      push_named_feature(&mut features, format!("aat:{}", feature.feature()));
    }
  }
  if face.morx().is_ok() {
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

fn push_opentype_feature<'a>(features: &mut Vec<OpenTypeFeature<'a>>, tag: skrifa::Tag) {
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

fn skrifa_name_by_id(face: &SkrifaFontRef<'_>, name_id: StringId) -> Option<String> {
  face
    .localized_strings(name_id)
    .english_or_first()
    .map(|name| name.to_string())
}

fn tag_to_string(tag: skrifa::Tag) -> String {
  String::from_utf8_lossy(&tag.to_be_bytes())
    .trim_end()
    .to_string()
}

fn first_strong_text_script(text: &str, options: ScriptScanOptions) -> Option<TextScript> {
  text.chars().find_map(|ch| {
    (!is_nonspacing_mark(ch))
      .then(|| strong_text_script(ch, options))
      .flatten()
  })
}

fn strong_text_script(ch: char, options: ScriptScanOptions) -> Option<TextScript> {
  // ECMA-376 Part 1 §17.3.2.26 assigns Basic Latin text to the ASCII font
  // slot. Unicode Script marks digits and punctuation as Common; leaving
  // printable characters weak would incorrectly attach a trailing
  // chart/shape number to a preceding East Asian run instead of selecting the
  // Latin face. Layout controls and spaces remain weak so they stay with an
  // adjacent painted run and do not create empty PDF font subsets.
  if options.wordprocessingml_font_slots && matches!(ch as u32, 0x0021..=0x007E) {
    return Some(TextScript::Latin);
  }
  // ECMA-376 Part 1 §17.3.2.26 assigns the Latin-1 Supplement to the High
  // ANSI font slot unless w:hint=eastAsia activates one of its enumerated
  // exceptions. Treat its Common punctuation as Latin for the default case;
  // Unicode Script alone would incorrectly attach a trailing guillemet to a
  // preceding Han run.
  if options.wordprocessingml_font_slots && matches!(ch as u32, 0x00A0..=0x00FF) {
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

fn wordprocessing_font_slot(ch: char, options: ScriptScanOptions) -> WordprocessingFontSlot {
  use WordprocessingFontSlot::{Ascii, ComplexScript, HighAnsi};

  // [MS-OI29500] section 2.1.88 documents that either run-level property
  // forces the cs face for every Unicode value. Word fixed output has one
  // narrower compatibility exception: U+0030..U+0039 retain the ASCII family.
  // Keep this at the font-slot boundary; szCs/bCs/iCs still apply to the whole
  // run in the layout layer.
  if options.wordprocessingml_complex_font_override {
    return if ch.is_ascii_digit() {
      Ascii
    } else {
      ComplexScript
    };
  }

  // ST_Hint resolves otherwise ambiguous glyphs, but `eastAsia` is already
  // part of the code-point table below. In particular, ECMA-376 Part 1
  // §17.3.2.26 and [MS-OI29500] §2.1.88 classify all of Basic Latin,
  // including U+0020, as ASCII. Applying the East Asian hint before that
  // table would incorrectly move ordinary spaces to the East Asian face.
  // The `ascii` extension and the standard `cs` hint remain relevant only to
  // characters whose slot is otherwise ambiguous.
  if wordprocessing_ambiguous_character(ch) {
    match options.wordprocessingml_font_hint {
      Some(WordprocessingFontTypeHint::Ascii) => return Ascii,
      Some(WordprocessingFontTypeHint::ComplexScript) => return ComplexScript,
      Some(WordprocessingFontTypeHint::Default | WordprocessingFontTypeHint::EastAsia) | None => {}
    }
  }

  let code = ch as u32;
  let east_asia_hint = matches!(
    options.wordprocessingml_font_hint,
    Some(WordprocessingFontTypeHint::EastAsia)
  );
  let chinese = options.wordprocessingml_east_asia_language_is_chinese;
  let chinese_charset = matches!(
    options.wordprocessingml_east_asia_font_charset,
    Some(FontCharset::Gb2312 | FontCharset::ChineseBig5)
  );
  let east_asia = || wordprocessing_east_asia_slot(options);

  match code {
    // Explicit Word table rows.
    0x0000..=0x007F => Ascii,
    0x00A1
    | 0x00A4
    | 0x00A7..=0x00A8
    | 0x00AA
    | 0x00AD
    | 0x00AF
    | 0x00B0..=0x00B4
    | 0x00B6..=0x00BA
    | 0x00BC..=0x00BF
    | 0x00D7
    | 0x00F7
      if east_asia_hint =>
    {
      east_asia()
    }
    0x00E0..=0x00E1
    | 0x00E8..=0x00EA
    | 0x00EC..=0x00ED
    | 0x00F2..=0x00F3
    | 0x00F9..=0x00FA
    | 0x00FC
      if east_asia_hint && chinese =>
    {
      east_asia()
    }
    0x00A0..=0x02AF => {
      if east_asia_hint && code >= 0x0100 && (chinese || chinese_charset) {
        east_asia()
      } else {
        HighAnsi
      }
    }
    0x02B0..=0x04FF | 0x2000..=0x27BF => {
      if east_asia_hint {
        east_asia()
      } else {
        HighAnsi
      }
    }
    0x0590..=0x07BF => Ascii,
    0x1100..=0x11FF => east_asia(),
    0x1E00..=0x1EFF => {
      if east_asia_hint && chinese {
        east_asia()
      } else {
        HighAnsi
      }
    }
    0x2E80..=0x2EFF => {
      if east_asia_hint {
        east_asia()
      } else {
        HighAnsi
      }
    }
    0x2F00..=0x2FDF
    | 0x2FF0..=0x4DBF
    | 0x4E00..=0x9FAF
    | 0xA000..=0xA4CF
    | 0xAC00..=0xD7AF
    | 0xF900..=0xFAFF
    | 0xFE30..=0xFE6F
    | 0xFF00..=0xFFEF => east_asia(),
    // Word applies the UTF-16 surrogate table rows to supplementary scalar
    // values. Rust exposes the decoded scalar, so retain that effective slot.
    0x10000..=0x10FFFF => east_asia(),
    0xE000..=0xF8FF | 0xFB00..=0xFB1C if east_asia_hint => east_asia(),
    0xFB1D..=0xFDFF | 0xFE70..=0xFEFE => Ascii,
    // [MS-OI29500] explicitly assigns all unlisted ranges to hAnsi.
    _ => HighAnsi,
  }
}

fn wordprocessing_east_asia_slot(options: ScriptScanOptions) -> WordprocessingFontSlot {
  if options.wordprocessingml_east_asia_uses_ascii {
    WordprocessingFontSlot::Ascii
  } else {
    WordprocessingFontSlot::EastAsia
  }
}

fn wordprocessing_ambiguous_character(ch: char) -> bool {
  // Basic Latin has an unconditional ASCII classification in both the
  // standard and Word's documented implementation. Only Common/Inherited
  // characters outside that range can require the `ascii`/`cs` hint path;
  // `eastAsia` continues through the complete block table above.
  !matches!(ch as u32, 0x0000..=0x007F)
    && matches!(
      ch.script(),
      UnicodeScriptValue::Common | UnicodeScriptValue::Inherited
    )
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

fn font_weight_from_opentype(weight: u16) -> FontWeight {
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

fn font_slant_from_skrifa(style: SkrifaStyle) -> FontSlant {
  match style {
    SkrifaStyle::Italic => FontSlant::Italic,
    SkrifaStyle::Oblique(_) => FontSlant::Oblique,
    SkrifaStyle::Normal => FontSlant::Upright,
  }
}

fn font_stretch_from_opentype(width: u16) -> FontStretch {
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

fn harf_script_for_shape_options(options: &ShapeOptions<'_>) -> Option<HarfScript> {
  // OpenType MATH specifies the `math` OpenType script tag, with only its
  // default language system, for math-engine features. HarfRust represents
  // that tag through ISO 15924 Zmth (`script::MATH`). Unicode script scanning
  // still owns font-slot selection, but an explicit math-only feature must
  // shape through the font's math Script table rather than Latn/Greek/DFLT.
  if options
    .features
    .iter()
    .any(|feature| matches!(feature.tag.as_ref(), "ssty" | "flac" | "dtls"))
  {
    Some(script::MATH)
  } else {
    options.script.and_then(harf_script)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn platform_has_font(family: &str, postscript_name: &str) -> bool {
    let request = FontRequest {
      family: Some(Cow::Borrowed(family)),
      ..FontRequest::default()
    };
    platform_system_query_fonts(&PlatformFontQueryFamily::Name(family.to_string()), &request)
      .iter()
      .any(|font| {
        FontFaceInfo::from_ttf_bytes("platform-font-probe", &font.data, font.face_index)
          .ok()
          .and_then(|face| face.postscript_name)
          .is_some_and(|name| name == postscript_name)
      })
  }

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
  fn default_office_policy_keeps_legacy_sungti_on_a_simplified_chinese_serif() {
    let mut registry = FontRegistry::with_default_policy();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("noto-serif-cjk-sc", "Noto Serif CJK SC"),
    );

    let request = FontRequest {
      family: Some(Cow::Borrowed("AR PL SungtiL GB")),
      script: Some(TextScript::Han),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();

    assert_eq!(resolved.font_id, FontId(Arc::from("noto-serif-cjk-sc")));
    assert_eq!(resolved.resolved_family, Cow::Borrowed("Noto Serif CJK SC"));
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
  fn platform_font_query_cache_stays_bounded() {
    let mut cache = PlatformFontQueryCache::default();
    for index in 0..=PLATFORM_FONT_QUERY_CACHE_ENTRIES {
      cache.slot(PlatformFontQueryKey {
        family: PlatformFontQueryFamily::Name(format!("Fixture {index}")),
        weight: FontWeight::Normal,
        slant: FontSlant::Upright,
        stretch: FontStretch::Normal,
      });
    }

    assert_eq!(cache.slots.len(), PLATFORM_FONT_QUERY_CACHE_ENTRIES);
    assert!(!cache.slots.contains_key(&PlatformFontQueryKey {
      family: PlatformFontQueryFamily::Name("Fixture 0".to_string()),
      weight: FontWeight::Normal,
      slant: FontSlant::Upright,
      stretch: FontStretch::Normal,
    }));
  }

  #[test]
  fn unspecified_script_does_not_preload_script_specific_fallbacks() {
    let registry = FontRegistry::with_default_policy();
    let request = FontRequest::default();
    let families = registry.fallback_families(&request);

    assert!(!families.contains(&"Amiri"));
    assert!(!families.contains(&"Malgun Gothic"));
    assert!(!families.contains(&"Cambria Math"));

    let arabic = FontRequest {
      script: Some(TextScript::Arabic),
      ..FontRequest::default()
    };
    assert!(registry.fallback_families(&arabic).contains(&"Amiri"));
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
  fn roman_family_class_queries_the_windows_representative_before_host_generics() {
    let mut queries = SmallVec::new();

    push_platform_generic_queries(&mut queries, Some(FontFamilyClass::Serif));

    assert_eq!(
      queries.as_slice(),
      &[
        PlatformFontQueryFamily::Name("Times New Roman".to_string()),
        PlatformFontQueryFamily::Serif,
        PlatformFontQueryFamily::SansSerif,
      ]
    );
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
    if !platform_has_font("Calibri Light", "Calibri-Light") {
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
  fn system_query_keeps_installed_calibri_light_for_bold_request() {
    if !platform_has_font("Calibri Light", "Calibri-Light") {
      return;
    }
    let mut registry = FontRegistry::with_default_policy();
    let request = FontRequest {
      family: Some(Cow::Borrowed("Calibri Light")),
      bold: true,
      ..FontRequest::default()
    };

    registry.register_system_query_fonts(&request).unwrap();
    let resolved = registry.resolve_with_diagnostics(&request).unwrap();

    assert!(
      resolved.font_id.0.contains("Calibri-Light"),
      "resolved={}",
      resolved.font_id.0
    );
    assert!(resolved.synthetic_bold);
  }

  #[test]
  fn system_query_prefers_installed_aptos_face_over_display_alias() {
    if !platform_has_font("Aptos", "Aptos") {
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
    if !platform_has_font("Noto Sans", "NotoSans-Regular") {
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
        directwrite_baseline_offset_pt: 1.25,
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
    assert_eq!(metrics.vertical.directwrite_baseline_offset_pt, 15.0);
  }

  #[test]
  fn directwrite_default_baseline_preserves_hhea_height_with_windows_extents() {
    // Calibri Bold's hhea box totals 2500 units, exactly matching its
    // 1950/550 Windows alignment box, so DirectWrite adds no line gap.
    assert_eq!(
      directwrite_default_baseline_offset(1536.0, 512.0, 452.0, 1950.0, 550.0),
      1950.0
    );

    // A smaller Windows clipping box keeps the hhea total by placing the
    // residual above the baseline, matching DirectWrite/Wine line metrics.
    assert_eq!(
      directwrite_default_baseline_offset(1600.0, 400.0, 400.0, 1800.0, 400.0),
      2000.0
    );
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
      substitution: None,
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
  fn missing_family_substitution_is_separate_from_glyph_fallback() {
    // LO PhysicalFontCollection.cxx selects a primary family before
    // GetGlyphFallbackFont(); Skia and ReactOS keep the same boundary between
    // family matching and character/font-link fallback.
    let mut registry = FontRegistry::new();
    let mut text = FontFaceInfo::synthetic("text", "Text Face");
    text.coverage.unicode_ranges = std::iter::once(u32::from('A')..u32::from('A') + 1).collect();
    registry.register_face(FontSource::System, text);
    let mut symbol = FontFaceInfo::synthetic("symbol", "Symbol Face");
    symbol.coverage.unicode_ranges =
      std::iter::once(u32::from('\u{2610}')..u32::from('\u{2610}') + 1).collect();
    registry.register_face(FontSource::System, symbol);
    registry
      .book
      .family_substitution_chains
      .push(FontFallbackChain {
        requested_family: None,
        script: None,
        language: None,
        families: vec![Cow::Borrowed("Text Face")],
      });
    registry.book.fallback_chains.push(FontFallbackChain {
      requested_family: None,
      script: None,
      language: None,
      families: vec![Cow::Borrowed("Symbol Face")],
    });

    let request = FontRequest {
      family: Some(Cow::Borrowed("Unavailable Face")),
      script: Some(TextScript::Common),
      size_pt: FontSize(12.0),
      ..FontRequest::default()
    };
    let resolved = registry.resolve(&request).unwrap();
    assert_eq!(resolved.font_id, FontId(Arc::from("text")));
    assert_eq!(resolved.match_diagnostics.fallback_level, None);
    assert_eq!(
      resolved.substitution,
      Some(FontSubstitution {
        requested_family: Cow::Owned("Unavailable Face".to_string()),
        substituted_family: Cow::Borrowed("Text Face"),
        reason: FontSubstitutionReason::LastResort,
      })
    );

    let runs = registry
      .shape_text_runs(&request, "A\u{2610}", TextDirection::LeftToRight)
      .unwrap();
    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].font_id, FontId(Arc::from("text")));
    assert_eq!(runs[1].font_id, FontId(Arc::from("symbol")));
    assert_eq!(
      runs[1].diagnostics.fallback_runs[0].reason,
      FontSubstitutionReason::MissingGlyph
    );
  }

  #[test]
  fn requested_family_substitution_precedes_last_resort() {
    let mut registry = FontRegistry::new();
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("specific", "Specific Substitute"),
    );
    registry.register_face(
      FontSource::System,
      FontFaceInfo::synthetic("last-resort", "Last Resort"),
    );
    registry.book.family_substitution_chains.extend([
      FontFallbackChain {
        requested_family: Some(Cow::Borrowed("Missing Face")),
        script: None,
        language: None,
        families: vec![Cow::Borrowed("Specific Substitute")],
      },
      FontFallbackChain {
        requested_family: None,
        script: None,
        language: None,
        families: vec![Cow::Borrowed("Last Resort")],
      },
    ]);

    let resolved = registry
      .resolve(&FontRequest {
        family: Some(Cow::Borrowed("Missing Face")),
        ..FontRequest::default()
      })
      .unwrap();
    assert_eq!(resolved.font_id, FontId(Arc::from("specific")));
    assert_eq!(
      resolved.substitution.as_ref().map(|item| item.reason),
      Some(FontSubstitutionReason::MissingFamily)
    );
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

    assert_eq!(runs.len(), 4);
    assert_eq!(runs[0].script, TextScript::Latin);
    assert_eq!(&"Junzha«问候语»"[runs[0].text_range.clone()], "Junzha");
    assert_eq!(
      runs[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );
    assert_eq!(runs[1].script, TextScript::Latin);
    assert_eq!(&"Junzha«问候语»"[runs[1].text_range.clone()], "«");
    assert_eq!(
      runs[1].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::HighAnsi)
    );
    assert_eq!(runs[2].script, TextScript::Han);
    assert_eq!(&"Junzha«问候语»"[runs[2].text_range.clone()], "问候语");
    assert_eq!(
      runs[2].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::EastAsia)
    );
    assert_eq!(runs[3].script, TextScript::Latin);
    assert_eq!(&"Junzha«问候语»"[runs[3].text_range.clone()], "»");
    assert_eq!(
      runs[3].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::HighAnsi)
    );
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
  fn wordprocessingml_east_asia_hint_routes_weak_text_without_changing_scripts() {
    let text = "Before “水” After";
    let runs = script_direction_runs_with_options(
      text,
      FontSize(11.0),
      ScriptScanOptions {
        wordprocessingml_font_slots: true,
        wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
        ..ScriptScanOptions::default()
      },
    );

    assert_eq!(runs.len(), 3);
    assert_eq!(&text[runs[0].text_range.clone()], "Before ");
    assert_eq!(runs[0].script, TextScript::Latin);
    assert_eq!(
      runs[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );
    assert_eq!(&text[runs[1].text_range.clone()], "“水”");
    assert_eq!(runs[1].script, TextScript::Han);
    assert_eq!(
      runs[1].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::EastAsia)
    );
    assert_eq!(&text[runs[2].text_range.clone()], " After");
    assert_eq!(runs[2].script, TextScript::Latin);
    assert_eq!(
      runs[2].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );
  }

  #[test]
  fn wordprocessingml_east_asia_hint_preserves_code_point_table_for_spaces() {
    let options = ScriptScanOptions {
      wordprocessingml_font_slots: true,
      wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
      ..ScriptScanOptions::default()
    };
    let text = "1    ";
    let runs = script_direction_runs_with_options(text, FontSize(11.0), options);

    // ECMA-376 Part 1 §17.3.2.26 and [MS-OI29500] §2.1.88 assign the
    // complete Basic Latin range, including stored leading/trailing spaces,
    // to the ASCII slot even when w:hint="eastAsia".
    assert_eq!(runs.len(), 1);
    assert_eq!(&text[runs[0].text_range.clone()], text);
    assert_eq!(
      runs[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );

    let text = " (ctrl + I)";
    let runs = script_direction_runs_with_options(text, FontSize(11.0), options);
    assert_eq!(runs.len(), 1);
    assert_eq!(&text[runs[0].text_range.clone()], text);
    assert_eq!(
      runs[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );

    // U+00A0 is High ANSI with no eastAsia exception, while U+2026 belongs
    // to General Punctuation and therefore follows the eastAsia hint. Pin
    // both sides so whitespace heuristics cannot replace the block table.
    let text = "A\u{00a0}A\u{2026}";
    let runs = script_direction_runs_with_options(text, FontSize(11.0), options);
    assert_eq!(runs.len(), 4);
    assert_eq!(&text[runs[0].text_range.clone()], "A");
    assert_eq!(
      runs[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );
    assert_eq!(&text[runs[1].text_range.clone()], "\u{00a0}");
    assert_eq!(
      runs[1].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::HighAnsi)
    );
    assert_eq!(&text[runs[2].text_range.clone()], "A");
    assert_eq!(
      runs[2].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );
    assert_eq!(&text[runs[3].text_range.clone()], "\u{2026}");
    assert_eq!(
      runs[3].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::EastAsia)
    );
  }

  #[test]
  fn wordprocessingml_font_slots_follow_word_ranges_and_preserve_unicode_script() {
    let scan = |text, options| {
      script_direction_runs_with_options(text, FontSize(11.0), options)
        .into_iter()
        .next()
        .expect("font run")
    };
    let base = ScriptScanOptions {
      wordprocessingml_font_slots: true,
      ..ScriptScanOptions::default()
    };

    let latin1 = scan("é", base);
    assert_eq!(latin1.script, TextScript::Latin);
    assert_eq!(
      latin1.wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::HighAnsi)
    );

    let arabic = scan("ع", base);
    assert_eq!(arabic.script, TextScript::Arabic);
    assert_eq!(
      arabic.wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );

    let greek = scan(
      "α",
      ScriptScanOptions {
        wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
        ..base
      },
    );
    assert_eq!(greek.script, TextScript::Greek);
    assert_eq!(
      greek.wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::EastAsia)
    );

    let latin_extended = scan(
      "Ā",
      ScriptScanOptions {
        wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
        wordprocessingml_east_asia_font_charset: Some(FontCharset::Gb2312),
        ..base
      },
    );
    assert_eq!(latin_extended.script, TextScript::Latin);
    assert_eq!(
      latin_extended.wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::EastAsia)
    );
  }

  #[test]
  fn wordprocessingml_run_overrides_precede_the_unicode_font_table() {
    let complex = script_direction_runs_with_options(
      "A水",
      FontSize(11.0),
      ScriptScanOptions {
        wordprocessingml_font_slots: true,
        wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
        wordprocessingml_complex_font_override: true,
        ..ScriptScanOptions::default()
      },
    );
    assert!(complex.iter().all(|run| {
      run.wordprocessingml_font_slot == Some(WordprocessingFontSlot::ComplexScript)
    }));

    // Word fixed output keeps only Basic Latin decimal digits on the ASCII
    // family. The adjacent Latin and CJK counterexamples above remain on cs.
    let decimal_digit = script_direction_runs_with_options(
      "0",
      FontSize(11.0),
      ScriptScanOptions {
        wordprocessingml_font_slots: true,
        wordprocessingml_complex_font_override: true,
        ..ScriptScanOptions::default()
      },
    );
    assert_eq!(
      decimal_digit[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );

    let east_asia_as_ascii = script_direction_runs_with_options(
      "水",
      FontSize(11.0),
      ScriptScanOptions {
        wordprocessingml_font_slots: true,
        wordprocessingml_east_asia_uses_ascii: true,
        ..ScriptScanOptions::default()
      },
    );
    assert_eq!(
      east_asia_as_ascii[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );
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

  #[test]
  fn math_engine_features_select_the_opentype_math_script_system() {
    let ordinary = ShapeOptions {
      script: Some(TextScript::Latin),
      ..ShapeOptions::default()
    };
    assert_eq!(
      harf_script_for_shape_options(&ordinary),
      Some(script::LATIN)
    );

    for tag in ["ssty", "flac", "dtls"] {
      let math = ShapeOptions {
        script: Some(TextScript::Latin),
        features: vec![FeatureValue {
          tag: Cow::Borrowed(tag),
          value: 1,
        }],
        ..ShapeOptions::default()
      };
      assert_eq!(harf_script_for_shape_options(&math), Some(script::MATH));
    }
  }
}
