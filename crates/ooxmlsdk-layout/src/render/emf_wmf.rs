pub use emfsdk::render::{
  DecodedMetafile, EmfTextAdvanceQuantization, MetafileBitmapLayer, MetafileBitmapPicture,
  MetafileImageBounds, MetafilePhysicalSize, MetafileSolidRect, MetafileVectorDraw,
  MetafileVectorDrawing, MetafileVectorFill, MetafileVectorFillRule, MetafileVectorLineCap,
  MetafileVectorPoint, MetafileVectorPrefix, MetafileVectorScene, RenderOptions,
  WmfConversionProfile, WmfExternalHeader, WmfSystemFontProfile,
};

#[derive(Clone, Debug)]
pub struct MetafileTextRun {
  pub text: String,
  pub x: f32,
  pub y: f32,
  pub rotation_degrees: f32,
  pub font_size: Option<f32>,
  pub font_family: Option<String>,
  pub bold: bool,
  pub italic: bool,
  pub color: [u8; 3],
  pub clip: Option<[f32; 4]>,
  pub width: Option<f32>,
  pub advances: Option<Vec<f32>>,
}

pub fn decode_metafile_as_raster(
  data: &[u8],
  content_type: Option<&str>,
) -> Result<Option<DecodedMetafile>, String> {
  emfsdk::render::decode_metafile_as_raster(data, content_type).map_err(|err| err.to_string())
}

pub fn decode_metafile_as_raster_with_options(
  data: &[u8],
  content_type: Option<&str>,
  options: RenderOptions,
) -> Result<Option<DecodedMetafile>, String> {
  emfsdk::render::decode_metafile_as_raster_with_options(data, content_type, options)
    .map_err(|err| err.to_string())
}

pub fn extract_metafile_vector_scene(
  data: &[u8],
  content_type: Option<&str>,
) -> Result<Option<MetafileVectorScene>, String> {
  emfsdk::render::extract_metafile_vector_scene(data, content_type).map_err(|err| err.to_string())
}

pub fn extract_metafile_vector_scene_with_options(
  data: &[u8],
  content_type: Option<&str>,
  options: RenderOptions,
) -> Result<Option<MetafileVectorScene>, String> {
  emfsdk::render::extract_metafile_vector_scene_with_options(data, content_type, options)
    .map_err(|err| err.to_string())
}

pub fn extract_metafile_vector_drawing_with_options(
  data: &[u8],
  content_type: Option<&str>,
  options: RenderOptions,
) -> Result<Option<MetafileVectorDrawing>, String> {
  emfsdk::render::extract_metafile_vector_drawing_with_options(data, content_type, options)
    .map_err(|err| err.to_string())
}

pub fn extract_metafile_vector_prefix_with_options(
  data: &[u8],
  content_type: Option<&str>,
  options: RenderOptions,
) -> Result<Option<MetafileVectorPrefix>, String> {
  emfsdk::render::extract_metafile_vector_prefix_with_options(data, content_type, options)
    .map_err(|err| err.to_string())
}

pub fn metafile_physical_size(
  data: &[u8],
  content_type: Option<&str>,
) -> Option<MetafilePhysicalSize> {
  emfsdk::render::metafile_physical_size(data, content_type)
}

pub fn metafile_text_frame_size(data: &[u8], content_type: Option<&str>) -> Option<[u32; 2]> {
  emfsdk::render::metafile_text_frame_size(data, content_type)
}

pub fn metafile_image_bounds(data: &[u8]) -> Option<MetafileImageBounds> {
  emfsdk::render::metafile_image_bounds(data)
}

pub fn extract_metafile_text_runs(
  data: &[u8],
  content_type: Option<&str>,
  include_raster_backdrop: bool,
) -> Vec<MetafileTextRun> {
  extract_metafile_text_runs_with_options(
    data,
    content_type,
    include_raster_backdrop,
    RenderOptions::default(),
  )
}

pub fn extract_metafile_text_runs_with_options(
  data: &[u8],
  content_type: Option<&str>,
  include_raster_backdrop: bool,
  options: RenderOptions,
) -> Vec<MetafileTextRun> {
  emfsdk::render::extract_metafile_text_runs_with_options(data, content_type, options)
    .into_iter()
    // [MS-EMF] defines ternary ROPs as bitwise combinations of source,
    // pattern, and the existing destination. LibreOffice's MtfTools uses the
    // same truth-table dependency test, and Cairo's PDF backend sends
    // destination-dependent operators to raster fallback. Office likewise
    // emits the SRCAND/SRCINVERT OLE previews in tdf135653, tdf91122, and
    // Apache POI WithEmbeded.xlsx as one image, while the AlphaBlend
    // tdf133035 counterexample retains its following text as PDF text.
    .filter(|run| include_raster_backdrop || !run.requires_raster_backdrop)
    .map(|run| {
      let text = metafile_semantic_text(run.text, run.font_family.as_deref());
      MetafileTextRun {
        text,
        x: run.x,
        y: run.y,
        rotation_degrees: run.rotation_degrees,
        font_size: run.font_size,
        font_family: run.font_family,
        bold: run.bold,
        italic: run.italic,
        color: run.color,
        clip: run.clip,
        width: run.width,
        advances: run.advances,
      }
    })
    .collect()
}

/// Whether visible metafile text depends on an earlier destination-reading
/// raster operation.
///
/// OLE replacement previews commonly encode a transparent icon as the
/// canonical `SRCAND`/`SRCINVERT` pair and then draw its label into the same
/// destination DC.  Such a preview has to preserve the unpainted destination
/// when it is flattened; an ordinary EMF whose text follows `AlphaBlend` is
/// the counterexample and remains independent of a raster backdrop.
pub fn metafile_text_requires_raster_backdrop(data: &[u8], content_type: Option<&str>) -> bool {
  emfsdk::render::extract_metafile_text_runs(data, content_type)
    .iter()
    .any(|run| run.requires_raster_backdrop)
}

pub fn metafile_text_can_be_lifted(data: &[u8], content_type: Option<&str>) -> bool {
  emfsdk::render::metafile_text_can_be_lifted(data, content_type)
}

pub fn word_metafile_text_can_be_lifted(data: &[u8], content_type: Option<&str>) -> bool {
  emfsdk::render::metafile_text_can_be_lifted_with_options(
    data,
    content_type,
    RenderOptions {
      word_emf_vector_pens: true,
      word_emf_logical_font_metrics: true,
      ..RenderOptions::default()
    },
  )
}

pub fn extract_metafile_solid_rects(
  data: &[u8],
  content_type: Option<&str>,
) -> Vec<MetafileSolidRect> {
  emfsdk::render::extract_metafile_solid_rects(data, content_type)
}

pub fn extract_metafile_solid_rects_with_options(
  data: &[u8],
  content_type: Option<&str>,
  options: RenderOptions,
) -> Vec<MetafileSolidRect> {
  emfsdk::render::extract_metafile_solid_rects_with_options(data, content_type, options)
}

pub fn extract_metafile_bitmap_layers(
  data: &[u8],
  content_type: Option<&str>,
) -> Vec<MetafileBitmapLayer> {
  emfsdk::render::extract_metafile_bitmap_layers(data, content_type)
}

pub fn extract_metafile_bitmap_layers_with_options(
  data: &[u8],
  content_type: Option<&str>,
  options: RenderOptions,
) -> Vec<MetafileBitmapLayer> {
  emfsdk::render::extract_metafile_bitmap_layers_with_options(data, content_type, options)
}

pub fn extract_metafile_bitmap_picture_with_options(
  data: &[u8],
  options: RenderOptions,
) -> Option<MetafileBitmapPicture> {
  emfsdk::render::extract_metafile_bitmap_picture_with_options(data, options)
}

pub fn extract_metafile_full_bitmap_with_options(
  data: &[u8],
  options: RenderOptions,
) -> Option<MetafileBitmapLayer> {
  emfsdk::render::extract_metafile_full_bitmap_with_options(data, options)
}

fn metafile_semantic_text(text: String, font_family: Option<&str>) -> String {
  if font_family.is_some_and(|family| {
    family.eq_ignore_ascii_case("Symbol") || family.eq_ignore_ascii_case("SymbolMT")
  }) {
    // Preserve legacy selectors through glyph selection. Word's WMF output
    // keeps F03C/F03E and digit selectors, while its sparse printable aliases
    // are resolved by the PDF semantic layer after shaping. Converting every
    // ASCII-looking glyph here loses that distinction and the original code.
    // The extracted run already owns this string, so retain its allocation.
    return text;
  }
  text
    .chars()
    .map(|character| {
      let mapped =
        super::symbol::font_symbol_code(font_family, character as u32).unwrap_or(character);
      // Retain the existing normalization for other legacy font families.
      // Only the verified physical Symbol path above delegates all selector
      // semantics to the PDF layer after shaping.
      if mapped.is_ascii() { mapped } else { character }
    })
    .collect()
}

pub fn supports_semantic_text(content_type: Option<&str>) -> bool {
  content_type.is_some_and(|content_type| {
    matches!(
      content_type.to_ascii_lowercase().as_str(),
      "image/emf"
        | "image/x-emf"
        | "application/emf"
        | "application/x-emf"
        | "image/wmf"
        | "image/x-wmf"
        | "application/wmf"
        | "application/x-wmf"
    )
  })
}

#[cfg(test)]
mod tests {
  use super::{metafile_semantic_text, supports_semantic_text};

  #[test]
  fn semantic_text_is_supported_only_for_vector_metafiles() {
    assert!(supports_semantic_text(Some("image/x-emf")));
    assert!(supports_semantic_text(Some("APPLICATION/WMF")));
    assert!(!supports_semantic_text(Some("image/png")));
    assert!(!supports_semantic_text(None));
  }

  #[test]
  fn symbol_selectors_reach_glyph_selection_before_pdf_semantics() {
    let source = "\u{f02b}\u{f03d}\u{f077}\u{f02d}\u{f03c}\u{f03e}\u{f030}\u{f031}";
    for family in ["Symbol", "SymbolMT", "sYmBoL"] {
      assert_eq!(
        metafile_semantic_text(source.to_owned(), Some(family)),
        source
      );
    }
    assert_eq!(
      metafile_semantic_text("<>01".to_owned(), Some("Arial")),
      "<>01"
    );
    assert_eq!(
      metafile_semantic_text("\u{f0e8}".to_owned(), Some("Wingdings")),
      "\u{f0e8}"
    );
  }
}
