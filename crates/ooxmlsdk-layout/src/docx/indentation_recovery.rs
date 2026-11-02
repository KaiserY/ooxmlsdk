//! Word ignores the SDK's unqualified `left` read alias on w:ind. Keep that
//! Office import policy separate from the SDK's compatible XML round-trip.

use std::borrow::Cow;

use ooxmlsdk::{common::SdkError, sdk::SdkType};
use quick_xml::{events::Event, name::ResolveResult};

pub(super) fn root<'a, T: SdkType + Clone>(
  xml: &[u8],
  original: impl FnOnce() -> std::result::Result<&'a T, SdkError>,
) -> crate::Result<Cow<'a, T>> {
  match self::xml(xml) {
    Some(recovered) => Ok(Cow::Owned(T::from_bytes(&recovered)?)),
    None => Ok(Cow::Borrowed(original()?)),
  }
}

pub(super) fn xml(xml: &[u8]) -> Option<Vec<u8>> {
  // Qualified w:left is common; only a standalone attribute can need this
  // lexical pass. Ordinary imports retain the existing borrowed typed root.
  if !xml
    .windows(5)
    .any(|bytes| bytes[0].is_ascii_whitespace() && &bytes[1..] == b"left")
  {
    return None;
  }
  let mut reader = quick_xml::NsReader::from_reader(xml);
  let mut replacements = Vec::new();
  loop {
    let start = usize::try_from(reader.buffer_position()).ok()?;
    match reader.read_resolved_event().ok()? {
      (namespace, event @ (Event::Start(_) | Event::Empty(_))) => {
        let word = matches!(namespace, ResolveResult::Bound(uri) if matches!(uri.as_ref(),
          b"http://schemas.openxmlformats.org/wordprocessingml/2006/main"
          | b"http://purl.oclc.org/ooxml/wordprocessingml/main"));
        let empty = matches!(event, Event::Empty(_));
        let (Event::Start(element) | Event::Empty(element)) = event else {
          unreachable!()
        };
        if !word
          || element.local_name().as_ref() != b"ind"
          || !element
            .attributes()
            .flatten()
            .any(|attribute| attribute.key.as_ref() == b"left")
        {
          continue;
        }
        let mut recovered = element.to_owned();
        recovered.clear_attributes();
        for attribute in element.attributes() {
          let attribute = attribute.ok()?;
          if attribute.key.as_ref() != b"left" {
            recovered.push_attribute(attribute);
          }
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
  use super::super::w;
  use super::*;

  #[test]
  fn office_indent_import_preserves_the_sdk_read_alias_contract() {
    for (attributes, expected, changed) in [
      (r#"left="2880""#, None, true),
      (r#"left = '2880'"#, None, true),
      (r#"w:left="1440" left="2880""#, Some("1440"), true),
      (r#"left="2880" w:left="1440""#, Some("1440"), true),
      (r#"w:left="1440""#, Some("1440"), false),
    ] {
      let xml = format!(
        r#"<w:ind xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" {attributes}/>"#
      );
      let sdk = w::Indentation::from_bytes(xml.as_bytes()).unwrap();
      assert!(sdk.left.is_some(), "SDK read aliases remain available");
      let office = root(xml.as_bytes(), || Ok(&sdk)).unwrap();
      assert_eq!(
        office.left.as_ref().map(ToString::to_string).as_deref(),
        expected
      );
      assert_eq!(matches!(office, Cow::Owned(_)), changed);
    }
  }

  #[test]
  fn indentation_alias_filter_respects_element_and_attribute_namespaces() {
    for namespace in [
      "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
      "http://purl.oclc.org/ooxml/wordprocessingml/main",
    ] {
      for prefix in ["alternate:", ""] {
        let declaration = if prefix.is_empty() {
          "xmlns"
        } else {
          "xmlns:alternate"
        };
        let xml =
          format!(r#"<{prefix}ind {declaration}="{namespace}" left="2880" other="unchanged"/>"#);
        let expected = xml.replace(" left=\"2880\"", "");
        assert_eq!(self::xml(xml.as_bytes()).unwrap(), expected.as_bytes());
      }
    }
    for xml in [
      r#"<x:ind xmlns:x="urn:other" left="2880"/>"#,
      r#"<w:other xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" left="2880"/>"#,
      r#"<w:ind xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:left="1440"/>"#,
      r#"<w:ind xmlns:w="urn:other" left="2880"/>"#,
    ] {
      assert!(self::xml(xml.as_bytes()).is_none(), "{xml}");
    }
  }
}
