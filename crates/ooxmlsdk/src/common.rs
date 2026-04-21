use quick_xml::Decoder;
use quick_xml::events::attributes::Attribute;

mod error;
mod xml;

pub use error::{
  SdkError, invalid_enum_value, invalid_field_value, missing_field, unexpected_eof, unexpected_tag,
  validation_error,
};
pub use xml::resolve_relationship_target_path;
pub use xml::resolve_zip_file_path;
pub(crate) use xml::{
  IoReader, IoTagEvent, SliceReader, decode_attr_value, from_bytes_inner, from_reader_inner,
  from_str_inner, read_outer_xml_borrowed, read_outer_xml_io, write_attr_value,
  write_attr_value_str, write_end_tag, write_escaped_str, write_escaped_text, write_start_tag_open,
  write_xmlns_attr,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum XmlHeaderType {
  #[default]
  None,
  Plain,
  Standalone,
}

#[inline(always)]
pub fn parse_bool_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_bool_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_attr_value<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<T, SdkError>
where
  T: std::str::FromStr,
{
  xml::parse_attr_value(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_enum_attr<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
) -> Result<T, SdkError>
where
  T: std::str::FromStr<Err = SdkError>,
{
  xml::parse_enum_attr(attr, decoder, ty)
}

#[inline(always)]
pub fn parse_value<T>(value: &str, ty: &'static str, field: &'static str) -> Result<T, SdkError>
where
  T: std::str::FromStr,
{
  xml::parse_value(value, ty, field)
}

#[inline(always)]
pub fn parse_bool_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_bool_str(value, ty, field)
}

#[inline(always)]
pub fn parse_bool_bytes(b: &[u8]) -> Result<bool, SdkError> {
  xml::parse_bool_bytes(b)
}

#[inline(always)]
pub fn parse_boolean_value_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_boolean_value_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_on_off_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_on_off_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_true_false_blank_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_true_false_blank_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_true_false_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_true_false_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_boolean_value_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_boolean_value_str(value, ty, field)
}

#[inline(always)]
pub fn parse_on_off_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_on_off_str(value, ty, field)
}

#[inline(always)]
pub fn parse_true_false_blank_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_true_false_blank_str(value, ty, field)
}

#[inline(always)]
pub fn parse_true_false_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_true_false_str(value, ty, field)
}

#[inline(always)]
pub(crate) fn push_xml_text(
  value: &mut Option<String>,
  text: quick_xml::events::BytesText<'_>,
) -> Result<(), SdkError> {
  xml::push_xml_text(value, text)
}

#[inline(always)]
pub(crate) fn push_xml_general_ref(
  value: &mut Option<String>,
  text: quick_xml::events::BytesRef<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<(), SdkError> {
  xml::push_xml_general_ref(value, text, ty, field)
}

pub(crate) fn is_foreign_prefixed_child(name: &[u8], expected_prefix: &str) -> bool {
  let Some(separator_index) = name.iter().position(|b| *b == b':') else {
    return false;
  };
  let prefix = &name[..separator_index];

  prefix != b"mc" && prefix != expected_prefix.as_bytes()
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn parent_zip_path(path: &str) -> String {
  path
    .rsplit_once('/')
    .map(|(dir_path, _)| {
      let resolved = resolve_zip_file_path(&format!("{dir_path}/"));
      if resolved.is_empty() {
        resolved
      } else {
        format!("{resolved}/")
      }
    })
    .unwrap_or_default()
}

pub(crate) fn process_foreign_element_children_borrowed<'de, F>(
  xml_reader: &mut SliceReader<'de>,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  F: FnMut(
    &mut SliceReader<'de>,
    quick_xml::events::BytesStart<'de>,
    bool,
  ) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  loop {
    match xml_reader.next()? {
      quick_xml::events::Event::Start(e) => match visitor(xml_reader, e, false)? {
        true => {}
        false => {
          process_foreign_element_children_borrowed(xml_reader, false, visitor)?;
        }
      },
      quick_xml::events::Event::Empty(e) => {
        visitor(xml_reader, e, true)?;
      }
      quick_xml::events::Event::End(_) => break,
      quick_xml::events::Event::Eof => Err(unexpected_eof("process_foreign_element_children"))?,
      _ => {}
    }
  }

  Ok(())
}

pub(crate) fn process_foreign_element_children_io<R, F>(
  xml_reader: &mut IoReader<R>,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  R: std::io::BufRead,
  F:
    FnMut(&mut IoReader<R>, quick_xml::events::BytesStart<'static>, bool) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  loop {
    let next_event = match xml_reader.next_borrowed()? {
      quick_xml::events::Event::Start(e) => Some((e.into_owned(), false)),
      quick_xml::events::Event::Empty(e) => Some((e.into_owned(), true)),
      quick_xml::events::Event::End(_) => break,
      quick_xml::events::Event::Eof => Err(unexpected_eof("process_foreign_element_children_io"))?,
      _ => None,
    };

    match next_event {
      Some((e, false)) => match visitor(xml_reader, e, false)? {
        true => {}
        false => {
          process_foreign_element_children_io(xml_reader, false, visitor)?;
        }
      },
      Some((e, true)) => {
        visitor(xml_reader, e, true)?;
      }
      None => {}
    }
  }

  Ok(())
}

pub(crate) fn process_markup_compatibility_children_borrowed<'de, F>(
  xml_reader: &mut SliceReader<'de>,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  F: FnMut(
    &mut SliceReader<'de>,
    quick_xml::events::BytesStart<'de>,
    bool,
  ) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  let mut selected_branch = false;

  loop {
    match xml_reader.next()? {
      quick_xml::events::Event::Start(e) => match e.name().as_ref() {
        b"mc:Choice" | b"Choice" => {
          let should_use =
            !selected_branch && markup_compatibility_choice_supported(&e, xml_reader.decoder())?;
          if should_use {
            selected_branch = true;
            process_foreign_element_children_borrowed(xml_reader, false, visitor)?;
          } else {
            skip_foreign_element_children_borrowed(xml_reader, false)?;
          }
        }
        b"mc:Fallback" | b"Fallback" => {
          if selected_branch {
            skip_foreign_element_children_borrowed(xml_reader, false)?;
          } else {
            selected_branch = true;
            process_foreign_element_children_borrowed(xml_reader, false, visitor)?;
          }
        }
        _ => {
          skip_foreign_element_children_borrowed(xml_reader, false)?;
        }
      },
      quick_xml::events::Event::Empty(e) => match e.name().as_ref() {
        b"mc:Choice" | b"Choice"
          if !selected_branch
            && markup_compatibility_choice_supported(&e, xml_reader.decoder())? =>
        {
          selected_branch = true;
        }
        b"mc:Fallback" | b"Fallback" if !selected_branch => {
          selected_branch = true;
        }
        _ => {}
      },
      quick_xml::events::Event::End(e) => match e.name().as_ref() {
        b"mc:AlternateContent" | b"AlternateContent" => break,
        _ => {}
      },
      quick_xml::events::Event::Eof => {
        Err(unexpected_eof("process_markup_compatibility_children"))?
      }
      _ => {}
    }
  }

  Ok(())
}

pub(crate) fn process_markup_compatibility_children_io<R, F>(
  xml_reader: &mut IoReader<R>,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  R: std::io::BufRead,
  F:
    FnMut(&mut IoReader<R>, quick_xml::events::BytesStart<'static>, bool) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  let mut selected_branch = false;

  loop {
    let decoder = xml_reader.decoder();
    let next_event = match xml_reader.next_borrowed()? {
      quick_xml::events::Event::Start(e) => Some(quick_xml::events::Event::Start(e.into_owned())),
      quick_xml::events::Event::Empty(e) => Some(quick_xml::events::Event::Empty(e.into_owned())),
      quick_xml::events::Event::End(e) => Some(quick_xml::events::Event::End(e.into_owned())),
      quick_xml::events::Event::Eof => Some(quick_xml::events::Event::Eof),
      _ => None,
    };
    match next_event {
      Some(quick_xml::events::Event::Start(e)) => match e.name().as_ref() {
        b"mc:Choice" | b"Choice" => {
          let should_use = !selected_branch && markup_compatibility_choice_supported(&e, decoder)?;
          if should_use {
            selected_branch = true;
            process_foreign_element_children_io(xml_reader, false, visitor)?;
          } else {
            skip_foreign_element_children_io(xml_reader, false)?;
          }
        }
        b"mc:Fallback" | b"Fallback" => {
          if selected_branch {
            skip_foreign_element_children_io(xml_reader, false)?;
          } else {
            selected_branch = true;
            process_foreign_element_children_io(xml_reader, false, visitor)?;
          }
        }
        _ => {
          skip_foreign_element_children_io(xml_reader, false)?;
        }
      },
      Some(quick_xml::events::Event::Empty(e)) => match e.name().as_ref() {
        b"mc:Choice" | b"Choice"
          if !selected_branch && markup_compatibility_choice_supported(&e, decoder)? =>
        {
          selected_branch = true;
        }
        b"mc:Fallback" | b"Fallback" if !selected_branch => {
          selected_branch = true;
        }
        _ => {}
      },
      Some(quick_xml::events::Event::End(e)) => match e.name().as_ref() {
        b"mc:AlternateContent" | b"AlternateContent" => break,
        _ => {}
      },
      Some(quick_xml::events::Event::Eof) => {
        Err(unexpected_eof("process_markup_compatibility_children_io"))?
      }
      None => {}
      _ => {}
    }
  }

  Ok(())
}

fn markup_compatibility_choice_supported<'a>(
  choice_start: &quick_xml::events::BytesStart<'a>,
  decoder: Decoder,
) -> Result<bool, SdkError> {
  let mut requires = None;

  for attr in choice_start.attributes().with_checks(false) {
    let attr = attr?;
    if attr.key.as_ref() == b"Requires" {
      requires = Some(decode_attr_value(&attr, decoder)?);
      break;
    }
  }

  let Some(requires) = requires else {
    return Ok(false);
  };

  for prefix in requires.split_ascii_whitespace() {
    if !markup_compatibility_prefix_supported(choice_start, decoder, prefix)? {
      return Ok(false);
    }
  }

  Ok(true)
}

fn markup_compatibility_prefix_supported(
  choice_start: &quick_xml::events::BytesStart<'_>,
  decoder: Decoder,
  prefix: &str,
) -> Result<bool, SdkError> {
  if crate::namespaces::uri_by_prefix(prefix).is_some() {
    return Ok(true);
  }

  let xmlns_name = format!("xmlns:{prefix}");
  for attr in choice_start.attributes().with_checks(false) {
    let attr = attr?;
    if attr.key.as_ref() == xmlns_name.as_bytes() {
      let uri = decode_attr_value(&attr, decoder)?;
      return Ok(crate::namespaces::prefix_by_uri(uri.as_str()).is_some());
    }
  }

  Ok(false)
}

fn skip_foreign_element_children_borrowed<'de>(
  xml_reader: &mut SliceReader<'de>,
  empty_tag: bool,
) -> Result<(), SdkError> {
  process_foreign_element_children_borrowed(
    xml_reader,
    empty_tag,
    &mut |_xml_reader, _e, _e_empty| Ok(false),
  )
}

fn skip_foreign_element_children_io<R: std::io::BufRead>(
  xml_reader: &mut IoReader<R>,
  empty_tag: bool,
) -> Result<(), SdkError> {
  process_foreign_element_children_io(xml_reader, empty_tag, &mut |_xml_reader, _e, _e_empty| {
    Ok(false)
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mc_choice_requires_supports_local_alias_for_known_namespace() {
    let xml = r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
  <mc:Choice xmlns:foo="http://schemas.openxmlformats.org/drawingml/2006/main" Requires="foo">
    <foo:bar/>
  </mc:Choice>
  <mc:Fallback>
    <fallback/>
  </mc:Fallback>
</mc:AlternateContent>"#;

    let mut reader = from_str_inner(xml).expect("reader");
    let start = match reader.next().expect("alternate content start") {
      quick_xml::events::Event::Start(e) => e,
      other => panic!("expected start, got {other:?}"),
    };
    assert_eq!(start.name().as_ref(), b"mc:AlternateContent");

    let mut selected = Vec::new();
    process_markup_compatibility_children_borrowed(
      &mut reader,
      false,
      &mut |_reader, e, _empty| {
        selected.push(String::from_utf8_lossy(e.name().as_ref()).into_owned());
        Ok(false)
      },
    )
    .expect("process alternate content");

    assert_eq!(selected, vec!["foo:bar"]);
  }

  #[test]
  fn mc_choice_requires_rejects_unknown_local_alias_namespace() {
    let xml = r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
  <mc:Choice xmlns:foo="http://example.com/unknown" Requires="foo">
    <foo:bar/>
  </mc:Choice>
  <mc:Fallback>
    <fallback/>
  </mc:Fallback>
</mc:AlternateContent>"#;

    let mut reader = from_str_inner(xml).expect("reader");
    let start = match reader.next().expect("alternate content start") {
      quick_xml::events::Event::Start(e) => e,
      other => panic!("expected start, got {other:?}"),
    };
    assert_eq!(start.name().as_ref(), b"mc:AlternateContent");

    let mut selected = Vec::new();
    process_markup_compatibility_children_borrowed(
      &mut reader,
      false,
      &mut |_reader, e, _empty| {
        selected.push(String::from_utf8_lossy(e.name().as_ref()).into_owned());
        Ok(false)
      },
    )
    .expect("process alternate content");

    assert_eq!(selected, vec!["fallback"]);
  }
}
