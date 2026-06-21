pub(crate) use emfsdk::render::MetafileTextRun;
pub use emfsdk::render::{DecodedMetafile, RenderOptions};

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

pub(crate) fn extract_metafile_text_runs(
  data: &[u8],
  content_type: Option<&str>,
) -> Vec<MetafileTextRun> {
  emfsdk::render::extract_metafile_text_runs(data, content_type)
}
