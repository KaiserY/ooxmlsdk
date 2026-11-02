//! Word's lexical recovery of DrawingML position and source-crop offsets.
//!
//! XML Schema permits whitespace around an integer, but Word imports a
//! wp:posOffset containing whitespace as zero. Preserve that distinction before
//! the SDK's typed parser collapses the lexical whitespace. This is a layout
//! import policy; neither the source package nor shared XML parsing is changed.
//! The same boundary preserves Word's repair of non-integer, unqualified
//! srcRect attributes before the SDK's compatibility reader truncates decimals.

use std::borrow::Cow;

use ooxmlsdk::{common::SdkError, sdk::SdkType};
use quick_xml::{
  events::{Event, attributes::Attribute},
  name::ResolveResult,
};

const WORDPROCESSING_DRAWING: &[u8] =
  b"http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing";
const DRAWING: &[u8] = b"http://schemas.openxmlformats.org/drawingml/2006/main";

pub(super) fn root<'a, T: SdkType + Clone>(
  xml: &[u8],
  original: impl FnOnce() -> std::result::Result<&'a T, SdkError>,
) -> crate::Result<Cow<'a, T>> {
  let indentation = super::indentation_recovery::xml(xml);
  let numbering =
    super::numbering_recovery::xml(indentation.as_deref().unwrap_or(xml)).or(indentation);
  match recover_xml(numbering.as_deref().unwrap_or(xml)).or(numbering) {
    Some(recovered) => Ok(Cow::Owned(T::from_bytes(&recovered)?)),
    None => Ok(Cow::Borrowed(original()?)),
  }
}

fn recover_xml(xml: &[u8]) -> Option<Vec<u8>> {
  if !xml
    .windows(b"posOffset".len())
    .any(|word| word == b"posOffset")
    && !xml.windows(b"srcRect".len()).any(|word| word == b"srcRect")
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
      (namespace, event @ (Event::Start(_) | Event::Empty(_))) => {
        let wordprocessing_drawing =
          matches!(&namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORDPROCESSING_DRAWING);
        let drawing = matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == DRAWING);
        let empty = matches!(event, Event::Empty(_));
        let (Event::Start(element) | Event::Empty(element)) = event else {
          unreachable!()
        };
        if !empty {
          depth += 1;
        }
        if !empty && element.local_name().as_ref() == b"posOffset" && wordprocessing_drawing {
          position = Some((depth, usize::try_from(reader.buffer_position()).ok()?));
        }
        if element.local_name().as_ref() == b"srcRect"
          && drawing
          && element
            .attributes()
            .flatten()
            .any(|attr| invalid_crop_attribute(&attr))
        {
          let mut recovered = element.to_owned();
          recovered.clear_attributes();
          for attr in element.attributes() {
            let mut attr = attr.ok()?;
            if invalid_crop_attribute(&attr) {
              attr.value = Cow::Borrowed(b"0");
            }
            recovered.push_attribute(attr);
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
            event_start..usize::try_from(reader.buffer_position()).ok()?,
            writer.into_inner(),
          ));
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
            replacements.push((start..event_start, vec![b'0']));
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
  for (span, replacement) in replacements {
    recovered.extend_from_slice(&xml[previous..span.start]);
    recovered.extend_from_slice(&replacement);
    previous = span.end;
  }
  recovered.extend_from_slice(&xml[previous..]);
  Some(recovered)
}

fn invalid_crop_attribute(attribute: &Attribute<'_>) -> bool {
  if !matches!(attribute.key.as_ref(), b"l" | b"t" | b"r" | b"b") {
    return false;
  }
  let Ok(value) = attribute.normalized_value(quick_xml::XmlVersion::Implicit1_0) else {
    return false;
  };
  // MS-OI29500 2.1.1329 accepts percent strings or integer thousandths.
  // Word OpenAndRepair drops each invalid unqualified edge independently,
  // including bare decimals and whitespace-padded integers. Valid signed
  // integers and percent strings retain the normal typed import path.
  !value.ends_with('%') && value.parse::<i32>().is_err()
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
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

  #[test]
  fn source_crop_lexical_recovery_matches_native_integer_and_percent_controls() {
    for (lexical, expected, changed) in [
      ("20000", 20_000, false),
      ("+20000", 20_000, false),
      ("020000", 20_000, false),
      ("-10000", -10_000, false),
      ("20%", 20_000, false),
      ("20.5%", 20_500, false),
      ("20000.0", 0, true),
      ("20000.5", 0, true),
      (" 20000 ", 0, true),
      ("2e4", 0, true),
      ("20000x", 0, true),
    ] {
      let xml = format!(
        r#"<a:srcRect xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" l="{lexical}" t="10000"/>"#
      );
      let original = a::SourceRectangle::from_bytes(xml.as_bytes()).ok();
      let result = root(xml.as_bytes(), || {
        Ok(original.as_ref().expect("unchanged typed root"))
      })
      .unwrap();
      assert_eq!(
        result.left.unwrap().as_drawingml_percent(),
        expected,
        "{lexical}"
      );
      assert_eq!(result.top.unwrap().as_drawingml_percent(), 10_000);
      assert_eq!(matches!(result, Cow::Owned(_)), changed, "{lexical}");
    }
  }

  #[test]
  fn source_crop_recovery_is_namespace_and_attribute_scoped() {
    let xml = r#"<root xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"><a:srcRect l="20000.5" t="20.5%" r="10000" b="20000.0"/><a:fillRect l="20000.5"/><other:srcRect xmlns:other="urn:other" l="20000.5"/><wp:posOffset> 123 </wp:posOffset></root>"#;
    let expected = xml
      .replace("<a:srcRect l=\"20000.5\"", "<a:srcRect l=\"0\"")
      .replace("b=\"20000.0\"", "b=\"0\"")
      .replace("> 123 <", ">0<");
    assert_eq!(recover_xml(xml.as_bytes()).unwrap(), expected.as_bytes());
    for xml in [
      r#"<srcRect xmlns="http://schemas.openxmlformats.org/drawingml/2006/main" l="&#50;0000.5"></srcRect>"#,
      r#"<x:srcRect xmlns:x="http://schemas.openxmlformats.org/drawingml/2006/main" l="20000.5"/>"#,
    ] {
      let original = a::SourceRectangle::from_bytes(xml.as_bytes()).unwrap();
      let recovered = root(xml.as_bytes(), || Ok(&original)).unwrap();
      assert_eq!(recovered.left.unwrap().as_drawingml_percent(), 0);
    }
  }
}
