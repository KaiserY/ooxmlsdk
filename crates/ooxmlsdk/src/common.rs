use quick_xml::Decoder;
use quick_xml::events::attributes::Attribute;
use std::collections::HashMap;

mod error;
mod xml;

pub use error::{
  SdkError, invalid_enum_value, invalid_field_value, missing_field, unexpected_eof, unexpected_tag,
};
pub use xml::resolve_relationship_target_path;
pub use xml::resolve_zip_file_path;
pub(crate) use xml::{
  XmlReader, decode_attr_value, expect_event_start, from_reader_inner, from_str_inner,
  read_outer_xml, write_attr_value, write_end_tag, write_escaped_text, write_start_tag_open,
  write_xmlns_attr,
};

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

pub(crate) fn merge_strict_bitmask_attr(
  current: Option<&str>,
  raw_value: &str,
  bit: u32,
  radix: u32,
  width: usize,
  ty: &'static str,
  field: &'static str,
) -> Result<String, SdkError> {
  let is_true = parse_bool_str(raw_value, ty, field)?;
  let current_value = current
    .and_then(|value| u32::from_str_radix(value, radix).ok())
    .unwrap_or(0);
  let combined = if is_true {
    current_value | bit
  } else {
    current_value & !bit
  };

  match radix {
    2 => Ok(format!("{combined:0width$b}")),
    16 => Ok(format!("{combined:0width$x}")),
    _ => Err(SdkError::CommonError(format!(
      "unsupported strict bitmask radix: {radix}"
    ))),
  }
}

pub(crate) fn map_compat_attr_value(raw_value: String, mappings: &[(&str, &str)]) -> String {
  for (from, to) in mappings {
    if raw_value == *from {
      return (*to).to_string();
    }
  }

  raw_value
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

pub(crate) fn process_foreign_element_children<'de, R, F>(
  xml_reader: &mut R,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  R: XmlReader<'de>,
  F: FnMut(&mut R, quick_xml::events::BytesStart<'de>, bool) -> Result<bool, SdkError>,
{
  if empty_tag {
    Ok(())
  } else {
    loop {
      match xml_reader.next()? {
        quick_xml::events::Event::Start(e) => {
          if !visitor(xml_reader, e, false)? {
            process_foreign_element_children(xml_reader, false, visitor)?;
          }
        }
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
}

pub(crate) fn process_markup_compatibility_children<'de, R, F>(
  xml_reader: &mut R,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  R: XmlReader<'de>,
  F: FnMut(&mut R, quick_xml::events::BytesStart<'de>, bool) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  let alternate_content_namespaces = HashMap::new();
  let mut selected_branch = false;

  loop {
    match xml_reader.next()? {
      quick_xml::events::Event::Start(e) => match e.name().as_ref() {
        b"mc:Choice" | b"Choice" => {
          let should_use = !selected_branch
            && markup_compatibility_choice_supported(
              &e,
              xml_reader.decoder(),
              &alternate_content_namespaces,
            )?;
          if should_use {
            selected_branch = true;
            process_foreign_element_children(xml_reader, false, visitor)?;
          } else {
            skip_foreign_element_children(xml_reader, false)?;
          }
        }
        b"mc:Fallback" | b"Fallback" => {
          if selected_branch {
            skip_foreign_element_children(xml_reader, false)?;
          } else {
            selected_branch = true;
            process_foreign_element_children(xml_reader, false, visitor)?;
          }
        }
        _ => {
          skip_foreign_element_children(xml_reader, false)?;
        }
      },
      quick_xml::events::Event::Empty(e) => match e.name().as_ref() {
        b"mc:Choice" | b"Choice" => {
          if !selected_branch
            && markup_compatibility_choice_supported(
              &e,
              xml_reader.decoder(),
              &alternate_content_namespaces,
            )?
          {
            selected_branch = true;
          }
        }
        b"mc:Fallback" | b"Fallback" => {
          if !selected_branch {
            selected_branch = true;
          }
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

fn markup_compatibility_choice_supported<'a>(
  choice_start: &quick_xml::events::BytesStart<'a>,
  decoder: Decoder,
  alternate_content_namespaces: &HashMap<String, String>,
) -> Result<bool, SdkError> {
  let mut requires = None;
  let mut namespaces = alternate_content_namespaces.clone();
  namespaces.extend(collect_namespace_declarations(choice_start, decoder)?);

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
    let Some(namespace_uri) = namespaces.get(prefix) else {
      return Ok(false);
    };

    if crate::namespaces::prefix_by_uri(namespace_uri).is_none() {
      return Ok(false);
    }
  }

  Ok(true)
}

fn collect_namespace_declarations<'a>(
  start: &quick_xml::events::BytesStart<'a>,
  decoder: Decoder,
) -> Result<HashMap<String, String>, SdkError> {
  let mut namespaces = HashMap::new();

  for attr in start.attributes().with_checks(false) {
    let attr = attr?;
    let key = attr.key.as_ref();
    if key == b"xmlns" {
      namespaces.insert(String::new(), decode_attr_value(&attr, decoder)?);
    } else if let Some(prefix) = key.strip_prefix(b"xmlns:") {
      namespaces.insert(
        String::from_utf8_lossy(prefix).into_owned(),
        decode_attr_value(&attr, decoder)?,
      );
    }
  }

  Ok(namespaces)
}

fn skip_foreign_element_children<'de, R: XmlReader<'de>>(
  xml_reader: &mut R,
  empty_tag: bool,
) -> Result<(), SdkError> {
  process_foreign_element_children(xml_reader, empty_tag, &mut |_xml_reader, _e, _e_empty| {
    Ok(false)
  })
}

pub(crate) fn normalize_relationship_type(value: &str) -> &str {
  match value {
    "http://purl.oclc.org/ooxml/officeDocument/relationships/aFChunk" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/attachedTemplate" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/attachedTemplate"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/audio" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/calcChain" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/chart" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/chartsheet" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/chartUserShapes" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/commentAuthors" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/comments" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/connections" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/control" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/customProperties" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/customProperty" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/customXml" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/customXmlProps" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/diagramColors" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/diagramData" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/diagramLayout" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/diagramQuickStyle" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/dialogsheet" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/drawing" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/endnotes" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/extendedProperties" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/externalLink" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/externalLinkPath" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLinkPath"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/font" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/fontTable" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/footer" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/footnotes" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/frame" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/frame"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/glossaryDocument" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/handoutMaster" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/header" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/htmlPubSaveAs" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/htmlPubSaveAs"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/hyperlink" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/image" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/mailMergeHeaderSource" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/mailMergeHeaderSource"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/mailMergeRecipientData" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/mailMergeRecipientData"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/mailMergeSource" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/mailMergeSource"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/metadata/thumbnail" => {
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/movie" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/movie"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/notesMaster" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/notesSlide" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/numbering" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/oleObject" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/package" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/pivotCacheDefinition" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/pivotCacheRecords" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/pivotTable" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/presProps" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/printerSettings" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/queryTable" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/revisionHeaders" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/revisionLog" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/settings" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/sheetMetadata" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/slide" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/slideLayout" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/slideUpdateInfo" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/slideUpdateUrl" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateUrl"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/styles" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/subDocument" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/subDocument"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/table" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/tableSingleCells" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/tableStyles" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/tags" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/theme" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/themeOverride" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/transform" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/transform"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/usernames" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/video" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/viewProps" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/volatileDependencies" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/webSettings" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet"
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships/xmlMaps" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps"
    }
    _ => value,
  }
}
