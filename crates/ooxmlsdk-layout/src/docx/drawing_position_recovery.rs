//! Word's lexical recovery of DrawingML integer position offsets.
//!
//! XML Schema permits whitespace around an integer, but Word imports a
//! wp:posOffset containing whitespace as zero. Preserve that distinction before
//! the SDK's typed parser collapses the lexical whitespace. This is a layout
//! import policy; neither the source package nor shared XML parsing is changed.

use std::borrow::Cow;

use ooxmlsdk::{common::SdkError, sdk::SdkType};
use quick_xml::{events::Event, name::ResolveResult};

const WORDPROCESSING_DRAWING: &[u8] =
  b"http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing";

pub(super) fn root<'a, T: SdkType + Clone>(
  xml: &[u8],
  original: impl FnOnce() -> std::result::Result<&'a T, SdkError>,
) -> crate::Result<Cow<'a, T>> {
  match recover_xml(xml) {
    Some(recovered) => Ok(Cow::Owned(T::from_bytes(&recovered)?)),
    None => Ok(Cow::Borrowed(original()?)),
  }
}

fn recover_xml(xml: &[u8]) -> Option<Vec<u8>> {
  if !xml
    .windows(b"posOffset".len())
    .any(|word| word == b"posOffset")
  {
    return None;
  }
  let mut reader = quick_xml::NsReader::from_reader(xml);
  let mut depth = 0usize;
  let mut position = None;
  let mut replacements = Vec::new();
  loop {
    let event_start = usize::try_from(reader.buffer_position()).ok()?;
    match reader.read_resolved_event().ok()? {
      (namespace, Event::Start(element)) => {
        depth += 1;
        if element.local_name().as_ref() == b"posOffset"
          && matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORDPROCESSING_DRAWING)
        {
          position = Some((depth, usize::try_from(reader.buffer_position()).ok()?));
        }
      }
      (_, Event::End(_)) => {
        if let Some((position_depth, start)) = position
          && position_depth == depth
        {
          let lexical = std::str::from_utf8(xml.get(start..event_start)?).ok()?;
          let lexical = quick_xml::escape::unescape(lexical).ok()?;
          if lexical.bytes().any(|byte| byte.is_ascii_whitespace())
            && lexical.bytes().any(|byte| byte.is_ascii_digit())
            && lexical.bytes().all(|byte| {
              byte.is_ascii_digit() || byte.is_ascii_whitespace() || matches!(byte, b'+' | b'-')
            })
          {
            replacements.push(start..event_start);
          }
          position = None;
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
  // Copy unchanged byte ranges: namespace prefixes, MCE and colliding child
  // QNames must remain exactly as authored. No whole-tree reserialization.
  let mut recovered = Vec::with_capacity(xml.len());
  let mut previous = 0;
  for span in replacements {
    recovered.extend_from_slice(&xml[previous..span.start]);
    recovered.push(b'0');
    previous = span.end;
  }
  recovered.extend_from_slice(&xml[previous..]);
  Some(recovered)
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing as wp;

  #[test]
  fn drawing_position_lexical_recovery_matches_native_integer_controls() {
    for (lexical, expected) in [
      ("48895", 48895),
      ("\n48895", 0),
      ("48895\n", 0),
      (" 48895", 0),
      ("48895 ", 0),
      ("\t48895", 0),
      ("+48895", 48895),
      ("-48895", -48895),
      ("&#13;48895", 0),
      (" 48895 ", 0),
      ("48\n895", 0),
      ("048895", 48895),
    ] {
      let xml = format!(
        r#"<wp:positionV xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" relativeFrom="page"><wp:posOffset>{lexical}</wp:posOffset></wp:positionV>"#
      );
      let original = wp::VerticalPosition::from_bytes(xml.as_bytes()).ok();
      let result = root(xml.as_bytes(), || {
        Ok(original.as_ref().expect("unmodified typed root"))
      })
      .unwrap();
      assert!(
        matches!(result.vertical_position_choice.as_ref(), Some(wp::VerticalPositionChoice::PositionOffset(value)) if *value == expected),
        "{lexical:?}: {result:?}"
      );
      assert_eq!(matches!(result, Cow::Borrowed(_)), expected != 0);
    }
  }

  #[test]
  fn drawing_position_recovery_resolves_namespaces_and_preserves_other_xml() {
    for prefix in ["wp:", "alternate:", ""] {
      let declaration = if prefix.is_empty() {
        "xmlns".to_string()
      } else {
        format!("xmlns:{}", prefix.trim_end_matches(':'))
      };
      let xml = format!(
        r#"<{prefix}positionV {declaration}="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" relativeFrom="page"><{prefix}posOffset>&#10;123 </{prefix}posOffset></{prefix}positionV>"#
      );
      let expected = xml.replace("&#10;123 ", "0");
      assert_eq!(recover_xml(xml.as_bytes()).unwrap(), expected.as_bytes());
    }
    for xml in [
      r#"<x:posOffset xmlns:x="urn:other"> 123 </x:posOffset>"#,
      r#"<root xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"><wp:posOffset xmlns:wp="urn:other"> 123 </wp:posOffset></root>"#,
      r#"<root xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"><text> 123 </text><wp:posOffset>123</wp:posOffset></root>"#,
    ] {
      assert!(recover_xml(xml.as_bytes()).is_none(), "{xml}");
    }
  }
}
