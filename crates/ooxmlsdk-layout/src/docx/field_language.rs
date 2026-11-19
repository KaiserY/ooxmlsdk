//! Word's default field locale can belong to preceding story text.
//!
//! Native bibliography and unpictured CREATEDATE controls distinguish that
//! context from the field's own begin/code/cache/paragraph-mark languages.
//! Whitespace does not replace it; an empty story uses the authoring locale.

use super::InlineItem;
use std::sync::Arc;

pub(super) fn preceding_text_language(inlines: &[InlineItem]) -> Option<Option<&Arc<str>>> {
  inlines.iter().rev().find_map(|inline| match inline {
    InlineItem::Text(run) if run.text.chars().any(|ch| !ch.is_whitespace()) => {
      Some(run.style.language.as_ref())
    }
    _ => None,
  })
}

/// Retain the existing style's language allocation across paragraphs. No
/// interning table, synchronization or new string allocation is needed.
/// Some(None) distinguishes text with no language from an empty story.
#[derive(Clone, Debug, Default)]
pub(super) struct Context(Option<Option<Arc<str>>>);

impl Context {
  pub(super) fn at(&self, inlines: &[InlineItem]) -> Self {
    let mut next = self.clone();
    next.update(inlines);
    next
  }

  pub(super) fn update(&mut self, inlines: &[InlineItem]) {
    if let Some(language) = preceding_text_language(inlines)
      && self.0.as_ref().map(|previous| previous.as_deref()) != Some(language.map(Arc::as_ref))
    {
      self.0 = Some(language.cloned());
    }
  }

  pub(super) fn language<'a>(&'a self, authoring_language: Option<&'a str>) -> Option<&'a str> {
    match &self.0 {
      Some(language) => language.as_deref(),
      None => authoring_language,
    }
  }
}
