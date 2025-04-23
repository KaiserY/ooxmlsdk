#[derive(Clone, Debug, Default)]
pub struct CoreProperties {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
  pub category: Option<String>,
  pub content_status: Option<String>,
  pub created: Option<String>,
  pub creator: Option<String>,
  pub description: Option<String>,
  pub identifier: Option<String>,
  pub keywords: Option<String>,
  pub language: Option<String>,
  pub last_modified_by: Option<String>,
  pub last_printed: Option<String>,
  pub modified: Option<String>,
  pub revision: Option<String>,
  pub subject: Option<String>,
  pub title: Option<String>,
  pub version: Option<String>,
}

impl std::str::FromStr for CoreProperties {
  type Err = super::super::common::SdkError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = super::super::common::from_str_inner(s)?;

    Self::deserialize_inner(&mut xml_reader, None)
  }
}

impl CoreProperties {
  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_reader_inner(reader)?;

    Self::deserialize_inner(&mut xml_reader, None)
  }

  pub fn deserialize_inner<'de, R: super::super::common::XmlReader<'de>>(
    xml_reader: &mut R,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
  ) -> Result<Self, super::super::common::SdkError> {
    let (e, empty_tag) = super::super::common::expect_event_start!(
      xml_reader,
      xml_event,
      b"cp:coreProperties",
      b"coreProperties"
    );

    let mut xmlns = None;

    let mut xmlns_map = std::collections::HashMap::<String, String>::new();

    let mut mc_ignorable = None;

    let mut category: Option<String> = None;

    let mut content_status: Option<String> = None;

    let mut created: Option<String> = None;

    let mut creator: Option<String> = None;

    let mut description: Option<String> = None;

    let mut identifier: Option<String> = None;

    let mut keywords: Option<String> = None;

    let mut language: Option<String> = None;

    let mut last_modified_by: Option<String> = None;

    let mut last_printed: Option<String> = None;

    let mut modified: Option<String> = None;

    let mut revision: Option<String> = None;

    let mut subject: Option<String> = None;

    let mut title: Option<String> = None;

    let mut version: Option<String> = None;

    for attr in e.attributes() {
      let attr = attr?;
      match attr.key.as_ref() {
        b"xmlns" => {
          xmlns = Some(attr.unescape_value()?.to_string());
        }
        b"mc:Ignorable" => {
          mc_ignorable = Some(attr.unescape_value()?.to_string());
        }
        key => {
          if key.starts_with(b"xmlns:") {
            xmlns_map.insert(
              String::from_utf8_lossy(&key[6..]).to_string(),
              attr.unescape_value()?.to_string(),
            );
          }
        }
      }
    }

    if !empty_tag {
      loop {
        match xml_reader.next()? {
          quick_xml::events::Event::Start(e) | quick_xml::events::Event::Empty(e) => {
            match e.name().as_ref() {
              b"cp:category" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  category = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"cp:contentStatus" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  content_status = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"dcterms:created" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  created = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"dc:creator" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  creator = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"dc:description" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  description = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"dc:identifier" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  identifier = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"cp:keywords" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  keywords = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"dc:language" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  language = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"cp:lastModifiedBy" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  last_modified_by = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"cp:lastPrinted" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  last_printed = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"dcterms:modified" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  modified = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"cp:revision" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  revision = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"dc:subject" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  subject = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"dc:title" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  title = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              b"cp:version" => {
                if let quick_xml::events::Event::Text(t) = xml_reader.next()? {
                  version = Some(t.unescape()?.to_string())
                }

                xml_reader.next()?;
              }
              _ => Err(super::super::common::SdkError::CommonError(
                "coreProperties".to_string(),
              ))?,
            }
          }
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            b"cp:coreProperties" | b"coreProperties" => {
              break;
            }
            _ => (),
          },
          quick_xml::events::Event::Eof => Err(super::super::common::SdkError::UnknownError)?,
          _ => (),
        }
      }
    }

    Ok(Self {
      xmlns,
      xmlns_map,
      mc_ignorable,
      category,
      content_status,
      created,
      creator,
      description,
      identifier,
      keywords,
      language,
      last_modified_by,
      last_printed,
      modified,
      revision,
      subject,
      title,
      version,
    })
  }
}

impl CoreProperties {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    self.to_string_inner(if let Some(xmlns) = &self.xmlns {
      xmlns != "http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
    } else {
      true
    })
  }

  pub fn to_string_inner(&self, with_xmlns: bool) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut writer = String::new();

    writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;

    writer.write_char('<')?;

    if with_xmlns {
      writer.write_str("cp:coreProperties")?;
    } else {
      writer.write_str("coreProperties")?;
    }

    if let Some(xmlns) = &self.xmlns {
      writer.write_str(r#" xmlns=""#)?;
      writer.write_str(xmlns)?;
      writer.write_str("\"")?;
    }

    for (k, v) in &self.xmlns_map {
      writer.write_str(" xmlns:")?;
      writer.write_str(k)?;
      writer.write_str("=\"")?;
      writer.write_str(v)?;
      writer.write_str("\"")?;
    }

    if let Some(mc_ignorable) = &self.mc_ignorable {
      writer.write_str(r#" mc:Ignorable=""#)?;
      writer.write_str(mc_ignorable)?;
      writer.write_str("\"")?;
    }

    writer.write_char('>')?;

    if let Some(category) = &self.category {
      writer.write_str("<cp:category>")?;
      writer.write_str(&quick_xml::escape::escape(category))?;
      writer.write_str("</cp:category>")?;
    }

    if let Some(content_status) = &self.content_status {
      writer.write_str("<cp:contentStatus>")?;
      writer.write_str(&quick_xml::escape::escape(content_status))?;
      writer.write_str("</cp:contentStatus>")?;
    }

    if let Some(created) = &self.created {
      writer.write_str(r#"<dcterms:created xsi:type="dcterms:W3CDTF">"#)?;
      writer.write_str(&quick_xml::escape::escape(created))?;
      writer.write_str("</dcterms:created>")?;
    }

    if let Some(creator) = &self.creator {
      writer.write_str("<dc:creator>")?;
      writer.write_str(&quick_xml::escape::escape(creator))?;
      writer.write_str("</dc:creator>")?;
    }

    if let Some(description) = &self.description {
      writer.write_str("<dc:description>")?;
      writer.write_str(&quick_xml::escape::escape(description))?;
      writer.write_str("</dc:description>")?;
    }

    if let Some(identifier) = &self.identifier {
      writer.write_str("<dc:identifier>")?;
      writer.write_str(&quick_xml::escape::escape(identifier))?;
      writer.write_str("</dc:identifier>")?;
    }

    if let Some(keywords) = &self.keywords {
      writer.write_str("<cp:keywords>")?;
      writer.write_str(&quick_xml::escape::escape(keywords))?;
      writer.write_str("</cp:keywords>")?;
    }

    if let Some(language) = &self.language {
      writer.write_str("<dc:language>")?;
      writer.write_str(&quick_xml::escape::escape(language))?;
      writer.write_str("</dc:language>")?;
    }

    if let Some(last_modified_by) = &self.last_modified_by {
      writer.write_str("<cp:lastModifiedBy>")?;
      writer.write_str(&quick_xml::escape::escape(last_modified_by))?;
      writer.write_str("</cp:lastModifiedBy>")?;
    }

    if let Some(last_printed) = &self.last_printed {
      writer.write_str("<cp:lastPrinted>")?;
      writer.write_str(&quick_xml::escape::escape(last_printed))?;
      writer.write_str("</cp:lastPrinted>")?;
    }

    if let Some(modified) = &self.modified {
      writer.write_str(r#"<dcterms:modified xsi:type="dcterms:W3CDTF">"#)?;
      writer.write_str(&quick_xml::escape::escape(modified))?;
      writer.write_str("</dcterms:modified>")?;
    }

    if let Some(revision) = &self.revision {
      writer.write_str("<cp:revision>")?;
      writer.write_str(&quick_xml::escape::escape(revision))?;
      writer.write_str("</cp:revision>")?;
    }

    if let Some(subject) = &self.subject {
      writer.write_str("<dc:subject>")?;
      writer.write_str(&quick_xml::escape::escape(subject))?;
      writer.write_str("</dc:subject>")?;
    }

    if let Some(title) = &self.title {
      writer.write_str("<dc:title>")?;
      writer.write_str(&quick_xml::escape::escape(title))?;
      writer.write_str("</dc:title>")?;
    }

    if let Some(version) = &self.version {
      writer.write_str("<cp:version>")?;
      writer.write_str(&quick_xml::escape::escape(version))?;
      writer.write_str("</cp:version>")?;
    }

    writer.write_str("</")?;

    if with_xmlns {
      writer.write_str("cp:coreProperties")?;
    } else {
      writer.write_str("coreProperties")?;
    }

    writer.write_char('>')?;

    Ok(writer)
  }
}
