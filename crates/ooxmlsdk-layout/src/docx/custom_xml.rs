use super::*;
use quick_xml::events::Event;
use quick_xml::name::{QName, ResolveResult};
use quick_xml::{NsReader, Reader};
use std::cell::RefCell;

const CORE_PROPERTIES_STORE_ITEM_ID: &str = "{6C3C8BC8-F283-45AE-878A-BAB7291924A1}";
const EXTENDED_PROPERTIES_STORE_ITEM_ID: &str = "{6668398D-A668-4E3E-A5EB-62B293D839F1}";
const COVER_PAGE_PROPERTIES_STORE_ITEM_ID: &str = "{55AF091B-3C7A-41E3-B477-F2FDAA23CFDA}";
const COVER_PAGE_PROPERTIES_NAMESPACE: &[u8] =
  b"http://schemas.microsoft.com/office/2006/coverPageProps";
const DEFAULT_COVER_PAGE_PROPERTIES: &str = concat!(
  "<CoverPageProperties xmlns=\"http://schemas.microsoft.com/office/2006/coverPageProps\">",
  "<PublishDate/><Abstract/><CompanyAddress/><CompanyPhone/><CompanyFax/><CompanyEmail/>",
  "</CoverPageProperties>",
);

#[derive(Clone, Debug, Default)]
pub(crate) struct CustomXmlBindings {
  entries: Vec<CustomXmlBindingEntry>,
  glossary_placeholders: HashMap<String, w::DocPartBody>,
  inline_placeholder_contents: RefCell<HashMap<String, Vec<InlineItem>>>,
}

#[derive(Clone, Debug)]
struct CustomXmlBindingEntry {
  store_item_id: Option<String>,
  xml: String,
}

fn ensure_cover_page_properties(entries: &mut Vec<CustomXmlBindingEntry>) {
  if !entries.iter().any(is_cover_page_properties_entry) {
    // [MS-OE376] §3.5.1 defines this Office-owned schema. Word exposes it
    // through CustomXMLParts with BuiltIn=true even when the package omits
    // the part. Keep its reserved identity ahead of arbitrary custom stores,
    // while preserving any explicitly stored version of the built-in part.
    entries.insert(
      0,
      CustomXmlBindingEntry {
        store_item_id: Some(COVER_PAGE_PROPERTIES_STORE_ITEM_ID.to_owned()),
        xml: DEFAULT_COVER_PAGE_PROPERTIES.to_owned(),
      },
    );
  }
}

fn is_cover_page_properties_entry(entry: &CustomXmlBindingEntry) -> bool {
  entry
    .store_item_id
    .as_deref()
    .is_some_and(|id| id.eq_ignore_ascii_case(COVER_PAGE_PROPERTIES_STORE_ITEM_ID))
}

impl CustomXmlBindings {
  pub(super) fn load(package: &WordprocessingDocument, main: &MainDocumentPart) -> Self {
    let parts = main.custom_xml_parts(package).collect::<Vec<_>>();
    let mut entries: Vec<CustomXmlBindingEntry> = parts
      .iter()
      .filter_map(|part| {
        let store_item_id = part
          .custom_xml_properties_part(package)
          .and_then(|props| props.root_element(package).ok())
          .map(|props| props.item_id.clone());
        let xml = part.data_as_str(package).ok().flatten()?.to_owned();
        Some(CustomXmlBindingEntry { store_item_id, xml })
      })
      .collect();
    ensure_cover_page_properties(&mut entries);
    if let Some(part) = package.core_file_properties_part()
      && let Ok(Some(xml)) = part.data_as_str(package)
    {
      entries.push(CustomXmlBindingEntry {
        store_item_id: Some(CORE_PROPERTIES_STORE_ITEM_ID.to_owned()),
        xml: xml.to_owned(),
      });
    }
    let extended_properties_part = {
      package
        .get_parts_of_type::<
          ooxmlsdk::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
        >()
        .next()
    };
    if let Some(part) = extended_properties_part
      && let Ok(Some(xml)) = part.data_as_str(package)
    {
      // Word exposes docProps/app.xml to SDT XML mappings through this
      // reserved store item ID. LibreOffice SdtHelper::loadPropertiesXMLs()
      // uses the same fixed mapping alongside the core-properties store.
      entries.push(CustomXmlBindingEntry {
        store_item_id: Some(EXTENDED_PROPERTIES_STORE_ITEM_ID.to_owned()),
        xml: xml.to_owned(),
      });
    }
    let glossary_placeholders = main
      .glossary_document_part(package)
      .and_then(|part| part.root_element(package).ok())
      .and_then(|glossary| glossary.doc_parts.as_ref())
      .into_iter()
      .flat_map(|parts| &parts.doc_part)
      .filter(|part| {
        part
          .doc_part_properties
          .as_deref()
          .and_then(|properties| properties.doc_part_types.as_ref())
          .is_some_and(|types| {
            types
              .doc_part_type
              .iter()
              .any(|part_type| matches!(part_type.val, w::DocPartValues::SdtPlaceholder))
          })
      })
      .filter_map(|part| {
        let name = part
          .doc_part_properties
          .as_deref()?
          .doc_part_name
          .as_ref()?
          .val
          .as_str();
        let body = part.doc_part_body.as_deref()?;
        (!name.is_empty()).then(|| (name.to_owned(), body.clone()))
      })
      .collect();
    Self {
      entries,
      glossary_placeholders,
      inline_placeholder_contents: RefCell::default(),
    }
  }

  #[cfg(test)]
  pub(super) fn from_test_xml(store_item_id: Option<&str>, xml: &str) -> Self {
    let mut entries = vec![CustomXmlBindingEntry {
      store_item_id: store_item_id.map(str::to_owned),
      xml: xml.to_owned(),
    }];
    ensure_cover_page_properties(&mut entries);
    Self {
      entries,
      glossary_placeholders: HashMap::new(),
      inline_placeholder_contents: RefCell::default(),
    }
  }

  #[cfg(test)]
  pub(super) fn with_test_placeholder(mut self, name: &str, body: w::DocPartBody) -> Self {
    self.glossary_placeholders.insert(name.to_owned(), body);
    self
  }

  pub(super) fn glossary_placeholder(&self, name: &str) -> Option<&w::DocPartBody> {
    self.glossary_placeholders.get(name)
  }

  pub(super) fn inline_placeholder_content(&self, name: &str) -> Option<Vec<InlineItem>> {
    self.inline_placeholder_contents.borrow().get(name).cloned()
  }

  pub(super) fn remember_inline_placeholder_content(&self, name: &str, content: &[InlineItem]) {
    if content.is_empty() {
      return;
    }
    self
      .inline_placeholder_contents
      .borrow_mut()
      .entry(name.to_owned())
      .or_insert_with(|| content.to_vec());
  }

  pub(super) fn value_for_sdt(&self, properties: &w::SdtProperties) -> Option<String> {
    if let Some(binding) = sdt_data_binding(properties) {
      let store_item_id = binding.store_item_id.as_deref().unwrap_or("");
      if let Some(value) = if store_item_id.is_empty() {
        // When storeItemID is omitted, prefixMappings still selects the
        // intended custom-XML namespace. Matching local names alone can pick
        // a newer unrelated store first (tdf#147724: 2007 `ABC` vs 2020
        // `HERUNTERLADEN`).
        self.value_for_binding_namespace(binding)
      } else {
        self.value(
          store_item_id,
          &binding.x_path,
          binding.prefix_mappings.as_deref(),
        )
      } {
        return Some(value);
      }
    }

    let tag = sdt_tag(properties)?;
    self.value("", &format!("//*[@ref='{tag}']/@text"), None)
  }

  fn value(
    &self,
    store_item_id: &str,
    xpath: &str,
    prefix_mappings: Option<&str>,
  ) -> Option<String> {
    let value_for_entry = |entry: &CustomXmlBindingEntry| {
      if is_cover_page_properties_entry(entry) {
        cover_page_property_value(&entry.xml, xpath, prefix_mappings)
      } else {
        custom_xml_xpath_value(&entry.xml, xpath)
      }
    };
    if let Some(value) = self
      .entries
      .iter()
      .filter(|entry| {
        !store_item_id.is_empty()
          && entry
            .store_item_id
            .as_deref()
            .is_some_and(|id| id.eq_ignore_ascii_case(store_item_id))
      })
      .find_map(value_for_entry)
    {
      return Some(value);
    }

    self.entries.iter().find_map(value_for_entry)
  }

  fn value_for_binding_namespace(&self, binding: &w::DataBinding) -> Option<String> {
    let prefix = binding
      .x_path
      .split("name()='")
      .nth(1)
      .and_then(|value| value.split(':').next())
      .filter(|prefix| !prefix.is_empty())?;
    let mappings = binding.prefix_mappings.as_deref()?;
    let namespace = mappings.split_whitespace().find_map(|mapping| {
      let mapping = mapping.strip_prefix("xmlns:")?;
      let (mapping_prefix, uri) = mapping.split_once('=')?;
      (mapping_prefix == prefix).then(|| uri.trim_matches(['\'', '"']).to_owned())
    })?;
    self
      .entries
      .iter()
      .filter(|entry| entry.xml.contains(&namespace))
      .find_map(|entry| custom_xml_xpath_value(&entry.xml, &binding.x_path))
  }
}

fn cover_page_property_value(
  xml: &str,
  xpath: &str,
  prefix_mappings: Option<&str>,
) -> Option<String> {
  // Most mappings target other stores. Avoid parsing the implicit part on
  // their fallback lookup, and never make an unrelated namespace look empty.
  if !xpath.contains("CoverPageProperties") {
    return None;
  }
  let mut reader = NsReader::from_str(xml);
  loop {
    match reader.read_resolved_event().ok()? {
      (namespace, Event::Start(element) | Event::Empty(element)) => {
        if element.local_name().as_ref() != b"CoverPageProperties"
          || !matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == COVER_PAGE_PROPERTIES_NAMESPACE)
        {
          return None;
        }
        break;
      }
      (_, Event::Decl(_) | Event::Comment(_) | Event::PI(_)) => {}
      (_, Event::Text(text)) if text.iter().all(u8::is_ascii_whitespace) => {}
      _ => return None,
    }
  }
  if let Some(mappings) = prefix_mappings.filter(|mappings| !mappings.trim().is_empty()) {
    let scope = format!("<mapping {mappings}>");
    let mut reader = NsReader::from_str(&scope);
    if !matches!(reader.read_event().ok()?, Event::Start(_)) {
      return None;
    }
    for step in xpath.strip_prefix('/')?.split('/') {
      let name = step.split('[').next()?;
      if !matches!(reader.resolver().resolve_element(QName(name.as_bytes())).0,
        ResolveResult::Bound(uri) if uri.as_ref() == COVER_PAGE_PROPERTIES_NAMESPACE)
      {
        return None;
      }
    }
  }
  // Word retains supplied values, including whitespace, and supplies empty
  // values for missing built-in fields. An absent prefixMappings is accepted
  // by its built-in store; an explicit wrong namespace is not.
  custom_xml_xpath_value(xml, xpath)
    .or_else(|| custom_xml_xpath_value(DEFAULT_COVER_PAGE_PROPERTIES, xpath))
}

fn sdt_data_binding(properties: &w::SdtProperties) -> Option<&w::DataBinding> {
  properties
    .sdt_properties_choice
    .iter()
    .find_map(|choice| match choice {
      w::SdtPropertiesChoice::WDataBinding(binding) => Some(binding),
      _ => None,
    })
}

fn sdt_tag(properties: &w::SdtProperties) -> Option<&str> {
  properties
    .sdt_properties_choice
    .iter()
    .find_map(|choice| match choice {
      w::SdtPropertiesChoice::Tag(tag) if !tag.val.is_empty() => Some(tag.val.as_str()),
      _ => None,
    })
}

fn custom_xml_xpath_value(xml: &str, xpath: &str) -> Option<String> {
  let Some((_, attr_name)) = xpath.rsplit_once("/@") else {
    return custom_xml_xpath_element_text(xml, xpath);
  };
  if attr_name.is_empty()
    || attr_name
      .bytes()
      .any(|byte| !(byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-' || byte == b':'))
  {
    return None;
  }
  let predicates = xpath_attr_predicates(xpath);
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(true);
  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) | Ok(Event::Empty(event)) => {
        let decoder = reader.decoder();
        if !predicates.iter().all(|(name, value)| {
          xml_event_attr_value(&event, name.as_bytes(), decoder).as_deref() == Some(value.as_str())
        }) {
          continue;
        }
        if let Some(value) = xml_event_attr_value(&event, attr_name.as_bytes(), decoder)
          && !value.is_empty()
        {
          return Some(value);
        }
      }
      Ok(Event::Eof) => break,
      Ok(_) => {}
      Err(_) => break,
    }
  }
  None
}

fn custom_xml_xpath_element_text(xml: &str, xpath: &str) -> Option<String> {
  let expected_path = xpath
    .strip_prefix('/')?
    .split('/')
    .map(xpath_local_name)
    .collect::<Option<Vec<_>>>()?;
  if expected_path.is_empty() {
    return None;
  }

  let mut reader = Reader::from_str(xml);
  // ECMA-376 §17.5.2.6 replaces the SDT cache with the mapped element's
  // value. XPath 1.0 §5.2 concatenates its descendant text without trimming:
  // leading tabs remain layout controls, including when separated by a
  // comment or CDATA boundary. Native Word binding controls confirm this.
  let mut path = Vec::new();
  let mut child_positions = vec![HashMap::<String, usize>::new()];
  let mut matched_depth = None;
  let mut value = String::new();
  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        let name = xml_local_name(event.name().as_ref())?;
        let position = child_positions
          .last_mut()?
          .entry(name.clone())
          .and_modify(|position| *position += 1)
          .or_insert(1);
        path.push((name, *position));
        child_positions.push(HashMap::new());
        if xpath_path_matches(&path, &expected_path) {
          matched_depth = Some(path.len());
        }
      }
      Ok(Event::Empty(event)) => {
        let name = xml_local_name(event.name().as_ref())?;
        let position = child_positions
          .last_mut()?
          .entry(name.clone())
          .and_modify(|position| *position += 1)
          .or_insert(1);
        path.push((name, *position));
        if xpath_path_matches(&path, &expected_path) {
          return Some(String::new());
        }
        path.pop();
      }
      Ok(Event::Text(text)) if matched_depth.is_some() => {
        value.push_str(&text.xml10_content().ok()?);
      }
      Ok(Event::CData(text)) if matched_depth.is_some() => {
        value.push_str(&text.xml10_content().ok()?);
      }
      Ok(Event::GeneralRef(reference)) if matched_depth.is_some() => {
        if let Some(character) = reference.resolve_char_ref().ok()? {
          value.push(character);
        } else {
          value.push_str(quick_xml::escape::resolve_predefined_entity(
            &reference.decode().ok()?,
          )?);
        }
      }
      Ok(Event::End(_)) => {
        if matched_depth == Some(path.len()) {
          return Some(value);
        }
        path.pop();
        child_positions.pop();
      }
      Ok(Event::Eof) => break,
      Ok(_) => {}
      Err(_) => break,
    }
  }
  None
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct XPathStep {
  name: String,
  position: Option<usize>,
}

fn xpath_local_name(segment: &str) -> Option<XPathStep> {
  let (name, position) = if let Some((name, suffix)) = segment.rsplit_once('[') {
    (name, Some(suffix.strip_suffix(']')?.parse::<usize>().ok()?))
  } else {
    (segment, None)
  };
  if name.is_empty()
    || name
      .bytes()
      .any(|byte| !(byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b':' | b'.')))
  {
    return None;
  }
  Some(XPathStep {
    name: name
      .rsplit_once(':')
      .map_or(name, |(_, name)| name)
      .to_owned(),
    position,
  })
}

fn xpath_path_matches(path: &[(String, usize)], expected: &[XPathStep]) -> bool {
  path.len() == expected.len()
    && path
      .iter()
      .zip(expected)
      .all(|((name, position), expected)| {
        name == &expected.name
          && expected
            .position
            .is_none_or(|expected| expected == *position)
      })
}

fn xml_local_name(name: &[u8]) -> Option<String> {
  let name = std::str::from_utf8(name).ok()?;
  Some(
    name
      .rsplit_once(':')
      .map_or(name, |(_, name)| name)
      .to_owned(),
  )
}

fn xpath_attr_predicates(xpath: &str) -> Vec<(String, String)> {
  let mut predicates = Vec::new();
  let mut rest = xpath;
  while let Some(index) = rest.find('@') {
    rest = &rest[index + 1..];
    let Some(eq_index) = rest.find('=') else {
      break;
    };
    let name = rest[..eq_index].trim();
    let value_start = rest[eq_index + 1..].trim_start();
    let Some(value_body) = value_start.strip_prefix('\'') else {
      rest = value_start;
      continue;
    };
    let Some(value_end) = value_body.find('\'') else {
      break;
    };
    if !name.is_empty() {
      predicates.push((name.to_owned(), value_body[..value_end].to_owned()));
    }
    rest = &value_body[value_end + 1..];
  }
  predicates
}

fn xml_event_attr_value(
  event: &quick_xml::events::BytesStart<'_>,
  key: &[u8],
  decoder: quick_xml::Decoder,
) -> Option<String> {
  event
    .attributes()
    .with_checks(false)
    .filter_map(|attr| attr.ok())
    .find(|attr| qname_ends_with(attr.key.as_ref(), key))
    .and_then(|attr| {
      attr
        .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, decoder)
        .ok()
    })
    .map(|value| value.into_owned())
}

fn qname_ends_with(qname: &[u8], local_name: &[u8]) -> bool {
  qname == local_name
    || qname
      .strip_suffix(local_name)
      .is_some_and(|prefix| prefix.ends_with(b":"))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn cover_page_mapping(id: &str, path: &str, mappings: Option<&str>) -> w::SdtProperties {
    let mappings = mappings.map_or_else(String::new, |value| {
      format!(" w:prefixMappings=\"{value}\"")
    });
    w::SdtProperties::from_bytes(format!(
      r#"<w:sdtPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:dataBinding w:storeItemID="{id}" w:xpath="{path}"{mappings}/><w:text/></w:sdtPr>"#,
    ).as_bytes()).expect("cover-page XML mapping")
  }

  #[test]
  fn implicit_cover_page_fields_are_empty_and_do_not_override_selected_custom_stores() {
    let mappings = "xmlns:p='http://schemas.microsoft.com/office/2006/coverPageProps'";
    let bindings = CustomXmlBindings::from_test_xml(
      Some("custom-store"),
      r#"<CoverPageProperties xmlns="http://schemas.microsoft.com/office/2006/coverPageProps"><PublishDate>2024-06-07T00:00:00Z</PublishDate></CoverPageProperties>"#,
    );
    for field in [
      "PublishDate",
      "Abstract",
      "CompanyAddress",
      "CompanyPhone",
      "CompanyFax",
      "CompanyEmail",
    ] {
      let path = format!("/p:CoverPageProperties[1]/p:{field}[1]");
      for id in [COVER_PAGE_PROPERTIES_STORE_ITEM_ID, "unresolved-store"] {
        assert_eq!(
          bindings.value_for_sdt(&cover_page_mapping(id, &path, Some(mappings))),
          Some(String::new())
        );
      }
    }
    assert_eq!(
      bindings
        .value_for_sdt(&cover_page_mapping(
          "custom-store",
          "/p:CoverPageProperties[1]/p:PublishDate[1]",
          Some(mappings),
        ))
        .as_deref(),
      Some("2024-06-07T00:00:00Z")
    );
  }

  #[test]
  fn explicit_cover_page_values_preserve_whitespace_and_supply_missing_fields() {
    let mappings = "xmlns:p='http://schemas.microsoft.com/office/2006/coverPageProps'";
    for (content, expected) in [
      (
        "<PublishDate>2024-06-07T00:00:00Z</PublishDate>",
        "2024-06-07T00:00:00Z",
      ),
      ("<PublishDate> </PublishDate>", " "),
      ("<Abstract>summary</Abstract>", ""),
    ] {
      let bindings = CustomXmlBindings::from_test_xml(
        Some(COVER_PAGE_PROPERTIES_STORE_ITEM_ID),
        &format!(
          "<?xml version=\"1.0\"?>\n<CoverPageProperties xmlns=\"http://schemas.microsoft.com/office/2006/coverPageProps\">{content}</CoverPageProperties>",
        ),
      );
      for prefixes in [None, Some(mappings)] {
        let properties = cover_page_mapping(
          COVER_PAGE_PROPERTIES_STORE_ITEM_ID,
          "/p:CoverPageProperties[1]/p:PublishDate[1]",
          prefixes,
        );
        assert_eq!(
          bindings.value_for_sdt(&properties).as_deref(),
          Some(expected)
        );
      }
    }
  }

  #[test]
  fn implicit_cover_page_properties_require_matching_schema_and_xpath() {
    let bindings = CustomXmlBindings::from_test_xml(None, "<unrelated/>");
    let correct = "xmlns:p='http://schemas.microsoft.com/office/2006/coverPageProps'";
    for (path, mappings) in [
      (
        "/p:CoverPageProperties[1]/p:PublishDate[1]",
        "xmlns:p='urn:other'",
      ),
      (
        "/p:CoverPageProperties[1]/q:PublishDate[1]",
        "xmlns:p='http://schemas.microsoft.com/office/2006/coverPageProps' xmlns:q='urn:other'",
      ),
      ("/p:CoverPageProperties[2]/p:PublishDate[1]", correct),
      ("/p:CoverPageProperties[1]/p:Unknown[1]", correct),
      ("/p:Other[1]/p:PublishDate[1]", correct),
    ] {
      assert!(
        bindings
          .value_for_sdt(&cover_page_mapping(
            COVER_PAGE_PROPERTIES_STORE_ITEM_ID,
            path,
            Some(mappings)
          ))
          .is_none(),
        "{path}, {mappings}"
      );
    }
    let wrong_schema = CustomXmlBindings::from_test_xml(
      Some(COVER_PAGE_PROPERTIES_STORE_ITEM_ID),
      "<CoverPageProperties xmlns=\"urn:other\"><PublishDate>2024-06-07</PublishDate></CoverPageProperties>",
    );
    assert!(
      wrong_schema
        .value_for_sdt(&cover_page_mapping(
          COVER_PAGE_PROPERTIES_STORE_ITEM_ID,
          "/p:CoverPageProperties[1]/p:PublishDate[1]",
          Some(correct)
        ))
        .is_none()
    );
  }

  #[test]
  fn custom_xml_xpath_uses_typed_binding_scope_and_minimal_attribute_scan() {
    let xml = r#"<root xmlns:a="urn:test">
      <a:item a:ref="choice-1" a:text="A &amp; B"/>
      <a:item a:ref="choice-2" a:text="ignored"/>
    </root>"#;

    assert_eq!(
      custom_xml_xpath_value(xml, "//*[@ref='choice-1']/@text").as_deref(),
      Some("A & B")
    );
  }

  #[test]
  fn custom_xml_xpath_rejects_unsupported_attribute_expressions() {
    assert_eq!(
      custom_xml_xpath_value("<root text='value'/>", "//root/text()"),
      None
    );
    assert_eq!(
      custom_xml_xpath_value("<root text='value'/>", "//root/@text]"),
      None
    );
  }

  #[test]
  fn custom_xml_xpath_reads_absolute_element_text() {
    let xml = r#"<BlogPostInfo xmlns="urn:blog"><PostTitle>ignored</PostTitle><PostTitle>Coquilles saint jacques</PostTitle></BlogPostInfo>"#;

    assert_eq!(
      custom_xml_xpath_value(xml, "/ns0:BlogPostInfo[1]/ns0:PostTitle[2]").as_deref(),
      Some("Coquilles saint jacques")
    );
  }

  #[test]
  fn custom_xml_xpath_preserves_mapped_whitespace_and_text_boundaries() {
    for (content, expected) in [
      ("\t\t\tBound title", "\t\t\tBound title"),
      ("  Bound title  ", "  Bound title  "),
      ("\t \t", "\t \t"),
      (
        "\t<!--split--><![CDATA[\t\tBound]]> title",
        "\t\t\tBound title",
      ),
      (" A <child>B</child> C ", " A B C "),
      ("A\r\nB<![CDATA[\r\nC]]>", "A\nB\nC"),
    ] {
      let xml = format!("<root>\n <title>{content}</title>\n</root>");
      assert_eq!(
        custom_xml_xpath_value(&xml, "/root/title").as_deref(),
        Some(expected),
        "{content:?}"
      );
    }
  }

  #[test]
  fn custom_xml_xpath_resolves_mapped_character_and_predefined_references() {
    let xml = "<root><title>&#9;&#x9;&#9;A &amp; B &lt;&gt;&quot;&apos; &#x1F600;</title></root>";
    assert_eq!(
      custom_xml_xpath_value(xml, "/root/title").as_deref(),
      Some("\t\t\tA & B <>\"' 😀")
    );
    // An unsupported entity must not silently produce a partial bound value.
    assert_eq!(
      custom_xml_xpath_value("<root>A &unknown; B</root>", "/root"),
      None
    );
  }

  #[test]
  fn custom_xml_xpath_distinguishes_empty_element_from_no_match() {
    let xml = r#"<coreProperties><subject/></coreProperties>"#;

    assert_eq!(
      custom_xml_xpath_value(xml, "/coreProperties[1]/subject[1]").as_deref(),
      Some("")
    );
    assert_eq!(
      custom_xml_xpath_value(xml, "/coreProperties[1]/title[1]"),
      None
    );
  }
}
