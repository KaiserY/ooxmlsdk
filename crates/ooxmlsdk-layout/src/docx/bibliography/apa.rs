//! Word APA sixth-edition bibliography profiles. Source selection is ECMA-376
//! 17.16.5.7; Office's APA templates own punctuation, names and rich paragraphs.
//! Native mixed-language/source controls verify the formatter independently of
//! cached field results. Unsupported metadata or locales retain the whole cache.

use super::*;

#[derive(Clone, Debug, Default)]
pub(super) struct Entry {
  kind: b::DataSourceValues,
  tag: String,
  authors: Authors,
  book_authors: Authors,
  interviewees: Option<Authors>,
  interviewers: Option<Authors>,
  title: String,
  book_title: String,
  year: String,
  month: String,
  day: String,
  city: String,
  publisher: String,
  edition: String,
  pages: String,
  journal: String,
  volume: String,
  issue: String,
  doi: String,
  url: String,
  accessed: WebsiteDate,
  locale: Option<Locale>,
}

#[derive(Clone, Debug, Default)]
struct Authors {
  corporate: Option<String>,
  people: Vec<Person>,
}

#[derive(Clone, Debug)]
struct Person {
  last: String,
  first: String,
  middle: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Locale {
  English,
  Chinese,
}

impl Locale {
  pub(super) fn language(language: &str) -> Option<Self> {
    if language.starts_with("en") {
      Some(Self::English)
    } else if matches!(language, "zh-CN" | "zh-Hans" | "zh-SG") {
      Some(Self::Chinese)
    } else {
      None
    }
  }

  fn lcid(lcid: &str) -> Option<Self> {
    match lcid {
      "1033" | "2057" | "3081" | "4105" | "5129" => Some(Self::English),
      "2052" | "4100" => Some(Self::Chinese),
      _ => None,
    }
  }
}

pub(super) fn parse(sources: &b::Sources, xml: &str) -> Option<Vec<Entry>> {
  // Office emits DOI in the bibliography namespace, although the ECMA schema
  // model has no DOI child. Read this verified extension alongside typed data;
  // the parent validator rejects unknown nesting, duplicates and foreign nodes.
  let mut reader = quick_xml::Reader::from_str(xml);
  let mut dois = Vec::new();
  loop {
    match reader.read_event().ok()? {
      quick_xml::events::Event::Start(element) if element.local_name().as_ref() == b"Source" => {
        dois.push(String::new());
      }
      quick_xml::events::Event::Start(element) if element.local_name().as_ref() == b"DOI" => {
        let raw = reader.read_text(element.name()).ok()?;
        *dois.last_mut()? = quick_xml::escape::unescape(&raw.decode().ok()?)
          .ok()?
          .into_owned();
      }
      quick_xml::events::Event::Eof => break,
      _ => {}
    }
  }
  if dois.len() != sources.source.len() {
    return None;
  }
  sources
    .source
    .iter()
    .zip(dois)
    .map(|(source, doi)| {
      let mut entry = Entry {
        doi,
        ..Entry::default()
      };
      for choice in &source.source_choice {
        match choice {
          b::SourceChoice::SourceType(kind) => entry.kind = *kind,
          b::SourceChoice::Tag(v) => entry.tag = v.to_string(),
          b::SourceChoice::Title(v) => entry.title = v.to_string(),
          b::SourceChoice::BookTitle(v) => entry.book_title = v.to_string(),
          b::SourceChoice::Year(v) => entry.year = v.to_string(),
          b::SourceChoice::Month(v) => entry.month = v.to_string(),
          b::SourceChoice::Day(v) => entry.day = v.to_string(),
          b::SourceChoice::YearAccessed(v) => entry.accessed.year = Some(v.to_string()),
          b::SourceChoice::MonthAccessed(v) => entry.accessed.month = Some(v.to_string()),
          b::SourceChoice::DayAccessed(v) => entry.accessed.day = Some(v.to_string()),
          b::SourceChoice::City(v) => entry.city = v.to_string(),
          b::SourceChoice::Publisher(v) => entry.publisher = v.to_string(),
          b::SourceChoice::Edition(v) => entry.edition = v.to_string(),
          b::SourceChoice::Pages(v) => entry.pages = v.to_string(),
          b::SourceChoice::JournalName(v) => entry.journal = v.to_string(),
          b::SourceChoice::Volume(v) => entry.volume = v.to_string(),
          b::SourceChoice::Issue(v) => entry.issue = v.to_string(),
          b::SourceChoice::UrlString(v) => entry.url = v.to_string(),
          b::SourceChoice::GuidString(_)
          | b::SourceChoice::ReferenceOrder(_)
          | b::SourceChoice::ChapterNumber(_) => {}
          b::SourceChoice::LcId(v) if v.as_str() == "0" || v.as_str().is_empty() => {}
          b::SourceChoice::LcId(v) => entry.locale = Some(Locale::lcid(v.as_str())?),
          b::SourceChoice::AuthorList(list) => {
            for author in &list.author_list_choice {
              match author {
                b::AuthorListChoice::Author(author) => match author.author_choice.as_ref()? {
                  b::AuthorChoice::Corporate(value) => {
                    entry.authors.corporate = Some(value.to_string())
                  }
                  b::AuthorChoice::NameList(names) => entry.authors = Authors::parse(names)?,
                },
                b::AuthorListChoice::BookAuthor(author) => {
                  entry.book_authors = Authors::parse(&author.name_list)?
                }
                b::AuthorListChoice::Interviewee(author) => {
                  entry.interviewees = Some(Authors::parse(&author.name_list)?)
                }
                b::AuthorListChoice::Interviewer(author) => {
                  entry.interviewers = Some(Authors::parse(&author.name_list)?)
                }
                _ => return None,
              }
            }
          }
          _ => return None,
        }
      }
      // These roles belong to Interview's APA template. Keep the whole cache
      // for other source/metadata combinations whose templates are not covered.
      if entry.kind != b::DataSourceValues::Interview
        && (entry.interviewees.is_some() || entry.interviewers.is_some())
      {
        return None;
      }
      if !matches!(
        entry.kind,
        b::DataSourceValues::InternetSite | b::DataSourceValues::ElectronicSource
      ) && (entry.accessed.year.is_some()
        || entry.accessed.month.is_some()
        || entry.accessed.day.is_some())
      {
        return None;
      }
      if entry.kind == b::DataSourceValues::Interview
        && (!entry.book_authors.people.is_empty()
          || !entry.book_title.is_empty()
          || !entry.edition.is_empty()
          || !entry.pages.is_empty()
          || !entry.journal.is_empty()
          || !entry.volume.is_empty()
          || !entry.issue.is_empty()
          || !entry.doi.is_empty()
          || !entry.url.is_empty())
      {
        return None;
      }
      (matches!(
        entry.kind,
        b::DataSourceValues::Book
          | b::DataSourceValues::BookSection
          | b::DataSourceValues::JournalArticle
          | b::DataSourceValues::Report
          | b::DataSourceValues::ElectronicSource
          | b::DataSourceValues::InternetSite
          | b::DataSourceValues::Interview
      ) && !entry.tag.is_empty())
      .then_some(entry)
    })
    .collect()
}

impl Authors {
  fn parse(names: &b::NameList) -> Option<Self> {
    Some(Self {
      corporate: None,
      people: names
        .person
        .iter()
        .map(|p| {
          Some(Person {
            last: name(&p.last)?.to_owned(),
            first: name(&p.first)?.to_owned(),
            middle: name(&p.middle)?.to_owned(),
          })
        })
        .collect::<Option<_>>()?,
    })
  }

  fn text(&self, locale: Locale, secondary: bool) -> String {
    if let Some(corporate) = &self.corporate {
      return corporate.clone();
    }
    let count = self.people.len();
    let mut output = String::new();
    for (index, person) in self.people.iter().enumerate() {
      // APA lists at most seven people: for larger lists, six then the last.
      if count > 7 && index >= 6 && index + 1 < count {
        continue;
      }
      if index > 0 {
        output.push_str(if index + 1 == count {
          if count > 7 { ", . . . " } else { ", & " }
        } else {
          ", "
        });
      }
      if locale == Locale::Chinese {
        output.push_str(&person.last);
        output.push_str(&person.middle);
        output.push_str(&person.first);
      } else {
        let initials = [&person.first, &person.middle]
          .into_iter()
          .flat_map(|part| part.split_whitespace())
          .map(|word| {
            word
              .split('-')
              .filter_map(|part| part.chars().next())
              .map(|c| format!("{c}."))
              .collect::<Vec<_>>()
              .join("-")
          })
          .collect::<Vec<_>>()
          .join(" ");
        if secondary {
          output.push_str(&initials);
          if !initials.is_empty() && !person.last.is_empty() {
            output.push(' ');
          }
          output.push_str(&person.last);
        } else {
          output.push_str(&person.last);
          if !initials.is_empty() && !person.last.is_empty() {
            output.push_str(", ");
          }
          output.push_str(&initials);
        }
      }
    }
    output
  }
}

#[derive(Default)]
struct Text {
  runs: Vec<(String, bool)>,
}
impl Text {
  fn push(&mut self, text: impl Into<String>, italic: bool) {
    let text = text.into();
    if text.is_empty() {
      return;
    }
    if let Some((last, last_italic)) = self.runs.last_mut()
      && *last_italic == italic
    {
      last.push_str(&text);
    } else {
      self.runs.push((text, italic));
    }
  }
  fn space(&mut self) {
    if !self.runs.is_empty() {
      self.push(" ", false);
    }
  }
  fn sentence(&mut self, text: &str, italic: bool) {
    if text.is_empty() {
      return;
    }
    let mut text = text.to_owned();
    sentence_end(&mut text);
    self.space();
    self.push(text, italic);
  }
}

impl Entry {
  fn primary_author(&self, locale: Locale) -> String {
    // APASixthEditionOfficeOnline's Interview branch and sort key both select
    // Interviewee. A generic Author is not its fallback (native role matrix).
    if self.kind == b::DataSourceValues::Interview {
      self
        .interviewees
        .as_ref()
        .map_or_else(String::new, |authors| authors.text(locale, false))
    } else {
      self.authors.text(locale, false)
    }
  }

  fn sort_key(&self, locale: Locale) -> String {
    let author = self.primary_author(self.locale.unwrap_or(locale));
    let parts = if author.is_empty() {
      [&self.title, &self.year, &author]
    } else {
      [&author, &self.year, &self.title]
    };
    parts
      .into_iter()
      .filter(|part| !part.is_empty())
      .map(String::as_str)
      .collect::<Vec<_>>()
      .join(" ")
      .to_lowercase()
  }

  fn date(&self, locale: Locale) -> String {
    if self.year.is_empty() {
      return if locale == Locale::Chinese {
        "无日期"
      } else {
        "n.d."
      }
      .into();
    }
    let mut date = self.year.clone();
    if !self.month.is_empty() {
      if locale == Locale::Chinese {
        date.push_str(&format!("年{}月", self.month));
        if !self.day.is_empty() {
          date.push_str(&format!("{}日", self.day));
        }
      } else {
        date.push_str(&format!(", {}", self.month));
        if !self.day.is_empty() {
          date.push_str(&format!(" {}", self.day));
        }
      }
    }
    date
  }

  fn accessed_date(&self, locale: Locale) -> String {
    // formatDateAccessed: year is required; day requires month. Unlike the
    // publication date, English retrieval dates put the year last.
    let Some(year) = self.accessed.year.as_deref().filter(|v| !v.is_empty()) else {
      return String::new();
    };
    let Some(month) = self.accessed.month.as_deref().filter(|v| !v.is_empty()) else {
      return year.to_owned();
    };
    let day = self.accessed.day.as_deref().filter(|v| !v.is_empty());
    match (locale, day) {
      (Locale::English, Some(day)) => format!("{month} {day}, {year}"),
      (Locale::English, None) => format!("{month} {year}"),
      (Locale::Chinese, Some(day)) => format!("{year}年{month}月{day}日"),
      (Locale::Chinese, None) => format!("{year}年{month}月"),
    }
  }

  fn title(&self, locale: Locale, book: bool) -> Text {
    let mut text = Text::default();
    text.push(
      if book { &self.book_title } else { &self.title },
      locale == Locale::English,
    );
    let mut details = Vec::new();
    if !self.edition.is_empty() {
      details.push(format!(
        "{} {}",
        self.edition,
        if locale == Locale::Chinese {
          "版本"
        } else {
          "ed."
        }
      ));
    }
    if book && !self.pages.is_empty() {
      details.push(format!(
        "{} {}",
        if locale == Locale::Chinese {
          "页"
        } else {
          "pp."
        },
        self.pages
      ));
    }
    if !details.is_empty() {
      text.push(format!(" ({})", details.join(", ")), false);
    }
    if !text.runs.is_empty() {
      let (last, _) = text.runs.last_mut().unwrap();
      sentence_end(last);
    }
    text
  }

  fn formatted(&self, locale: Locale) -> Text {
    let author = self.primary_author(locale);
    let date = format!("({})", self.date(locale));
    let mut out = Text::default();
    let is_book = self.kind == b::DataSourceValues::Book;
    let is_report = self.kind == b::DataSourceValues::Report;
    let is_interview = self.kind == b::DataSourceValues::Interview;
    let titled_first = author.is_empty() && !is_report && !is_interview;
    let push_title = |out: &mut Text| {
      if is_book {
        let title = self.title(locale, false);
        if !title.runs.is_empty() {
          out.space();
        }
        for (text, italic) in title.runs {
          out.push(text, italic);
        }
      } else if self.kind == b::DataSourceValues::InternetSite && !self.title.is_empty() {
        // templateTV/templateCItalic wrap the title, then append an unstyled
        // sentence terminator. ElectronicSource uses plain titleDot instead.
        out.space();
        out.push(&self.title, locale == Locale::English);
        if !self.title.ends_with(['.', '?', '!']) {
          out.push(".", false);
        }
      } else {
        out.sentence(&self.title, is_report && locale == Locale::English);
      }
    };
    if titled_first {
      push_title(&mut out);
    } else {
      out.sentence(&author, false);
    }
    out.sentence(&date, false);
    if !titled_first {
      push_title(&mut out);
    }
    match self.kind {
      b::DataSourceValues::Interview => {
        if let Some(interviewers) = &self.interviewers {
          let people = interviewers.text(locale, true);
          if !people.is_empty() {
            let role = match (locale, interviewers.people.len()) {
              (Locale::Chinese, _) => "采访人",
              (Locale::English, 1) => "Interviewer",
              (Locale::English, _) => "Interviewers",
            };
            out.space();
            out.push(format!("({people}, {role})"), false);
          }
        }
      }
      b::DataSourceValues::BookSection => {
        let authors = self.book_authors.text(locale, true);
        if !authors.is_empty() || !self.book_title.is_empty() {
          out.space();
          out.push(
            if locale == Locale::Chinese {
              "出处 "
            } else {
              "In "
            },
            false,
          );
          if !authors.is_empty() {
            out.push(authors, false);
            if !self.book_title.is_empty() {
              out.push(", ", false);
            } else {
              out.push(".", false);
            }
          }
          for (text, italic) in self.title(locale, true).runs {
            out.push(text, italic);
          }
        }
      }
      b::DataSourceValues::JournalArticle => {
        let mut journal = self.journal.clone();
        if !self.volume.is_empty() {
          if !journal.is_empty() {
            journal.push_str(", ");
          }
          journal.push_str(&self.volume);
        }
        if !journal.is_empty() {
          out.space();
          out.push(journal, locale == Locale::English);
        }
        if !self.issue.is_empty() {
          out.push(format!("({})", self.issue), false);
        }
        if !self.pages.is_empty() {
          if !self.journal.is_empty() || !self.volume.is_empty() || !self.issue.is_empty() {
            out.push(", ", false);
          } else {
            out.space();
          }
          out.push(&self.pages, false);
        }
        if !self.journal.is_empty()
          || !self.volume.is_empty()
          || !self.issue.is_empty()
          || !self.pages.is_empty()
        {
          let (last, _) = out.runs.last_mut().unwrap();
          sentence_end(last);
        }
      }
      _ => {}
    }
    let mut publication = self.city.clone();
    if !self.publisher.is_empty() {
      if !publication.is_empty() {
        publication.push_str(": ");
      }
      publication.push_str(&self.publisher);
    }
    out.sentence(&publication, false);
    if !self.doi.is_empty() {
      out.space();
      out.push(format!("doi:{}", self.doi), false);
    } else {
      let accessed = self.accessed_date(locale);
      if !accessed.is_empty() {
        out.space();
        out.push(
          if locale == Locale::Chinese {
            "检索日期: "
          } else {
            "Retrieved "
          },
          false,
        );
        out.push(accessed, false);
        if !self.url.is_empty() {
          out.push(
            if locale == Locale::Chinese {
              "，来源: "
            } else {
              ", from "
            },
            false,
          );
          out.push(&self.url, false);
        }
      } else if !self.url.is_empty() {
        out.space();
        out.push(
          if locale == Locale::Chinese {
            "检索来源: "
          } else {
            "Retrieved from "
          },
          false,
        );
        out.push(&self.url, false);
      }
    }
    out
  }
}

pub(super) fn refresh(
  entries: &[Entry],
  instruction: &str,
  styles: &StylesCatalog,
  context: &TextStyle,
  preceding_locale: LocaleContext,
) -> Option<Result> {
  let tokens = field_instruction_tokens(instruction);
  if !tokens.first()?.eq_ignore_ascii_case("BIBLIOGRAPHY") {
    return None;
  }
  let mut locale = preceding_locale.0.unwrap_or_else(|| {
    Locale::language(
      styles
        .locales
        .default_document_language()
        .or(context.language.as_deref())
        .unwrap_or("en-US"),
    )
  });
  let mut tag = None;
  let mut index = 1;
  while index < tokens.len() {
    match tokens[index].to_ascii_lowercase().as_str() {
      "\\l" => locale = Locale::lcid(tokens.get(index + 1)?),
      "\\m" => tag = Some(tokens.get(index + 1)?.as_str()),
      _ => return None,
    }
    index += 2;
  }
  let locale = locale?;
  let mut selected = entries
    .iter()
    .filter(|entry| tag.is_none_or(|tag| tag == entry.tag))
    .collect::<Vec<_>>();
  if selected.is_empty() {
    return None;
  }
  selected.sort_by_cached_key(|entry| entry.sort_key(locale));
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
  let mut format = styles.paragraph_format_with_base(style_id, ParagraphFormat::default());
  // APA's emitted HTML owns a half-inch hanging indent; line spacing and font
  // still inherit the Bibliography paragraph style, not the old field cache.
  format.indent_left_pt = 36.0;
  format.first_line_indent_pt = -36.0;
  format.indent_left_set = true;
  format.first_line_indent_set = true;
  format.indent_left_character_units = None;
  format.first_line_indent_character_units = None;
  let mut result = Result {
    inlines: Vec::new(),
    paragraphs: Vec::new(),
  };
  for (index, entry) in selected.into_iter().enumerate() {
    let locale = entry.locale.unwrap_or(locale);
    for (text, italic) in entry.formatted(locale).runs {
      let mut style = base.clone();
      style.italic |= italic;
      if italic {
        style.complex_italic = Some(true);
      }
      if locale == Locale::Chinese {
        style.wordprocessingml_font_hint =
          Some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia);
      }
      push_resolved_field_text(&mut result.inlines, text, style, None);
    }
    let mut mark = base.clone();
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

#[cfg(test)]
mod tests {
  use super::*;

  fn authors(count: usize) -> Authors {
    Authors {
      corporate: None,
      people: [
        "Adams", "Baker", "Clark", "Davis", "Evans", "Frost", "Green", "Hill", "Ives",
      ]
      .into_iter()
      .take(count)
      .map(|last| Person {
        last: last.into(),
        first: "Anne".into(),
        middle: "Beth".into(),
      })
      .collect(),
    }
  }

  fn plain(text: Text) -> String {
    text.runs.into_iter().map(|(text, _)| text).collect()
  }

  #[test]
  fn native_author_limit_and_secondary_names() {
    assert_eq!(
      authors(2).text(Locale::Chinese, false),
      "AdamsBethAnne, & BakerBethAnne"
    );
    assert_eq!(
      authors(2).text(Locale::English, false),
      "Adams, A. B., & Baker, A. B."
    );
    assert_eq!(
      authors(2).text(Locale::English, true),
      "A. B. Adams, & A. B. Baker"
    );
    assert!(
      authors(7)
        .text(Locale::English, false)
        .ends_with(", & Green, A. B.")
    );
    for (count, last) in [(8, "Hill"), (9, "Ives")] {
      let actual = authors(count).text(Locale::English, false);
      assert_eq!(
        actual,
        format!(
          "Adams, A. B., Baker, A. B., Clark, A. B., Davis, A. B., Evans, A. B., Frost, A. B., . . . {last}, A. B."
        )
      );
    }
  }

  #[test]
  fn native_book_section_and_journal_rich_runs() {
    let mut entry = Entry {
      kind: b::DataSourceValues::BookSection,
      authors: authors(2),
      book_authors: authors(2),
      title: "Chapter".into(),
      book_title: "Collected Studies".into(),
      year: "2021".into(),
      edition: "3".into(),
      pages: "21-35".into(),
      city: "Cambridge".into(),
      publisher: "Sample Press".into(),
      ..Default::default()
    };
    assert_eq!(
      plain(entry.formatted(Locale::Chinese)),
      "AdamsBethAnne, & BakerBethAnne. (2021). Chapter. 出处 AdamsBethAnne, & BakerBethAnne, Collected Studies (3 版本, 页 21-35). Cambridge: Sample Press."
    );
    let english = entry.formatted(Locale::English);
    assert_eq!(
      english
        .runs
        .iter()
        .filter(|(_, italic)| *italic)
        .map(|(text, _)| text.as_str())
        .collect::<Vec<_>>(),
      ["Collected Studies"]
    );
    assert_eq!(
      plain(english),
      "Adams, A. B., & Baker, A. B. (2021). Chapter. In A. B. Adams, & A. B. Baker, Collected Studies (3 ed., pp. 21-35). Cambridge: Sample Press."
    );
    entry.kind = b::DataSourceValues::JournalArticle;
    entry.journal = "Research Journal".into();
    entry.volume = "7".into();
    entry.issue = "2".into();
    entry.city.clear();
    entry.publisher.clear();
    entry.doi = "10.1234/example".into();
    entry.url = "https://example.org/resource".into();
    let english = entry.formatted(Locale::English);
    assert_eq!(
      english
        .runs
        .iter()
        .filter(|(_, italic)| *italic)
        .map(|(text, _)| text.as_str())
        .collect::<Vec<_>>(),
      ["Research Journal, 7"]
    );
    assert_eq!(
      plain(english),
      "Adams, A. B., & Baker, A. B. (2021). Chapter. Research Journal, 7(2), 21-35. doi:10.1234/example"
    );
  }

  #[test]
  fn native_missing_author_date_and_sorting() {
    let mut entry = Entry {
      kind: b::DataSourceValues::ElectronicSource,
      title: "Resource".into(),
      year: "2021".into(),
      month: "October".into(),
      day: "3".into(),
      url: "https://example.org/resource".into(),
      ..Default::default()
    };
    assert_eq!(
      plain(entry.formatted(Locale::English)),
      "Resource. (2021, October 3). Retrieved from https://example.org/resource"
    );
    assert_eq!(
      plain(entry.formatted(Locale::Chinese)),
      "Resource. (2021年October月3日). 检索来源: https://example.org/resource"
    );
    entry.month.clear();
    assert_eq!(entry.date(Locale::English), "2021");
    entry.authors = authors(2);
    let dated_key = entry.sort_key(Locale::English);
    entry.year.clear();
    assert_eq!(entry.date(Locale::Chinese), "无日期");
    assert_eq!(entry.date(Locale::English), "n.d.");
    assert!(dated_key < entry.sort_key(Locale::English));
    entry.authors = Authors::default();
    entry.kind = b::DataSourceValues::Report;
    entry.url.clear();
    assert_eq!(plain(entry.formatted(Locale::English)), "(n.d.). Resource.");
  }

  #[test]
  fn native_interview_roles_override_generic_author_and_title_fallback() {
    let mut entry = Entry {
      kind: b::DataSourceValues::Interview,
      authors: authors(2),
      title: "Interview".into(),
      year: "2021".into(),
      month: "October".into(),
      day: "3".into(),
      ..Default::default()
    };
    assert_eq!(
      plain(entry.formatted(Locale::English)),
      "(2021, October 3). Interview."
    );
    assert_eq!(entry.sort_key(Locale::English), "interview 2021");
    entry.interviewees = Some(authors(1));
    entry.interviewers = Some(authors(1));
    let formatted = entry.formatted(Locale::English);
    assert!(formatted.runs.iter().all(|(_, italic)| !italic));
    assert_eq!(
      plain(formatted),
      "Adams, A. B. (2021, October 3). Interview. (A. B. Adams, Interviewer)"
    );
    assert_eq!(
      entry.sort_key(Locale::English),
      "adams, a. b. 2021 interview"
    );
    entry.interviewers = Some(authors(2));
    assert_eq!(
      plain(entry.formatted(Locale::Chinese)),
      "AdamsBethAnne. (2021年October月3日). Interview. (AdamsBethAnne, & BakerBethAnne, 采访人)"
    );
    assert!(
      plain(entry.formatted(Locale::English))
        .ends_with("(A. B. Adams, & A. B. Baker, Interviewers)")
    );
    entry.title.clear();
    entry.interviewers = None;
    assert_eq!(
      plain(entry.formatted(Locale::English)),
      "Adams, A. B. (2021, October 3)."
    );
  }

  #[test]
  fn native_accessed_date_requires_year_and_month_before_day() {
    for (locale, expected) in [
      (
        Locale::English,
        [
          "",
          "2022",
          "",
          "November 2022",
          "",
          "2022",
          "",
          "November 17, 2022",
        ],
      ),
      (
        Locale::Chinese,
        [
          "",
          "2022",
          "",
          "2022年November月",
          "",
          "2022",
          "",
          "2022年November月17日",
        ],
      ),
    ] {
      for (mask, date) in expected.into_iter().enumerate() {
        let mut entry = Entry {
          kind: b::DataSourceValues::InternetSite,
          accessed: WebsiteDate {
            year: (mask & 1 != 0).then(|| "2022".into()),
            month: (mask & 2 != 0).then(|| "November".into()),
            day: (mask & 4 != 0).then(|| "17".into()),
          },
          ..Default::default()
        };
        assert_eq!(entry.accessed_date(locale), date);
        for url in ["", "https://example.org"] {
          entry.url = url.into();
          let suffix = match (locale, date.is_empty(), url.is_empty()) {
            (_, true, true) => String::new(),
            (Locale::English, true, false) => format!(" Retrieved from {url}"),
            (Locale::Chinese, true, false) => format!(" 检索来源: {url}"),
            (Locale::English, false, true) => format!(" Retrieved {date}"),
            (Locale::Chinese, false, true) => format!(" 检索日期: {date}"),
            (Locale::English, false, false) => format!(" Retrieved {date}, from {url}"),
            (Locale::Chinese, false, false) => format!(" 检索日期: {date}，来源: {url}"),
          };
          let prefix = if locale == Locale::English {
            "(n.d.)."
          } else {
            "(无日期)."
          };
          assert_eq!(plain(entry.formatted(locale)), format!("{prefix}{suffix}"));
        }
      }
    }
  }

  #[test]
  fn native_internet_title_has_distinct_rich_boundary() {
    let mut entry = Entry {
      kind: b::DataSourceValues::InternetSite,
      title: "Website".into(),
      ..Default::default()
    };
    assert_eq!(
      entry.formatted(Locale::English).runs,
      [("Website".into(), true), (". (n.d.).".into(), false)]
    );
    assert_eq!(
      entry.formatted(Locale::Chinese).runs,
      [("Website. (无日期).".into(), false)]
    );
    entry.kind = b::DataSourceValues::ElectronicSource;
    assert_eq!(
      entry.formatted(Locale::English).runs,
      [("Website. (n.d.).".into(), false)]
    );
  }

  #[test]
  fn interview_roles_parse_without_accepting_unsupported_source_profiles() {
    let xml = r#"<b:Sources xmlns:b="http://schemas.openxmlformats.org/officeDocument/2006/bibliography" SelectedStyle="\APASixthEditionOfficeOnline.xsl"><b:Source><b:Tag>I</b:Tag><b:SourceType>Interview</b:SourceType><b:Author><b:Author><b:Corporate>Ignored</b:Corporate></b:Author><b:Interviewee><b:NameList><b:Person><b:Last>Smith</b:Last></b:Person></b:NameList></b:Interviewee><b:Interviewer><b:NameList><b:Person><b:Last>Jones</b:Last></b:Person></b:NameList></b:Interviewer></b:Author><b:Title>Interview</b:Title></b:Source></b:Sources>"#;
    let catalog = Catalog::parse(xml).unwrap();
    assert_eq!(
      plain(catalog.apa[0].formatted(Locale::English)),
      "Smith. (n.d.). Interview. (Jones, Interviewer)"
    );
    for other in [
      xml.replace(">Interview<", ">Book<"),
      xml.replace("Interviewer", "Editor"),
      xml.replace("APASixthEditionOfficeOnline", "IEEE2006OfficeOnline"),
      xml.replace("</b:Source>", "<b:Pages>4-7</b:Pages></b:Source>"),
      xml.replace(
        "</b:Source>",
        "<b:YearAccessed>2022</b:YearAccessed></b:Source>",
      ),
      xml.replace("</b:Interviewer>", "text</b:Interviewer>"),
    ] {
      assert!(Catalog::parse(&other).is_none(), "{other}");
    }
    let start = xml.find("<b:Author>").unwrap();
    let end = xml.find("<b:Title>").unwrap();
    let website = format!("{}{}", &xml[..start], &xml[end..])
      .replace(">Interview<", ">InternetSite<")
      .replace("</b:Source>", "<b:YearAccessed>2022</b:YearAccessed><b:MonthAccessed>November</b:MonthAccessed><b:DayAccessed>17</b:DayAccessed></b:Source>");
    let catalog = Catalog::parse(&website).unwrap();
    assert_eq!(
      catalog.apa[0].accessed_date(Locale::English),
      "November 17, 2022"
    );
    assert!(Catalog::parse(&website.replace(">InternetSite<", ">Book<")).is_none());
  }

  #[test]
  fn doi_extension_and_unhandled_metadata_preserve_cache() {
    let source = r#"<b:Source><b:Tag>Journal</b:Tag><b:SourceType>JournalArticle</b:SourceType><b:Title>Research</b:Title><b:DOI>10.1234/a&amp;b</b:DOI></b:Source>"#;
    let collection = |style: &str, source: &str| {
      format!(
        r#"<b:Sources xmlns:b="http://schemas.openxmlformats.org/officeDocument/2006/bibliography" SelectedStyle="\{style}.xsl">{source}</b:Sources>"#
      )
    };
    let xml = collection("APASixthEditionOfficeOnline", source);
    let catalog = Catalog::parse(&xml).unwrap();
    assert_eq!(catalog.apa[0].doi, "10.1234/a&b");
    assert!(
      Catalog::parse(&xml.replace(
        "</b:Source>",
        "<b:ProductionCompany>Company</b:ProductionCompany></b:Source>"
      ))
      .is_none()
    );
    assert!(
      Catalog::parse(&xml.replace("</b:Source>", "<b:DOI>duplicate</b:DOI></b:Source>")).is_none()
    );
    assert!(
      Catalog::parse(&xml.replace("</b:Source>", "<b:LCID>1031</b:LCID></b:Source>")).is_none()
    );
    assert!(Catalog::parse(&collection("GB", &source.replace("JournalArticle", "Book"))).is_none());
    assert!(
      Catalog::parse(&collection(
        "IEEE2006OfficeOnline",
        &source.replace("JournalArticle", "InternetSite")
      ))
      .is_none()
    );
  }

  #[test]
  fn native_preceding_text_locale_ignores_blank_runs_and_marks() {
    let catalog = Catalog {
      apa: vec![Entry::default()],
      ..Default::default()
    };
    let run = |text: &str, language: &str| {
      let mut inlines = Vec::new();
      push_resolved_field_text(
        &mut inlines,
        text.into(),
        TextStyle {
          language: Some(language.into()),
          ..Default::default()
        },
        None,
      );
      inlines
    };
    let chinese = catalog.preceding_locale(&run("PRIOR", "zh-CN"), LocaleContext::default());
    assert_eq!(chinese.0, Some(Some(Locale::Chinese)));
    assert_eq!(
      catalog.preceding_locale(&run(" \t", "en-US"), chinese).0,
      chinese.0
    );
    assert_eq!(catalog.preceding_locale(&[], chinese).0, chinese.0);
    let english = catalog.preceding_locale(&run("PREFIX ", "en-US"), chinese);
    assert_eq!(english.0, Some(Some(Locale::English)));
    assert_eq!(
      catalog.preceding_locale(&run(" \t", "zh-CN"), english).0,
      english.0
    );
    let unknown = catalog.preceding_locale(&run("Text", "de-DE"), english);
    assert_eq!(unknown.0, Some(None));
    let styles = StylesCatalog::default();
    assert!(
      refresh(
        &catalog.apa,
        "BIBLIOGRAPHY",
        &styles,
        &TextStyle::default(),
        unknown
      )
      .is_none()
    );
    assert!(
      refresh(
        &catalog.apa,
        r"BIBLIOGRAPHY \l 1033",
        &styles,
        &TextStyle::default(),
        unknown
      )
      .is_some()
    );
  }
}
