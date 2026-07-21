use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::ops::Range;
use std::sync::Arc;

use parley::fontique::{Blob, FontInfoOverride};
use parley::{
  FontContext, FontFamily, FontFamilyName, FontStyle, FontWeight, LayoutContext, TextStyle,
};
use rustc_hash::FxHashMap as HashMap;

use crate::fonts::{FontFaceCacheKey, FontStyleRef};
use crate::text_metrics::TextMetrics;

thread_local! {
  /// Parley documents `FontContext` as an application- or thread-level
  /// resource. Keeping both contexts here also lets Fontique retain parsed font
  /// metadata and Parley retain its shaping scratch buffers across paragraphs.
  static PARLEY_TEXT_CONTEXT: RefCell<ParleyTextContext> =
    RefCell::new(ParleyTextContext::default());
}

#[derive(Default)]
struct ParleyTextContext {
  font_context: FontContext,
  layout_context: LayoutContext<()>,
  aliases: HashMap<FontFaceCacheKey, Arc<str>>,
  next_alias: usize,
}

pub(crate) struct StyledTextSpan<'a, S: ?Sized> {
  pub range: Range<usize>,
  pub style: &'a S,
}

/// Uses Parley's Unicode paragraph analysis and line breaker while keeping
/// `ooxmlsdk-fonts` as the source of truth for font selection. The returned
/// ranges index the original UTF-8 text and are intended to be shaped and
/// painted by the existing Office-specific text pipeline.
pub(crate) fn break_text_lines<S: FontStyleRef + ?Sized>(
  text: &str,
  spans: &[StyledTextSpan<'_, S>],
  max_advance: Option<f32>,
  text_metrics: &mut TextMetrics,
) -> Option<Vec<Range<usize>>> {
  if text.is_empty() || spans.is_empty() {
    return Some(std::iter::once(0..0).collect());
  }

  PARLEY_TEXT_CONTEXT.with_borrow_mut(|context| {
    break_text_lines_with_context(text, spans, max_advance, text_metrics, context)
  })
}

fn break_text_lines_with_context<S: FontStyleRef + ?Sized>(
  text: &str,
  spans: &[StyledTextSpan<'_, S>],
  max_advance: Option<f32>,
  text_metrics: &mut TextMetrics,
  context: &mut ParleyTextContext,
) -> Option<Vec<Range<usize>>> {
  let mut span_aliases = Vec::with_capacity(spans.len());
  for span in spans {
    let span_text = text.get(span.range.clone())?;
    let shaped = text_metrics.shape_text(span_text, span.style)?;
    let mut names = Vec::with_capacity(shaped.font_faces.len());
    for face in shaped.font_faces {
      let key = face.cache_key();
      let alias = match context.aliases.entry(key) {
        Entry::Occupied(entry) => entry.get().clone(),
        Entry::Vacant(entry) => {
          let alias: Arc<str> = format!("ooxmlsdk-font-{}", context.next_alias).into();
          context.next_alias += 1;
          let data: Arc<dyn AsRef<[u8]> + Send + Sync> = face.data.clone();
          context.font_context.collection.register_fonts(
            Blob::new(data),
            Some(FontInfoOverride {
              family_name: Some(alias.as_ref()),
              ..FontInfoOverride::default()
            }),
          );
          entry.insert(alias.clone());
          alias
        }
      };
      if !names.iter().any(|name| name == &alias) {
        names.push(alias);
      }
    }
    if names.is_empty() {
      return None;
    }
    span_aliases.push(names);
  }

  let ParleyTextContext {
    font_context,
    layout_context,
    ..
  } = context;
  let mut builder = layout_context.style_run_builder(font_context, text, 1.0, false);
  builder.reserve(spans.len(), spans.len());
  for (span, aliases) in spans.iter().zip(span_aliases) {
    let mut style = TextStyle::<()>::default();
    style.font_family = FontFamily::List(Cow::Owned(
      aliases
        .into_iter()
        .map(|alias| FontFamilyName::Named(Cow::Owned(alias.to_string())))
        .collect(),
    ));
    style.font_size = span.style.font_size_pt();
    style.font_weight = if span.style.bold() {
      FontWeight::BOLD
    } else {
      FontWeight::NORMAL
    };
    style.font_style = if span.style.italic() {
      FontStyle::Italic
    } else {
      FontStyle::Normal
    };
    style.letter_spacing = span.style.character_spacing_pt();
    let style_index = builder.push_style(style);
    builder.push_style_run(style_index, span.range.clone());
  }

  let mut layout = builder.build(text);
  layout.break_all_lines(max_advance);
  Some(layout.lines().map(|line| line.text_range()).collect())
}
