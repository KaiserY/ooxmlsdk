pub use emfsdk::render::{DecodedMetafile, RenderOptions};

#[derive(Clone, Debug)]
pub struct MetafileTextRun {
  pub text: String,
  pub x: f32,
  pub y: f32,
  pub font_size: Option<f32>,
  pub font_family: Option<String>,
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
  let runs = emfsdk::render::extract_metafile_text_runs(data, content_type);
  let metadata = emf_text_run_metadata(data);
  runs
    .into_iter()
    .enumerate()
    .map(|(index, run)| MetafileTextRun {
      text: run.text,
      x: run.x,
      y: run.y,
      font_size: run.font_size,
      font_family: metadata
        .get(index)
        .and_then(|metadata| metadata.font_family.clone()),
      width: metadata.get(index).and_then(|metadata| metadata.width),
    })
    .collect()
}

#[derive(Clone, Debug, Default)]
struct EmfTextRunMetadata {
  font_family: Option<String>,
  width: Option<f32>,
}

fn emf_text_run_metadata(data: &[u8]) -> Vec<EmfTextRunMetadata> {
  use std::collections::HashMap;

  use emfsdk::{EmfMetafileRef, EmfRecordData, ExtTextOutOptions};

  let Ok(metafile) = EmfMetafileRef::from_bytes(data) else {
    return Vec::new();
  };
  let mut fonts = HashMap::<u32, String>::new();
  let mut current_font = None;
  let mut canvas_width = None;
  let mut window_width = None;
  let mut viewport_width = None;
  let mut transformed = false;
  let mut metadata = Vec::new();
  for record in metafile.records() {
    let Ok(record) = record.parse_data() else {
      continue;
    };
    match record {
      EmfRecordData::Header(header) => {
        let width = header.bounds.right - header.bounds.left + 1;
        canvas_width = (width > 0).then_some(width as f32);
        window_width = canvas_width;
        viewport_width = canvas_width;
      }
      EmfRecordData::SetWindowExtEx(extent) => {
        window_width = Some(extent.size.cx.unsigned_abs().max(1) as f32);
      }
      EmfRecordData::SetViewportExtEx(extent) => {
        viewport_width = Some(extent.size.cx.unsigned_abs().max(1) as f32);
      }
      EmfRecordData::SetWorldTransform(_) | EmfRecordData::ModifyWorldTransform(_) => {
        transformed = true;
      }
      EmfRecordData::ExtCreateFontIndirectW(font) => {
        let family = font
          .log_font()
          .and_then(|font| font.face_name.as_str().ok())
          .map(|family| family.trim_matches('\0').trim().to_string())
          .filter(|family| !family.is_empty());
        if let Some(family) = family {
          fonts.insert(font.object_index, family);
        }
      }
      EmfRecordData::SelectObject(object) if fonts.contains_key(&object.object_index) => {
        current_font = Some(object.object_index);
      }
      EmfRecordData::DeleteObject(object) => {
        fonts.remove(&object.object_index);
        if current_font == Some(object.object_index) {
          current_font = None;
        }
      }
      EmfRecordData::ExtTextOutA(text) | EmfRecordData::ExtTextOutW(text)
        if text
          .text
          .text
          .as_str()
          .is_ok_and(|text| !text.trim_matches('\0').trim().is_empty()) =>
      {
        let width = (!transformed && text.text.dx_buffer_present)
          .then(|| {
            let stride = if text.text.options.contains(ExtTextOutOptions::PDY) {
              2
            } else {
              1
            };
            text
              .text
              .dx
              .iter()
              .step_by(stride)
              .map(|advance| *advance as f32)
              .sum::<f32>()
          })
          .filter(|width| *width > 0.0)
          .and_then(|width| Some(width * viewport_width? / window_width? / canvas_width?));
        metadata.push(EmfTextRunMetadata {
          font_family: current_font.and_then(|font| fonts.get(&font).cloned()),
          width,
        });
      }
      _ => {}
    }
  }
  metadata
}
