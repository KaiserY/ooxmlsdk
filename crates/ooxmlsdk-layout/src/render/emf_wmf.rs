pub use emfsdk::render::DecodedMetafile;
pub(crate) use emfsdk::render::MetafileTextRun;

pub fn decode_metafile_as_raster(
  data: &[u8],
  content_type: Option<&str>,
) -> Result<Option<DecodedMetafile>, String> {
  emfsdk::render::decode_metafile_as_raster(data, content_type).map_err(|err| err.to_string())
}

pub(crate) fn extract_metafile_text_runs(
  data: &[u8],
  content_type: Option<&str>,
) -> Vec<MetafileTextRun> {
  emfsdk::render::extract_metafile_text_runs(data, content_type)
}
