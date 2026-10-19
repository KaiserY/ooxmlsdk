//! Recover orphan deleted-text runs using Word's source-order revision context.
use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, Ordering};

use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

#[derive(Default)]
pub(super) struct Context {
  seen_metadata: AtomicBool,
}

impl Context {
  pub(super) fn observe(&self) {
    self.seen_metadata.store(true, Ordering::Relaxed);
  }

  pub(super) fn observe_paragraph(&self, paragraph: &w::Paragraph) {
    if paragraph
      .paragraph_properties
      .as_deref()
      .is_some_and(|properties| {
        properties.paragraph_properties_change.is_some()
          || properties
            .paragraph_mark_run_properties
            .as_deref()
            .is_some_and(|mark| {
              mark.inserted.is_some()
                || mark.deleted.is_some()
                || mark.move_from.is_some()
                || mark.move_to.is_some()
                || mark.paragraph_mark_run_properties_change.is_some()
            })
      })
    {
      self.observe();
    }
  }
}

pub(super) fn run<'a>(run: &'a w::Run, context: Option<&Context>) -> Option<Cow<'a, w::Run>> {
  let Some(context) = context else {
    return Some(Cow::Borrowed(run));
  };
  if run
    .run_properties
    .as_deref()
    .is_some_and(|properties| properties.run_properties_change.is_some())
  {
    context.observe();
  }
  if !context.seen_metadata.load(Ordering::Relaxed) {
    return Some(Cow::Borrowed(run));
  }
  let Some(start) = run
    .run_choice
    .iter()
    .position(|choice| matches!(choice, w::RunChoice::DeletedText(_)))
  else {
    return Some(Cow::Borrowed(run));
  };
  // ECMA 17.3.3.7 requires delText inside del. Word OpenAndRepair keeps an
  // orphan literal before any revision metadata, but repairs it as a deletion
  // after that metadata. Native saved models put the remaining run content in
  // a new del range; following runs stay independent. Drop the deleted suffix
  // before field parsing, hidden style references and empty-run font ownership.
  if start == 0 {
    return None;
  }
  let mut prefix = run.clone();
  prefix.run_choice.truncate(start);
  Some(Cow::Owned(prefix))
}

#[cfg(test)]
mod tests {
  use ooxmlsdk::sdk::SdkType;

  use super::super::{
    AltChunkCatalog, Block, BodySectionEnv, CustomXmlBindings, FormWidgetIdAllocator,
    HyperlinkCatalog, ImageCatalog, InlineItem, NumberingCatalog, StylesCatalog, body_sections,
  };
  use super::w;

  fn imported_text(body: &str) -> String {
    let xml = format!(
      r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>{body}</w:body></w:document>"#
    );
    let document = w::Document::from_bytes(xml.as_bytes()).expect("revision control document");
    let styles = StylesCatalog::default();
    let mut numbering = NumberingCatalog::default();
    let images = ImageCatalog::default();
    let alt_chunks = AltChunkCatalog::default();
    let hyperlinks = HyperlinkCatalog::default();
    let bindings = CustomXmlBindings::default();
    let mut form_widget_ids = FormWidgetIdAllocator::default();
    let sections = body_sections(
      document.body.as_deref().expect("document body"),
      BodySectionEnv {
        styles: &styles,
        numbering: &mut numbering,
        images: &images,
        alt_chunks: &alt_chunks,
        hyperlinks: &hyperlinks,
        custom_xml_bindings: &bindings,
        form_widget_ids: &mut form_widget_ids,
        no_column_balance: false,
        fixed_html_paragraph_auto_spacing: false,
      },
    );
    sections
      .iter()
      .flat_map(|section| &section.blocks)
      .filter_map(|block| match block {
        Block::Paragraph(paragraph) => Some(&paragraph.inlines),
        _ => None,
      })
      .flatten()
      .filter_map(|inline| match inline {
        InlineItem::Text(run) => Some(run.text.as_str()),
        _ => None,
      })
      .collect()
  }

  #[test]
  fn native_orphan_text_visibility_depends_on_revision_context_and_run_boundary() {
    // Word OpenAndRepair PDF/XPS: 18 wrapper/paragraph-mark controls. In a
    // mixed run the prefix survives, but everything from delText onward is
    // repaired as a deletion; the next run remains visible.
    for (mark, properties) in [
      ("normal", ""),
      (
        "deleted",
        r#"<w:pPr><w:rPr><w:del w:id="1" w:author="Probe"/></w:rPr></w:pPr>"#,
      ),
      (
        "inserted",
        r#"<w:pPr><w:rPr><w:ins w:id="1" w:author="Probe"/></w:rPr></w:pPr>"#,
      ),
    ] {
      let deleted = "<w:r><w:delText>REMOVED</w:delText></w:r>";
      let mixed = "<w:r><w:t>A</w:t><w:delText>REMOVED</w:delText><w:t>B</w:t></w:r>";
      for (name, content, has_revision) in [
        ("bare", deleted.to_string(), false),
        (
          "inserted-wrapper",
          format!(r#"<w:ins w:id="2" w:author="Probe">{deleted}</w:ins>"#),
          true,
        ),
        (
          "hyperlink",
          format!("<w:hyperlink>{deleted}</w:hyperlink>"),
          false,
        ),
        (
          "sdt",
          format!("<w:sdt><w:sdtContent>{deleted}</w:sdtContent></w:sdt>"),
          false,
        ),
        (
          "proper-deletion",
          format!(r#"<w:del w:id="2" w:author="Probe">{deleted}</w:del>"#),
          true,
        ),
        ("mixed", mixed.to_string(), false),
      ] {
        let body = format!(
          "<w:p>{properties}<w:r><w:t>LEFT</w:t></w:r>{content}<w:r><w:t>RIGHT</w:t></w:r></w:p><w:p><w:r><w:t>AFTER</w:t></w:r></w:p>"
        );
        let repaired = has_revision || mark != "normal";
        let expected = match (name == "mixed", repaired) {
          (true, true) => "LEFTARIGHTAFTER",
          (true, false) => "LEFTAREMOVEDBRIGHTAFTER",
          (false, true) => "LEFTRIGHTAFTER",
          (false, false) => "LEFTREMOVEDRIGHTAFTER",
        };
        assert_eq!(imported_text(&body), expected, "{name}/{mark}");
      }
    }
  }

  #[test]
  fn native_revision_context_follows_source_order_across_paragraphs() {
    // Native revision-kind/location controls: later metadata does not
    // retroactively turn earlier orphan delText into a deletion.
    let content = "<w:p><w:r><w:t>LEFT</w:t></w:r><w:r><w:t>A</w:t><w:delText>REMOVED</w:delText><w:t>B</w:t></w:r><w:r><w:t>RIGHT</w:t></w:r></w:p><w:p><w:r><w:t>AFTER</w:t></w:r></w:p>";
    for marker in [
      r#"<w:p><w:pPr><w:rPr><w:ins w:id="2" w:author="Probe"/></w:rPr></w:pPr><w:r><w:t>HISTORY</w:t></w:r></w:p>"#,
      r#"<w:p><w:pPr><w:rPr><w:del w:id="2" w:author="Probe"/></w:rPr></w:pPr><w:r><w:t>HISTORY</w:t></w:r></w:p>"#,
      r#"<w:p><w:ins w:id="2" w:author="Probe"><w:r><w:t>HISTORY</w:t></w:r></w:ins></w:p>"#,
      r#"<w:p><w:r><w:rPr><w:rPrChange w:id="2" w:author="Probe"><w:rPr/></w:rPrChange></w:rPr><w:t>HISTORY</w:t></w:r></w:p>"#,
      r#"<w:p><w:pPr><w:pPrChange w:id="2" w:author="Probe"><w:pPr/></w:pPrChange></w:pPr><w:r><w:t>HISTORY</w:t></w:r></w:p>"#,
    ] {
      assert_eq!(
        imported_text(&format!("{marker}{content}")),
        "HISTORYLEFTARIGHTAFTER"
      );
      assert_eq!(
        imported_text(&format!("{content}{marker}")),
        "LEFTAREMOVEDBRIGHTAFTERHISTORY"
      );
    }
  }

  #[test]
  fn native_simple_field_cache_is_consumed_in_source_order() {
    // Six configured Office PDF/XPS controls, including GREETINGLINE's
    // speculative cached-result path and locked/unknown field results.
    let mixed = "<w:r><w:t>A</w:t><w:delText>REMOVED</w:delText><w:t>B</w:t></w:r>";
    let marker = r#"<w:r><w:rPr><w:rPrChange w:id="2" w:author="Probe"><w:rPr/></w:rPrChange></w:rPr><w:t>HISTORY</w:t></w:r>"#;
    for (instruction, locked) in [(" GREETINGLINE ", 0), (" GREETINGLINE ", 1), (" TEST ", 1)] {
      for (content, expected) in [
        (format!("{marker}{mixed}"), "LEFTHISTORYARIGHTEND"),
        (format!("{mixed}{marker}"), "LEFTAREMOVEDBHISTORYRIGHTEND"),
      ] {
        let body = format!(
          r#"<w:p><w:r><w:t>LEFT</w:t></w:r><w:fldSimple w:instr="{instruction}" w:fldLock="{locked}">{content}</w:fldSimple><w:r><w:t>RIGHT</w:t></w:r><w:r><w:delText>TAIL</w:delText></w:r><w:r><w:t>END</w:t></w:r></w:p>"#
        );
        assert_eq!(imported_text(&body), expected, "{instruction}/{locked}");
      }
    }
  }

  #[test]
  fn revision_recovery_retains_authored_xml_for_roundtrip() {
    let xml = br#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:t>A</w:t><w:delText>REMOVED</w:delText><w:t>B</w:t></w:r>"#;
    let run = w::Run::from_bytes(xml).expect("mixed run");
    let context = super::Context::default();
    context.observe();
    let source_before = run.to_xml().expect("source XML before recovery");
    let recovered = super::run(&run, Some(&context)).expect("visible prefix");
    assert_eq!(recovered.run_choice.len(), 1);
    assert_eq!(
      run.to_xml().expect("source XML after recovery"),
      source_before
    );
  }
}
