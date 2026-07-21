use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::ops::Range;
use std::sync::Arc;

use parley::fontique::{Blob, FontInfoOverride};
use parley::{
  FontContext, FontFamily, FontFamilyName, FontStyle, FontWeight, LayoutContext, TextStyle,
};
use rustc_hash::FxHashMap as HashMap;

use crate::fonts::FontStyleRef;
use crate::text_metrics::TextMetrics;

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

  let mut font_context = FontContext::new();
  let mut aliases = HashMap::<String, Arc<str>>::default();
  let mut span_aliases = Vec::with_capacity(spans.len());
  for span in spans {
    let span_text = text.get(span.range.clone())?;
    let shaped = text_metrics.shape_text(span_text, span.style)?;
    let mut names = Vec::with_capacity(shaped.font_faces.len());
    for face in shaped.font_faces {
      let key = format!("{}#{}", face.id(), face.index);
      let alias_index = aliases.len();
      let alias = match aliases.entry(key) {
        Entry::Occupied(entry) => entry.get().clone(),
        Entry::Vacant(entry) => {
          let alias: Arc<str> = format!("ooxmlsdk-font-{alias_index}").into();
          let data = Blob::new(Arc::new(face.data.as_slice().to_vec()));
          font_context.collection.register_fonts(
            data,
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

  let mut layout_context = LayoutContext::<()>::new();
  let mut builder = layout_context.style_run_builder(&mut font_context, text, 1.0, false);
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
