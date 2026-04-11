#![cfg(feature = "parts")]

use std::{
  collections::BTreeMap,
  fs,
  io::{Cursor, Read},
  path::Path,
};

use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk_test::fixtures::doc_sample_path;
use quick_xml::{Reader, escape::unescape, events::Event};
use zip::ZipArchive;

#[derive(Clone, Copy, Debug)]
enum DocSampleKind {
  Wordprocessing,
  Spreadsheet,
  Presentation,
}

fn assert_doc_sample_round_trip(file_name: &str) {
  let kind = doc_sample_kind(file_name);
  let path = test_file_path(file_name);
  let original_bytes = fs::read(&path).unwrap();

  match kind {
    DocSampleKind::Wordprocessing => {
      let original = WordprocessingDocument::new_from_file(&path).unwrap();
      let mut buffer = Cursor::new(Vec::new());
      original.save(&mut buffer).unwrap();
      let roundtripped_bytes = buffer.into_inner();
      let reopened = WordprocessingDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
      assert_wordprocessing_document_round_trip(&original, &reopened);
      assert_doc_sample_zip_equivalent(&original_bytes, &roundtripped_bytes, file_name);
    }
    DocSampleKind::Spreadsheet => {
      let original = SpreadsheetDocument::new_from_file(&path).unwrap();
      let mut buffer = Cursor::new(Vec::new());
      original.save(&mut buffer).unwrap();
      let roundtripped_bytes = buffer.into_inner();
      let reopened = SpreadsheetDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
      assert_spreadsheet_document_round_trip(&original, &reopened);
      assert_doc_sample_zip_equivalent(&original_bytes, &roundtripped_bytes, file_name);
    }
    DocSampleKind::Presentation => {
      let original = PresentationDocument::new_from_file(&path).unwrap();
      let mut buffer = Cursor::new(Vec::new());
      original.save(&mut buffer).unwrap();
      let roundtripped_bytes = buffer.into_inner();
      let reopened = PresentationDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
      assert_presentation_document_round_trip(&original, &reopened);
      assert_doc_sample_zip_equivalent(&original_bytes, &roundtripped_bytes, file_name);
    }
  }
}

fn assert_doc_sample_invalid(file_name: &str) {
  let kind = doc_sample_kind(file_name);
  let path = test_file_path(file_name);

  let result = match kind {
    DocSampleKind::Wordprocessing => WordprocessingDocument::new_from_file(&path).map(|_| ()),
    DocSampleKind::Spreadsheet => SpreadsheetDocument::new_from_file(&path).map(|_| ()),
    DocSampleKind::Presentation => PresentationDocument::new_from_file(&path).map(|_| ()),
  };

  assert!(
    result.is_err(),
    "expected {file_name} to be invalid so we can keep it out of round-trip coverage"
  );
}

fn assert_doc_sample_opens(file_name: &str) {
  let kind = doc_sample_kind(file_name);
  let path = test_file_path(file_name);

  match kind {
    DocSampleKind::Wordprocessing => {
      let package = WordprocessingDocument::new_from_file(&path).unwrap();
      assert_eq!(package.main_document_part.inner_path, "word/document.xml");
    }
    DocSampleKind::Spreadsheet => {
      let package = SpreadsheetDocument::new_from_file(&path).unwrap();
      assert_eq!(package.workbook_part.inner_path, "xl/workbook.xml");
    }
    DocSampleKind::Presentation => {
      let package = PresentationDocument::new_from_file(&path).unwrap();
      assert_eq!(package.presentation_part.inner_path, "ppt/presentation.xml");
    }
  }
}

fn assert_wordprocessing_document_round_trip(
  original: &WordprocessingDocument,
  roundtripped: &WordprocessingDocument,
) {
  assert_eq!(original.inner_path, roundtripped.inner_path);
  assert_eq!(
    original.main_document_part.inner_path,
    roundtripped.main_document_part.inner_path
  );
  assert_eq!(
    original
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    original
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .main_document_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
}

fn assert_spreadsheet_document_round_trip(
  original: &SpreadsheetDocument,
  roundtripped: &SpreadsheetDocument,
) {
  assert_eq!(original.inner_path, roundtripped.inner_path);
  assert_eq!(
    original.workbook_part.inner_path,
    roundtripped.workbook_part.inner_path
  );
  assert_eq!(
    original
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    original
      .workbook_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .workbook_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    original.workbook_part.worksheet_parts.len(),
    roundtripped.workbook_part.worksheet_parts.len()
  );
}

fn assert_presentation_document_round_trip(
  original: &PresentationDocument,
  roundtripped: &PresentationDocument,
) {
  assert_eq!(original.inner_path, roundtripped.inner_path);
  assert_eq!(
    original.presentation_part.inner_path,
    roundtripped.presentation_part.inner_path
  );
  assert_eq!(
    original
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    original
      .presentation_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len()),
    roundtripped
      .presentation_part
      .relationships
      .as_ref()
      .map(|relationships| relationships.relationship.len())
  );
  assert_eq!(
    original.presentation_part.slide_parts.len(),
    roundtripped.presentation_part.slide_parts.len()
  );
  assert_eq!(
    original.presentation_part.slide_master_parts.len(),
    roundtripped.presentation_part.slide_master_parts.len()
  );
}

fn doc_sample_kind(file_name: &str) -> DocSampleKind {
  match Path::new(file_name)
    .extension()
    .and_then(|ext| ext.to_str())
  {
    Some("docx") | Some("dotx") => DocSampleKind::Wordprocessing,
    Some("xlsx") | Some("xltx") => DocSampleKind::Spreadsheet,
    Some("pptx") | Some("potx") => DocSampleKind::Presentation,
    other => panic!("unsupported doc sample extension for {file_name}: {other:?}"),
  }
}

fn test_file_path(file_name: &str) -> std::path::PathBuf {
  let path = doc_sample_path(file_name);
  assert!(path.is_file(), "missing doc sample: {}", path.display());
  path
}

fn assert_doc_sample_zip_equivalent(original: &[u8], roundtripped: &[u8], file_name: &str) {
  let original = read_zip_entries(original, file_name);
  let roundtripped = read_zip_entries(roundtripped, file_name);

  let original_names: Vec<_> = original.keys().collect();
  let roundtripped_names: Vec<_> = roundtripped.keys().collect();
  assert_eq!(
    original_names, roundtripped_names,
    "zip entry names differ for {file_name}"
  );

  for name in original_names {
    let original_bytes = original.get(name).expect("original entry missing");
    let roundtripped_bytes = roundtripped.get(name).expect("roundtripped entry missing");

    if is_xml_entry(name) {
      assert_xml_equivalent(original_bytes, roundtripped_bytes, file_name, name);
    } else {
      assert_eq!(
        original_bytes, roundtripped_bytes,
        "binary entry mismatch for {file_name}:{name}"
      );
    }
  }
}

fn read_zip_entries(bytes: &[u8], file_name: &str) -> BTreeMap<String, Vec<u8>> {
  let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap_or_else(|err| {
    panic!("failed to open zip for {file_name}: {err}");
  });

  let mut entries = BTreeMap::new();
  for idx in 0..archive.len() {
    let mut file = archive.by_index(idx).unwrap_or_else(|err| {
      panic!("failed to read zip entry {idx} for {file_name}: {err}");
    });

    if file.is_dir() {
      continue;
    }

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap_or_else(|err| {
      panic!(
        "failed to read zip entry {} for {file_name}: {err}",
        file.name()
      );
    });

    entries.insert(file.name().to_string(), data);
  }

  entries
}

fn is_xml_entry(name: &str) -> bool {
  name == "[Content_Types].xml" || name.ends_with(".xml") || name.ends_with(".rels")
}

fn assert_xml_equivalent(original: &[u8], roundtripped: &[u8], file_name: &str, entry_name: &str) {
  let strict_original = canonicalize_xml(original, false, file_name, entry_name);
  let strict_roundtripped = canonicalize_xml(roundtripped, false, file_name, entry_name);

  if strict_original == strict_roundtripped {
    return;
  }

  let relaxed_original = canonicalize_xml(original, true, file_name, entry_name);
  let relaxed_roundtripped = canonicalize_xml(roundtripped, true, file_name, entry_name);

  assert_eq!(
    relaxed_original,
    relaxed_roundtripped,
    "xml mismatch for {file_name}:{entry_name}\n{}",
    xml_diff_context(&relaxed_original, &relaxed_roundtripped)
  );
}

fn xml_diff_context(left: &str, right: &str) -> String {
  if left == right {
    return String::new();
  }

  let mut left_iter = left.char_indices();
  let mut right_iter = right.char_indices();

  loop {
    match (left_iter.next(), right_iter.next()) {
      (Some((li, lc)), Some((ri, rc))) if lc == rc => {
        let _ = (li, ri);
      }
      (Some((li, _)), Some((ri, _))) => {
        let start = li.saturating_sub(160).min(ri.saturating_sub(160));
        let end = (li + 160).max(ri + 160).min(left.len().max(right.len()));
        return format!(
          "first diff around byte {li}/{ri}\nleft:  {:?}\nright: {:?}",
          &left[start..left.len().min(end)],
          &right[start..right.len().min(end)]
        );
      }
      (Some((li, _)), None) => {
        let start = li.saturating_sub(160);
        let end = (li + 160).min(left.len());
        return format!(
          "right shorter around byte {li}\nleft:  {:?}\nright: {:?}",
          &left[start..end],
          &right[right.len().saturating_sub(160)..]
        );
      }
      (None, Some((ri, _))) => {
        let start = ri.saturating_sub(160);
        let end = (ri + 160).min(right.len());
        return format!(
          "left shorter around byte {ri}\nleft:  {:?}\nright: {:?}",
          &left[left.len().saturating_sub(160)..],
          &right[start..end]
        );
      }
      (None, None) => return String::from("xml contents differ but no diff point found"),
    }
  }
}

#[derive(Debug)]
struct XmlElement {
  name: String,
  attrs: Vec<(String, String)>,
  children: Vec<XmlNode>,
}

#[derive(Debug)]
enum XmlNode {
  Element(XmlElement),
  Text(String),
}

#[derive(Debug)]
struct XmlFrame {
  name: String,
  attrs: Vec<(String, String)>,
  children: Vec<XmlNode>,
  ns: BTreeMap<String, String>,
}

fn canonicalize_xml(xml: &[u8], relaxed_bool: bool, file_name: &str, entry_name: &str) -> String {
  let mut reader = Reader::from_reader(Cursor::new(xml));
  reader.config_mut().trim_text(false);
  let mut buf = Vec::new();
  let mut roots = Vec::new();
  let mut stack: Vec<XmlFrame> = Vec::new();

  loop {
    match reader.read_event_into(&mut buf).unwrap_or_else(|err| {
      panic!("failed to parse xml for {file_name}:{entry_name}: {err}");
    }) {
      Event::Start(event) => {
        let inherited_ns = stack.last().map(|frame| &frame.ns);
        let (name, attrs, ns) = parse_xml_node(
          &reader,
          &event,
          inherited_ns,
          relaxed_bool,
          file_name,
          entry_name,
        );
        let frame = XmlFrame {
          name,
          attrs,
          children: Vec::new(),
          ns,
        };
        stack.push(frame);
      }
      Event::Empty(event) => {
        let inherited_ns = stack.last().map(|frame| &frame.ns);
        let (name, attrs, _) = parse_xml_node(
          &reader,
          &event,
          inherited_ns,
          relaxed_bool,
          file_name,
          entry_name,
        );
        let node = XmlNode::Element(XmlElement {
          name,
          attrs,
          children: Vec::new(),
        });
        push_xml_node(&mut roots, &mut stack, node);
      }
      Event::End(_) => {
        let frame = stack
          .pop()
          .unwrap_or_else(|| panic!("unexpected xml end event for {file_name}:{entry_name}"));
        let node = XmlNode::Element(XmlElement {
          name: frame.name,
          attrs: frame.attrs,
          children: frame.children,
        });
        push_xml_node(&mut roots, &mut stack, node);
      }
      Event::Text(event) => {
        let raw = unescape(&String::from_utf8_lossy(event.as_ref()))
          .unwrap_or_else(|err| {
            panic!("failed to decode xml text for {file_name}:{entry_name}: {err}");
          })
          .into_owned();
        if raw.chars().all(|ch| ch.is_whitespace()) {
          // Skip formatting-only whitespace.
        } else {
          let text = normalize_xml_text(&raw, relaxed_bool);
          push_xml_node(&mut roots, &mut stack, XmlNode::Text(text));
        }
      }
      Event::CData(event) => {
        let raw = unescape(&String::from_utf8_lossy(event.as_ref()))
          .unwrap_or_else(|err| {
            panic!("failed to decode xml cdata for {file_name}:{entry_name}: {err}");
          })
          .into_owned();
        if !raw.chars().all(|ch| ch.is_whitespace()) {
          let text = normalize_xml_text(&raw, relaxed_bool);
          push_xml_node(&mut roots, &mut stack, XmlNode::Text(text));
        }
      }
      Event::Decl(_) | Event::Comment(_) | Event::PI(_) | Event::DocType(_) => {}
      Event::GeneralRef(event) => {
        let raw = event.decode().unwrap_or_else(|err| {
          panic!("failed to decode xml general ref for {file_name}:{entry_name}: {err}");
        });
        let text = unescape(&format!("&{raw};"))
          .unwrap_or_else(|err| {
            panic!("failed to decode xml general ref for {file_name}:{entry_name}: {err}");
          })
          .into_owned();
        push_xml_node(&mut roots, &mut stack, XmlNode::Text(text));
      }
      Event::Eof => break,
    }

    buf.clear();
  }

  assert!(
    stack.is_empty(),
    "unterminated xml for {file_name}:{entry_name}"
  );
  render_xml_nodes_for_entry(&roots, relaxed_bool, entry_name)
}

fn parse_xml_node(
  reader: &Reader<Cursor<&[u8]>>,
  event: &quick_xml::events::BytesStart<'_>,
  inherited_ns: Option<&BTreeMap<String, String>>,
  relaxed_bool: bool,
  file_name: &str,
  entry_name: &str,
) -> (String, Vec<(String, String)>, BTreeMap<String, String>) {
  let mut raw_attrs = Vec::new();
  let mut ns = inherited_ns.cloned().unwrap_or_default();
  for attr in event.attributes().with_checks(false) {
    let attr = attr.unwrap_or_else(|err| {
      panic!("failed to parse xml attribute for {file_name}:{entry_name}: {err}");
    });
    let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
    let value = attr
      .decode_and_unescape_value(reader.decoder())
      .unwrap_or_else(|err| {
        panic!("failed to decode xml attribute for {file_name}:{entry_name}: {err}");
      })
      .into_owned();
    if key == "xmlns" {
      ns.insert(String::new(), value.clone());
    } else if let Some(prefix) = key.strip_prefix("xmlns:") {
      ns.insert(prefix.to_string(), value.clone());
    }
    raw_attrs.push((key, value));
  }

  let name = expand_xml_name(&String::from_utf8_lossy(event.name().as_ref()), &ns, false);

  let mut attrs = Vec::new();
  for (key, value) in raw_attrs {
    let value = if entry_name.ends_with(".rels") && key == "Type" {
      normalize_relationship_type_uri(&value)
    } else {
      value
    };
    let value = if relaxed_bool {
      normalize_bool_lexeme(&value)
    } else {
      value
    };
    let value = if relaxed_bool {
      normalize_numeric_lexeme(&value)
    } else {
      value
    };
    if key == "xmlns" || key.starts_with("xmlns:") || key == "mc:Ignorable" {
      continue;
    }

    attrs.push((expand_xml_name(&key, &ns, true), value));
  }

  attrs.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
  (name, attrs, ns)
}

fn expand_xml_name(name: &str, namespaces: &BTreeMap<String, String>, is_attr: bool) -> String {
  if let Some((prefix, local_name)) = name.split_once(':') {
    let uri = namespaces.get(prefix).cloned().unwrap_or_default();
    format!("{{{uri}}}{local_name}")
  } else if is_attr {
    name.to_string()
  } else if let Some(uri) = namespaces.get("") {
    format!("{{{uri}}}{name}")
  } else {
    name.to_string()
  }
}

fn push_xml_node(roots: &mut Vec<XmlNode>, stack: &mut [XmlFrame], node: XmlNode) {
  if let Some(frame) = stack.last_mut() {
    frame.children.push(node);
  } else {
    roots.push(node);
  }
}

fn render_xml_nodes_for_entry(nodes: &[XmlNode], relaxed_bool: bool, entry_name: &str) -> String {
  let mut out = String::new();
  for node in nodes {
    out.push_str(&render_xml_node_to_string(node, relaxed_bool, entry_name));
  }
  out
}

fn render_xml_node_to_string(node: &XmlNode, relaxed_bool: bool, entry_name: &str) -> String {
  let mut out = String::new();
  match node {
    XmlNode::Text(text) => out.push_str(text),
    XmlNode::Element(element) => {
      if relaxed_bool
        && entry_name == "docProps/core.xml"
        && element.children.is_empty()
        && element.attrs.is_empty()
      {
        return out;
      }

      out.push('<');
      out.push_str(&element.name);

      for (key, value) in &element.attrs {
        out.push(' ');
        out.push_str(key);
        out.push_str("=\"");
        out.push_str(&escape_xml_text(value));
        out.push('"');
      }

      if element.children.is_empty() {
        out.push_str("/>");
      } else {
        out.push('>');
        if entry_name == "docProps/core.xml"
          || (entry_name == "docProps/app.xml" && is_extended_properties_root(&element.name))
        {
          let mut children = element
            .children
            .iter()
            .map(|child| render_xml_node_to_string(child, relaxed_bool, entry_name))
            .collect::<Vec<_>>();
          children.sort();
          for child in children {
            out.push_str(&child);
          }
        } else {
          for child in &element.children {
            out.push_str(&render_xml_node_to_string(child, relaxed_bool, entry_name));
          }
        }
        out.push_str("</");
        out.push_str(&element.name);
        out.push('>');
      }
    }
  }

  out
}

fn is_extended_properties_root(name: &str) -> bool {
  name == "{http://schemas.openxmlformats.org/officeDocument/2006/extended-properties}Properties"
    || name == "{http://purl.oclc.org/ooxml/officeDocument/extendedProperties}Properties"
}

fn escape_xml_text(value: &str) -> String {
  let mut out = String::with_capacity(value.len());
  for ch in value.chars() {
    match ch {
      '&' => out.push_str("&amp;"),
      '<' => out.push_str("&lt;"),
      '>' => out.push_str("&gt;"),
      _ => out.push(ch),
    }
  }
  out
}

fn normalize_bool_lexeme(value: &str) -> String {
  match value {
    "1" | "true" => "true".to_string(),
    "0" | "false" => "false".to_string(),
    "t" | "T" => "true".to_string(),
    "f" | "F" => "false".to_string(),
    _ => value.to_string(),
  }
}

fn normalize_numeric_lexeme(value: &str) -> String {
  if (value.contains('.') || value.contains('e') || value.contains('E'))
    && let Ok(parsed) = value.parse::<f64>()
  {
    return parsed.to_string();
  }

  value.to_string()
}

fn normalize_xml_text(value: &str, relaxed_bool: bool) -> String {
  let value = value.replace("\r\n", "\n").replace('\r', "\n");
  if relaxed_bool {
    normalize_bool_lexeme(&value)
  } else {
    value
  }
}

fn normalize_relationship_type_uri(value: &str) -> String {
  const PURL_PREFIX: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/";
  const OPENXML_PREFIX: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/";

  if let Some(suffix) = value.strip_prefix(PURL_PREFIX) {
    return match suffix {
      "customProperties" => format!("{OPENXML_PREFIX}custom-properties"),
      "extendedProperties" => format!("{OPENXML_PREFIX}extended-properties"),
      other => format!("{OPENXML_PREFIX}{other}"),
    };
  }

  match value {
    "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
        .to_string()
    }
    _ => value.to_string(),
  }
}

include!(concat!(env!("OUT_DIR"), "/doc_samples_tests.rs"));
