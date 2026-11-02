//! Word's style/list association and out-of-place numbering-property recovery.
//! MS-OI29500 §2.1.50 documents Word's departure from the standard's style ilvl
//! rule. Native controls distinguish source order, built-ins, explicit pStyle,
//! inherited properties and duplicate abstract-list claims. Writer's
//! DomainMapper_Impl::ValidateListLevel independently documents the conflict.

use std::{collections::HashMap, ops::Range};

use quick_xml::{events::Event, name::ResolveResult};

use super::{NumberingCatalog, StylesCatalog, w};

pub(super) fn reconcile_styles(styles: &mut StylesCatalog, numbering: &NumberingCatalog) {
  let mut claimed = HashMap::new();
  for (&id, definition) in &numbering.abstract_nums {
    for (&level, properties) in &definition.levels {
      if let Some(owner) = &properties.paragraph_style_id
        && styles.styles.contains_key(owner)
      {
        claimed.insert((id, level), owner.to_string());
      }
    }
  }
  let mut ordered = styles
    .styles
    .iter()
    .map(|(id, entry)| {
      let builtin = if entry.custom_style {
        None
      } else {
        entry.name.as_deref().and_then(|name| {
          if name.eq_ignore_ascii_case("Normal") {
            return Some(0);
          }
          if super::is_reserved_office_heading_style_name(name) {
            return name
              .rsplit_once(' ')
              .and_then(|(_, level)| level.parse::<usize>().ok());
          }
          None
        })
      };
      (
        (builtin.unwrap_or(usize::MAX), entry.source_order),
        id.clone(),
      )
    })
    .collect::<Vec<_>>();
  ordered.sort_by(|(a, aid), (b, bid)| a.cmp(b).then_with(|| aid.cmp(bid)));
  for (_, id) in ordered {
    let entry = &styles.styles[&id];
    if entry.style_type != Some(w::StyleValues::Paragraph) || entry.paragraph_numbering.is_none() {
      continue;
    }
    let Some(reference) = styles.paragraph_numbering_reference(Some(&id)) else {
      continue;
    };
    if !(0..=8).contains(&reference.level_index()) || reference.num_id() == 0 {
      continue;
    }
    // Redundant properties do not make a descendant a second list-level owner.
    // Keep inheritance intact even when the parent's explicit numPr is removed
    // by Word's serializer; a genuinely conflicting child cancels both slots.
    let parent = entry
      .based_on
      .as_deref()
      .and_then(|parent| styles.paragraph_numbering_reference(Some(parent)));
    if parent.is_some_and(|parent| {
      parent.num_id() == reference.num_id() && parent.level_index() == reference.level_index()
    }) {
      continue;
    }
    let Some(instance) = numbering.nums.get(&reference.num_id()) else {
      continue;
    };
    let key = (instance.abstract_num_id, reference.level_index());
    match claimed.get(&key) {
      None => {
        claimed.insert(key, id);
      }
      Some(owner) if owner == &id => {}
      Some(_) => {
        styles.styles.get_mut(&id).unwrap().paragraph_numbering =
          Some(Box::new(w::NumberingProperties {
            numbering_id: Some(w::NumberingId { val: 0 }),
            numbering_level_reference: Some(w::NumberingLevelReference { val: 0 }),
            ..Default::default()
          }));
      }
    }
  }
}

const WORD: &[u8] = b"http://schemas.openxmlformats.org/wordprocessingml/2006/main";

struct Child {
  span: Range<usize>,
  // A valid numPr keeps its entire opening/closing tags and namespace bindings.
  contents: Option<Range<usize>>,
  orphan: Option<Vec<u8>>,
}

struct ParagraphProperties {
  depth: usize,
  children: Vec<Child>,
  open_child: Option<(usize, Child)>,
}

/// Recover only direct w:pPr children. Keep source bytes, shared schema parsing,
/// and source packages unchanged. Last authored numId/ilvl wins, whether inside
/// or outside numPr; a foreign-namespace lookalike never participates.
pub(super) fn xml(xml: &[u8]) -> Option<Vec<u8>> {
  if ![b"numId".as_slice(), b"ilvl".as_slice()]
    .iter()
    .any(|name| xml.windows(name.len()).any(|part| part == *name))
  {
    return None;
  }
  let mut reader = quick_xml::NsReader::from_reader(xml);
  let mut depth = 0;
  let mut paragraphs: Vec<ParagraphProperties> = Vec::new();
  let mut replacements = Vec::new();
  loop {
    let start = usize::try_from(reader.buffer_position()).ok()?;
    match reader.read_resolved_event().ok()? {
      (namespace, event @ (Event::Start(_) | Event::Empty(_))) => {
        let word = matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORD);
        let empty = matches!(event, Event::Empty(_));
        let (Event::Start(element) | Event::Empty(element)) = event else {
          unreachable!()
        };
        let end = usize::try_from(reader.buffer_position()).ok()?;
        let local = element.local_name();
        if word && local.as_ref() == b"pPr" && !empty {
          paragraphs.push(ParagraphProperties {
            depth: depth + 1,
            children: Vec::new(),
            open_child: None,
          });
        } else if word
          && let Some(paragraph) = paragraphs.last_mut()
          && paragraph.depth == depth
        {
          let is_container = local.as_ref() == b"numPr";
          let is_orphan = matches!(local.as_ref(), b"numId" | b"ilvl");
          if is_container || is_orphan {
            let orphan = if is_orphan {
              // Use a local namespace declaration so moving a child into an
              // existing numPr cannot accidentally change its QName binding.
              let value = element.attributes().filter_map(Result::ok).find(|attr| {
                let (ns, local) = reader.resolver().resolve_attribute(attr.key);
                local.as_ref() == b"val"
                  && matches!(ns, ResolveResult::Bound(uri) if uri.as_ref() == WORD)
              })?;
              let mut recovered = quick_xml::events::BytesStart::new(format!(
                "n:{}",
                std::str::from_utf8(local.as_ref()).ok()?
              ));
              recovered.push_attribute((b"xmlns:n".as_slice(), WORD));
              recovered.push_attribute((b"n:val".as_slice(), value.value.as_ref()));
              let mut writer = quick_xml::Writer::new(Vec::new());
              writer.write_event(Event::Empty(recovered)).ok()?;
              Some(writer.into_inner())
            } else {
              None
            };
            let child = Child {
              span: start..end,
              contents: is_container.then_some(end..end),
              orphan,
            };
            if empty {
              paragraph.children.push(child);
            } else {
              paragraph.open_child = Some((depth + 1, child));
            }
          }
        }
        if !empty {
          depth += 1;
        }
      }
      (_, Event::End(_)) => {
        if let Some(paragraph) = paragraphs.last_mut() {
          if paragraph
            .open_child
            .as_ref()
            .is_some_and(|(child_depth, _)| *child_depth == depth)
          {
            let (_, mut child) = paragraph.open_child.take()?;
            child.span.end = usize::try_from(reader.buffer_position()).ok()?;
            if let Some(contents) = &mut child.contents {
              contents.end = start;
            }
            paragraph.children.push(child);
          }
          if paragraph.depth == depth {
            let paragraph = paragraphs.pop()?;
            recover_paragraph(xml, paragraph, &mut replacements);
          }
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
  replacements.sort_by_key(|(span, _)| span.start);
  let mut output = Vec::with_capacity(xml.len());
  let mut previous = 0;
  for (span, replacement) in replacements {
    output.extend_from_slice(xml.get(previous..span.start)?);
    output.extend_from_slice(&replacement);
    previous = span.end;
  }
  output.extend_from_slice(xml.get(previous..)?);
  Some(output)
}

fn recover_paragraph(
  xml: &[u8],
  paragraph: ParagraphProperties,
  replacements: &mut Vec<(Range<usize>, Vec<u8>)>,
) {
  let children = paragraph.children;
  if !children.iter().any(|child| child.orphan.is_some()) {
    return;
  }
  let containers = children
    .iter()
    .filter(|child| child.contents.is_some())
    .collect::<Vec<_>>();
  // Multiple valid numPr elements are a separate malformed-container policy.
  // Do not change their existing typed-reader selection as a side effect.
  if containers.len() > 1 {
    return;
  }
  let mut recovered = Vec::new();
  let mut closing = b"</n:numPr>".as_slice();
  if let Some(container) = containers.first() {
    let contents = container.contents.as_ref().unwrap();
    if xml[container.span.clone()].ends_with(b"/>") {
      recovered.extend_from_slice(&xml[container.span.start..container.span.end - 2]);
      recovered.push(b'>');
      // Empty valid containers need the matching original prefix on close.
      let tag_end = xml[container.span.start + 1..]
        .iter()
        .position(|byte| byte.is_ascii_whitespace() || *byte == b'/')
        .unwrap();
      let name = &xml[container.span.start + 1..container.span.start + 1 + tag_end];
      for child in &children {
        if let Some(orphan) = &child.orphan {
          recovered.extend_from_slice(orphan);
        }
      }
      recovered.extend_from_slice(b"</");
      recovered.extend_from_slice(name);
      recovered.push(b'>');
      for (index, child) in children.iter().enumerate() {
        replacements.push((
          child.span.clone(),
          if index == 0 {
            recovered.clone()
          } else {
            Vec::new()
          },
        ));
      }
      return;
    }
    recovered.extend_from_slice(&xml[container.span.start..contents.start]);
    closing = &xml[contents.end..container.span.end];
  } else {
    recovered.extend_from_slice(
      b"<n:numPr xmlns:n=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">",
    );
  }
  for child in &children {
    if let Some(orphan) = &child.orphan {
      recovered.extend_from_slice(orphan);
    } else if let Some(contents) = &child.contents {
      recovered.extend_from_slice(&xml[contents.clone()]);
    }
  }
  recovered.extend_from_slice(closing);
  for (index, child) in children.iter().enumerate() {
    replacements.push((
      child.span.clone(),
      if index == 0 {
        recovered.clone()
      } else {
        Vec::new()
      },
    ));
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::{
    AbstractNumbering, NumberingInstance, NumberingReference, StyleEntry,
    select_paragraph_numbering,
  };
  use ooxmlsdk::sdk::SdkType;

  #[test]
  fn numbering_slots_cascade_independently_and_keep_body_level() {
    for (direct_id, direct_level, style_id, style_level, expected) in [
      (Some(1), None, Some(2), Some(4), Some((1, 4))),
      (Some(1), None, Some(1), Some(9), Some((1, 9))),
      (None, Some(1), Some(2), Some(4), Some((2, 1))),
      (Some(3), None, None, Some(2), Some((3, 2))),
      (None, Some(3), None, Some(1), None),
      (Some(0), None, Some(2), Some(4), None),
      (None, Some(3), Some(0), Some(1), None),
    ] {
      let direct = NumberingReference {
        num_id: direct_id,
        level_index: direct_level,
      };
      let style = NumberingReference {
        num_id: style_id,
        level_index: style_level,
      };
      let (resolved, _) = select_paragraph_numbering(Some(direct), Some(style));
      assert_eq!(
        resolved.map(|value| (value.num_id(), value.level_index())),
        expected
      );
    }
  }

  fn style(
    name: &str,
    order: usize,
    parent: Option<&str>,
    num: Option<i32>,
    level: Option<i32>,
  ) -> StyleEntry {
    StyleEntry {
      source_order: order,
      style_type: Some(w::StyleValues::Paragraph),
      name: Some(name.into()),
      based_on: parent.map(str::to_owned),
      paragraph_numbering: Some(Box::new(w::NumberingProperties {
        numbering_id: num.map(|val| w::NumberingId { val }),
        numbering_level_reference: level.map(|val| w::NumberingLevelReference { val }),
        ..Default::default()
      })),
      ..Default::default()
    }
  }

  #[test]
  fn numbering_style_claims_use_abstract_list_and_native_style_order() {
    let numbering = NumberingCatalog {
      nums: HashMap::from([
        (
          1,
          NumberingInstance {
            abstract_num_id: 7,
            overrides: HashMap::new(),
          },
        ),
        (
          2,
          NumberingInstance {
            abstract_num_id: 7,
            overrides: HashMap::new(),
          },
        ),
      ]),
      abstract_nums: HashMap::from([(7, AbstractNumbering::default())]),
      ..Default::default()
    };
    let mut styles = StylesCatalog {
      styles: HashMap::from([
        (
          "first-in-file".into(),
          style("Zulu", 0, None, Some(2), Some(4)),
        ),
        (
          "heading".into(),
          style("Heading 5", 1, None, Some(1), Some(4)),
        ),
        (
          "child".into(),
          style("Child", 2, Some("heading"), Some(1), Some(4)),
        ),
        (
          "partial".into(),
          style("Partial", 3, Some("heading"), None, Some(2)),
        ),
        (
          "duplicate".into(),
          style("Duplicate", 4, Some("heading"), Some(1), Some(2)),
        ),
      ]),
      ..Default::default()
    };
    reconcile_styles(&mut styles, &numbering);
    for (id, expected) in [
      ("first-in-file", (0, 0)),
      ("heading", (1, 4)),
      ("child", (1, 4)),
      ("partial", (1, 2)),
      ("duplicate", (0, 0)),
    ] {
      let reference = styles.paragraph_numbering_reference(Some(id)).unwrap();
      assert_eq!(
        (reference.num_id(), reference.level_index()),
        expected,
        "{id}"
      );
    }
    // The custom flag removes built-in priority; display names and ids do not
    // replace authored order for custom styles.
    styles.styles.get_mut("heading").unwrap().custom_style = true;
    styles.styles.insert(
      "first-in-file".into(),
      style("Zulu", 0, None, Some(2), Some(4)),
    );
    reconcile_styles(&mut styles, &numbering);
    assert_eq!(
      styles
        .paragraph_numbering_reference(Some("heading"))
        .unwrap()
        .num_id(),
      0
    );
  }

  #[test]
  fn numbering_explicit_style_link_precedes_implicit_claims() {
    let level = w::Level::from_bytes(br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="4"><w:pStyle w:val="late"/></w:lvl>"#).unwrap();
    let numbering = NumberingCatalog {
      nums: HashMap::from([(
        1,
        NumberingInstance {
          abstract_num_id: 7,
          overrides: HashMap::new(),
        },
      )]),
      abstract_nums: HashMap::from([(
        7,
        AbstractNumbering {
          levels: HashMap::from([(
            4,
            super::super::numbering_level_model(&level, Default::default()),
          )]),
          ..Default::default()
        },
      )]),
      ..Default::default()
    };
    let mut styles = StylesCatalog {
      styles: HashMap::from([
        (
          "early".into(),
          style("Heading 4", 0, None, Some(1), Some(4)),
        ),
        ("late".into(), style("Heading 5", 1, None, Some(1), Some(4))),
      ]),
      ..Default::default()
    };
    reconcile_styles(&mut styles, &numbering);
    assert_eq!(
      styles
        .paragraph_numbering_reference(Some("early"))
        .unwrap()
        .num_id(),
      0
    );
    assert_eq!(
      styles
        .paragraph_numbering_reference(Some("late"))
        .unwrap()
        .level_index(),
      4
    );
  }

  #[test]
  fn orphan_numbering_properties_recover_in_source_order() {
    for (children, id, level) in [
      ("<w:numId w:val=\"0\"/>", Some(0), None),
      ("<w:ilvl w:val=\"2\"/>", None, Some(2)),
      (
        "<w:numPr><w:numId w:val=\"1\"/><w:ilvl w:val=\"4\"/></w:numPr><w:numId w:val=\"0\"/>",
        Some(0),
        Some(4),
      ),
      (
        "<w:numId w:val=\"0\"/><w:numPr><w:numId w:val=\"1\"/></w:numPr>",
        Some(1),
        None,
      ),
      ("<w:numPr/><w:ilvl w:val=\"1\"/>", None, Some(1)),
      (
        "<w:numId w:val=\"3\"></w:numId><w:numPr><w:ilvl w:val=\"5\"/></w:numPr>",
        Some(3),
        Some(5),
      ),
      (
        "<w:numId w:val=\"2\"/><w:numPr xmlns:n=\"foreign\"><w:ilvl w:val=\"1\"/></w:numPr>",
        Some(2),
        Some(1),
      ),
    ] {
      let source = format!(
        "<w:pPr xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">{children}<w:keepNext/></w:pPr>"
      );
      let recovered = xml(source.as_bytes()).expect("orphan recovery");
      let properties = w::ParagraphProperties::from_bytes(&recovered).unwrap();
      let numbering = properties.numbering_properties.unwrap();
      assert_eq!(
        numbering.numbering_id.map(|value| value.val),
        id,
        "{children}"
      );
      assert_eq!(
        numbering.numbering_level_reference.map(|value| value.val),
        level,
        "{children}"
      );
      assert!(properties.keep_next.is_some());
    }
    for source in [
      r#"<pPr xmlns="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><numPr><numId xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:val="1"/></numPr></pPr>"#,
      r#"<w:pPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:f="foreign"><f:numId w:val="0"/></w:pPr>"#,
    ] {
      assert!(xml(source.as_bytes()).is_none());
    }
  }
}
