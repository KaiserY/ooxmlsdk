//! Preserve Word's source-order table-style reset before typed XML import.

use std::ops::Range;

use quick_xml::{events::Event, name::ResolveResult};

use super::StylesCatalog;

const WORD: &[u8] = b"http://schemas.openxmlformats.org/wordprocessingml/2006/main";

struct Properties {
  depth: usize,
  positions: Vec<Range<usize>>,
  open_position: Option<usize>,
}

pub(super) fn xml(xml: &[u8], styles: &StylesCatalog) -> Option<Vec<u8>> {
  // Normal-order tables need no repair. Most stories have no floating table,
  // so avoid another XML walk unless a style follows some positioning markup.
  let position = xml.windows(b"tblpPr".len()).position(|s| s == b"tblpPr")?;
  if !xml[position..]
    .windows(b"tblStyle".len())
    .any(|s| s == b"tblStyle")
  {
    return None;
  }
  let mut reader = quick_xml::NsReader::from_reader(xml);
  let mut parents = Vec::new();
  let mut properties: Vec<Properties> = Vec::new();
  let mut removals = Vec::new();
  loop {
    let start = usize::try_from(reader.buffer_position()).ok()?;
    match reader.read_resolved_event().ok()? {
      (namespace, event @ (Event::Start(_) | Event::Empty(_))) => {
        let word = matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORD);
        let empty = matches!(event, Event::Empty(_));
        let (Event::Start(element) | Event::Empty(element)) = event else {
          unreachable!()
        };
        let local = element.local_name();
        if word && local.as_ref() == b"tblPr" && parents.last() == Some(&true) && !empty {
          properties.push(Properties {
            depth: parents.len() + 1,
            positions: Vec::new(),
            open_position: None,
          });
        } else if word
          && let Some(properties) = properties.last_mut()
          && properties.depth == parents.len()
        {
          if local.as_ref() == b"tblpPr" {
            if empty {
              properties
                .positions
                .push(start..usize::try_from(reader.buffer_position()).ok()?);
            } else {
              properties.open_position = Some(start);
            }
          } else if local.as_ref() == b"tblStyle" && !properties.positions.is_empty() {
            for attribute in element.attributes() {
              let attribute = attribute.ok()?;
              let (namespace, name) = reader.resolver().resolve_attribute(attribute.key);
              if name.as_ref() == b"val"
                && matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORD)
                && styles.has_table_style(
                  &attribute
                    .decoded_and_normalized_value(
                      quick_xml::XmlVersion::Implicit1_0,
                      reader.decoder(),
                    )
                    .ok()?,
                )
              {
                // Native PDF, Rows.WrapAroundText and WordOpenXML controls:
                // applying a resolved table style resets preceding tblpPr,
                // even for an empty style. Unknown IDs and paragraph styles
                // do not reset it; later tblpPr remains effective. This is
                // Word's repair of noncanonical CT_TblPr child order, not a
                // change to shared schema parsing or the source package.
                removals.append(&mut properties.positions);
                break;
              }
            }
          }
        }
        if !empty {
          parents.push(word && local.as_ref() == b"tbl");
        }
      }
      (_, Event::End(_)) => {
        if let Some(properties) = properties.last_mut()
          && properties.depth + 1 == parents.len()
          && let Some(start) = properties.open_position.take()
        {
          properties
            .positions
            .push(start..usize::try_from(reader.buffer_position()).ok()?);
        }
        if properties.last().is_some_and(|p| p.depth == parents.len()) {
          properties.pop();
        }
        parents.pop()?;
      }
      (_, Event::Eof) => break,
      _ => {}
    }
  }
  if removals.is_empty() {
    return None;
  }
  removals.sort_unstable_by_key(|span| span.start);
  let mut recovered = Vec::with_capacity(xml.len());
  let mut previous = 0;
  for span in removals {
    recovered.extend_from_slice(xml.get(previous..span.start)?);
    previous = span.end;
  }
  recovered.extend_from_slice(xml.get(previous..)?);
  Some(recovered)
}

#[cfg(test)]
mod tests {
  use ooxmlsdk::sdk::SdkType;

  use super::super::{StyleEntry, drawing_position_recovery, w};
  use super::*;

  fn styles() -> StylesCatalog {
    let mut styles = StylesCatalog::default();
    for (id, kind) in [
      ("Table&One", w::StyleValues::Table),
      ("Normal", w::StyleValues::Paragraph),
    ] {
      styles.styles.insert(
        id.to_string(),
        StyleEntry {
          style_type: Some(kind),
          ..Default::default()
        },
      );
    }
    styles
  }

  #[test]
  fn native_table_style_reset_depends_on_order_and_resolved_style_type() {
    for style in ["Table&amp;One", "Normal", "Missing"] {
      for first in [false, true] {
        for position in [
          r#"<w:tblpPr w:tblpY="6000"/>"#,
          r#"<w:tblpPr w:tblpY="6000"></w:tblpPr>"#,
        ] {
          let reference = format!(r#"<w:tblStyle w:val="{style}"/>"#);
          let children = if first {
            format!("{position}{reference}")
          } else {
            format!("{reference}{position}")
          };
          let source = format!(
            r#"<w:tbl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:tblPr>{children}<w:tblW w:w="5000" w:type="pct"/></w:tblPr></w:tbl>"#
          );
          let original = w::Table::from_bytes(source.as_bytes()).unwrap();
          let imported =
            drawing_position_recovery::root(source.as_bytes(), &styles(), || Ok(&original))
              .unwrap();
          let removed = first && style == "Table&amp;One";
          assert_eq!(
            imported
              .table_properties
              .as_ref()
              .unwrap()
              .table_position_properties
              .is_none(),
            removed,
            "{source}"
          );
          assert!(
            original
              .table_properties
              .as_ref()
              .unwrap()
              .table_position_properties
              .is_some()
          );
          assert_eq!(matches!(imported, std::borrow::Cow::Owned(_)), removed);
        }
      }
    }
  }

  #[test]
  fn table_style_reset_is_scoped_to_direct_properties_and_namespace() {
    let position = r#"<a:tblpPr a:tblpY="6000"/>"#;
    let style = r#"<a:tblStyle a:val="Table&amp;One"/>"#;
    let xml = format!(
      r#"<root xmlns:a="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:f="urn:foreign"><a:tbl><a:tblPr>{position}{style}<a:tblpPr a:tblpY="8000"/></a:tblPr><a:tr><a:tc><a:tbl><a:tblPr>{position}<f:tblStyle a:val="Table&amp;One"/></a:tblPr></a:tbl></a:tc></a:tr></a:tbl><a:tbl><a:tblPr>{position}<a:tblStyle f:val="Table&amp;One"/></a:tblPr></a:tbl><a:tblPr>{position}{style}</a:tblPr></root>"#
    );
    let expected = xml.replacen(&format!("{position}{style}"), style, 1);
    assert_eq!(
      super::xml(xml.as_bytes(), &styles()).unwrap(),
      expected.as_bytes()
    );
    assert!(super::xml(expected.as_bytes(), &styles()).is_none());
  }
}
