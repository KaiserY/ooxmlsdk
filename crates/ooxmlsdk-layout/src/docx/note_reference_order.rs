use super::{HashMap, HashSet, NoteKind, w};

/// Source-order numbering is separate from visible footnote allocation. Word's
/// final export hides unaccepted deletions but still counts their automatic
/// references. Physical removal and custom marks are the opposite controls.
#[derive(Clone, Debug, Default)]
pub(super) struct Order {
  footnotes: Vec<(i64, bool)>,
  endnotes: Vec<(i64, bool)>,
}

impl Order {
  pub(super) fn collect_paragraph(&mut self, source: &w::Paragraph) {
    paragraph(source, self, false);
  }

  pub(super) fn append(&mut self, other: &mut Self) {
    self.footnotes.append(&mut other.footnotes);
    self.endnotes.append(&mut other.endnotes);
  }

  pub(super) fn include_deleted(&self, kind: NoteKind, visible: &mut Vec<i64>) {
    let source = match kind {
      NoteKind::Footnote => &self.footnotes,
      NoteKind::Endnote => &self.endnotes,
    };
    if !source.iter().any(|(_, deleted)| *deleted) {
      return;
    }
    // Anchor each reserved number to its next surviving reference. Keep the
    // existing visible order, including references imported from nested frames;
    // trailing deletions still advance continuous numbering in the next section.
    let visible_ids = visible.iter().copied().collect::<HashSet<_>>();
    let mut before = HashMap::<i64, Vec<i64>>::new();
    let mut pending = Vec::new();
    for &(id, deleted) in source {
      if deleted {
        pending.push(id);
      } else if visible_ids.contains(&id) && !pending.is_empty() {
        before.entry(id).or_default().append(&mut pending);
      }
    }
    let mut ordered = Vec::with_capacity(visible.len() + source.len());
    for id in visible.drain(..) {
      if let Some(mut deleted) = before.remove(&id) {
        ordered.append(&mut deleted);
      }
      ordered.push(id);
    }
    ordered.append(&mut pending);
    *visible = ordered;
  }
}

fn run(source: &w::Run, order: &mut Order, deleted: bool) {
  for choice in &source.run_choice {
    let (id, custom, references) = match choice {
      w::RunChoice::FootnoteReference(reference) => (
        reference.id,
        reference.custom_mark_follows,
        &mut order.footnotes,
      ),
      w::RunChoice::EndnoteReference(reference) => (
        reference.id,
        reference.custom_mark_follows,
        &mut order.endnotes,
      ),
      _ => continue,
    };
    // ECMA-376 §§17.11.7/14: a custom mark never advances the automatic
    // counter, including when its reference is inside an unaccepted deletion.
    if id >= 0 && !custom.is_some_and(|value| value.as_bool()) {
      references.push((id, deleted));
    }
  }
}

fn sdt(source: &w::SdtRun, order: &mut Order, deleted: bool) {
  if let Some(content) = source.sdt_content_run.as_ref() {
    sdt_content(content, order, deleted);
  }
}

// Generated run-choice enums share revision/container nodes. Keep this traversal
// independent of painting: reading a deleted reference must not import its text,
// allocate its footnote body, or give an empty deleted paragraph flow height.
macro_rules! collect_choices {
  ($function:ident, $type:ident, $field:ident, $choice:ident $(, $variant:ident => $visit:ident)*) => {
    fn $function(source: &w::$type, order: &mut Order, deleted: bool) {
      for choice in &source.$field {
        match choice {
          w::$choice::WRun(source) => run(source, order, deleted),
          w::$choice::SdtRun(source) => sdt(source, order, deleted),
          w::$choice::InsertedRun(source) => inserted(source, order, deleted),
          w::$choice::DeletedRun(source) => deleted_run(source, order, true),
          // A move's destination already owns its reference. Do not count the
          // source a second time as a tracked deletion.
          w::$choice::MoveFromRun(_) => {},
          w::$choice::MoveToRun(source) => moved_to(source, order, deleted),
          w::$choice::BidirectionalOverride(source) => directional_override(source, order, deleted),
          w::$choice::BidirectionalEmbedding(source) => directional_embedding(source, order, deleted),
          $(w::$choice::$variant(source) => $visit(source, order, deleted),)*
          _ => {},
        }
      }
    }
  };
}

collect_choices!(paragraph, Paragraph, paragraph_choice, ParagraphChoice,
  SimpleField => simple_field, Hyperlink => hyperlink,
  CustomXmlRun => custom_xml, SmartTagRun => custom_xml);
collect_choices!(simple_field, SimpleField, simple_field_choice, SimpleFieldChoice,
  SimpleField => simple_field, Hyperlink => hyperlink, CustomXmlRun => custom_xml);
collect_choices!(hyperlink, Hyperlink, hyperlink_choice, HyperlinkChoice,
  SimpleField => simple_field, Hyperlink => hyperlink, CustomXmlRun => custom_xml);
collect_choices!(custom_xml, CustomXmlRun, custom_xml_run_choice, CustomXmlRunChoice,
  SimpleField => simple_field, Hyperlink => hyperlink, CustomXmlRun => custom_xml);
collect_choices!(sdt_content, SdtContentRun, sdt_content_run_choice, SdtContentRunChoice,
  SimpleField => simple_field, Hyperlink => hyperlink, CustomXmlRun => custom_xml);
collect_choices!(
  inserted,
  InsertedRun,
  inserted_run_choice,
  InsertedRunChoice
);
collect_choices!(
  deleted_run,
  DeletedRun,
  deleted_run_choice,
  DeletedRunChoice
);
collect_choices!(moved_to, MoveToRun, move_to_run_choice, MoveToRunChoice);
collect_choices!(directional_override, BidirectionalOverride, bidirectional_override_choice,
  BidirectionalOverrideChoice, SimpleField => simple_field, Hyperlink => hyperlink,
  CustomXmlRun => custom_xml);
collect_choices!(directional_embedding, BidirectionalEmbedding, bidirectional_embedding_choice,
  BidirectionalEmbeddingChoice, SimpleField => simple_field, Hyperlink => hyperlink,
  CustomXmlRun => custom_xml);
