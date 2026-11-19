use super::*;

/// Word reads a SEQ bookmark at the range's inclusive end. The print result
/// is independent of the source/cached field text, and never consumes a value.
/// ECMA-376 Part 1 §17.16.5.56; native endpoint/forward/repeated-reference
/// controls in /tmp/ooxmlsdk-tdf154319-toc-sequence-20261107.
pub(super) fn refresh(
  sections: &mut [ImportedSection],
  scan: &StoryScan,
  ui_language: Option<&str>,
) {
  let requested = (0..scan.paragraphs.len())
    .filter_map(|ordinal| paragraph(sections, scan, ordinal))
    .flat_map(|paragraph| &paragraph.inlines)
    .filter_map(|inline| match inline {
      InlineItem::Text(TextRun {
        dynamic_field:
          Some(DynamicFieldKind::Sequence {
            identifier,
            bookmark_name: Some(name),
            ..
          }),
        ..
      }) => Some((identifier.clone(), name.to_ascii_lowercase())),
      _ => None,
    })
    .collect::<HashSet<_>>();
  if requested.is_empty() {
    return;
  }
  let available_bookmarks = {
    let mut names = scan
      .bookmark_names
      .iter()
      .map(String::as_str)
      .collect::<HashSet<_>>();
    for section in sections.iter() {
      references::collect_reference_bookmarks(&section.blocks, &mut names);
    }
    names
      .into_iter()
      .map(str::to_ascii_lowercase)
      .collect::<HashSet<_>>()
  };

  let identifiers = requested
    .iter()
    .map(|(id, _)| id.as_ref())
    .collect::<HashSet<_>>();
  let mut starts = HashMap::new();
  let mut ends = HashMap::new();
  let mut fields = Vec::<(usize, String, bool)>::new();
  let mut sources = Vec::new();
  let mut position = 0usize;
  for ordinal in 0..scan.paragraphs.len() {
    let Some(paragraph) = paragraph(sections, scan, ordinal) else {
      continue;
    };
    for event in &paragraph.field_events {
      match event {
        ParagraphFieldEvent::BookmarkStart { id, name } => {
          starts.insert(id.as_str(), name.to_ascii_lowercase());
        }
        ParagraphFieldEvent::BookmarkEnd { id } => {
          if let Some(name) = starts.remove(id.as_str()) {
            ends.insert(name, position);
          }
        }
        ParagraphFieldEvent::Begin { locked, .. } => {
          fields.push((position, String::new(), *locked));
          position += 1;
        }
        ParagraphFieldEvent::Instruction(text) => {
          if let Some((_, code, _)) = fields.last_mut() {
            code.push_str(text);
          }
          position += 1;
        }
        ParagraphFieldEvent::End => {
          if let Some((start, code, locked)) = fields.pop() {
            collect_source(&mut sources, &identifiers, start, &code, locked);
          }
          position += 1;
        }
        ParagraphFieldEvent::Simple {
          instruction,
          locked,
          ..
        } => {
          collect_source(&mut sources, &identifiers, position, instruction, *locked);
          position += 1;
        }
        ParagraphFieldEvent::Content
        | ParagraphFieldEvent::PageBreak
        | ParagraphFieldEvent::Separate { .. } => position += 1,
        _ => {}
      }
    }
    position += 1; // Paragraph mark separates otherwise adjacent endpoints.
  }
  sources.sort_unstable_by_key(|(position, _, _)| *position);
  let mut counters = HashMap::<String, Option<usize>>::new();
  let mut timelines = HashMap::<String, Vec<(usize, Option<usize>)>>::new();
  for (position, identifier, advances) in sources {
    let value = counters.entry(identifier.clone()).or_insert(Some(0));
    *value = if advances {
      value.map(|value| value.saturating_add(1))
    } else {
      None
    };
    timelines
      .entry(identifier)
      .or_default()
      .push((position, *value));
  }
  let values = requested
    .into_iter()
    .filter_map(|(identifier, name)| {
      let end = *ends.get(&name)?;
      let value = timelines
        .get(identifier.as_ref())
        .and_then(|timeline| {
          let index = timeline.partition_point(|(position, _)| *position <= end);
          index.checked_sub(1).map(|index| timeline[index].1)
        })
        .unwrap_or(Some(0))?;
      Some(((identifier, name), value))
    })
    .collect::<HashMap<_, _>>();
  for ordinal in 0..scan.paragraphs.len() {
    let Some(paragraph) = paragraph_mut(sections, scan, ordinal) else {
      continue;
    };
    for inline in &mut paragraph.inlines {
      let InlineItem::Text(run) = inline else {
        continue;
      };
      let Some(DynamicFieldKind::Sequence {
        identifier,
        bookmark_name: Some(name),
        number_format,
      }) = &run.dynamic_field
      else {
        continue;
      };
      if let Some(value) = values.get(&(identifier.clone(), name.to_ascii_lowercase())) {
        run.text = format_field_number(*value, *number_format);
        run.dynamic_field = None;
      } else if !available_bookmarks.contains(&name.to_ascii_lowercase()) {
        let message = FieldMessage::UndefinedBookmark;
        run.text = localized_field_message(message, ui_language);
        apply_generated_field_message_style(&mut run.style, message, ui_language);
        run.dynamic_field = None;
      }
      // Existing but unsupported targets (e.g. a locked/reset source) retain
      // their cache and reference marker; layout must not increment them.
    }
  }
}

fn collect_source(
  sources: &mut Vec<(usize, String, bool)>,
  requested: &HashSet<&str>,
  position: usize,
  instruction: &str,
  locked: bool,
) {
  let tokens = field_instruction_tokens(instruction);
  let [name, identifier, rest @ ..] = tokens.as_slice() else {
    return;
  };
  if !name.eq_ignore_ascii_case("SEQ") {
    return;
  }
  let identifier = identifier.to_ascii_lowercase();
  if !requested.contains(identifier.as_str()) {
    return;
  }
  // A positional bookmark argument reads, rather than changes, this sequence.
  if rest.first().is_some_and(|token| !token.starts_with('\\')) {
    return;
  }
  let kind = sequence_field_kind(&tokens[1..]);
  if matches!(
    &kind,
    Some(DynamicFieldKind::Sequence {
      bookmark_name: Some(_),
      ..
    })
  ) {
    return;
  }
  let advances = !locked
    && matches!(
      kind,
      Some(DynamicFieldKind::Sequence {
        bookmark_name: None,
        ..
      })
    );
  sources.push((position, identifier, advances));
}
