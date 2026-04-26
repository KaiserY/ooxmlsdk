#![cfg(feature = "parts")]

use std::{
  collections::BTreeMap,
  fs,
  io::{Cursor, Read},
  path::Path,
};

use ooxmlsdk::common::{PartId, RelationshipInfo};
use ooxmlsdk::parts::{
  PartRef, presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::sdk::{SdkPackage, SdkPartHandle};
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
      assert_modeled_relationships_match_relationship_sets(&original, file_name);
      assert_relationship_child_descriptor_coverage(&original, file_name);
      let mut buffer = Cursor::new(Vec::new());
      original.save(&mut buffer).unwrap();
      let roundtripped_bytes = buffer.into_inner();
      let reopened = WordprocessingDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
      assert_wordprocessing_document_round_trip(&original, &reopened);
      assert_relationship_sets_round_trip(&original, &reopened, file_name);
      assert_doc_sample_zip_equivalent(&original_bytes, &roundtripped_bytes, file_name);
    }
    DocSampleKind::Spreadsheet => {
      let original = SpreadsheetDocument::new_from_file(&path).unwrap();
      assert_modeled_relationships_match_relationship_sets(&original, file_name);
      assert_relationship_child_descriptor_coverage(&original, file_name);
      let mut buffer = Cursor::new(Vec::new());
      original.save(&mut buffer).unwrap();
      let roundtripped_bytes = buffer.into_inner();
      let reopened = SpreadsheetDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
      assert_spreadsheet_document_round_trip(&original, &reopened);
      assert_relationship_sets_round_trip(&original, &reopened, file_name);
      assert_doc_sample_zip_equivalent(&original_bytes, &roundtripped_bytes, file_name);
    }
    DocSampleKind::Presentation => {
      let original = PresentationDocument::new_from_file(&path).unwrap();
      assert_modeled_relationships_match_relationship_sets(&original, file_name);
      assert_relationship_child_descriptor_coverage(&original, file_name);
      let mut buffer = Cursor::new(Vec::new());
      original.save(&mut buffer).unwrap();
      let roundtripped_bytes = buffer.into_inner();
      let reopened = PresentationDocument::new(Cursor::new(roundtripped_bytes.clone())).unwrap();
      assert_presentation_document_round_trip(&original, &reopened);
      assert_relationship_sets_round_trip(&original, &reopened, file_name);
      assert_doc_sample_zip_equivalent(&original_bytes, &roundtripped_bytes, file_name);
    }
  }
}

fn assert_doc_sample_invalid(file_name: &str) {
  let kind = doc_sample_kind(file_name);
  let path = test_file_path(file_name);

  let result = match kind {
    DocSampleKind::Wordprocessing => {
      WordprocessingDocument::new_from_file(&path).and_then(|mut package| {
        package
          .main_document_part()?
          .root_element(&mut package)
          .map(|_| ())
      })
    }
    DocSampleKind::Spreadsheet => {
      SpreadsheetDocument::new_from_file(&path).and_then(|mut package| {
        package
          .workbook_part()?
          .root_element(&mut package)
          .map(|_| ())
      })
    }
    DocSampleKind::Presentation => {
      PresentationDocument::new_from_file(&path).and_then(|mut package| {
        package
          .presentation_part()?
          .root_element(&mut package)
          .map(|_| ())
      })
    }
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
      let main_part = package.main_document_part().unwrap();
      assert_eq!(part_path(&package, &main_part), "word/document.xml");
    }
    DocSampleKind::Spreadsheet => {
      let package = SpreadsheetDocument::new_from_file(&path).unwrap();
      let workbook_part = package.workbook_part().unwrap();
      assert_eq!(part_path(&package, &workbook_part), "xl/workbook.xml");
    }
    DocSampleKind::Presentation => {
      let package = PresentationDocument::new_from_file(&path).unwrap();
      let presentation_part = package.presentation_part().unwrap();
      assert_eq!(
        part_path(&package, &presentation_part),
        "ppt/presentation.xml"
      );
    }
  }
}

fn assert_wordprocessing_document_round_trip(
  original: &WordprocessingDocument,
  roundtripped: &WordprocessingDocument,
) {
  let original_main = original.main_document_part().unwrap();
  let roundtripped_main = roundtripped.main_document_part().unwrap();
  assert_eq!(
    part_path(original, &original_main),
    part_path(roundtripped, &roundtripped_main)
  );
  assert_eq!(
    original.relationships().len(),
    roundtripped.relationships().len()
  );
  assert_eq!(
    original_main.relationships(original).map(|rels| rels.len()),
    roundtripped_main
      .relationships(roundtripped)
      .map(|rels| rels.len())
  );
}

fn assert_spreadsheet_document_round_trip(
  original: &SpreadsheetDocument,
  roundtripped: &SpreadsheetDocument,
) {
  let original_workbook = original.workbook_part().unwrap();
  let roundtripped_workbook = roundtripped.workbook_part().unwrap();
  assert_eq!(
    part_path(original, &original_workbook),
    part_path(roundtripped, &roundtripped_workbook)
  );
  assert_eq!(
    original.relationships().len(),
    roundtripped.relationships().len()
  );
  assert_eq!(
    original_workbook
      .relationships(original)
      .map(|rels| rels.len()),
    roundtripped_workbook
      .relationships(roundtripped)
      .map(|rels| rels.len())
  );
  assert_eq!(
    original_workbook.worksheet_parts(original).count(),
    roundtripped_workbook.worksheet_parts(roundtripped).count()
  );
}

fn assert_presentation_document_round_trip(
  original: &PresentationDocument,
  roundtripped: &PresentationDocument,
) {
  let original_presentation = original.presentation_part().unwrap();
  let roundtripped_presentation = roundtripped.presentation_part().unwrap();
  assert_eq!(
    part_path(original, &original_presentation),
    part_path(roundtripped, &roundtripped_presentation)
  );
  assert_eq!(
    original.relationships().len(),
    roundtripped.relationships().len()
  );
  assert_eq!(
    original_presentation
      .relationships(original)
      .map(|rels| rels.len()),
    roundtripped_presentation
      .relationships(roundtripped)
      .map(|rels| rels.len())
  );
  assert_eq!(
    original_presentation.slide_parts(original).count(),
    roundtripped_presentation.slide_parts(roundtripped).count()
  );
  assert_eq!(
    original_presentation.slide_master_parts(original).count(),
    roundtripped_presentation
      .slide_master_parts(roundtripped)
      .count()
  );
}

fn assert_modeled_relationships_match_relationship_sets<P: SdkPackage>(
  package: &P,
  file_name: &str,
) {
  assert_eq!(
    package.modeled_relationships().unwrap(),
    package.storage().package_relationships().clone(),
    "package relationship fields do not model RelationshipSet for {file_name}"
  );

  for (index, part) in package.storage().parts().iter().enumerate() {
    if part.is_deleted() {
      continue;
    }

    let part_id = PartId::from_index(index);
    let Some(part_ref) = PartRef::from_part_id(package, part_id) else {
      continue;
    };
    let modeled_relationships = part_ref.modeled_relationships(package).unwrap();
    let relationships = package
      .storage()
      .relationships(part_id)
      .expect("active part has relationships");
    assert_eq!(
      modeled_relationships,
      relationships.clone(),
      "part relationship fields do not model RelationshipSet for {file_name}:{}",
      part.path()
    );
  }
}

fn assert_relationship_sets_round_trip<P: SdkPackage>(
  original: &P,
  roundtripped: &P,
  file_name: &str,
) {
  assert_eq!(
    original.storage().package_relationships(),
    roundtripped.storage().package_relationships(),
    "package relationships changed after save/reopen for {file_name}"
  );

  for (index, original_part) in original.storage().parts().iter().enumerate() {
    if original_part.is_deleted() {
      continue;
    }

    let original_part_id = PartId::from_index(index);
    let original_relationships = original
      .storage()
      .relationships(original_part_id)
      .expect("active part has relationships");
    let (roundtripped_part_id, _) = roundtripped
      .storage()
      .part_by_path(original_part.path())
      .unwrap_or_else(|| {
        panic!(
          "missing round-tripped part for {file_name}:{}",
          original_part.path()
        )
      });
    let roundtripped_relationships = roundtripped
      .storage()
      .relationships(roundtripped_part_id)
      .expect("active part has relationships");

    assert_eq!(
      original_relationships,
      roundtripped_relationships,
      "part relationships changed after save/reopen for {file_name}:{}",
      original_part.path()
    );
  }
}

fn assert_relationship_child_descriptor_coverage<P: SdkPackage>(package: &P, file_name: &str) {
  let package_descriptors = P::child_descriptors();
  for relationship in package
    .storage()
    .package_relationships()
    .part_relationships()
  {
    assert_relationship_covered_by_descriptors(
      package,
      package_descriptors,
      relationship,
      file_name,
      "package",
    );
  }

  for (index, part) in package.storage().parts().iter().enumerate() {
    if part.is_deleted() {
      continue;
    }

    let part_id = PartId::from_index(index);
    let Some(part_ref) = ooxmlsdk::parts::PartRef::from_part_id(package, part_id) else {
      continue;
    };
    let descriptors = part_ref.child_descriptors();
    let Some(relationships) = package.storage().relationships(part_id) else {
      continue;
    };

    for relationship in relationships.part_relationships() {
      assert_relationship_covered_by_descriptors(
        package,
        descriptors,
        relationship,
        file_name,
        part.path(),
      );
    }
  }
}

fn assert_relationship_covered_by_descriptors(
  package: &impl SdkPackage,
  descriptors: &[ooxmlsdk::sdk::PartChildDescriptor],
  relationship: &RelationshipInfo,
  file_name: &str,
  source: &str,
) {
  assert!(
    descriptors.iter().any(|descriptor| {
      ooxmlsdk::common::relationship_type_matches(
        relationship.relationship_type(),
        descriptor.relationship_type,
      )
    }) || relationship_targets_extended_part(package, relationship),
    "relationship is not covered by generated part child descriptors for {file_name}:{source}: id={} type={} target={}",
    relationship.id(),
    relationship.relationship_type(),
    relationship.target()
  );
}

fn relationship_targets_extended_part(
  package: &impl SdkPackage,
  relationship: &RelationshipInfo,
) -> bool {
  relationship
    .target_part_id()
    .and_then(|part_id| PartRef::from_part_id(package, part_id))
    .is_some_and(|part| matches!(part, PartRef::ExtendedPart(_)))
}

fn part_path<'a, P, T>(package: &'a P, part: &T) -> &'a str
where
  P: SdkPackage,
  T: SdkPartHandle,
{
  package.storage().part(part.part_id()).unwrap().path()
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
        if raw.chars().all(|ch| ch.is_whitespace()) && should_skip_whitespace_text(&stack) {
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
        if !raw.chars().all(|ch| ch.is_whitespace()) || !should_skip_whitespace_text(&stack) {
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
    let expanded_key = expand_xml_name(&key, &ns, true);
    let value = if entry_name.ends_with(".rels") && key == "Type" {
      normalize_relationship_type_uri(&value)
    } else {
      value
    };
    let value =
      if relaxed_bool && should_normalize_bool_attr(file_name, entry_name, &name, &expanded_key) {
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

    attrs.push((expanded_key, value));
  }

  attrs.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
  (name, attrs, ns)
}

fn expand_xml_name(name: &str, namespaces: &BTreeMap<String, String>, is_attr: bool) -> String {
  if let Some((prefix, local_name)) = name.split_once(':') {
    let uri = if prefix == "xml" {
      "http://www.w3.org/XML/1998/namespace".to_string()
    } else {
      namespaces
        .get(prefix)
        .map(|uri| normalize_namespace_uri(uri))
        .unwrap_or_default()
    };
    format!("{{{uri}}}{local_name}")
  } else if is_attr {
    name.to_string()
  } else if let Some(uri) = namespaces.get("") {
    format!("{{{}}}{name}", normalize_namespace_uri(uri))
  } else {
    name.to_string()
  }
}

fn should_skip_whitespace_text(stack: &[XmlFrame]) -> bool {
  let Some(frame) = stack.last() else {
    return true;
  };

  !matches!(frame.children.last(), Some(XmlNode::Text(_)))
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
    "" => "false".to_string(),
    "1" | "true" => "true".to_string(),
    "0" | "false" => "false".to_string(),
    "t" | "T" => "true".to_string(),
    "f" | "F" => "false".to_string(),
    _ => value.to_string(),
  }
}

fn should_normalize_bool_attr(
  file_name: &str,
  entry_name: &str,
  element_name: &str,
  attr_name: &str,
) -> bool {
  let _ = (file_name, entry_name);

  let (element_ns, element_local) = split_expanded_name(element_name);
  let (attr_ns, attr_local) = split_expanded_name(attr_name);

  if element_ns == "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
    && ((element_local == "sheetView" && attr_local == "tabSelected")
      || (element_local == "col" && attr_local == "customWidth")
      || (element_local == "col" && attr_local == "bestFit")
      || (element_local == "c" && attr_local == "l")
      || (element_local == "brk" && attr_local == "man")
      || (element_local == "pageSetUpPr" && attr_local == "fitToPage")
      || (element_local == "sheetFormatPr" && attr_local == "customHeight")
      || (element_local == "ignoredError" && attr_local == "emptyCellReference")
      || (element_local == "border"
        && matches!(attr_local, "diagonalDown" | "diagonalUp" | "outline"))
      || (element_local == "row"
        && matches!(
          attr_local,
          "thickBot" | "thickTop" | "customFormat" | "customHeight"
        ))
      || (element_local == "table" && attr_local == "totalsRowShown")
      || (element_local == "cellStyle" && attr_local == "customBuiltin")
      || (element_local == "pivotSelection" && attr_local == "showHeader")
      || (element_local == "pivotArea" && matches!(attr_local, "dataOnly" | "outline"))
      || (element_local == "sharedItems"
        && matches!(
          attr_local,
          "containsBlank"
            | "containsDate"
            | "containsInteger"
            | "containsMixedTypes"
            | "containsNonDate"
            | "containsNumber"
            | "containsSemiMixedTypes"
            | "containsString"
            | "longText"
        ))
      || (element_local == "xf"
        && matches!(
          attr_local,
          "applyAlignment"
            | "applyBorder"
            | "applyFill"
            | "applyFont"
            | "applyNumberFormat"
            | "applyProtection"
            | "pivotButton"
            | "quotePrefix"
        ))
      || (element_local == "headers" && attr_local == "diskRevisions")
      || (element_local == "pivotTableDefinition"
        && matches!(
          attr_local,
          "applyAlignmentFormats"
            | "applyBorderFormats"
            | "applyFontFormats"
            | "applyNumberFormats"
            | "applyPatternFormats"
            | "applyWidthHeightFormats"
            | "itemPrintTitles"
            | "multipleFieldFilters"
            | "outline"
            | "outlineData"
            | "showCalcMbrs"
            | "useAutoFormatting"
        ))
      || (element_local == "pivotField" && matches!(attr_local, "dataField" | "showAll"))
      || (element_local == "dataValidation"
        && matches!(
          attr_local,
          "allowBlank" | "showErrorMessage" | "showInputMessage"
        ))
      || (element_local == "filterColumn" && attr_local == "hiddenButton")
      || (element_local == "pivotTableStyleInfo"
        && matches!(
          attr_local,
          "showColHeaders"
            | "showColStripes"
            | "showLastColumn"
            | "showRowHeaders"
            | "showRowStripes"
        ))
      || (element_local == "tableStyleInfo"
        && matches!(
          attr_local,
          "showColumnStripes" | "showFirstColumn" | "showLastColumn" | "showRowStripes"
        ))
      || (element_local == "tableStyle" && matches!(attr_local, "table" | "pivot"))
      || (element_local == "alignment"
        && matches!(attr_local, "wrapText" | "justifyLastLine" | "shrinkToFit"))
      || (element_local == "f" && attr_local == "ca")
      || (matches!(
        element_local,
        "b" | "i" | "strike" | "condense" | "extend" | "outline" | "shadow"
      ) && attr_local == "val")
      || (element_local == "workbookPr" && attr_local == "filterPrivacy")
      || (element_local == "customWorkbookView"
        && matches!(attr_local, "maximized" | "personalView")))
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac"
    && element_local == "fonts"
    && attr_local == "knownFonts"
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/spreadsheetml/2010/11/main"
    && element_local == "workbookPr"
    && attr_local == "chartTrackingRefBase"
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main"
    && element_local == "pivotTableDefinition"
    && attr_local == "hideValuesRow"
  {
    return true;
  }

  if element_ns == "http://schemas.openxmlformats.org/presentationml/2006/main"
    && ((element_local == "presentation"
      && matches!(attr_local, "saveSubsetFonts" | "autoCompressPictures"))
      || (element_local == "showPr" && attr_local == "showNarration")
      || (element_local == "sldMaster" && attr_local == "preserve")
      || (element_local == "sldLayout"
        && matches!(attr_local, "preserve" | "showMasterSp" | "userDrawn"))
      || (element_local == "sld" && attr_local == "showMasterPhAnim")
      || (element_local == "normalViewPr" && attr_local == "showOutlineIcons")
      || (element_local == "restoredLeft" && attr_local == "autoAdjust")
      || (element_local == "cNvPr" && attr_local == "hidden")
      || (element_local == "nvPr" && attr_local == "userDrawn")
      || (element_local == "ph" && attr_local == "hasCustomPrompt")
      || (element_local == "seq" && attr_local == "concurrent")
      || (element_local == "cTn" && attr_local == "display")
      || (element_local == "sndTgt" && attr_local == "builtIn")
      || (element_local == "bldP" && matches!(attr_local, "animBg" | "autoUpdateAnimBg" | "rev"))
      || (element_local == "blipFill" && attr_local == "rotWithShape")
      || (element_local == "cNvSpPr" && attr_local == "txBox")
      || (element_local == "cViewPr" && attr_local == "varScale")
      || (element_local == "cSldViewPr" && matches!(attr_local, "snapToGrid" | "showGuides")))
  {
    return true;
  }

  if element_ns == "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing"
    && element_local == "cNvSpPr"
    && attr_local == "txBox"
  {
    return true;
  }

  if element_ns == "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing"
    && element_local == "cNvSpPr"
    && attr_local == "txBox"
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/powerpoint/2010/main"
    && element_local == "discardImageEditData"
    && attr_local == "val"
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/powerpoint/2012/main"
    && element_local == "chartTrackingRefBased"
    && attr_local == "val"
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/2020/mipLabelMetadata"
    && element_local == "label"
    && matches!(attr_local, "enabled" | "removed")
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/drawing/2014/chartex"
    && ((element_local == "externalData" && attr_local == "autoUpdate")
      || (matches!(element_local, "title" | "legend") && attr_local == "overlay")
      || (element_local == "visibility"
        && matches!(
          attr_local,
          "categoryName"
            | "connectorLines"
            | "meanLine"
            | "meanMarker"
            | "nonoutliers"
            | "outliers"
            | "seriesName"
            | "value"
        )))
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/drawing/2007/8/2/chart"
    && matches!(
      element_local,
      "dropZoneFilter"
        | "dropZoneCategories"
        | "dropZoneData"
        | "dropZoneSeries"
        | "dropZonesVisible"
    )
    && attr_local == "val"
  {
    return true;
  }

  if element_ns == "http://schemas.openxmlformats.org/drawingml/2006/main"
    && ((matches!(
      element_local,
      "lvl1pPr"
        | "lvl2pPr"
        | "lvl3pPr"
        | "lvl4pPr"
        | "lvl5pPr"
        | "lvl6pPr"
        | "lvl7pPr"
        | "lvl8pPr"
        | "lvl9pPr"
    ) && matches!(
      attr_local,
      "rtl" | "eaLnBrk" | "hangingPunct" | "latinLnBrk"
    )) || (matches!(element_local, "defPPr" | "pPr")
      && matches!(
        attr_local,
        "rtl" | "eaLnBrk" | "hangingPunct" | "latinLnBrk"
      ))
      || (matches!(element_local, "rPr" | "endParaRPr" | "defRPr")
        && matches!(attr_local, "b" | "i"))
      || (matches!(element_local, "rPr" | "endParaRPr" | "defRPr")
        && matches!(attr_local, "dirty" | "smtClean" | "kumimoji" | "normalizeH"))
      || (element_local == "blipFill" && attr_local == "rotWithShape")
      || (element_local == "xfrm" && matches!(attr_local, "flipH" | "flipV"))
      || (element_local == "bodyPr"
        && matches!(
          attr_local,
          "anchorCtr"
            | "spcFirstLastPara"
            | "rtlCol"
            | "fromWordArt"
            | "forceAA"
            | "compatLnSpc"
            | "upright"
        ))
      || (matches!(
        element_local,
        "spLocks" | "picLocks" | "graphicFrameLocks" | "cxnSpLocks"
      ) && matches!(
        attr_local,
        "noChangeAspect"
          | "noChangeArrowheads"
          | "noChangeShapeType"
          | "noGrp"
          | "noTextEdit"
          | "noRot"
      )))
  {
    return true;
  }

  if element_ns == "http://schemas.microsoft.com/office/drawing/2012/chartStyle"
    && ((element_local == "bodyPr"
      && matches!(
        attr_local,
        "anchorCtr" | "spcFirstLastPara" | "rtlCol" | "fromWordArt" | "forceAA" | "compatLnSpc"
      ))
      || (element_local == "defRPr" && matches!(attr_local, "b" | "i")))
  {
    return true;
  }

  if element_ns == "urn:schemas-microsoft-com:vml"
    && element_local == "rect"
    && attr_local == "stroked"
  {
    return true;
  }

  if element_local == "rect" && matches!(attr_local, "hr" | "hrstd" | "hrnoshade") {
    return true;
  }

  if element_ns == "urn:schemas-microsoft-com:vml"
    && element_local == "shape"
    && attr_ns == "urn:schemas-microsoft-com:office:office"
    && attr_local == "bullet"
  {
    return true;
  }

  if element_ns == "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    && attr_ns == "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    && ((matches!(attr_local, "val")
      && matches!(
        element_local,
        "b"
          | "bCs"
          | "i"
          | "iCs"
          | "adjustRightInd"
          | "caps"
          | "noProof"
          | "pageBreakBefore"
          | "snapToGrid"
          | "widowControl"
          | "autoSpaceDE"
          | "autoSpaceDN"
      ))
      || (element_local == "spacing"
        && matches!(attr_local, "afterAutospacing" | "beforeAutospacing")))
  {
    return true;
  }

  matches!(
    (element_name, attr_name),
    (
      "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}sheetView",
      "tabSelected"
    ) | (
      "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}col",
      "customWidth"
    ) | (
      "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}sharedItems",
      "containsBlank"
        | "containsDate"
        | "containsInteger"
        | "containsMixedTypes"
        | "containsNonDate"
        | "containsNumber"
        | "containsSemiMixedTypes"
        | "containsString"
        | "longText"
    ) | (
      "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}xf",
      "applyAlignment"
        | "applyBorder"
        | "applyFill"
        | "applyFont"
        | "applyNumberFormat"
        | "applyProtection"
        | "pivotButton"
        | "quotePrefix"
    ) | (
      "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}c",
      "l"
    ) | (
      "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}brk",
      "man"
    ) | (
      "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}fonts",
      "{http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac}knownFonts"
    ) | (
      "{http://schemas.openxmlformats.org/presentationml/2006/main}presentation",
      "saveSubsetFonts"
    ) | (
      "{http://schemas.openxmlformats.org/presentationml/2006/main}sld",
      "showMasterPhAnim"
    ) | (
      "{http://schemas.openxmlformats.org/presentationml/2006/main}ph",
      "hasCustomPrompt"
    ) | (
      "{http://schemas.openxmlformats.org/presentationml/2006/main}cTn",
      "display"
    ) | (
      "{http://schemas.openxmlformats.org/presentationml/2006/main}normalViewPr",
      "showOutlineIcons"
    ) | (
      "{http://schemas.openxmlformats.org/presentationml/2006/main}sndTgt",
      "builtIn"
    ) | (
      "{http://schemas.openxmlformats.org/presentationml/2006/main}bldP",
      "animBg" | "autoUpdateAnimBg" | "rev"
    ) | (
      "{http://schemas.microsoft.com/office/powerpoint/2010/main}discardImageEditData",
      "val"
    ) | (
      "{http://schemas.microsoft.com/office/drawing/2014/chartex}externalData",
      "{http://schemas.microsoft.com/office/drawing/2014/chartex}autoUpdate"
    ) | (
      "{http://schemas.microsoft.com/office/drawing/2014/chartex}title",
      "overlay"
    ) | (
      "{http://schemas.microsoft.com/office/drawing/2014/chartex}legend",
      "overlay"
    ) | (
      "{http://schemas.microsoft.com/office/drawing/2014/chartex}visibility",
      "categoryName" | "seriesName" | "value"
    ) | (
      "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}i",
      "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}val"
    ) | (
      "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}caps",
      "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}val"
    ) | (
      "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}widowControl",
      "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}val"
    ) | (
      "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl1pPr"
        | "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl2pPr"
        | "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl3pPr"
        | "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl4pPr"
        | "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl5pPr"
        | "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl6pPr"
        | "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl7pPr"
        | "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl8pPr"
        | "{http://schemas.openxmlformats.org/drawingml/2006/main}lvl9pPr",
      "rtl" | "eaLnBrk" | "hangingPunct" | "latinLnBrk"
    ) | (
      "{http://schemas.microsoft.com/office/drawing/2012/chartStyle}bodyPr",
      "anchorCtr" | "spcFirstLastPara" | "rtlCol" | "fromWordArt" | "forceAA" | "compatLnSpc"
    ) | (
      "{http://schemas.openxmlformats.org/drawingml/2006/main}rPr",
      "smtClean" | "err" | "kumimoji" | "normalizeH"
    ) | (
      "{http://schemas.openxmlformats.org/drawingml/2006/main}pPr",
      "eaLnBrk" | "hangingPunct" | "latinLnBrk"
    ) | (
      "{http://schemas.openxmlformats.org/drawingml/2006/main}defPPr",
      "rtl" | "eaLnBrk" | "hangingPunct" | "latinLnBrk"
    ) | (
      "{http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing}anchor",
      "simplePos" | "behindDoc" | "locked" | "layoutInCell" | "allowOverlap"
    ) | (
      "{http://schemas.openxmlformats.org/drawingml/2006/main}xfrm",
      "flipV"
    ) | (
      "{http://schemas.openxmlformats.org/drawingml/2006/main}graphicFrameLocks",
      "noChangeAspect" | "noChangeArrowheads"
    ) | (
      "{urn:schemas-microsoft-com:vml}shapetype",
      "filled"
        | "stroked"
        | "{urn:schemas-microsoft-com:office:office}oned"
        | "{urn:schemas-microsoft-com:office:office}preferrelative"
    ) | (
      "{urn:schemas-microsoft-com:vml}path",
      "arrowok"
        | "fillok"
        | "gradientshapeok"
        | "textpathok"
        | "{urn:schemas-microsoft-com:office:office}extrusionok"
    ) | (
      "{urn:schemas-microsoft-com:office:office}lock",
      "shapetype" | "aspectratio" | "text"
    ) | (
      "{urn:schemas-microsoft-com:vml}shape",
      "{urn:schemas-microsoft-com:office:office}ole"
        | "{urn:schemas-microsoft-com:office:office}allowincell"
        | "filled"
        | "stroked"
    ) | ("{urn:schemas-microsoft-com:vml}textpath", "fitshape" | "on")
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}latentStyles",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}defLockedState"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}defQFormat"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}defSemiHidden"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}defUnhideWhenUsed"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}lsdException",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}locked"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}qFormat"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}semiHidden"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}unhideWhenUsed"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}style",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}default"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}customStyle"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}hyperlink",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}history"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}b",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}val"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}bCs",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}val"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}noProof",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}val"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}lvl",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}tentative"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}tblLook",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}firstRow"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}lastRow"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}firstColumn"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}lastColumn"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}noHBand"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}noVBand"
      )
      | (
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}cnfStyle",
        "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}firstRow"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}lastRow"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}firstColumn"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}lastColumn"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}oddVBand"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}evenVBand"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}oddHBand"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}evenHBand"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}firstRowFirstColumn"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}firstRowLastColumn"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}lastRowFirstColumn"
          | "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}lastRowLastColumn"
      )
      | (
        "{http://schemas.microsoft.com/office/word/2012/wordml}commentEx",
        "{http://schemas.microsoft.com/office/word/2012/wordml}done"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}date1904",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}roundedCorners",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}overlay",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}autoTitleDeleted",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}varyColors",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}invertIfNegative",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}showLegendKey"
          | "{http://schemas.openxmlformats.org/drawingml/2006/chart}showVal"
          | "{http://schemas.openxmlformats.org/drawingml/2006/chart}showCatName"
          | "{http://schemas.openxmlformats.org/drawingml/2006/chart}showSerName"
          | "{http://schemas.openxmlformats.org/drawingml/2006/chart}showPercent"
          | "{http://schemas.openxmlformats.org/drawingml/2006/chart}showBubbleSize",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}delete",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}numFmt",
        "sourceLinked"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}auto",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}noMultiLvlLbl",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}plotVisOnly",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}showDLblsOverMax",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/chart}autoUpdate",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/main}bodyPr",
        "anchorCtr" | "spcFirstLastPara" | "rtlCol" | "fromWordArt" | "forceAA" | "compatLnSpc"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/main}gradFill",
        "rotWithShape"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/main}lin",
        "scaled"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/main}outerShdw",
        "rotWithShape"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/main}defRPr",
        "b" | "i"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/main}picLocks",
        "noChangeAspect" | "noChangeArrowheads"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/main}spLocks",
        "noChangeArrowheads" | "noChangeShapeType"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/main}cxnSpLocks",
        "noChangeShapeType"
      )
      | (
        "{http://schemas.microsoft.com/office/word/2010/wordprocessingShape}bodyPr",
        "anchor"
          | "anchorCtr"
          | "spcFirstLastPara"
          | "rtlCol"
          | "fromWordArt"
          | "forceAA"
          | "compatLnSpc"
      )
      | (
        "{http://schemas.microsoft.com/office/word/2010/wordprocessingShape}cNvSpPr",
        "txBox"
      )
      | (
        "{http://schemas.microsoft.com/office/drawing/2010/main}useLocalDpi",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/diagram}prSet",
        "phldr"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/diagram}bulletEnabled",
        "val"
      )
      | (
        "{http://schemas.openxmlformats.org/drawingml/2006/diagram}shape",
        "hideGeom"
      )
  )
}

fn split_expanded_name(name: &str) -> (&str, &str) {
  if let Some(rest) = name.strip_prefix('{')
    && let Some((ns, local)) = rest.split_once('}')
  {
    (ns, local)
  } else {
    ("", name)
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

fn normalize_namespace_uri(value: &str) -> String {
  match value {
    "http://purl.oclc.org/ooxml/descriptions/base" => {
      "http://descriptions.openxmlformats.org/description/base".to_string()
    }
    "http://purl.oclc.org/ooxml/descriptions/full" => {
      "http://descriptions.openxmlformats.org/description/full".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/chart" => {
      "http://schemas.openxmlformats.org/drawingml/2006/chart".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/chartDrawing" => {
      "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/compatibility" => {
      "http://schemas.openxmlformats.org/drawingml/2006/compatibility".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/diagram" => {
      "http://schemas.openxmlformats.org/drawingml/2006/diagram".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/lockedCanvas" => {
      "http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/main" => {
      "http://schemas.openxmlformats.org/drawingml/2006/main".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/picture" => {
      "http://schemas.openxmlformats.org/drawingml/2006/picture".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/spreadsheetDrawing" => {
      "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing".to_string()
    }
    "http://purl.oclc.org/ooxml/drawingml/wordprocessingDrawing" => {
      "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/bibliography" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/bibliography".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/customProperties" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/customXml" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/customXml".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/customXmlDataProps" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/customXmlDataProps".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/docPropsVTypes" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/extendedProperties" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/math" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/math".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/relationships" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships".to_string()
    }
    "http://purl.oclc.org/ooxml/officeDocument/sharedTypes" => {
      "http://schemas.openxmlformats.org/officeDocument/2006/sharedTypes".to_string()
    }
    "http://purl.oclc.org/ooxml/presentationml/main" => {
      "http://schemas.openxmlformats.org/presentationml/2006/main".to_string()
    }
    "http://purl.oclc.org/ooxml/schemaLibrary/main" => {
      "http://schemas.openxmlformats.org/schemaLibrary/2006/main".to_string()
    }
    "http://purl.oclc.org/ooxml/spreadsheetml/main" => {
      "http://schemas.openxmlformats.org/spreadsheetml/2006/main".to_string()
    }
    "http://purl.oclc.org/ooxml/wordprocessingml/main" => {
      "http://schemas.openxmlformats.org/wordprocessingml/2006/main".to_string()
    }
    _ => value.to_string(),
  }
}

include!(concat!(env!("OUT_DIR"), "/doc_samples_tests.rs"));
