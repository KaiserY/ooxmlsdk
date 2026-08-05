pub use emfsdk::render::{
  DecodedMetafile, MetafileBitmapLayer, MetafilePhysicalSize, MetafileSolidRect, RenderOptions,
  WmfExternalHeader,
};

#[derive(Clone, Debug)]
pub struct MetafileTextRun {
  pub text: String,
  pub x: f32,
  pub y: f32,
  pub font_size: Option<f32>,
  pub font_family: Option<String>,
  pub bold: bool,
  pub italic: bool,
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

pub fn metafile_physical_size(
  data: &[u8],
  content_type: Option<&str>,
) -> Option<MetafilePhysicalSize> {
  emfsdk::render::metafile_physical_size(data, content_type)
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
      let text = metafile_semantic_text(&run.text, run.font_family.as_deref());
      MetafileTextRun {
        text,
        x: run.x,
        y: run.y,
        font_size: run.font_size,
        font_family: run.font_family,
        bold: run.bold,
        italic: run.italic,
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

fn metafile_semantic_text(text: &str, font_family: Option<&str>) -> String {
  text
    .chars()
    .map(|character| {
      let mapped =
        super::symbol::font_symbol_code(font_family, character as u32).unwrap_or(character);
      // ASCII Symbol operators can be exposed directly: the text shaper
      // converts them back to the selected font's F0XX transport cmap while
      // PDF semantics retain the ordinary scalar. Keep non-ASCII mappings in
      // transport form so Greek and operators such as Symbol F02D stay in the
      // authored face; the PDF ToUnicode layer standardizes the few codes for
      // which Office does so as well.
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
  fn symbol_ascii_operators_drop_transport_codes_without_remapping_greek() {
    assert_eq!(
      metafile_semantic_text("\u{f02b}\u{f03d}\u{f077}\u{f02d}", Some("Symbol")),
      "+=\u{f077}\u{f02d}"
    );
  }
}
