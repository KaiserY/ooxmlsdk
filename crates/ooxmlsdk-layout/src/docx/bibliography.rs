//! Fixed-output refresh of supported Word bibliography source profiles.
//!
//! ECMA-376 17.16.5.7/8 owns data selection, not style-specific punctuation.
//! Word's GB/ISO690 book templates and native PDF/updated-range observations
//! establish the rich runs and terminating paragraph. Unsupported styles or
//! source fields retain their cached result; never discard part of a source.

use ooxmlsdk::schemas::schemas_openxmlformats_org_office_document_2006_bibliography as b;

use super::*;

mod apa;

/// None means no preceding text; Some(None) retains an unsupported language.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct LocaleContext(Option<Option<apa::Locale>>);

#[derive(Clone, Debug, Default)]
pub(super) struct Catalog {
  books: Vec<Book>,
  internet_sites: Vec<InternetSite>,
  apa: Vec<apa::Entry>,
}

#[derive(Clone, Debug, Default)]
struct InternetSite {
  tag: String,
  corporate_author: Option<String>,
  title: Option<String>,
  published: WebsiteDate,
  accessed: WebsiteDate,
  url: Option<String>,
  language: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct WebsiteDate {
  year: Option<String>,
  month: Option<String>,
  day: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct Book {
  tag: String,
  authors: Vec<(String, String)>,
  title: String,
  year: String,
  city: String,
  publisher: String,
  language: Option<String>,
}

pub(super) struct Result {
  pub inlines: Vec<InlineItem>,
  pub paragraphs: Vec<ParagraphFieldEvent>,
}

impl Catalog {
  pub fn generates_paragraphs(&self) -> bool {
    !self.books.is_empty() || !self.apa.is_empty()
  }

  pub fn preceding_locale(&self, inlines: &[InlineItem], previous: LocaleContext) -> LocaleContext {
    if self.apa.is_empty() {
      return previous;
    }
    // Native Word controls vary preceding runs/paragraphs independently of
    // field-code, paragraph-mark and trailing-space language. DefaultLCID
    // follows the nearest non-whitespace text, falling back to the configured
    // authoring language only when no preceding text exists in this story.
    inlines
      .iter()
      .rev()
      .find_map(|inline| match inline {
        InlineItem::Text(run) if run.text.chars().any(|ch| !ch.is_whitespace()) => {
          Some(LocaleContext(Some(apa::Locale::language(
            run.style.language.as_deref().unwrap_or("en-US"),
          ))))
        }
        _ => None,
      })
      .unwrap_or(previous)
  }

  pub fn load(package: &WordprocessingDocument, main: &MainDocumentPart) -> Self {
    let mut result = None;
    for part in main.custom_xml_parts(package) {
      let Ok(Some(xml)) = part.data_as_str(package) else {
        continue;
      };
      if !is_word_bibliography_custom_xml(xml) {
        continue;
      }
      // A second source collection needs its own merge/style policy.
      if result.is_some() {
        return Self::default();
      }
      result = Some(Self::parse(xml).unwrap_or_default());
    }
    let result = result.unwrap_or_default();
    // Word recomputes RefOrder from citations, then appends uncited sources
    // in source-list order. This profile currently covers uncited collections;
    // do not use their persisted RefOrder as a substitute for citation order.
    if !result.internet_sites.is_empty() {
      let mut stories = vec![main.data_as_str(package).ok().flatten()];
      stories.extend(
        main
          .header_parts(package)
          .map(|p| p.data_as_str(package).ok().flatten()),
      );
      stories.extend(
        main
          .footer_parts(package)
          .map(|p| p.data_as_str(package).ok().flatten()),
      );
      if let Some(part) = main.footnotes_part(package) {
        stories.push(part.data_as_str(package).ok().flatten());
      }
      if let Some(part) = main.endnotes_part(package) {
        stories.push(part.data_as_str(package).ok().flatten());
      }
      if stories
        .into_iter()
        .any(|xml| xml.is_none_or(has_citation_field))
      {
        return Self::default();
      }
    }
    result
  }

  fn parse(xml: &str) -> Option<Self> {
    // SDK readers deliberately tolerate unknown elements. A field formatter
    // must instead keep its cache if it cannot represent all source data.
    let sources = b::Sources::from_bytes(xml.as_bytes()).ok()?;
    let style = sources
      .selected_style
      .as_ref()?
      .as_str()
      .trim_start_matches(['\\', '/']);
    let apa = style.eq_ignore_ascii_case("APASixthEditionOfficeOnline.xsl");
    if !supported_source_xml(xml, apa) {
      return None;
    }
    if apa {
      return Some(Self {
        apa: apa::parse(&sources, xml)?,
        ..Self::default()
      });
    }
    if style.eq_ignore_ascii_case("IEEE2006OfficeOnline.xsl") {
      return Some(Self {
        internet_sites: sources
          .source
          .iter()
          .map(InternetSite::parse)
          .collect::<Option<_>>()?,
        ..Self::default()
      });
    }
    if !style.eq_ignore_ascii_case("GB.XSL") && !style.eq_ignore_ascii_case("ISO690.XSL") {
      return None;
    }
    let books = sources
      .source
      .iter()
      .map(Book::parse)
      .collect::<Option<Vec<_>>>()?;
    Some(Self {
      books,
      ..Self::default()
    })
  }

  pub fn refresh_block_content(
    &self,
    content: &w::SdtContentBlock,
    styles: &StylesCatalog,
  ) -> Option<w::SdtContentBlock> {
    if self.internet_sites.is_empty() {
      return None;
    }
    // A block bibliography owns its complete field cache, including the table.
    // Preserve the field's opening/closing paragraphs; never rewrite a partial,
    // locked or nested field, or discard adjacent authored text.
    let choices = &content.sdt_content_block_choice;
    let w::SdtContentBlockChoice::Paragraph(first) = choices.first()? else {
      return None;
    };
    let w::SdtContentBlockChoice::Paragraph(last) = choices.last()? else {
      return None;
    };
    if choices.len() < 2
      || !plain_bibliography_boundary(first, true)
      || !plain_bibliography_boundary(last, false)
    {
      return None;
    }
    let style_id = styles
      .styles
      .iter()
      .find(|(_, entry)| {
        entry
          .name
          .as_deref()
          .is_some_and(|name| name.eq_ignore_ascii_case("Bibliography"))
      })
      .map(|(id, _)| id.as_str())?;
    let base = styles.run_style_with_base(
      Some(style_id),
      TextStyle::default(),
      RunStyleOverrides::default(),
    );
    for site in &self.internet_sites {
      if !site
        .language
        .as_deref()
        .or(base.language.as_deref())
        .unwrap_or("en-US")
        .starts_with("en")
      {
        return None;
      }
    }
    // The IEEE stylesheet emits a 100%-wide HTML table, a 1% label column,
    // top-aligned cells and default one-pixel padding/spacing. Import the
    // equivalent WML through the normal table path, letting AutoFit measure
    // labels instead of retaining the old cache's saved grid or font metrics.
    let mut xml = String::from(
      r#"<w:tbl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:tblPr><w:tblW w:w="5000" w:type="pct"/><w:tblCellSpacing w:w="15" w:type="dxa"/><w:tblCellMar><w:top w:w="15" w:type="dxa"/><w:left w:w="15" w:type="dxa"/><w:bottom w:w="15" w:type="dxa"/><w:right w:w="15" w:type="dxa"/></w:tblCellMar></w:tblPr>"#,
    );
    let escaped_style = quick_xml::escape::escape(style_id);
    for (index, site) in self.internet_sites.iter().enumerate() {
      xml.push_str("<w:tr>");
      for (column, text) in [format!("[{}] ", index + 1), site.text()]
        .into_iter()
        .enumerate()
      {
        let width = if column == 0 {
          r#"w:w="50" w:type="pct""#
        } else {
          r#"w:w="0" w:type="auto""#
        };
        xml.push_str(&format!(r#"<w:tc><w:tcPr><w:tcW {width}/><w:hideMark/></w:tcPr><w:p><w:pPr><w:pStyle w:val="{escaped_style}"/>"#));
        if index == 0 && column == 0 {
          xml.push_str(r#"<w:rPr><w:sz w:val="24"/><w:szCs w:val="24"/></w:rPr>"#);
        }
        xml.push_str("</w:pPr><w:r><w:t xml:space=\"preserve\">");
        xml.push_str(&quick_xml::escape::escape(text));
        xml.push_str("</w:t></w:r></w:p></w:tc>");
      }
      xml.push_str("</w:tr>");
    }
    xml.push_str("</w:tbl>");
    let table = w::Table::from_bytes(xml.as_bytes()).ok()?;
    Some(w::SdtContentBlock {
      sdt_content_block_choice: vec![
        choices.first()?.clone(),
        w::SdtContentBlockChoice::Table(Box::new(table)),
        w::SdtContentBlockChoice::Paragraph(Box::default()),
        choices.last()?.clone(),
      ],
    })
  }

  pub fn refresh(
    &self,
    instruction: &str,
    styles: &StylesCatalog,
    context: &TextStyle,
    preceding_locale: LocaleContext,
  ) -> Option<Result> {
    if !self.apa.is_empty() {
      return apa::refresh(&self.apa, instruction, styles, context, preceding_locale);
    }
    if self.books.is_empty() {
      return None;
    }
    let tokens = field_instruction_tokens(instruction);
    let name = tokens.first()?;
    let bibliography = name.eq_ignore_ascii_case("BIBLIOGRAPHY");
    let citation = name.eq_ignore_ascii_case("CITATION");
    if !bibliography && !citation {
      return None;
    }
    let mut index = 1;
    let mut tag = if citation {
      index += 1;
      Some(tokens.get(1)?.as_str())
    } else {
      None
    };
    let mut language = context.language.as_deref();
    while index < tokens.len() {
      match tokens[index].to_ascii_lowercase().as_str() {
        "\\l" => {
          language = Some(lcid_language(tokens.get(index + 1)?.as_str())?);
        }
        "\\m" if bibliography => {
          tag = Some(tokens.get(index + 1)?.as_str());
        }
        _ => return None,
      }
      index += 2;
    }
    let language = language.unwrap_or("en-US");
    let mut selected = self
      .books
      .iter()
      .filter(|book| tag.is_none_or(|tag| tag == book.tag))
      .collect::<Vec<_>>();
    if selected.is_empty() {
      return None;
    }
    // These verified name/punctuation resources cover the Latin English and
    // Spanish profiles. Other scripts/locales need their own native policy.
    for book in &selected {
      conjunction(book.language.as_deref().unwrap_or(language))?;
    }
    if citation {
      if selected.len() != 1 {
        return None;
      }
      let book = selected[0];
      let mut text = book
        .authors
        .first()
        .map(|author| author.0.clone())
        .unwrap_or_default();
      if book.authors.len() > 1 {
        text.push_str(if language.starts_with("es") {
          ", y otros"
        } else {
          ", et al."
        });
      }
      if !book.year.is_empty() {
        if !text.is_empty() {
          text.push_str(", ");
        }
        text.push_str(&book.year);
      }
      let mut inlines = Vec::new();
      push_resolved_field_text(&mut inlines, format!("({text})"), context.clone(), None);
      return Some(Result {
        inlines,
        paragraphs: Vec::new(),
      });
    }
    selected.sort_by_cached_key(|book| {
      (
        book.author_text(language).to_lowercase(),
        book.year.clone(),
        book.title.to_lowercase(),
      )
    });
    // Repeated-author substitution is a separate style feature. Preserve the
    // full cache until that policy is available rather than emitting duplicates.
    if selected
      .windows(2)
      .any(|pair| pair[0].authors == pair[1].authors && !pair[0].authors.is_empty())
    {
      return None;
    }
    let style_id = styles
      .styles
      .iter()
      .find(|(_, entry)| {
        entry
          .name
          .as_deref()
          .is_some_and(|name| name.eq_ignore_ascii_case("Bibliography"))
      })
      .map(|(id, _)| id.as_str());
    let base =
      styles.run_style_with_base(style_id, TextStyle::default(), RunStyleOverrides::default());
    let format = styles.paragraph_format_with_base(style_id, ParagraphFormat::default());
    let mut result = Result {
      inlines: Vec::new(),
      paragraphs: Vec::new(),
    };
    for (index, book) in selected.into_iter().enumerate() {
      let mut author = book.author_text(language);
      sentence_end(&mut author);
      if !book.year.is_empty() {
        if !author.is_empty() {
          author.push(' ');
        }
        author.push_str(&book.year);
        sentence_end(&mut author);
      }
      let mut bold = base.clone();
      bold.bold = true;
      bold.complex_bold = Some(true);
      push_resolved_field_text(&mut result.inlines, author, bold, None);
      push_resolved_field_text(&mut result.inlines, " ".into(), base.clone(), None);
      if !book.title.is_empty() {
        let mut title = book.title.clone();
        sentence_end(&mut title);
        title.push(' ');
        let mut italic = base.clone();
        italic.italic = true;
        italic.complex_italic = Some(true);
        push_resolved_field_text(&mut result.inlines, title, italic, None);
      }
      let mut publication = book.city.clone();
      if !book.publisher.is_empty() {
        if !publication.is_empty() {
          publication.push_str(" : ");
        }
        publication.push_str(&book.publisher);
      }
      if !book.year.is_empty() {
        if !publication.is_empty() {
          publication.push_str(", ");
        }
        publication.push_str(&book.year);
      }
      sentence_end(&mut publication);
      push_resolved_field_text(&mut result.inlines, publication, base.clone(), None);
      let mut mark = base.clone();
      // Word's HTML field replacement gives the first inserted paragraph's
      // mark the HTML default 12pt; later marks inherit Bibliography.
      if index == 0 {
        mark.font_size_pt = 12.0;
        mark.complex_font_size_pt = Some(12.0);
      }
      result
        .paragraphs
        .push(ParagraphFieldEvent::GeneratedParagraphBreak {
          inline_offset: result.inlines.len(),
          format: Box::new(format.clone()),
          base_style: Box::new(mark),
        });
    }
    Some(result)
  }
}

impl InternetSite {
  fn parse(source: &b::Source) -> Option<Self> {
    let mut site = Self::default();
    let mut is_site = false;
    for choice in &source.source_choice {
      match choice {
        b::SourceChoice::SourceType(b::DataSourceValues::InternetSite) => is_site = true,
        b::SourceChoice::Tag(value) => site.tag = value.as_str().to_owned(),
        b::SourceChoice::Title(value) => site.title = Some(value.as_str().to_owned()),
        b::SourceChoice::Year(value) => site.published.year = Some(value.as_str().to_owned()),
        b::SourceChoice::Month(value) => site.published.month = Some(value.as_str().to_owned()),
        b::SourceChoice::Day(value) => site.published.day = Some(value.as_str().to_owned()),
        b::SourceChoice::YearAccessed(value) => {
          site.accessed.year = Some(value.as_str().to_owned())
        }
        b::SourceChoice::MonthAccessed(value) => {
          site.accessed.month = Some(value.as_str().to_owned())
        }
        b::SourceChoice::DayAccessed(value) => site.accessed.day = Some(value.as_str().to_owned()),
        b::SourceChoice::UrlString(value) => site.url = Some(value.as_str().to_owned()),
        b::SourceChoice::GuidString(_) | b::SourceChoice::ReferenceOrder(_) => {}
        b::SourceChoice::LcId(value) if value.as_str() == "0" => {}
        b::SourceChoice::LcId(value) => {
          site.language = Some(lcid_language(value.as_str())?.to_owned())
        }
        b::SourceChoice::AuthorList(list) => {
          let [b::AuthorListChoice::Author(author)] = list.author_list_choice.as_slice() else {
            return None;
          };
          let b::AuthorChoice::Corporate(name) = author.author_choice.as_ref()? else {
            return None;
          };
          site.corporate_author = Some(name.as_str().to_owned());
        }
        _ => return None,
      }
    }
    (is_site && !site.tag.is_empty()).then_some(site)
  }

  fn text(&self) -> String {
    let mut text = String::new();
    if let Some(author) = &self.corporate_author {
      text.push_str(author);
      text.push_str(", ");
    }
    if let Some(title) = &self.title {
      text.push('"');
      text.push_str(title.trim_end());
      text.push_str(",\" ");
    }
    if let Some(date) = self.published.text() {
      text.push_str(&date);
      text.push_str(". ");
    }
    text.push_str("[Online].");
    if let Some(url) = &self.url {
      text.push_str(" Available: ");
      text.push_str(url);
      text.push('.');
    }
    if let Some(date) = self.accessed.text() {
      text.push_str(" [Accessed ");
      text.push_str(&date);
      text.push_str("].");
    }
    text
  }
}

impl WebsiteDate {
  fn text(&self) -> Option<String> {
    let year = self.year.as_deref()?;
    // IEEE only displays a day when both month and year are present. Source
    // month strings are retained, not interpreted using the host locale.
    Some(
      [
        self.month.as_ref().and(self.day.as_deref()),
        self.month.as_deref(),
        Some(year),
      ]
      .into_iter()
      .flatten()
      .collect::<Vec<_>>()
      .join(" "),
    )
  }
}

fn plain_bibliography_boundary(paragraph: &w::Paragraph, opening: bool) -> bool {
  let mut markers = Vec::new();
  let mut instruction = String::new();
  for item in &paragraph.paragraph_choice {
    let w::ParagraphChoice::WRun(run) = item else {
      return false;
    };
    for item in &run.run_choice {
      match item {
        w::RunChoice::FieldChar(field) => {
          if field
            .field_lock
            .is_some_and(ooxmlsdk::simple_type::OnOffValue::as_bool)
          {
            return false;
          }
          markers.push(field.field_char_type);
        }
        w::RunChoice::FieldCode(code) if opening && markers == [w::FieldCharValues::Begin] => {
          if let Some(text) = &code.xml_content {
            instruction.push_str(text.as_str());
          }
        }
        _ => return false,
      }
    }
  }
  if opening {
    markers == [w::FieldCharValues::Begin, w::FieldCharValues::Separate]
      && instruction.trim().eq_ignore_ascii_case("BIBLIOGRAPHY")
  } else {
    markers == [w::FieldCharValues::End]
  }
}

fn has_citation_field(xml: &str) -> bool {
  use quick_xml::events::Event;
  let mut reader = quick_xml::Reader::from_str(xml);
  let mut fields = Vec::<String>::new();
  loop {
    match reader.read_event() {
      Ok(Event::Start(element) | Event::Empty(element)) => match element.local_name().as_ref() {
        b"fldChar" => {
          let Some(kind) = element
            .attributes()
            .flatten()
            .find(|a| a.key.local_name().as_ref() == b"fldCharType")
          else {
            return true;
          };
          match kind.value.as_ref() {
            b"begin" => fields.push(String::new()),
            b"separate" | b"end" => {
              if let Some(instruction) = fields.last()
                && field_instruction_name(instruction).as_deref() == Some("CITATION")
              {
                return true;
              }
              if kind.value.as_ref() == b"end" {
                fields.pop();
              }
            }
            _ => return true,
          }
        }
        b"instrText" => {
          let Ok(text) = reader.read_text(element.name()) else {
            return true;
          };
          let Ok(text) = text.xml10_content() else {
            return true;
          };
          let Ok(text) = quick_xml::escape::unescape(&text) else {
            return true;
          };
          if let Some(instruction) = fields.last_mut() {
            instruction.push_str(&text);
          }
        }
        b"fldSimple" => {
          let Some(instruction) = element
            .attributes()
            .flatten()
            .find(|a| a.key.local_name().as_ref() == b"instr")
          else {
            return true;
          };
          let Ok(instruction) = instruction.normalized_value(quick_xml::XmlVersion::Implicit1_0)
          else {
            return true;
          };
          if field_instruction_name(&instruction).as_deref() == Some("CITATION") {
            return true;
          }
        }
        _ => {}
      },
      Ok(Event::Eof) => {
        return fields
          .iter()
          .any(|f| field_instruction_name(f).as_deref() == Some("CITATION"));
      }
      Err(_) => return true,
      _ => {}
    }
  }
}

impl Book {
  fn parse(source: &b::Source) -> Option<Self> {
    let mut book = Self::default();
    let mut is_book = false;
    for choice in &source.source_choice {
      match choice {
        b::SourceChoice::SourceType(b::DataSourceValues::Book) => is_book = true,
        b::SourceChoice::Tag(value) => book.tag = value.as_str().to_owned(),
        b::SourceChoice::Title(value) => book.title = value.as_str().to_owned(),
        b::SourceChoice::Year(value) => book.year = value.as_str().to_owned(),
        b::SourceChoice::City(value) => book.city = value.as_str().to_owned(),
        b::SourceChoice::Publisher(value) => book.publisher = value.as_str().to_owned(),
        b::SourceChoice::GuidString(_) | b::SourceChoice::ReferenceOrder(_) => {}
        b::SourceChoice::LcId(value) if value.as_str() == "0" => {}
        b::SourceChoice::LcId(value) => {
          book.language = Some(lcid_language(value.as_str())?.to_owned())
        }
        b::SourceChoice::AuthorList(list) => {
          let [b::AuthorListChoice::Author(author)] = list.author_list_choice.as_slice() else {
            return None;
          };
          match author.author_choice.as_ref()? {
            b::AuthorChoice::Corporate(name) => {
              book.authors.push((name.as_str().to_owned(), String::new()))
            }
            b::AuthorChoice::NameList(names) => {
              if names.person.len() > 2 {
                return None;
              }
              for person in &names.person {
                let last = name(&person.last)?.to_owned();
                let given = [name(&person.first)?, name(&person.middle)?]
                  .into_iter()
                  .filter(|part| !part.is_empty())
                  .collect::<Vec<_>>()
                  .join(" ");
                book.authors.push((last, given));
              }
            }
          }
        }
        _ => return None,
      }
    }
    (is_book && !book.tag.is_empty()).then_some(book)
  }

  fn author_text(&self, language: &str) -> String {
    let join = conjunction(self.language.as_deref().unwrap_or(language)).unwrap_or(" and ");
    self
      .authors
      .iter()
      .map(|(last, given)| match (last.is_empty(), given.is_empty()) {
        (false, false) => format!("{last}, {given}"),
        (true, _) => given.clone(),
        _ => last.clone(),
      })
      .collect::<Vec<_>>()
      .join(join)
  }
}

fn conjunction(language: &str) -> Option<&'static str> {
  if language.starts_with("es") {
    Some(" y ")
  } else if language.starts_with("en") {
    Some(" and ")
  } else {
    None
  }
}

fn lcid_language(lcid: &str) -> Option<&'static str> {
  match lcid {
    "1033" | "2057" => Some("en-US"),
    "3082" | "1034" | "2058" => Some("es-ES"),
    _ => None,
  }
}

fn sentence_end(text: &mut String) {
  if !text.is_empty() && !text.ends_with(['.', '?', '!']) {
    text.push('.');
  }
}

fn name(parts: &[ooxmlsdk::simple_type::StringValue]) -> Option<&str> {
  match parts {
    [] => Some(""),
    [part] => Some(part.as_str()),
    _ => None,
  }
}

fn supported_source_xml(xml: &str, allow_doi: bool) -> bool {
  use quick_xml::{events::Event, name::ResolveResult};
  let mut reader = quick_xml::NsReader::from_reader(xml.as_bytes());
  let mut stack: Vec<(Vec<u8>, HashSet<Vec<u8>>)> = Vec::new();
  let mut root_seen = false;
  loop {
    match reader.read_resolved_event() {
      Ok((namespace, event @ (Event::Start(_) | Event::Empty(_)))) => {
        if !matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORD_BIBLIOGRAPHY_NAMESPACE.as_bytes())
        {
          return false;
        }
        let empty = matches!(event, Event::Empty(_));
        let (Event::Start(element) | Event::Empty(element)) = event else {
          unreachable!()
        };
        let local = element.local_name();
        let local = local.as_ref();
        let valid = match stack.last().map(|(name, _)| name.as_slice()) {
          None => !root_seen && local == b"Sources",
          Some(b"Sources") => local == b"Source",
          Some(b"Source") => {
            matches!(
              local,
              b"Tag"
                | b"SourceType"
                | b"Author"
                | b"Title"
                | b"Year"
                | b"Month"
                | b"Day"
                | b"YearAccessed"
                | b"MonthAccessed"
                | b"DayAccessed"
                | b"URL"
                | b"City"
                | b"Publisher"
                | b"Guid"
                | b"RefOrder"
                | b"LCID"
                | b"BookTitle"
                | b"ChapterNumber"
                | b"Pages"
                | b"Edition"
                | b"JournalName"
                | b"Volume"
                | b"Issue"
            ) || (allow_doi && local == b"DOI")
          }
          Some(b"Author") if stack.len() == 3 => matches!(local, b"Author" | b"BookAuthor"),
          Some(b"Author") => matches!(local, b"NameList" | b"Corporate"),
          Some(b"BookAuthor") => local == b"NameList",
          Some(b"NameList") => local == b"Person",
          Some(b"Person") => matches!(local, b"Last" | b"First" | b"Middle"),
          _ => false,
        };
        if !valid {
          return false;
        }
        if let Some((parent, children)) = stack.last_mut()
          && !matches!(parent.as_slice(), b"Sources" | b"NameList")
          && !children.insert(local.to_vec())
        {
          return false;
        }
        root_seen = true;
        if !empty {
          stack.push((local.to_vec(), HashSet::new()));
        }
      }
      Ok((_, Event::End(_))) => {
        if stack.pop().is_none() {
          return false;
        }
      }
      Ok((_, Event::Text(text))) => {
        let container = stack.last().is_none_or(|(name, _)| {
          matches!(
            name.as_slice(),
            b"Sources" | b"Source" | b"Author" | b"BookAuthor" | b"NameList" | b"Person"
          )
        });
        if container && !text.iter().all(u8::is_ascii_whitespace) {
          return false;
        }
      }
      Ok((_, Event::Decl(_) | Event::Comment(_) | Event::PI(_))) => {}
      Ok((_, Event::GeneralRef(_))) if !stack.is_empty() => {}
      Ok((_, Event::Eof)) => return root_seen && stack.is_empty(),
      _ => return false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const SOURCE: &str = r#"<b:Source><b:Tag>Dix07</b:Tag><b:SourceType>Book</b:SourceType><b:Author><b:Author><b:NameList><b:Person><b:Last>Dixon</b:Last><b:First>Andrew</b:First></b:Person></b:NameList></b:Author></b:Author><b:Title>Collected Works</b:Title><b:Year>2007</b:Year></b:Source>"#;

  fn collection(style: &str, sources: &str) -> String {
    format!(
      r#"<b:Sources xmlns:b="http://schemas.openxmlformats.org/officeDocument/2006/bibliography" SelectedStyle="\{style}.XSL">{sources}</b:Sources>"#
    )
  }

  fn styles() -> StylesCatalog {
    StylesCatalog {
      bibliography: Catalog::parse(&collection("GB", SOURCE)).unwrap(),
      doc_default_run: TextStyle {
        font_size_pt: 11.0,
        language: Some("es-ES".into()),
        ..Default::default()
      },
      ..Default::default()
    }
  }

  fn text(inlines: &[InlineItem]) -> String {
    inlines
      .iter()
      .filter_map(|item| match item {
        InlineItem::Text(run) => Some(run.text.as_str()),
        _ => None,
      })
      .collect()
  }

  const WEBSITE: &str = r#"<b:Source><b:Tag>Web</b:Tag><b:SourceType>InternetSite</b:SourceType><b:Author><b:Author><b:Corporate>Example Institute</b:Corporate></b:Author></b:Author><b:Title>A &amp; B</b:Title><b:Year>2021</b:Year><b:Month>October</b:Month><b:Day>3</b:Day><b:URL>https://example.org/?x=1&amp;y=2</b:URL><b:YearAccessed>2022</b:YearAccessed><b:MonthAccessed>March</b:MonthAccessed><b:DayAccessed>22</b:DayAccessed><b:RefOrder>101</b:RefOrder></b:Source>"#;
  const BLOCK_CACHE: &str = r#"<w:sdtContent xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:p><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText> BIBLIOGRAPHY </w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r></w:p><w:p><w:r><w:t>stale cache</w:t></w:r></w:p><w:p><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p></w:sdtContent>"#;

  #[test]
  fn native_ieee_dates_and_quoted_titles_use_source_fields() {
    let catalog = Catalog::parse(&collection("IEEE2006OfficeOnline", WEBSITE)).unwrap();
    assert_eq!(
      catalog.internet_sites[0].text(),
      "Example Institute, \"A & B,\" 3 October 2021. [Online]. Available: https://example.org/?x=1&y=2. [Accessed 22 March 2022]."
    );
    let mut site = catalog.internet_sites[0].clone();
    site.published.month = None;
    site.accessed.day = None;
    assert!(site.text().contains("2021. [Online]."));
    assert!(site.text().ends_with("[Accessed March 2022]."));
    site.published.year = None;
    site.accessed.year = None;
    site.url = None;
    site.title = None;
    site.corporate_author = None;
    assert_eq!(site.text(), "[Online].");
    assert!(Catalog::parse(&collection("GB", WEBSITE)).is_none());
    assert!(Catalog::parse(&collection("IEEE2006OfficeOnline", SOURCE)).is_none());
    assert!(
      Catalog::parse(&collection(
        "IEEE2006OfficeOnline",
        &WEBSITE.replace(
          "</b:Source>",
          "<b:ProductionCompany>Publisher</b:ProductionCompany></b:Source>"
        )
      ))
      .is_none()
    );
  }

  #[test]
  fn ieee_block_refresh_rebuilds_table_and_preserves_field_boundaries() {
    let catalog = Catalog::parse(&collection("IEEE2006OfficeOnline", WEBSITE)).unwrap();
    let mut styles = StylesCatalog::default();
    styles.styles.insert(
      "Bib".into(),
      StyleEntry {
        name: Some("Bibliography".into()),
        ..Default::default()
      },
    );
    let cache = w::SdtContentBlock::from_bytes(BLOCK_CACHE.as_bytes()).unwrap();
    let refreshed = catalog.refresh_block_content(&cache, &styles).unwrap();
    assert_eq!(refreshed.sdt_content_block_choice.len(), 4);
    assert_eq!(
      refreshed.sdt_content_block_choice.first(),
      cache.sdt_content_block_choice.first()
    );
    assert_eq!(
      refreshed.sdt_content_block_choice.last(),
      cache.sdt_content_block_choice.last()
    );
    let w::SdtContentBlockChoice::Table(table) = &refreshed.sdt_content_block_choice[1] else {
      panic!("generated table")
    };
    let model = table_model(
      table,
      &mut TableModelEnv {
        styles: &styles,
        numbering: &mut NumberingCatalog::default(),
        images: &ImageCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        custom_xml_bindings: &CustomXmlBindings::default(),
        form_widget_ids: &mut FormWidgetIdAllocator::default(),
        complex_fields: &mut ComplexFieldImportState::default(),
      },
      TableModelContext {
        nested_table_level: 1,
        in_header_footer: false,
      },
    );
    assert_eq!(model.preferred_width_pct, Some(1.0));
    assert_eq!(model.rows.len(), 1);
    assert_eq!(model.rows[0].cells.len(), 2);
    assert_eq!(model.rows[0].cells[0].preferred_width_pct, Some(0.01));
    let Block::Paragraph(label) = &model.rows[0].cells[0].blocks[0] else {
      panic!("label")
    };
    let Block::Paragraph(body) = &model.rows[0].cells[1].blocks[0] else {
      panic!("body")
    };
    assert_eq!(text(&label.inlines), "[1] ");
    assert_eq!(text(&body.inlines), catalog.internet_sites[0].text());
    assert_eq!(label.base_style.font_size_pt, 12.0);
    for invalid in [
      BLOCK_CACHE.replace(
        "w:fldCharType=\"begin\"",
        "w:fldCharType=\"begin\" w:fldLock=\"true\"",
      ),
      BLOCK_CACHE.replace(" BIBLIOGRAPHY ", " BIBLIOGRAPHY \\m Web "),
      BLOCK_CACHE
        .replace("<w:instrText>", "<w:t>")
        .replace("</w:instrText>", "</w:t>"),
    ] {
      let cache = w::SdtContentBlock::from_bytes(invalid.as_bytes()).unwrap();
      assert!(catalog.refresh_block_content(&cache, &styles).is_none());
    }
  }

  #[test]
  fn citation_guard_detects_split_and_simple_instructions_without_matching_prose() {
    assert!(has_citation_field(
      r#"<d><fldChar fldCharType="begin"/><instrText> CI</instrText><instrText>TATION Web </instrText><fldChar fldCharType="separate"/><fldChar fldCharType="end"/></d>"#
    ));
    assert!(has_citation_field(
      r#"<d><fldSimple instr="CITATION Web"/></d>"#
    ));
    assert!(!has_citation_field(
      r#"<d><t>CITATION</t><fldChar fldCharType="begin"/><instrText>BIBLIOGRAPHY</instrText><fldChar fldCharType="separate"/><fldChar fldCharType="end"/></d>"#
    ));
  }

  #[test]
  fn native_book_runs_use_data_and_style_instead_of_cached_error_format() {
    let styles = styles();
    let result = styles
      .bibliography
      .refresh(
        "BIBLIOGRAPHY",
        &styles,
        &styles.doc_default_run,
        LocaleContext::default(),
      )
      .unwrap();
    assert_eq!(
      text(&result.inlines),
      "Dixon, Andrew. 2007. Collected Works. 2007."
    );
    let runs = result
      .inlines
      .iter()
      .filter_map(|item| match item {
        InlineItem::Text(run) => Some(run),
        _ => None,
      })
      .collect::<Vec<_>>();
    assert!(runs[0].style.bold);
    assert!(!runs[1].style.bold);
    assert!(runs[2].style.italic);
    assert!(!runs[3].style.italic);
    assert!(runs.iter().all(|run| run.style.font_size_pt == 11.0));
    assert!(
      matches!(&result.paragraphs[..], [ParagraphFieldEvent::GeneratedParagraphBreak { inline_offset, base_style, .. }] if *inline_offset == result.inlines.len() && base_style.font_size_pt == 12.0)
    );
  }

  #[test]
  fn unsupported_styles_and_source_fields_keep_the_complete_cache() {
    assert!(Catalog::parse(&collection("APA", SOURCE)).is_none());
    assert!(
      Catalog::parse(
        &collection("GB", SOURCE).replace(WORD_BIBLIOGRAPHY_NAMESPACE, "urn:unrelated")
      )
      .is_none()
    );
    for extra in [
      "<b:Edition>2</b:Edition>",
      "<b:LCID>1041</b:LCID>",
      "<b:Editor/>",
    ] {
      let xml = collection(
        "GB",
        &SOURCE.replace("</b:Source>", &format!("{extra}</b:Source>")),
      );
      assert!(Catalog::parse(&xml).is_none(), "{extra}");
    }
    assert!(Catalog::parse(&collection("GB", &SOURCE.replace("Book", "JournalArticle"))).is_none());
    let styles = styles();
    for field in [
      "BIBLIOGRAPHY \\f 1033",
      "CITATION Unknown",
      "CITATION Dix07 \\p 2",
      "BIBLIOGRAPHY \\l 1041",
    ] {
      assert!(
        styles
          .bibliography
          .refresh(
            field,
            &styles,
            &styles.doc_default_run,
            LocaleContext::default()
          )
          .is_none(),
        "{field}"
      );
    }
  }

  #[test]
  fn native_book_sorting_and_tag_selection_are_independent_of_xml_order() {
    let mut styles = styles();
    let other = SOURCE
      .replace("Dix07", "Ada23")
      .replace("Dixon", "Adams")
      .replace("Andrew", "Alan")
      .replace("2007", "2023");
    styles.bibliography =
      Catalog::parse(&collection("ISO690", &format!("{SOURCE}{other}"))).unwrap();
    let all = styles
      .bibliography
      .refresh(
        "BIBLIOGRAPHY",
        &styles,
        &styles.doc_default_run,
        LocaleContext::default(),
      )
      .unwrap();
    assert!(text(&all.inlines).starts_with("Adams, Alan. 2023."));
    assert_eq!(all.paragraphs.len(), 2);
    let one = styles
      .bibliography
      .refresh(
        "BIBLIOGRAPHY \\m Dix07",
        &styles,
        &styles.doc_default_run,
        LocaleContext::default(),
      )
      .unwrap();
    assert_eq!(one.paragraphs.len(), 1);
    assert!(text(&one.inlines).starts_with("Dixon"));
    let citation = styles
      .bibliography
      .refresh(
        "CITATION Dix07 \\l 3082",
        &styles,
        &styles.doc_default_run,
        LocaleContext::default(),
      )
      .unwrap();
    assert_eq!(text(&citation.inlines), "(Dixon, 2007)");
    assert!(citation.paragraphs.is_empty());
  }

  #[test]
  fn rich_field_paragraphs_survive_sdt_and_simple_field_import_with_locks() {
    for simple in [false, true] {
      for sdt in [false, true] {
        for locked in [false, true] {
          let field = if simple {
            format!(
              r#"<w:fldSimple w:instr="BIBLIOGRAPHY" w:fldLock="{}"><w:r><w:t>OLD</w:t></w:r></w:fldSimple>"#,
              u8::from(locked)
            )
          } else {
            format!(
              r#"<w:r><w:fldChar w:fldCharType="begin" w:fldLock="{}"/></w:r><w:r><w:instrText>BIBLIOGRAPHY</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>OLD</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r>"#,
              u8::from(locked)
            )
          };
          let field = if sdt {
            format!(
              "<w:sdt><w:sdtPr><w:bibliography/></w:sdtPr><w:sdtContent>{field}</w:sdtContent></w:sdt>"
            )
          } else {
            field
          };
          let xml = format!(
            r#"<w:body xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:p><w:r><w:t>Before</w:t></w:r>{field}<w:r><w:t>After</w:t></w:r></w:p><w:sectPr/></w:body>"#
          );
          let body = w::Body::from_bytes(xml.as_bytes()).unwrap();
          let styles = styles();
          let mut sections = body_sections(
            &body,
            BodySectionEnv {
              styles: &styles,
              numbering: &mut NumberingCatalog::default(),
              images: &ImageCatalog::default(),
              alt_chunks: &AltChunkCatalog::default(),
              hyperlinks: &HyperlinkCatalog::default(),
              custom_xml_bindings: &CustomXmlBindings::default(),
              form_widget_ids: &mut FormWidgetIdAllocator::default(),
              no_column_balance: false,
              fixed_html_paragraph_auto_spacing: false,
            },
          );
          normalize_complex_field_paragraph_breaks(&mut sections[0].blocks);
          let labels = sections[0]
            .blocks
            .iter()
            .filter_map(|block| match block {
              Block::Paragraph(p) => Some(text(&p.inlines)),
              _ => None,
            })
            .collect::<Vec<_>>();
          if locked {
            assert_eq!(labels, ["BeforeOLDAfter"]);
          } else {
            assert_eq!(
              labels,
              ["BeforeDixon, Andrew. 2007. Collected Works. 2007.", "After"],
              "simple={simple},sdt={sdt}"
            );
          }
        }
      }
    }
  }

  #[test]
  fn generated_paragraphs_recover_story_defaults_beneath_explicit_style_spacing() {
    let body = w::Body::from_bytes(
      br#"<w:body xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
        <w:p><w:pPr><w:spacing w:after="840"/></w:pPr><w:fldSimple w:instr="BIBLIOGRAPHY"><w:r><w:t>OLD</w:t></w:r></w:fldSimple></w:p>
        <w:sectPr/>
      </w:body>"#,
    )
    .unwrap();
    for bibliography_style in ["GB", "APASixthEditionOfficeOnline"] {
      for (has_styles_part, has_default_paragraph_properties) in
        [(false, false), (true, false), (true, true)]
      {
        for has_settings_part in [false, true] {
          for explicit_spacing in [false, true] {
            let mut styles = styles();
            styles.doc_default_run.language = Some("en-US".into());
            styles.has_styles_part = has_styles_part;
            styles.has_default_paragraph_properties = has_default_paragraph_properties;
            styles.import_settings.has_settings_part = has_settings_part;
            styles.bibliography = Catalog::parse(&collection(
              bibliography_style,
              &format!(
                "{SOURCE}{}",
                SOURCE
                  .replace("Dix07", "Second")
                  .replace("Dixon", "Edwards")
              ),
            ))
            .unwrap();
            if explicit_spacing {
              styles.styles.insert(
                "Bib".into(),
                StyleEntry {
                  name: Some("Bibliography".into()),
                  paragraph_format: ParagraphFormat {
                    spacing_after_pt: 0.0,
                    spacing_after_set: true,
                    line_height_pt: Some(1.0),
                    ..Default::default()
                  },
                  ..Default::default()
                },
              );
            }
            let mut sections = body_sections(
              &body,
              BodySectionEnv {
                styles: &styles,
                numbering: &mut NumberingCatalog::default(),
                images: &ImageCatalog::default(),
                alt_chunks: &AltChunkCatalog::default(),
                hyperlinks: &HyperlinkCatalog::default(),
                custom_xml_bindings: &CustomXmlBindings::default(),
                form_widget_ids: &mut FormWidgetIdAllocator::default(),
                no_column_balance: false,
                fixed_html_paragraph_auto_spacing: false,
              },
            );
            normalize_complex_field_paragraph_breaks(&mut sections[0].blocks);
            let generated = sections[0]
              .blocks
              .iter()
              .filter_map(|block| match block {
                Block::Paragraph(p) if text(&p.inlines).contains("Collected Works") => Some(p),
                _ => None,
              })
              .collect::<Vec<_>>();
            assert_eq!(generated.len(), 2);
            let recovered = !explicit_spacing && !has_default_paragraph_properties;
            for paragraph in generated {
              assert_eq!(
                paragraph.format.spacing_after_pt,
                if recovered { 8.0 } else { 0.0 }
              );
              assert_eq!(
                paragraph.format.line_height_pt,
                if explicit_spacing {
                  Some(1.0)
                } else if recovered {
                  Some(278.0 / 240.0)
                } else {
                  None
                }
              );
              assert_eq!(paragraph.format.office_recovered_line_height, recovered);
              assert_eq!(
                paragraph
                  .format
                  .office_recovered_line_height_without_settings_part,
                recovered && !has_settings_part
              );
            }
          }
        }
      }
    }
  }

  #[test]
  fn cross_paragraph_cache_is_replaced_or_restored_as_one_field() {
    for supported in [true, false] {
      for locked in [true, false] {
        for closed in [true, false] {
          let end = if closed {
            r#"<w:r><w:fldChar w:fldCharType="end"/></w:r>"#
          } else {
            ""
          };
          let xml = format!(
            r#"<w:body xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:p><w:pPr><w:ind w:left="100"/></w:pPr><w:r><w:t>Before</w:t><w:fldChar w:fldCharType="begin" w:fldLock="{}"/><w:instrText>BIBLIOGRAPHY</w:instrText><w:fldChar w:fldCharType="separate"/><w:t>Cache one</w:t></w:r></w:p>
            <w:p><w:pPr><w:ind w:left="200"/></w:pPr><w:r><w:t>Cache two</w:t></w:r></w:p>
            <w:p><w:pPr><w:ind w:left="300"/></w:pPr>{end}<w:r><w:t>After</w:t></w:r></w:p><w:sectPr/>
          </w:body>"#,
            u8::from(locked)
          );
          let body = w::Body::from_bytes(xml.as_bytes()).unwrap();
          let mut styles = styles();
          if !supported {
            styles.bibliography = Catalog::default();
          }
          let mut sections = body_sections(
            &body,
            BodySectionEnv {
              styles: &styles,
              numbering: &mut NumberingCatalog::default(),
              images: &ImageCatalog::default(),
              alt_chunks: &AltChunkCatalog::default(),
              hyperlinks: &HyperlinkCatalog::default(),
              custom_xml_bindings: &CustomXmlBindings::default(),
              form_widget_ids: &mut FormWidgetIdAllocator::default(),
              no_column_balance: false,
              fixed_html_paragraph_auto_spacing: false,
            },
          );
          normalize_complex_field_paragraph_breaks(&mut sections[0].blocks);
          let paragraphs = sections[0]
            .blocks
            .iter()
            .filter_map(|b| match b {
              Block::Paragraph(p) => Some(p),
              _ => None,
            })
            .collect::<Vec<_>>();
          let labels = paragraphs
            .iter()
            .map(|p| text(&p.inlines))
            .collect::<Vec<_>>();
          if supported && !locked && closed {
            assert_eq!(
              labels,
              ["BeforeDixon, Andrew. 2007. Collected Works. 2007.", "After"]
            );
          } else {
            assert_eq!(
              labels,
              ["BeforeCache one", "Cache two", "After"],
              "supported={supported},locked={locked},closed={closed}"
            );
            assert_eq!(
              paragraphs
                .iter()
                .map(|p| p.format.indent_left_pt)
                .collect::<Vec<_>>(),
              [5.0, 10.0, 15.0]
            );
          }
        }
      }
    }
  }

  #[test]
  fn ieee_block_boundary_does_not_insert_a_paragraph_in_first_label_cell() {
    let xml = format!(
      r#"<w:body xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:sdt><w:sdtPr><w:bibliography/></w:sdtPr>{BLOCK_CACHE}</w:sdt><w:sectPr/></w:body>"#
    );
    let body = w::Body::from_bytes(xml.as_bytes()).unwrap();
    let mut styles = StylesCatalog::default();
    styles.styles.insert(
      "Bibliography".into(),
      StyleEntry {
        name: Some("Bibliography".into()),
        ..Default::default()
      },
    );
    styles.bibliography = Catalog::parse(&collection("IEEE2006OfficeOnline", WEBSITE)).unwrap();
    let mut sections = body_sections(
      &body,
      BodySectionEnv {
        styles: &styles,
        numbering: &mut NumberingCatalog::default(),
        images: &ImageCatalog::default(),
        alt_chunks: &AltChunkCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        custom_xml_bindings: &CustomXmlBindings::default(),
        form_widget_ids: &mut FormWidgetIdAllocator::default(),
        no_column_balance: false,
        fixed_html_paragraph_auto_spacing: false,
      },
    );
    normalize_complex_field_paragraph_breaks(&mut sections[0].blocks);
    let table = sections[0]
      .blocks
      .iter()
      .find_map(|block| match block {
        Block::Table(table) => Some(table),
        _ => None,
      })
      .unwrap();
    let cell = &table.rows[0].cells[0];
    let labels = cell
      .blocks
      .iter()
      .filter_map(|block| match block {
        Block::Paragraph(p) => Some(text(&p.inlines)),
        _ => None,
      })
      .collect::<Vec<_>>();
    assert_eq!(labels, ["[1] "]);
  }
}
