//! Word chapter page numbers, independently of STYLEREF's localized names.
use std::sync::Arc;

use smallvec::SmallVec;

use crate::model::FieldNumberFormat;

use super::{Page, PageItem, style_ref_numbering_edge_delimiter};
use crate::docx::field_localization::NUMERIC_HEADING_STYLE_REF_KEY_PREFIX;

#[derive(Clone, Debug)]
pub(super) struct ChapterNumber {
  pub number: Arc<str>,
  pub separator: char,
}

struct HeadingNumber<'a> {
  level_index: usize,
  y_pt: f32,
  number: &'a Arc<str>,
}

fn headings(page: &Page) -> SmallVec<[HeadingNumber<'_>; 4]> {
  let mut headings: SmallVec<[HeadingNumber<'_>; 4]> = SmallVec::new();
  for item in &page.items {
    let PageItem::Text(text) = item else { continue };
    if text.dynamic_field.is_some() {
      continue;
    }
    let Some(number) = text.style_ref_numbering_text.as_ref().filter(|number| {
      !number
        .trim_matches(style_ref_numbering_edge_delimiter)
        .is_empty()
    }) else {
      continue;
    };
    let Some(level) = text.style_ref_keys.iter().find_map(|key| {
      key
        .strip_prefix(NUMERIC_HEADING_STYLE_REF_KEY_PREFIX)
        .and_then(|level| level.parse::<usize>().ok())
        .filter(|level| (1..=9).contains(level))
    }) else {
      continue;
    };
    headings.push(HeadingNumber {
      level_index: level - 1,
      y_pt: text.y_pt,
      number,
    });
  }
  headings.sort_by(|a, b| a.y_pt.total_cmp(&b.y_pt));
  headings
}

pub(super) fn resolve(pages: &[Page]) -> Vec<Option<ChapterNumber>> {
  if !pages
    .iter()
    .any(|page| page.setup.page_number_chapter.is_some())
  {
    return Vec::new();
  }
  // Native controls: the last numbered heading on the page wins; pages before
  // any numbered heading use the first following one. Unnumbered headings do
  // not reset the last number. This history continues across sections, while
  // each target page independently owns its chosen heading level/separator.
  // Read heading metadata directly, avoiding STYLEREF's full-label searches
  // for every body run. Reuse its immutable number Arc; no new lock/cache.
  let mut first: [Option<Arc<str>>; 9] = Default::default();
  for page in pages {
    for heading in headings(page) {
      first[heading.level_index].get_or_insert_with(|| heading.number.clone());
    }
  }
  let mut previous: [Option<Arc<str>>; 9] = Default::default();
  pages
    .iter()
    .map(|page| {
      for heading in headings(page) {
        previous[heading.level_index] = Some(heading.number.clone());
      }
      let config = page.setup.page_number_chapter?;
      let index = usize::from(config.heading_level.checked_sub(1)?);
      let number = previous
        .get(index)?
        .as_ref()
        .or(first.get(index)?.as_ref())?;
      Some(ChapterNumber {
        number: number.clone(),
        separator: config.separator,
      })
    })
    .collect()
}

pub(super) fn format(
  value: usize,
  number_format: FieldNumberFormat,
  chapter: Option<&ChapterNumber>,
) -> String {
  let page = super::super::format_field_number(value, number_format);
  if let Some(chapter) = chapter {
    let number = chapter
      .number
      .trim_matches(style_ref_numbering_edge_delimiter);
    format!("{number}{}{page}", chapter.separator)
  } else {
    page
  }
}
