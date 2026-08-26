use std::num::NonZeroU32;

use crate::error::{PdfError, Result};
use ooxmlsdk_layout::common;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PageSelection {
  pub(super) source_indices: Vec<usize>,
  pub(super) first_output_index: Vec<Option<usize>>,
}

impl PageSelection {
  pub(super) fn from_range(page_count: usize, range: Option<&str>) -> Result<Self> {
    if page_count == 0 {
      return Err(PdfError::Options(
        "cannot select pages from an empty layout document".to_string(),
      ));
    }
    let source_indices = match range {
      None => (0..page_count).collect(),
      Some(range) => parse_page_range(range, page_count)?,
    };
    if source_indices.is_empty() {
      return Err(PdfError::Options(
        "page range selects no generated pages".to_string(),
      ));
    }
    let mut first_output_index = vec![None; page_count];
    for (output_index, source_index) in source_indices.iter().copied().enumerate() {
      if let Some(slot) = first_output_index.get_mut(source_index) {
        slot.get_or_insert(output_index);
      }
    }
    Ok(Self {
      source_indices,
      first_output_index,
    })
  }

  pub(super) fn output_index(&self, source_index: usize) -> Option<usize> {
    self.first_output_index.get(source_index).copied().flatten()
  }
}

pub(super) fn parse_page_range(range: &str, page_count: usize) -> Result<Vec<usize>> {
  if range.trim().is_empty() {
    return Err(PdfError::Options(
      "page range must not be empty".to_string(),
    ));
  }
  let mut pages = Vec::new();
  for segment in range.split([',', ';']) {
    let segment = segment.trim();
    if segment.is_empty() {
      return Err(PdfError::Options(format!(
        "page range '{range}' contains an empty segment"
      )));
    }
    let hyphen_count = segment.bytes().filter(|byte| *byte == b'-').count();
    match hyphen_count {
      0 => {
        let page = parse_page_number(segment, range)?;
        if page <= page_count {
          pages.push(page - 1);
        }
      }
      1 => {
        let (first, last) = segment.split_once('-').unwrap();
        let first = if first.trim().is_empty() {
          1
        } else {
          parse_page_number(first.trim(), range)?
        };
        let last = if last.trim().is_empty() {
          page_count
        } else {
          parse_page_number(last.trim(), range)?
        };
        if (first > page_count && last > page_count) || page_count == 0 {
          continue;
        }
        let first = first.min(page_count);
        let last = last.min(page_count);
        if first <= last {
          pages.extend((first..=last).map(|page| page - 1));
        } else {
          pages.extend((last..=first).rev().map(|page| page - 1));
        }
      }
      _ => {
        return Err(PdfError::Options(format!(
          "page range segment '{segment}' contains more than one hyphen"
        )));
      }
    }
  }
  Ok(pages)
}

fn parse_page_number(value: &str, range: &str) -> Result<usize> {
  let page = value.parse::<usize>().map_err(|_| {
    PdfError::Options(format!(
      "page range '{range}' contains invalid page number '{value}'"
    ))
  })?;
  if page == 0 {
    return Err(PdfError::Options(
      "page range uses one-based page numbers; page 0 is invalid".to_string(),
    ));
  }
  Ok(page)
}

pub(super) fn pdf_page_dimension(engine_kind: common::LayoutEngineKind, dimension_pt: f32) -> f32 {
  match engine_kind {
    common::LayoutEngineKind::Pptx => {
      // PowerPoint's fixed-format writer quantizes presentation MediaBox
      // dimensions to its 600 dpi print-device grid, with positive half-grid
      // dimensions rounded upward. Keep the OOXML/layout coordinate space
      // exact and apply this only at PDF page creation.
      ooxmlsdk_layout::units::quantize_points_to_office_print_grid(dimension_pt)
    }
    common::LayoutEngineKind::Docx => {
      let print_grid_position = dimension_pt * ooxmlsdk_layout::units::OFFICE_FIXED_OUTPUT_DPI
        / ooxmlsdk_layout::units::POINTS_PER_INCH;
      // Word MediaBoxes use the same 600 dpi grid, but real corpus half-grid
      // dimensions round in both directions depending on printer/page state
      // that is not represented by w:pgSz. Preserve an exact half-grid source
      // dimension instead of choosing a contradicted tie rule. Non-ties have
      // one nearest device coordinate and can be normalized safely.
      if (print_grid_position.fract() - 0.5).abs() <= 0.001 {
        dimension_pt
      } else {
        ooxmlsdk_layout::units::quantize_points_to_office_print_grid(dimension_pt)
      }
    }
    _ => dimension_pt,
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PageLabelSpec {
  pub(super) start: Option<NonZeroU32>,
}

pub(super) fn page_label_spec(
  document: &common::LayoutDocument<'static>,
  page_index: usize,
) -> Option<PageLabelSpec> {
  let page = document.pages.get(page_index)?;
  let physical_page_number = page_index.saturating_add(1);
  let virtual_page_number = page
    .setup
    .page_number_start
    .and_then(|start| {
      i64::from(start)
        .checked_add(i64::try_from(page.section_page_index).ok()?)
        .and_then(|number| u32::try_from(number).ok())
    })
    .or_else(|| {
      document
        .anchor_pages
        .iter()
        .find(|anchor| anchor.page_index == page_index)
        .and_then(|anchor| u32::try_from(anchor.virtual_page_number).ok())
        .filter(|number| usize::try_from(*number).ok() != Some(physical_page_number))
    })?;
  Some(PageLabelSpec {
    start: NonZeroU32::new(virtual_page_number),
  })
}
