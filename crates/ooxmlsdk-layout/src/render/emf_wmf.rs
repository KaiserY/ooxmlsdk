pub use emfsdk::render::{DecodedMetafile, RenderOptions};

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

pub fn extract_metafile_text_runs(data: &[u8], content_type: Option<&str>) -> Vec<MetafileTextRun> {
  emfsdk::render::extract_metafile_text_runs(data, content_type)
    .into_iter()
    .map(|run| MetafileTextRun {
      text: run.text,
      x: run.x,
      y: run.y,
      font_size: run.font_size,
      font_family: run.font_family,
      bold: run.bold,
      italic: run.italic,
      width: run.width,
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
  use super::supports_semantic_text;

  #[test]
  fn semantic_text_is_supported_only_for_vector_metafiles() {
    assert!(supports_semantic_text(Some("image/x-emf")));
    assert!(supports_semantic_text(Some("APPLICATION/WMF")));
    assert!(!supports_semantic_text(Some("image/png")));
    assert!(!supports_semantic_text(None));
  }
}
