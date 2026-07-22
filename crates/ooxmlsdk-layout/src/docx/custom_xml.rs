use super::*;
use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Clone, Debug, Default)]
pub(crate) struct CustomXmlBindings {
  entries: Vec<CustomXmlBindingEntry>,
}

#[derive(Clone, Debug)]
struct CustomXmlBindingEntry {
  store_item_id: Option<String>,
  xml: String,
}

impl CustomXmlBindings {
  pub(super) fn load(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> Self {
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
    if let Some(part) = package.core_file_properties_part()
      && let Ok(Some(xml)) = part.data_as_str(package)
    {
      entries.push(CustomXmlBindingEntry {
        store_item_id: Some("{6C3C8BC8-F283-45AE-878A-BAB7291924A1}".to_owned()),
        xml: xml.to_owned(),
      });
    }
    Self { entries }
  }

  pub(super) fn value_for_sdt(&self, properties: &w::SdtProperties) -> Option<String> {
    if let Some(binding) = sdt_data_binding(properties)
      && let Some(value) = self.value(
        binding.store_item_id.as_deref().unwrap_or(""),
        &binding.x_path,
      )
    {
      return Some(value);
    }

    let tag = sdt_tag(properties)?;
    self.value("", &format!("//*[@ref='{tag}']/@text"))
  }

  fn value(&self, store_item_id: &str, xpath: &str) -> Option<String> {
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
      .find_map(|entry| custom_xml_xpath_value(&entry.xml, xpath))
    {
      return Some(value);
    }

    self
      .entries
      .iter()
      .find_map(|entry| custom_xml_xpath_value(&entry.xml, xpath))
  }
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
  reader.config_mut().trim_text(true);
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
