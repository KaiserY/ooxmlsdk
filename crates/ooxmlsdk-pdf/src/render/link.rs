use rustc_hash::FxHashMap as HashMap;

use super::page::PageSelection;
use super::paint::{PaintDocument, PaintItem};
use crate::error::{PdfError, Result};
use crate::options::{PdfLinkDefaultAction, PdfOptions};
use ooxmlsdk_layout::common;

const INTERNAL_LINK_DESTINATION_SHIFT_PT: f32 = 10.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct LinkRect {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct InternalLinkPosition {
  pub(super) output_page_index: usize,
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum ResolvedLinkTarget {
  Internal(InternalLinkPosition),
  Uri(String),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ResolvedLink {
  pub(super) rect: LinkRect,
  pub(super) alt_text: String,
  pub(super) target: ResolvedLinkTarget,
}

#[derive(Clone, Debug, Default)]
pub(super) struct InternalLinkTargets {
  positions: HashMap<String, InternalLinkPosition>,
}

impl InternalLinkTargets {
  pub(super) fn from_layout(
    paint: &PaintDocument<'_>,
    document: &common::LayoutDocument<'static>,
    page_selection: &PageSelection,
  ) -> Self {
    let mut positions = HashMap::default();
    for anchor in &document.anchor_pages {
      let Some(output_page_index) = page_selection.output_index(anchor.page_index) else {
        continue;
      };
      if anchor.name.is_empty() {
        continue;
      }
      positions
        .entry(format!("ooxmlsdk-pdf:bookmark:{}", anchor.name))
        .or_insert(InternalLinkPosition {
          output_page_index,
          x_pt: 0.0,
          y_pt: 0.0,
        });
    }
    for (output_page_index, page) in paint.pages.iter().enumerate() {
      collect_internal_link_targets(&page.items, output_page_index, &mut positions);
    }
    Self { positions }
  }

  fn target_for_url(&self, url: &str) -> Option<InternalLinkPosition> {
    self.positions.get(url).copied()
  }
}

pub(super) fn resolve_link(
  rect: LinkRect,
  alt_text: Option<&str>,
  url: &str,
  internal_links: &InternalLinkTargets,
  options: &PdfOptions,
) -> Result<Option<ResolvedLink>> {
  let right_pt = rect.x_pt + rect.width_pt;
  let bottom_pt = rect.y_pt + rect.height_pt;
  if rect.width_pt <= 0.0
    || rect.height_pt <= 0.0
    || ![
      rect.x_pt,
      rect.y_pt,
      rect.width_pt,
      rect.height_pt,
      right_pt,
      bottom_pt,
    ]
    .into_iter()
    .all(f32::is_finite)
  {
    return Ok(None);
  }

  let alt_text = alt_text
    .map(str::trim)
    .filter(|text| !text.is_empty())
    .unwrap_or(url)
    .to_string();
  let target = if is_internal_link_url(url) {
    let Some(position) = internal_links.target_for_url(url) else {
      return Ok(None);
    };
    ResolvedLinkTarget::Internal(position)
  } else {
    if matches!(
      options.links.default_action,
      PdfLinkDefaultAction::RemoveExternalLinks
    ) {
      return Ok(None);
    }
    let url = normalize_external_url(url);
    let url = if options.links.convert_office_targets_to_pdf_targets {
      office_target_as_pdf(&url)
    } else {
      url
    };
    match options.links.default_action {
      PdfLinkDefaultAction::Uri => ResolvedLinkTarget::Uri(url),
      PdfLinkDefaultAction::UriDestination | PdfLinkDefaultAction::Launch => {
        return Err(PdfError::Options(
          "the selected external link action is unavailable in the current PDF backend".to_string(),
        ));
      }
      PdfLinkDefaultAction::RemoveExternalLinks => unreachable!(),
    }
  };

  Ok(Some(ResolvedLink {
    rect,
    alt_text,
    target,
  }))
}

fn collect_internal_link_targets(
  items: &[PaintItem<'_>],
  output_page_index: usize,
  positions: &mut HashMap<String, InternalLinkPosition>,
) {
  for item in items {
    match item {
      PaintItem::Text(text) => {
        if let Some(url) = &text.item.hyperlink_url
          && let Some(source_url) = reciprocal_internal_link_url(url)
        {
          positions.entry(source_url).or_insert(InternalLinkPosition {
            output_page_index,
            x_pt: text.item.x_pt,
            y_pt: (text.baseline_y - INTERNAL_LINK_DESTINATION_SHIFT_PT).max(0.0),
          });
        }
      }
      PaintItem::Group { items, .. } => {
        collect_internal_link_targets(items, output_page_index, positions);
      }
      PaintItem::Image(_)
      | PaintItem::LinkArea(_)
      | PaintItem::Rect(_)
      | PaintItem::Line(_)
      | PaintItem::Polyline(_) => {}
    }
  }
}

fn is_internal_link_url(url: &str) -> bool {
  url.starts_with("ooxmlsdk-pdf:")
}

fn reciprocal_internal_link_url(url: &str) -> Option<String> {
  let (kind, id) = internal_link_url_parts(url)?;
  let (note_kind, target_suffix) = if let Some(note_kind) = kind.strip_suffix("-reference") {
    (note_kind, "-backlink")
  } else {
    (kind.strip_suffix("-backlink")?, "-reference")
  };
  let mut target_url = String::with_capacity(
    "ooxmlsdk-pdf:".len() + note_kind.len() + target_suffix.len() + id.len() + 1,
  );
  target_url.push_str("ooxmlsdk-pdf:");
  target_url.push_str(note_kind);
  target_url.push_str(target_suffix);
  target_url.push(':');
  target_url.push_str(id);
  Some(target_url)
}

fn internal_link_url_parts(url: &str) -> Option<(&str, &str)> {
  let rest = url.strip_prefix("ooxmlsdk-pdf:")?;
  rest.rsplit_once(':')
}

fn office_target_as_pdf(url: &str) -> String {
  let suffix_start = url.find(['?', '#']).unwrap_or(url.len());
  let (path, suffix) = url.split_at(suffix_start);
  const OFFICE_EXTENSIONS: [&str; 6] = [".docx", ".xlsx", ".pptx", ".doc", ".xls", ".ppt"];
  let Some(extension) = OFFICE_EXTENSIONS
    .into_iter()
    .find(|extension| path.to_ascii_lowercase().ends_with(extension))
  else {
    return url.to_string();
  };
  format!("{}.pdf{suffix}", &path[..path.len() - extension.len()])
}

fn normalize_external_url(url: &str) -> String {
  let normalized = url.replace('\\', "/");
  let url = normalized.as_str();
  if let Some(prefix) = url.strip_suffix("://").map(|scheme| format!("{scheme}://")) {
    return prefix;
  }
  if let Some((scheme, rest)) = url.split_once("://")
    && !rest.is_empty()
    && !rest.contains('/')
    && !rest.contains('?')
    && !rest.contains('#')
  {
    return format!("{scheme}://{rest}/");
  }
  normalized
}
