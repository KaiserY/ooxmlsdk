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
    let entries = parts
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
  let attr_name = xpath.rsplit_once("/@")?.1;
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
}
