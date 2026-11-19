use super::*;

pub(super) fn bookmark_text_sources(paragraph: &w::Paragraph) -> Vec<ParagraphFieldEvent> {
  let mut starts = HashMap::new();
  let mut sources = Vec::new();
  for (index, choice) in paragraph.paragraph_choice.iter().enumerate() {
    match choice {
      w::ParagraphChoice::BookmarkStart(bookmark) if !bookmark.name.is_empty() => {
        starts.insert(bookmark.id.as_str(), (bookmark.name.as_str(), index + 1));
      }
      w::ParagraphChoice::BookmarkEnd(bookmark) => {
        let Some((name, start)) = starts.remove(bookmark.id.as_str()) else {
          continue;
        };
        let mut source = w::Paragraph::default();
        let mut supported = true;
        for choice in &paragraph.paragraph_choice[start..index] {
          match choice {
            w::ParagraphChoice::WRun(run)
              if run.run_choice.iter().all(|choice| {
                matches!(
                  choice,
                  w::RunChoice::Text(_) | w::RunChoice::FieldChar(_) | w::RunChoice::FieldCode(_)
                )
              }) =>
            {
              source.paragraph_choice.push(choice.clone())
            }
            w::ParagraphChoice::BookmarkStart(_) | w::ParagraphChoice::BookmarkEnd(_) => {}
            _ => {
              supported = false;
              break;
            }
          }
        }
        if supported && supported_source_fields(&source) {
          sources.push(ParagraphFieldEvent::BookmarkTextSource {
            name: name.to_string(),
            paragraph: Box::new(source),
          });
        }
      }
      _ => {}
    }
  }
  sources
}

fn supported_source_fields(paragraph: &w::Paragraph) -> bool {
  let mut collector = FieldEventCollector::default();
  for choice in &paragraph.paragraph_choice {
    collect_paragraph_choice(choice, &mut collector);
  }
  let mut open = false;
  let mut instruction = String::new();
  for event in collector.events {
    match event {
      ParagraphFieldEvent::Begin { .. } if !open => {
        open = true;
        instruction.clear();
      }
      ParagraphFieldEvent::Instruction(text) if open => instruction.push_str(&text),
      ParagraphFieldEvent::Separate { .. } if open => {}
      ParagraphFieldEvent::End if open => {
        if !matches!(
          dynamic_field_kind(&instruction),
          Some(DynamicFieldKind::Sequence {
            bookmark_name: None,
            ..
          })
        ) {
          return false;
        }
        open = false;
      }
      ParagraphFieldEvent::Content => {}
      _ => return false,
    }
  }
  !open
}

pub(super) fn inline_refreshes(
  sections: &[ImportedSection],
  scan: &StoryScan,
  styles: &StylesCatalog,
  ui_language: Option<&str>,
) -> Vec<ReferenceResultRefresh> {
  let mut sources = HashMap::new();
  let mut requested = HashSet::new();
  let mut requested_numbers = HashSet::new();
  let mut has_references = false;
  for ordinal in 0..scan.paragraphs.len() {
    if let Some(paragraph) = paragraph(sections, scan, ordinal) {
      for event in &paragraph.field_events {
        match event {
          ParagraphFieldEvent::BookmarkTextSource { name, paragraph } => {
            sources.insert(name.to_ascii_lowercase(), paragraph.as_ref());
          }
          ParagraphFieldEvent::ReferenceResultSpan {
            bookmark_name,
            copy_text,
            numbering_only,
            ..
          } => {
            has_references = true;
            if *copy_text {
              requested.insert(bookmark_name.to_ascii_lowercase());
            }
            if *numbering_only {
              requested_numbers.insert(bookmark_name.to_ascii_lowercase());
            }
          }
          _ => {}
        }
      }
    }
  }
  if !has_references {
    return Vec::new();
  }
  let mut unnumbered_targets = HashSet::new();
  if !requested_numbers.is_empty() {
    // Resolve requested starts once in source order, before replacing any
    // field spans. A missing source, caret-only bookmark, unsupported list
    // or separate story keeps its previous owner rather than becoming zero.
    for (ordinal, entry) in scan.paragraphs.iter().enumerate() {
      if entry.source_bookmarks.is_empty() {
        continue;
      }
      let Some(source) = paragraph(sections, scan, ordinal) else {
        continue;
      };
      if source.list_label.is_some()
        || source.list_label_image.is_some()
        || source.format.numbering_id.is_some_and(|id| id != 0)
      {
        continue;
      }
      for name in &entry.source_bookmarks {
        if scan.caret_reference_bookmarks.contains(name) {
          continue;
        }
        if let Some(requested) = requested_numbers
          .iter()
          .find(|key| key.eq_ignore_ascii_case(name))
        {
          unnumbered_targets.insert(requested.as_str());
        }
      }
    }
  }
  sources.retain(|name, _| requested.contains(name));
  // Separate stories do not supply main-story TOC ranges or sequence state,
  // but their bookmarks are valid REF targets. A caption in a WPS textbox
  // must keep its supported cache instead of becoming a missing-source error.
  let mut available_bookmarks = scan
    .bookmark_names
    .iter()
    .map(String::as_str)
    .collect::<HashSet<_>>();
  for section in sections {
    collect_reference_bookmarks(&section.blocks, &mut available_bookmarks);
  }

  // Snapshot each target's sequence state in source order. REF may precede
  // its target and may occur repeatedly; its copied SEQ reads this state
  // without advancing the actual story's counter a second time.
  let mut sequence_numbers = HashMap::<Arc<str>, usize>::new();
  let mut source_sequences = HashMap::new();
  if !sources.is_empty() {
    for ordinal in 0..scan.paragraphs.len() {
      let Some(paragraph) = paragraph(sections, scan, ordinal) else {
        continue;
      };
      for inline in &paragraph.inlines {
        match inline {
          InlineItem::SequenceCounter(identifier) => {
            let value = sequence_numbers.entry(identifier.clone()).or_default();
            *value = value.saturating_add(1);
          }
          InlineItem::BookmarkStart(name) if sources.contains_key(&name.to_ascii_lowercase()) => {
            source_sequences.insert(name.to_ascii_lowercase(), sequence_numbers.clone());
          }
          InlineItem::Text(run) => {
            if let Some(DynamicFieldKind::Sequence {
              identifier,
              bookmark_name: None,
              ..
            }) = &run.dynamic_field
            {
              let value = sequence_numbers.entry(identifier.clone()).or_default();
              *value = value.saturating_add(1);
            }
          }
          _ => {}
        }
      }
    }
  }

  let mut refreshes = Vec::new();
  for ordinal in 0..scan.paragraphs.len() {
    let Some(paragraph) = paragraph(sections, scan, ordinal) else {
      continue;
    };
    for event in &paragraph.field_events {
      let ParagraphFieldEvent::ReferenceResultSpan {
        field_id,
        bookmark_name,
        reference_kind,
        inline_start,
        inline_end,
        merge_format,
        copy_text,
        numbering_only,
        separator_style,
      } = event
      else {
        continue;
      };
      let start = (*inline_start).min(paragraph.inlines.len());
      let end = (*inline_end).max(start).min(paragraph.inlines.len());
      let cached = &paragraph.inlines[start..end];
      if !cached
        .iter()
        .all(|item| matches!(item, InlineItem::Text(_)))
      {
        continue;
      }
      let template = cached.iter().find_map(|item| match item {
        InlineItem::Text(run) => Some(run),
        _ => None,
      });
      let missing = !available_bookmarks
        .iter()
        .any(|name| name.eq_ignore_ascii_case(bookmark_name));
      let mut replacement = if missing {
        if *reference_kind == field_reference::Kind::Implicit
          && cached
            .iter()
            .any(|item| matches!(item, InlineItem::Text(run) if !run.text.is_empty()))
        {
          // Word keeps a missing implicit REF's nonempty cache, including
          // spaces, even when the optional print-update pass is enabled.
          continue;
        }
        if *reference_kind == field_reference::Kind::Explicit
          && start == 0
          && end == paragraph.inlines.len()
        {
          // The existing whole-story owner also handles multi-paragraph REF
          // caches. Keep that structural replacement independent.
          continue;
        }
        let mut run = template
          .cloned()
          .unwrap_or_else(|| empty_run(paragraph.base_style.clone()));
        if let Some(style) = separator_style {
          run.style.clone_from(style);
        }
        let message = match reference_kind {
          field_reference::Kind::Explicit => {
            field_localization::apply_reference_separator_message_style(
              &mut run.style,
              ui_language,
            );
            FieldMessage::ReferenceSourceNotFound
          }
          field_reference::Kind::Implicit => {
            let bold = !run.style.bold;
            let complex_bold = !run.style.complex_bold.unwrap_or(run.style.bold);
            apply_generated_field_message_style(
              &mut run.style,
              FieldMessage::UndefinedBookmark,
              ui_language,
            );
            run.style.bold = bold;
            run.style.complex_bold = Some(complex_bold);
            FieldMessage::UndefinedBookmark
          }
        };
        run.text = run_display_text(
          localized_field_message(message, ui_language),
          run.style.clone(),
        );
        text::apply_wordprocessingml_cjk_text_metrics(&run.text, &mut run.style);
        run.dynamic_field = None;
        run.hyperlink_url = None;
        run.style_ref_keys.clear();
        run.style_ref_text = None;
        run.style_ref_numbering_text = None;
        run.preserve_text_portion = false;
        vec![InlineItem::Text(run)]
      } else if *copy_text {
        let key = bookmark_name.to_ascii_lowercase();
        let Some(source) = sources.get(&key) else {
          continue;
        };
        let Some(sequence_numbers) = source_sequences.get(&key) else {
          continue;
        };
        let Some(result) = source_result(
          source,
          paragraph,
          styles,
          sequence_numbers,
          template.filter(|_| *merge_format),
        ) else {
          continue;
        };
        result
      } else if *numbering_only
        && unnumbered_targets
          .iter()
          .any(|name| name.eq_ignore_ascii_case(bookmark_name))
      {
        // Word returns zero for an unnumbered REF target even with print
        // updates disabled. The insertion/separator properties own this new
        // result; explicit MERGEFORMAT instead retains cached formatting.
        let mut run = template
          .cloned()
          .unwrap_or_else(|| empty_run(paragraph.base_style.clone()));
        if !*merge_format && let Some(style) = separator_style {
          run.style.clone_from(style);
        }
        run.text = "0".to_owned();
        run.dynamic_field = None;
        run.style_ref_keys.clear();
        run.style_ref_text = None;
        run.style_ref_numbering_text = None;
        run.preserve_text_portion = false;
        vec![InlineItem::Text(run)]
      } else {
        continue;
      };
      mark_wordprocessing_field_result(&mut replacement);
      refreshes.push(ReferenceResultRefresh {
        paragraph_ordinal: ordinal,
        field_id: *field_id,
        inline_start: start,
        inline_end: end,
        replacement,
        paragraph_break_offsets: Vec::new(),
      });
    }
  }
  refreshes
}

pub(super) fn collect_reference_bookmarks<'a>(blocks: &'a [Block], names: &mut HashSet<&'a str>) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => {
        for event in &paragraph.field_events {
          if let ParagraphFieldEvent::BookmarkStart { name, .. } = event {
            names.insert(name);
          }
        }
        for inline in &paragraph.inlines {
          match inline {
            InlineItem::BookmarkStart(name) => {
              names.insert(name);
            }
            InlineItem::Shape(shape) => collect_reference_bookmarks(&shape.text_box_blocks, names),
            _ => {}
          }
        }
      }
      Block::Frame(frame) => collect_reference_bookmarks(&frame.blocks, names),
      Block::Table(table) => {
        for row in &table.rows {
          for cell in &row.cells {
            collect_reference_bookmarks(&cell.blocks, names);
          }
        }
      }
    }
  }
}

fn source_result(
  source: &w::Paragraph,
  destination: &Paragraph,
  styles: &StylesCatalog,
  sequence_numbers: &HashMap<Arc<str>, usize>,
  merge_template: Option<&TextRun>,
) -> Option<Vec<InlineItem>> {
  let mut result = paragraph_inlines_with_policy(
    source,
    destination.base_style.clone(),
    styles,
    &ImageCatalog::default(),
    &HyperlinkCatalog::default(),
    ParagraphInlineImport {
      custom_xml_bindings: &CustomXmlBindings::default(),
      form_widget_ids: &mut FormWidgetIdAllocator::default(),
      suppress_toc_hyperlink_style: false,
      complex_fields: None,
      table_depth: 0,
      style_outline_level: None,
    },
  );
  let mut sequence_numbers = sequence_numbers.clone();
  for item in &mut result {
    if let InlineItem::SequenceCounter(identifier) = item {
      let value = sequence_numbers.entry(identifier.clone()).or_default();
      *value = value.saturating_add(1);
      continue;
    }
    let InlineItem::Text(run) = item else {
      return None;
    };
    if let Some(kind) = run.dynamic_field.take() {
      let DynamicFieldKind::Sequence {
        identifier,
        bookmark_name: None,
        number_format,
      } = kind
      else {
        return None;
      };
      let value = sequence_numbers.entry(identifier).or_default();
      *value = value.saturating_add(1);
      run.text = format_field_number(*value, number_format);
    }
    if let Some(template) = merge_template {
      run.style.clone_from(&template.style);
      run.hyperlink_url.clone_from(&template.hyperlink_url);
    }
  }
  // The copied value reads the source state; it must not consume the same
  // instruction's sequence a second time in the destination story.
  result.retain(|item| !matches!(item, InlineItem::SequenceCounter(_)));
  Some(result)
}

fn empty_run(style: TextStyle) -> TextRun {
  TextRun {
    text: String::new(),
    style,
    hyperlink_url: None,
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    style_ref_numbering_text: None,
    preserve_text_portion: false,
  }
}
