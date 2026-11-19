//! Word interprets note-story page/column breaks as text-wrapping breaks.
//! Recover the type before run import so formatting, field events and bidi
//! ownership follow the ordinary soft-break path. Table cells keep their own
//! ignored-break rules; this is not a change to shared XML/package parsing.

use std::borrow::Cow;

use quick_xml::{events::Event, name::ResolveResult};

fn word_namespace(namespace: &ResolveResult<'_>) -> bool {
  matches!(namespace, ResolveResult::Bound(uri) if matches!(uri.as_ref(),
    b"http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    | b"http://purl.oclc.org/ooxml/wordprocessingml/main"))
}

pub(super) fn xml(xml: &[u8]) -> Option<Vec<u8>> {
  let mut reader = quick_xml::NsReader::from_reader(xml);
  let mut depth = 0usize;
  let mut normal_note = None;
  let mut excluded = None;
  let mut replacements = Vec::new();
  loop {
    let start = usize::try_from(reader.buffer_position()).ok()?;
    match reader.read_resolved_event().ok()? {
      (namespace, event @ (Event::Start(_) | Event::Empty(_))) => {
        let word = word_namespace(&namespace);
        let empty = matches!(event, Event::Empty(_));
        let (Event::Start(element) | Event::Empty(element)) = event else {
          unreachable!()
        };
        let name = element.local_name();
        if depth == 0 && !(word && matches!(name.as_ref(), b"footnotes" | b"endnotes")) {
          return None;
        }
        if !empty {
          depth += 1;
          if word && matches!(name.as_ref(), b"footnote" | b"endnote") {
            let mut normal = true;
            for attribute in element.attributes() {
              let attribute = attribute.ok()?;
              let (namespace, local) = reader.resolver().resolve_attribute(attribute.key);
              if word_namespace(&namespace) && local.as_ref() == b"type" {
                normal = attribute
                  .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                  .ok()?
                  == "normal";
              }
            }
            if normal {
              normal_note = Some(depth);
            }
          }
          if excluded.is_none() && word && matches!(name.as_ref(), b"tbl" | b"txbxContent") {
            excluded = Some(depth);
          }
        }
        if !word || name.as_ref() != b"br" || normal_note.is_none() || excluded.is_some() {
          continue;
        }
        let mut hard_break = false;
        for attribute in element.attributes() {
          let attribute = attribute.ok()?;
          let (namespace, local) = reader.resolver().resolve_attribute(attribute.key);
          if word_namespace(&namespace) && local.as_ref() == b"type" {
            let value = attribute
              .normalized_value(quick_xml::XmlVersion::Implicit1_0)
              .ok()?;
            hard_break = matches!(value.as_ref(), "page" | "column");
          }
        }
        if !hard_break {
          continue;
        }
        let mut recovered = element.to_owned();
        recovered.clear_attributes();
        for attribute in element.attributes() {
          let mut attribute = attribute.ok()?;
          let (namespace, local) = reader.resolver().resolve_attribute(attribute.key);
          if word_namespace(&namespace) {
            if local.as_ref() == b"clear" {
              // ECMA-376 §17.3.3.1: clear has no effect on a page/column break.
              continue;
            }
            if local.as_ref() == b"type" {
              attribute.value = Cow::Borrowed(b"textWrapping");
            }
          }
          recovered.push_attribute(attribute);
        }
        let mut writer = quick_xml::Writer::new(Vec::new());
        writer
          .write_event(if empty {
            Event::Empty(recovered)
          } else {
            Event::Start(recovered)
          })
          .ok()?;
        replacements.push((
          start..usize::try_from(reader.buffer_position()).ok()?,
          writer.into_inner(),
        ));
      }
      (_, Event::End(_)) => {
        if normal_note == Some(depth) {
          normal_note = None;
        }
        if excluded == Some(depth) {
          excluded = None;
        }
        depth = depth.checked_sub(1)?;
      }
      (_, Event::Eof) => break,
      _ => {}
    }
  }
  if replacements.is_empty() {
    return None;
  }
  let mut recovered = Vec::with_capacity(xml.len());
  let mut previous = 0;
  for (span, replacement) in replacements {
    recovered.extend_from_slice(&xml[previous..span.start]);
    recovered.extend_from_slice(&replacement);
    previous = span.end;
  }
  recovered.extend_from_slice(&xml[previous..]);
  Some(recovered)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_note_breaks_preserve_run_formatting_and_qualified_attributes() {
    for namespace in [
      "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
      "http://purl.oclc.org/ooxml/wordprocessingml/main",
    ] {
      for story in ["footnote", "endnote"] {
        for kind in ["page", "column"] {
          let source = format!(
            r#"<x:{story}s xmlns:x="{namespace}" xmlns:other="urn:other"><x:{story} x:id="1"><x:p><x:r><x:rPr><x:sz x:val="80"/></x:rPr><x:t>LEFT</x:t><x:br x:type="{kind}" x:clear="all" other:type="retained"/><x:t>RIGHT</x:t></x:r></x:p></x:{story}></x:{story}s>"#
          );
          let recovered = String::from_utf8(xml(source.as_bytes()).unwrap()).unwrap();
          assert_eq!(
            recovered,
            source.replace(
              &format!(r#"x:type="{kind}" x:clear="all""#),
              r#"x:type="textWrapping""#
            )
          );
        }
      }
    }
  }

  #[test]
  fn native_note_table_breaks_and_other_stories_keep_their_owners() {
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    for root in ["document", "hdr", "ftr"] {
      let source = format!(
        r#"<w:{root} xmlns:w="{namespace}"><w:p><w:r><w:br w:type="page"/></w:r></w:p></w:{root}>"#
      );
      assert!(xml(source.as_bytes()).is_none());
    }
    let source = format!(
      r#"<w:endnotes xmlns:w="{namespace}"><w:endnote w:type="separator" w:id="-1"><w:p><w:r><w:br w:type="page"/></w:r></w:p></w:endnote><w:endnote w:id="1"><w:tbl><w:tr><w:tc><w:p><w:r><w:br w:type="page"/><w:br w:type="column"/></w:r></w:p></w:tc></w:tr></w:tbl><w:p><w:r><w:br w:type="textWrapping" w:clear="all"/></w:r></w:p></w:endnote></w:endnotes>"#
    );
    assert!(xml(source.as_bytes()).is_none());
  }
}
