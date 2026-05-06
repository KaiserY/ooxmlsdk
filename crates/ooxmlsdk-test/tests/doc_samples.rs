#![cfg(feature = "parts")]
#![allow(dead_code)]

use std::{
  collections::BTreeMap,
  fs,
  io::{Cursor, Read},
  path::Path,
};

use ooxmlsdk::parts::{
  PartRef, presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::sdk::{SdkPackage, SdkPart};
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
  assert_eq!(original.parts().count(), roundtripped.parts().count());
  assert_eq!(
    original_main.get_all_parts(original).count(),
    roundtripped_main.get_all_parts(roundtripped).count()
  );
  assert_package_part_graph_equal(original, roundtripped);
  assert_part_subgraph_equal(original, roundtripped, original_main, roundtripped_main);
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
  assert_eq!(original.parts().count(), roundtripped.parts().count());
  assert_eq!(
    original_workbook.worksheet_parts(original).count(),
    roundtripped_workbook.worksheet_parts(roundtripped).count()
  );
  assert_package_part_graph_equal(original, roundtripped);
  assert_part_subgraph_equal(
    original,
    roundtripped,
    original_workbook,
    roundtripped_workbook,
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
  assert_eq!(original.parts().count(), roundtripped.parts().count());
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
  assert_package_part_graph_equal(original, roundtripped);
  assert_part_subgraph_equal(
    original,
    roundtripped,
    original_presentation,
    roundtripped_presentation,
  );
}

fn assert_package_part_graph_equal<P>(original: &P, roundtripped: &P)
where
  P: SdkPackage,
{
  assert_eq!(
    package_direct_part_signature(original),
    package_direct_part_signature(roundtripped),
    "package direct part relationship graph differs"
  );
  assert_eq!(
    all_part_signature(original.get_all_parts().collect()),
    all_part_signature(roundtripped.get_all_parts().collect()),
    "package reachable part graph differs"
  );
}

fn assert_part_subgraph_equal<P, T>(
  original_package: &P,
  roundtripped_package: &P,
  original_part: T,
  roundtripped_part: T,
) where
  P: SdkPackage,
  T: SdkPart,
{
  assert_eq!(
    direct_part_signature(
      original_package,
      original_part.parts(original_package).collect()
    ),
    direct_part_signature(
      roundtripped_package,
      roundtripped_part.parts(roundtripped_package).collect()
    ),
    "main part direct relationship graph differs"
  );
  assert_eq!(
    all_part_signature(original_part.get_all_parts(original_package).collect()),
    all_part_signature(
      roundtripped_part
        .get_all_parts(roundtripped_package)
        .collect()
    ),
    "main part reachable graph differs"
  );
}

fn package_direct_part_signature<P: SdkPackage>(package: &P) -> Vec<(String, String)> {
  direct_part_signature(package, package.parts().collect())
}

fn direct_part_signature<P: SdkPackage>(
  _package: &P,
  parts: Vec<ooxmlsdk::parts::IdPartPair<'_>>,
) -> Vec<(String, String)> {
  parts
    .into_iter()
    .map(|pair| {
      (
        pair.relationship_id.to_string(),
        format!("{:?}", pair.part.part_id()),
      )
    })
    .collect()
}

fn all_part_signature(parts: Vec<PartRef>) -> Vec<String> {
  parts
    .into_iter()
    .map(|part| format!("{:?}", part.part_id()))
    .collect()
}

fn part_path<'a, P, T>(package: &'a P, part: &T) -> &'a str
where
  P: ooxmlsdk::sdk::SdkPackage,
  T: SdkPart,
{
  part.path(package).unwrap()
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
  let mut errors = Vec::new();

  for name in original.keys() {
    if !roundtripped.contains_key(name) {
      errors.push(format!("missing zip entry: {name}"));
    }
  }

  for name in roundtripped.keys() {
    if !original.contains_key(name) {
      errors.push(format!("extra zip entry: {name}"));
    }
  }

  for name in original_names {
    let original_bytes = original.get(name).expect("original entry missing");
    let Some(roundtripped_bytes) = roundtripped.get(name) else {
      continue;
    };

    if is_xml_entry(name) {
      errors.extend(xml_equivalence_errors(
        original_bytes,
        roundtripped_bytes,
        file_name,
        name,
      ));
    } else if original_bytes != roundtripped_bytes {
      errors.push(format!(
        "{name}: binary entry mismatch: original {} bytes, roundtripped {} bytes",
        original_bytes.len(),
        roundtripped_bytes.len()
      ));
    }
  }

  assert!(
    errors.is_empty(),
    "doc sample round-trip mismatch for {file_name}\n{}",
    format_doc_sample_errors(&errors)
  );
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

fn format_doc_sample_errors(errors: &[String]) -> String {
  const MAX_ERRORS: usize = 120;

  let mut out = String::new();
  for error in errors.iter().take(MAX_ERRORS) {
    out.push_str("- ");
    out.push_str(error);
    out.push('\n');
  }

  if errors.len() > MAX_ERRORS {
    out.push_str(&format!(
      "- ... {} additional mismatches omitted\n",
      errors.len() - MAX_ERRORS
    ));
  }

  out
}

fn xml_equivalence_errors(
  original: &[u8],
  roundtripped: &[u8],
  file_name: &str,
  entry_name: &str,
) -> Vec<String> {
  let strict_options = CanonicalOptions::strict();
  let strict_original = canonicalize_xml(original, strict_options, file_name, entry_name);
  let strict_roundtripped = canonicalize_xml(roundtripped, strict_options, file_name, entry_name);
  let strict_errors = compare_xml_documents(&strict_original, &strict_roundtripped);

  if strict_errors.is_empty() {
    return Vec::new();
  }

  let relaxed_options = CanonicalOptions::relaxed_for_entry(entry_name);
  let relaxed_original = canonicalize_xml(original, relaxed_options, file_name, entry_name);
  let relaxed_roundtripped = canonicalize_xml(roundtripped, relaxed_options, file_name, entry_name);
  let relaxed_errors = compare_xml_documents(&relaxed_original, &relaxed_roundtripped);

  if relaxed_errors.is_empty() {
    return Vec::new();
  }

  format_xml_equivalence_errors(entry_name, relaxed_options, &strict_errors, &relaxed_errors)
}

fn format_xml_equivalence_errors(
  entry_name: &str,
  relaxed_options: CanonicalOptions,
  strict_errors: &[String],
  relaxed_errors: &[String],
) -> Vec<String> {
  let mut errors = Vec::new();
  errors.push(format!(
    "{entry_name}: xml mismatch after strict and compatible comparison ({})",
    relaxed_options.describe()
  ));

  for error in relaxed_errors.iter().take(24) {
    errors.push(format!("{entry_name}: relaxed: {error}"));
  }

  if relaxed_errors.len() > 24 {
    errors.push(format!(
      "{entry_name}: relaxed: ... {} additional XML mismatches omitted",
      relaxed_errors.len() - 24
    ));
  }

  errors.push(format!(
    "{entry_name}: strict comparison found {} mismatch(es); first strict mismatch: {}",
    strict_errors.len(),
    strict_errors.first().map(String::as_str).unwrap_or("none")
  ));

  errors
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct XmlElement {
  name: String,
  attrs: Vec<(String, String)>,
  children: Vec<XmlNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum XmlNode {
  Declaration(XmlDeclaration),
  Element(XmlElement),
  Text(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum XmlDeclaration {
  Plain,
  Standalone,
}

#[derive(Debug)]
struct XmlFrame {
  name: String,
  attrs: Vec<(String, String)>,
  children: Vec<XmlNode>,
  ns: BTreeMap<String, String>,
}

#[derive(Clone, Copy)]
struct CanonicalOptions {
  normalize_bool_attrs: bool,
  normalize_bool_text: bool,
  normalize_numeric: bool,
  ignore_empty_core_property: bool,
  sort_package_properties: bool,
}

impl CanonicalOptions {
  fn strict() -> Self {
    Self {
      normalize_bool_attrs: false,
      normalize_bool_text: false,
      normalize_numeric: false,
      ignore_empty_core_property: false,
      sort_package_properties: false,
    }
  }

  fn relaxed_for_entry(entry_name: &str) -> Self {
    Self {
      normalize_bool_attrs: true,
      normalize_bool_text: matches!(entry_name, "docProps/app.xml" | "docProps/custom.xml"),
      normalize_numeric: should_relax_numeric_lexemes(entry_name),
      ignore_empty_core_property: entry_name == "docProps/core.xml",
      sort_package_properties: matches!(entry_name, "docProps/app.xml" | "docProps/core.xml"),
    }
  }

  fn describe(self) -> String {
    let mut enabled = Vec::new();
    if self.normalize_bool_attrs {
      enabled.push("bool attrs");
    }
    if self.normalize_bool_text {
      enabled.push("bool text");
    }
    if self.normalize_numeric {
      enabled.push("numeric lexemes");
    }
    if self.ignore_empty_core_property {
      enabled.push("empty core properties");
    }
    if self.sort_package_properties {
      enabled.push("package property order");
    }

    if enabled.is_empty() {
      "no compatibility relaxations enabled".to_string()
    } else {
      format!("compatibility relaxations: {}", enabled.join(", "))
    }
  }
}

fn canonicalize_xml(
  xml: &[u8],
  options: CanonicalOptions,
  file_name: &str,
  entry_name: &str,
) -> Vec<XmlNode> {
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
          options,
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
          options,
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
          let text = normalize_xml_text(&raw, options, &stack);
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
          let text = normalize_xml_text(&raw, options, &stack);
          push_xml_node(&mut roots, &mut stack, XmlNode::Text(text));
        }
      }
      Event::Decl(event) => {
        push_xml_node(
          &mut roots,
          &mut stack,
          XmlNode::Declaration(xml_declaration_from_event(&event)),
        );
      }
      Event::Comment(_) | Event::PI(_) | Event::DocType(_) => {}
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
  normalize_xml_nodes_for_entry(roots, options, entry_name)
}

fn normalize_xml_nodes_for_entry(
  nodes: Vec<XmlNode>,
  options: CanonicalOptions,
  entry_name: &str,
) -> Vec<XmlNode> {
  let nodes = nodes
    .into_iter()
    .filter_map(|node| normalize_xml_node_for_entry(node, options, entry_name))
    .collect();
  collapse_adjacent_xml_text_nodes(nodes)
}

fn normalize_xml_node_for_entry(
  node: XmlNode,
  options: CanonicalOptions,
  entry_name: &str,
) -> Option<XmlNode> {
  match node {
    XmlNode::Element(mut element) => {
      element.children = element
        .children
        .into_iter()
        .filter_map(|child| normalize_xml_node_for_entry(child, options, entry_name))
        .collect();
      element.children = collapse_adjacent_xml_text_nodes(element.children);

      if options.ignore_empty_core_property
        && element.children.is_empty()
        && element.attrs.is_empty()
      {
        return None;
      }

      if options.sort_package_properties
        && (entry_name == "docProps/core.xml"
          || (entry_name == "docProps/app.xml" && is_extended_properties_root(&element.name))
          || (entry_name == "docProps/app.xml"
            && split_expanded_name(&element.name).1 == "Properties"))
      {
        element.children.sort_by_key(xml_node_structural_sort_key);
      }

      Some(XmlNode::Element(element))
    }
    other => Some(other),
  }
}

fn collapse_adjacent_xml_text_nodes(nodes: Vec<XmlNode>) -> Vec<XmlNode> {
  let mut collapsed = Vec::with_capacity(nodes.len());

  for node in nodes {
    match (collapsed.last_mut(), node) {
      (Some(XmlNode::Text(existing)), XmlNode::Text(text)) => existing.push_str(&text),
      (_, node) => collapsed.push(node),
    }
  }

  collapsed
}

fn compare_xml_documents(original: &[XmlNode], roundtripped: &[XmlNode]) -> Vec<String> {
  let mut errors = Vec::new();
  compare_xml_node_lists("$", original, roundtripped, &mut errors);
  errors
}

fn compare_xml_node_lists(
  parent_path: &str,
  original: &[XmlNode],
  roundtripped: &[XmlNode],
  errors: &mut Vec<String>,
) {
  let mut original_idx = 0;
  let mut roundtripped_idx = 0;

  while original_idx < original.len() && roundtripped_idx < roundtripped.len() {
    match (&original[original_idx], &roundtripped[roundtripped_idx]) {
      (XmlNode::Declaration(decl), node) if !matches!(node, XmlNode::Declaration(_)) => {
        errors.push(format!(
          "{}: missing XML declaration {} before {}",
          xml_child_path(parent_path, original, original_idx),
          xml_declaration_summary(*decl),
          xml_node_summary(node)
        ));
        original_idx += 1;
      }
      (node, XmlNode::Declaration(decl)) if !matches!(node, XmlNode::Declaration(_)) => {
        errors.push(format!(
          "{}: extra XML declaration {} before {}",
          xml_child_path(parent_path, original, original_idx),
          xml_declaration_summary(*decl),
          xml_node_summary(node)
        ));
        roundtripped_idx += 1;
      }
      _ => {
        let path = xml_child_path(parent_path, original, original_idx);
        compare_xml_node(
          &path,
          &original[original_idx],
          &roundtripped[roundtripped_idx],
          errors,
        );
        original_idx += 1;
        roundtripped_idx += 1;
      }
    }
  }

  for node in &original[original_idx..] {
    errors.push(format!(
      "{parent_path}: missing child in roundtripped XML: {}",
      xml_node_summary(node)
    ));
  }

  for node in &roundtripped[roundtripped_idx..] {
    errors.push(format!(
      "{parent_path}: extra child in roundtripped XML: {}",
      xml_node_summary(node)
    ));
  }
}

fn compare_xml_node(
  path: &str,
  original: &XmlNode,
  roundtripped: &XmlNode,
  errors: &mut Vec<String>,
) {
  match (original, roundtripped) {
    (XmlNode::Declaration(left), XmlNode::Declaration(right)) => {
      if left != right {
        errors.push(format!(
          "{path}: XML declaration mismatch: original {}, roundtripped {}",
          xml_declaration_summary(*left),
          xml_declaration_summary(*right)
        ));
      }
    }
    (XmlNode::Declaration(left), other) => {
      errors.push(format!(
        "{path}: missing XML declaration {} before {}",
        xml_declaration_summary(*left),
        xml_node_summary(other)
      ));
    }
    (other, XmlNode::Declaration(right)) => {
      errors.push(format!(
        "{path}: extra XML declaration {} before {}",
        xml_declaration_summary(*right),
        xml_node_summary(other)
      ));
    }
    (XmlNode::Text(left), XmlNode::Text(right)) => {
      if left != right {
        errors.push(format!(
          "{path}: text mismatch: original {:?}, roundtripped {:?}",
          truncate_for_error(left),
          truncate_for_error(right)
        ));
      }
    }
    (XmlNode::Element(left), XmlNode::Element(right)) => {
      compare_xml_element(path, left, right, errors);
    }
    (left, right) => {
      errors.push(format!(
        "{path}: node kind mismatch: original {}, roundtripped {}",
        xml_node_summary(left),
        xml_node_summary(right)
      ));
    }
  }
}

fn compare_xml_element(
  path: &str,
  original: &XmlElement,
  roundtripped: &XmlElement,
  errors: &mut Vec<String>,
) {
  if original.name != roundtripped.name {
    push_xml_name_error(path, "element", &original.name, &roundtripped.name, errors);
  }

  compare_xml_attrs(path, &original.attrs, &roundtripped.attrs, errors);
  compare_xml_node_lists(path, &original.children, &roundtripped.children, errors);
}

fn compare_xml_attrs(
  path: &str,
  original: &[(String, String)],
  roundtripped: &[(String, String)],
  errors: &mut Vec<String>,
) {
  let original = original
    .iter()
    .map(|(k, v)| (k.as_str(), v.as_str()))
    .collect::<BTreeMap<_, _>>();
  let roundtripped = roundtripped
    .iter()
    .map(|(k, v)| (k.as_str(), v.as_str()))
    .collect::<BTreeMap<_, _>>();

  for (name, value) in &original {
    match roundtripped.get(name) {
      Some(roundtripped_value) if roundtripped_value == value => {}
      Some(roundtripped_value) => errors.push(format!(
        "{path}: attr value mismatch for {}: original {:?}, roundtripped {:?}",
        readable_xml_name(name),
        truncate_for_error(value),
        truncate_for_error(roundtripped_value)
      )),
      None => errors.push(format!(
        "{path}: missing attr in roundtripped XML: {}={:?}",
        readable_xml_name(name),
        truncate_for_error(value)
      )),
    }
  }

  for (name, value) in &roundtripped {
    if !original.contains_key(name) {
      errors.push(format!(
        "{path}: extra attr in roundtripped XML: {}={:?}",
        readable_xml_name(name),
        truncate_for_error(value)
      ));
    }
  }
}

fn push_xml_name_error(
  path: &str,
  kind: &str,
  original: &str,
  roundtripped: &str,
  errors: &mut Vec<String>,
) {
  let (original_ns, original_local) = split_expanded_name(original);
  let (roundtripped_ns, roundtripped_local) = split_expanded_name(roundtripped);

  if original_local == roundtripped_local && !original_ns.is_empty() && roundtripped_ns.is_empty() {
    errors.push(format!(
      "{path}: missing namespace on {kind}: local name {original_local}, expected namespace {original_ns}"
    ));
  } else if original_local == roundtripped_local
    && original_ns.is_empty()
    && !roundtripped_ns.is_empty()
  {
    errors.push(format!(
      "{path}: extra namespace on {kind}: local name {original_local}, roundtripped namespace {roundtripped_ns}"
    ));
  } else if original_local == roundtripped_local {
    errors.push(format!(
      "{path}: {kind} namespace mismatch: original {original_ns}, roundtripped {roundtripped_ns}, local name {original_local}"
    ));
  } else {
    errors.push(format!(
      "{path}: {kind} name mismatch: original {}, roundtripped {}",
      readable_xml_name(original),
      readable_xml_name(roundtripped)
    ));
  }
}

fn xml_child_path(parent_path: &str, siblings: &[XmlNode], idx: usize) -> String {
  let node = &siblings[idx];
  let ordinal = xml_child_ordinal(siblings, idx);
  match node {
    XmlNode::Declaration(_) => format!("{parent_path}/xml-declaration[{ordinal}]"),
    XmlNode::Element(element) => {
      format!(
        "{parent_path}/{}[{ordinal}]",
        readable_xml_name(&element.name)
      )
    }
    XmlNode::Text(_) => format!("{parent_path}/text()[{ordinal}]"),
  }
}

fn xml_child_ordinal(siblings: &[XmlNode], idx: usize) -> usize {
  let node = &siblings[idx];
  siblings[..=idx]
    .iter()
    .filter(|candidate| xml_nodes_share_path_name(candidate, node))
    .count()
}

fn xml_nodes_share_path_name(left: &XmlNode, right: &XmlNode) -> bool {
  match (left, right) {
    (XmlNode::Declaration(_), XmlNode::Declaration(_)) => true,
    (XmlNode::Text(_), XmlNode::Text(_)) => true,
    (XmlNode::Element(left), XmlNode::Element(right)) => left.name == right.name,
    _ => false,
  }
}

fn xml_node_summary(node: &XmlNode) -> String {
  match node {
    XmlNode::Declaration(decl) => format!("XML declaration {}", xml_declaration_summary(*decl)),
    XmlNode::Element(element) => format!("element {}", readable_xml_name(&element.name)),
    XmlNode::Text(text) => format!("text {:?}", truncate_for_error(text)),
  }
}

fn xml_node_structural_sort_key(node: &XmlNode) -> String {
  match node {
    XmlNode::Declaration(decl) => format!("0:{}", xml_declaration_summary(*decl)),
    XmlNode::Element(element) => {
      let (_, local_name) = split_expanded_name(&element.name);
      let mut key = format!("1:{local_name}");
      for (name, value) in &element.attrs {
        let (_, attr_local_name) = split_expanded_name(name);
        key.push('|');
        key.push_str(attr_local_name);
        key.push('=');
        key.push_str(value);
      }
      for child in &element.children {
        key.push_str(">{");
        key.push_str(&xml_node_structural_sort_key(child));
        key.push('}');
      }
      key
    }
    XmlNode::Text(text) => format!("2:{text}"),
  }
}

fn xml_declaration_summary(decl: XmlDeclaration) -> &'static str {
  match decl {
    XmlDeclaration::Plain => "<?xml?>",
    XmlDeclaration::Standalone => "<?xml standalone=\"yes\"?>",
  }
}

fn readable_xml_name(name: &str) -> String {
  let (ns, local) = split_expanded_name(name);
  if ns.is_empty() {
    local.to_string()
  } else {
    format!("{{{ns}}}{local}")
  }
}

fn truncate_for_error(value: &str) -> String {
  const MAX_CHARS: usize = 160;

  let mut out = String::new();
  for (idx, ch) in value.chars().enumerate() {
    if idx == MAX_CHARS {
      out.push_str("...");
      return out;
    }
    out.push(ch);
  }

  out
}

fn parse_xml_node(
  reader: &Reader<Cursor<&[u8]>>,
  event: &quick_xml::events::BytesStart<'_>,
  inherited_ns: Option<&BTreeMap<String, String>>,
  options: CanonicalOptions,
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
    if key == "xmlns" || key.starts_with("xmlns:") {
      continue;
    }

    let expanded_key = expand_xml_name(&key, &ns, true);
    let value = if is_mc_ignorable_attr(&expanded_key) {
      normalize_ignorable_prefix_list(&value, &ns)
    } else if entry_name.ends_with(".rels") && key == "Type" {
      normalize_relationship_type_uri(&value)
    } else {
      value
    };
    let value = if options.normalize_bool_attrs
      && should_normalize_bool_attr(file_name, entry_name, &name, &expanded_key)
    {
      normalize_bool_lexeme(&value)
    } else {
      value
    };
    let value = if options.normalize_numeric {
      normalize_numeric_lexeme(&value)
    } else {
      value
    };

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

fn xml_declaration_from_event(event: &quick_xml::events::BytesDecl<'_>) -> XmlDeclaration {
  if matches!(
    event.standalone(),
    Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
  ) {
    XmlDeclaration::Standalone
  } else {
    XmlDeclaration::Plain
  }
}

fn render_xml_nodes_for_entry(
  nodes: &[XmlNode],
  options: CanonicalOptions,
  entry_name: &str,
) -> String {
  let mut out = String::new();
  for node in nodes {
    out.push_str(&render_xml_node_to_string(node, options, entry_name));
  }
  out
}

fn render_xml_node_to_string(
  node: &XmlNode,
  options: CanonicalOptions,
  entry_name: &str,
) -> String {
  let mut out = String::new();
  match node {
    XmlNode::Declaration(XmlDeclaration::Plain) => out.push_str("<?xml?>"),
    XmlNode::Declaration(XmlDeclaration::Standalone) => out.push_str("<?xml standalone=\"yes\"?>"),
    XmlNode::Text(text) => out.push_str(text),
    XmlNode::Element(element) => {
      if options.ignore_empty_core_property
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
        if options.sort_package_properties
          && (entry_name == "docProps/core.xml"
            || (entry_name == "docProps/app.xml" && is_extended_properties_root(&element.name)))
        {
          let mut children = element
            .children
            .iter()
            .map(|child| render_xml_node_to_string(child, options, entry_name))
            .collect::<Vec<_>>();
          children.sort();
          for child in children {
            out.push_str(&child);
          }
        } else {
          for child in &element.children {
            out.push_str(&render_xml_node_to_string(child, options, entry_name));
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
  is_schema_bool_attr(element_name, attr_name)
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

fn normalize_xml_text(value: &str, options: CanonicalOptions, stack: &[XmlFrame]) -> String {
  let value = value.replace("\r\n", "\n").replace('\r', "\n");
  if options.normalize_bool_text && should_normalize_bool_text(stack) {
    return normalize_bool_lexeme(&value);
  }
  value
}

fn should_relax_numeric_lexemes(entry_name: &str) -> bool {
  entry_name.starts_with("word/")
    || entry_name.starts_with("xl/")
    || entry_name.starts_with("ppt/")
    || entry_name == "docProps/app.xml"
    || entry_name == "docProps/custom.xml"
}

fn is_mc_ignorable_attr(attr_name: &str) -> bool {
  attr_name == "{http://schemas.openxmlformats.org/markup-compatibility/2006}Ignorable"
}

fn normalize_ignorable_prefix_list(value: &str, namespaces: &BTreeMap<String, String>) -> String {
  let mut values = value
    .split_whitespace()
    .map(|prefix| {
      namespaces
        .get(prefix)
        .map(|uri| normalize_namespace_uri(uri))
        .unwrap_or_else(|| format!("prefix:{prefix}"))
    })
    .collect::<Vec<_>>();
  values.sort();
  values.dedup();
  values.join(" ")
}

fn should_normalize_bool_text(stack: &[XmlFrame]) -> bool {
  let Some(frame) = stack.last() else {
    return false;
  };
  let (element_ns, element_local) = split_expanded_name(&frame.name);
  let is_extended_property_bool = matches!(
    element_local,
    "HyperlinksChanged" | "LinksUpToDate" | "ScaleCrop" | "SharedDoc"
  ) && (element_ns.is_empty()
    || element_ns == "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties");
  let is_vtype_bool = element_local == "bool"
    && (element_ns.is_empty()
      || element_ns == "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes");

  is_extended_property_bool || is_vtype_bool
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

include!(concat!(env!("OUT_DIR"), "/doc_samples_bool_attrs.rs"));
include!(concat!(env!("OUT_DIR"), "/doc_samples_tests.rs"));
